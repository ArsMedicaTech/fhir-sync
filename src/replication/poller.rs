//! Per-link HAPI `_history` poller: paging, filtering, echo suppression,
//! and retry coordination.

use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use serde_json::Value;
use tokio::sync::Notify;
use tracing::{info, warn};

use tokio::sync::mpsc;

use crate::config::{Config, ReplicationLink, ReplicationMode, ReplicationNode};
use crate::dispatch::DispatchNotification;

use super::conflict;
use super::counters::{spawn_reporter, Counters};
use super::token;
use super::util::{fabric_tag, load_checkpoint, parse_etag, save_checkpoint, token_for_node, write_dead_letter, DeadLetterRecord, LinkCheckpoint};
use super::writer::{ReplicateError, WriteResult};
use super::{writer, SharedState};

#[derive(Debug, Clone)]
pub(crate) enum HistoryOp {
    Create,
    Update,
    Delete,
}

#[derive(Debug, Clone)]
pub(crate) struct HistoryEvent {
    pub(crate) resource_type: String,
    pub(crate) id: String,
    pub(crate) version_id: String,
    pub(crate) last_updated: String,
    pub(crate) op: HistoryOp,
    pub(crate) resource: Option<Value>,
}

pub async fn run(
    client: reqwest::Client,
    cfg: Config,
    link: ReplicationLink,
    source_node: ReplicationNode,
    target_node: Option<ReplicationNode>,
    state: SharedState,
    poll_now: Arc<Notify>,
    dispatch_tx: Option<mpsc::Sender<DispatchNotification>>,
) -> anyhow::Result<()> {
    let checkpoint_path = format!("{}/replication/{}/checkpoint.json", cfg.replication.state_dir, link.name);
    let dead_letter_path = format!("{}/replication/{}/dead_letter.jsonl", cfg.replication.state_dir, link.name);
    let max_attempts = cfg.sync.retry_max_attempts.max(1);
    let base_ms = cfg.sync.retry_base_ms;

    let mut cp = load_checkpoint(&checkpoint_path);
    if cp.since.is_empty() {
        cp.since = "1900-01-01T00:00:00.000Z".to_string();
    }

    let token_provider = token::TokenProvider::new(client.clone());
    let counters = Arc::new(Counters::new());
    let _reporter = spawn_reporter(link.name.clone(), counters.clone());

    loop {
        match poll_one_cycle(
            &client,
            &cfg,
            &link,
            &source_node,
            target_node.as_ref(),
            &state,
            &counters,
            &mut cp,
            &checkpoint_path,
            &dead_letter_path,
            max_attempts,
            base_ms,
            &token_provider,
            dispatch_tx.as_ref(),
        )
        .await
        {
            Ok(processed) if processed > 0 => {
                info!("replication link {}: processed {} entries this cycle", link.name, processed);
            }
            Ok(_) => {}
            Err(e) => {
                warn!("replication link {}: poll cycle failed: {e:?}; retrying after interval", link.name);
            }
        }

        tokio::select! {
            _ = tokio::time::sleep(Duration::from_millis(cfg.replication.poll_interval_ms)) => {}
            _ = poll_now.notified() => {
                info!("replication link {}: poll woken by doorbell", link.name);
            }
        }
    }
}

async fn poll_one_cycle(
    client: &reqwest::Client,
    cfg: &Config,
    link: &ReplicationLink,
    source_node: &ReplicationNode,
    target_node: Option<&ReplicationNode>,
    state: &SharedState,
    counters: &Counters,
    cp: &mut LinkCheckpoint,
    checkpoint_path: &str,
    dead_letter_path: &str,
    max_attempts: u32,
    base_ms: u64,
    token_provider: &token::TokenProvider,
    dispatch_tx: Option<&mpsc::Sender<DispatchNotification>>,
) -> Result<usize> {
    let token = token_provider.token_for(source_node).await;
    let conflicts_path = format!("{}/replication/{}/conflicts.jsonl", cfg.replication.state_dir, link.name);
    let mut next_url = Some(build_history_url(&source_node.base_url, &cp.since, cfg.replication.page_size));
    let mut processed = 0;

    while let Some(url) = next_url {
        let mut req = client.get(&url).header("Accept", "application/fhir+json");
        if let Some(t) = &token {
            req = req.bearer_auth(t);
        }

        let resp = req
            .send()
            .await
            .with_context(|| format!("GET {url}"))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("HAPI history request failed ({status}): {text}");
        }

        let bundle: Value = resp.json().await?;
        next_url = next_page_url(&bundle, &source_node.base_url);

        let entries = bundle
            .get("entry")
            .and_then(Value::as_array)
            .map(|a| a.as_slice())
            .unwrap_or(&[]);

        for entry in entries {
            let Some(event) = parse_history_entry(entry) else { continue };
            counters.inc_seen();

            if !link.resources.iter().any(|r| r == &event.resource_type) {
                counters.inc_skipped_by_filter();
                continue;
            }

            if event.resource_type == "Provenance" || event.resource_type == "AuditEvent" {
                counters.inc_skipped_by_filter();
                continue;
            }

            if let Some(target_node) = target_node {
                if should_suppress(link, source_node, target_node, state, &event) {
                    counters.inc_suppressed();
                    continue;
                }
            }

            // Observe-only link: emit and advance. No target reads, no conflict
            // resolution, no writes.
            if target_node.is_none() {
                if let Some(tx) = dispatch_tx {
                    let notification = DispatchNotification::from_history_event(
                        &event,
                        &source_node.base_url,
                        &link.name,
                    );
                    if tx.try_send(notification).is_err() {
                        counters.inc_dispatch_dropped();
                        warn!(
                            "replication link {}: dispatch channel full, dropped {}/{}",
                            link.name, event.resource_type, event.id
                        );
                    }
                }
                cp.since = bump_past(&event.last_updated);
                processed += 1;
                continue;
            }

            let target_node = target_node.unwrap();
            match event.op {
                HistoryOp::Create | HistoryOp::Update => {
                    if let Some(resource) = &event.resource {
                        let recorded = cp
                            .last_versionids_seen
                            .get(&format!("{}/{}", event.resource_type, event.id))
                            .map(String::as_str);
                        if !conflict::check_and_resolve(
                            &client,
                            link,
                            target_node,
                            token_for_node(&target_node.token_env).as_deref(),
                            &event.resource_type,
                            &event.id,
                            &event.version_id,
                            resource,
                            recorded,
                            &conflicts_path,
                        )
                        .await
                        .map_err(|e| anyhow::anyhow!("{e:?}"))?
                        {
                            continue;
                        }
                        match replicate_with_retry(
                            &client,
                            link,
                            source_node,
                            target_node,
                            dead_letter_path,
                            counters,
                            resource,
                            &event.version_id,
                            max_attempts,
                            base_ms,
                        )
                        .await?
                        {
                            Some(result) => {
                                counters.inc_replicated();
                                let key = format!("{}/{}", event.resource_type, event.id);
                                cp.last_versionids_seen.insert(key, result.target_version.clone());
                                let echo_key = format!("{}/{}/{}", target_node.name, event.resource_type, result.target_id);
                                state.echo.lock().unwrap().insert(echo_key, result.target_version);
                            }
                            None => {}
                        }
                    } else {
                        warn!("replication link {}: upsert entry without resource for {}/{}", link.name, event.resource_type, event.id);
                    }
                }
                HistoryOp::Delete => {
                    match replicate_delete_with_retry(
                        &client,
                        link,
                        source_node,
                        target_node,
                        dead_letter_path,
                        counters,
                        &event.resource_type,
                        &event.id,
                        event.resource.as_ref(),
                        max_attempts,
                        base_ms,
                    )
                    .await
                    {
                        Ok(Some(())) => counters.inc_replicated(),
                        Ok(None) => {}
                        Err(e) => {
                            warn!("replication link {}: delete {}/{} failed after retries: {e:?}", link.name, event.resource_type, event.id);
                        }
                    }
                }
            }

            cp.since = bump_past(&event.last_updated);
            processed += 1;
        }

        save_checkpoint(checkpoint_path, cp)?;
    }

    Ok(processed)
}

/// HAPI's `_since` is inclusive. Nudge one millisecond past the last-seen
/// timestamp so the same resource isn't re-fetched on the next poll.
fn bump_past(ts: &str) -> String {
    match chrono::DateTime::parse_from_rfc3339(ts) {
        Ok(dt) => (dt + chrono::Duration::milliseconds(1)).to_rfc3339(),
        Err(_) => ts.to_string(),
    }
}

fn build_history_url(base_url: &str, since: &str, count: usize) -> String {
    format!(
        "{}/_history?_since={}&_count={}",
        base_url.trim_end_matches('/'),
        since,
        count
    )
}

fn next_page_url(bundle: &Value, base_url: &str) -> Option<String> {
    bundle
        .get("link")
        .and_then(Value::as_array)
        .and_then(|links| {
            links.iter().find_map(|l| {
                if l.get("relation").and_then(Value::as_str)? == "next" {
                    l.get("url").and_then(Value::as_str).map(String::from)
                } else {
                    None
                }
            })
        })
        .or_else(|| {
            // Some HAPI versions return a relative "next" URL.
            bundle
                .get("link")
                .and_then(Value::as_array)
                .and_then(|links| {
                    links.iter().find_map(|l| {
                        if l.get("relation").and_then(Value::as_str)? == "next" {
                            let rel = l.get("url").and_then(Value::as_str)?;
                            if rel.starts_with("http") {
                                Some(rel.to_string())
                            } else {
                                Some(format!("{}{}", base_url.trim_end_matches('/'), rel))
                            }
                        } else {
                            None
                        }
                    })
                })
        })
}

fn parse_history_entry(entry: &Value) -> Option<HistoryEvent> {
    let method = entry
        .get("request")
        .and_then(|r| r.get("method"))
        .and_then(Value::as_str)?;

    let op = match method {
        "POST" => HistoryOp::Create,
        "PUT" => HistoryOp::Update,
        "DELETE" => HistoryOp::Delete,
        _ => return None,
    };

    let resource = entry.get("resource").cloned();

    let (resource_type, id) = if let Some(r) = &resource {
        let rt = r.get("resourceType").and_then(Value::as_str)?.to_string();
        let id = r.get("id").and_then(Value::as_str)?.to_string();
        (rt, id)
    } else {
        let url = entry.get("request").and_then(|r| r.get("url")).and_then(Value::as_str)?;
        let mut parts = url.split('/').filter(|p| !p.is_empty());
        let rt = parts.next()?.to_string();
        let id = parts.next()?.to_string();
        (rt, id)
    };

    let version_id = if let Some(r) = &resource {
        r.get("meta")
            .and_then(|m| m.get("versionId"))
            .and_then(Value::as_str)
            .map(String::from)
    } else {
        None
    }
    .or_else(|| {
        entry
            .get("response")
            .and_then(|r| r.get("etag"))
            .and_then(Value::as_str)
            .and_then(parse_etag)
    })
    .unwrap_or_default();

    let last_updated = if let Some(r) = &resource {
        r.get("meta")
            .and_then(|m| m.get("lastUpdated"))
            .and_then(Value::as_str)
            .map(String::from)
    } else {
        None
    }
    .or_else(|| {
        entry
            .get("response")
            .and_then(|r| r.get("lastModified"))
            .and_then(Value::as_str)
            .map(String::from)
    })
    .unwrap_or_default();

    Some(HistoryEvent {
        resource_type,
        id,
        version_id,
        last_updated,
        op,
        resource,
    })
}

/// HAPI appends its request id as a URI fragment
/// (`urn:...:node-a#FRkkUAMqHdNorySm`), so compare the base only.
fn source_tag_matches(source: &str, tag: &str) -> bool {
    source.split('#').next().unwrap_or(source) == tag
}

fn should_suppress(
    _link: &ReplicationLink,
    source_node: &ReplicationNode,
    target_node: &ReplicationNode,
    state: &SharedState,
    event: &HistoryEvent,
) -> bool {
    // Primary: the entry was written by the reverse direction.
    let target_tag = fabric_tag(&target_node.name);
    if let Some(source) = event
        .resource
        .as_ref()
        .and_then(|r| r.get("meta"))
        .and_then(|m| m.get("source"))
        .and_then(Value::as_str)
    {
        if source_tag_matches(source, &target_tag) {
            return true;
        }
    }

    // Secondary: the versionId is one we produced on this source node.
    let echo_key = format!("{}/{}/{}", source_node.name, event.resource_type, event.id);
    if let Some(known) = state.echo.lock().unwrap().get(&echo_key) {
        if known == &event.version_id && !event.version_id.is_empty() {
            return true;
        }
    }

    false
}

async fn replicate_with_retry(
    client: &reqwest::Client,
    link: &ReplicationLink,
    source_node: &ReplicationNode,
    target_node: &ReplicationNode,
    dead_letter_path: &str,
    counters: &Counters,
    resource: &Value,
    version_id: &str,
    max_attempts: u32,
    base_ms: u64,
) -> Result<Option<WriteResult>> {
    let mut last_err = None;
    for attempt in 0..max_attempts {
        if attempt > 0 {
            counters.inc_retried();
        }
        match writer::upsert(
            client,
            link,
            source_node,
            target_node,
            dead_letter_path,
            token_for_node(&target_node.token_env).as_deref(),
            resource,
            version_id,
        )
        .await
        {
            Ok(result) => return Ok(result),
            Err(ReplicateError::Permanent(e)) => {
                warn!("replication link {}: permanent failure for {}/{}: {e}", link.name, resource["resourceType"].as_str().unwrap_or("?"), resource["id"].as_str().unwrap_or("?"));
                counters.inc_dead_lettered();
                return Ok(None);
            }
            Err(ReplicateError::Retryable(e)) => {
                last_err = Some(e);
                if attempt + 1 < max_attempts {
                    let delay = base_ms.saturating_mul(1u64 << attempt.min(10));
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                }
            }
        }
    }

    let err = last_err.unwrap_or_else(|| anyhow::anyhow!("upsert retry exhausted"));
    let resource_type = resource.get("resourceType").and_then(Value::as_str).unwrap_or("?");
    let id = resource.get("id").and_then(Value::as_str).unwrap_or("?");
    warn!("replication link {}: exhausted retries for {}/{}: {err:?}", link.name, resource_type, id);
    counters.inc_dead_lettered();
    write_dead_letter(
        dead_letter_path,
        &DeadLetterRecord {
            timestamp: chrono::Utc::now().to_rfc3339(),
            link: &link.name,
            resource_type,
            id,
            reason: "retry_exhausted",
            version_id: Some(version_id),
            error: Some(err.to_string()),
        },
    )?;
    Ok(None)
}

async fn replicate_delete_with_retry(
    client: &reqwest::Client,
    link: &ReplicationLink,
    source_node: &ReplicationNode,
    target_node: &ReplicationNode,
    dead_letter_path: &str,
    counters: &Counters,
    resource_type: &str,
    id: &str,
    resource_stub: Option<&Value>,
    max_attempts: u32,
    base_ms: u64,
) -> Result<Option<()>> {
    let mut last_err = None;
    for attempt in 0..max_attempts {
        if attempt > 0 {
            counters.inc_retried();
        }
        match writer::delete(
            client,
            link,
            source_node,
            target_node,
            dead_letter_path,
            token_for_node(&target_node.token_env).as_deref(),
            resource_type,
            id,
            resource_stub,
        )
        .await
        {
            Ok(Some(())) => return Ok(Some(())),
            Ok(None) => return Ok(None),
            Err(ReplicateError::Permanent(e)) => {
                warn!("replication link {}: permanent delete failure for {}/{}: {e}", link.name, resource_type, id);
                counters.inc_dead_lettered();
                return Ok(None);
            }
            Err(ReplicateError::Retryable(e)) => {
                last_err = Some(e);
                if attempt + 1 < max_attempts {
                    let delay = base_ms.saturating_mul(1u64 << attempt.min(10));
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                }
            }
        }
    }

    let err = last_err.unwrap_or_else(|| anyhow::anyhow!("delete retry exhausted"));
    warn!("replication link {}: exhausted delete retries for {}/{}: {err:?}", link.name, resource_type, id);
    write_dead_letter(
        dead_letter_path,
        &DeadLetterRecord {
            timestamp: chrono::Utc::now().to_rfc3339(),
            link: &link.name,
            resource_type,
            id,
            reason: "retry_exhausted",
            version_id: None,
            error: Some(err.to_string()),
        },
    )?;
    counters.inc_dead_lettered();
    Ok(None)
}

#[cfg(test)]
mod suppression_tests {
    use super::source_tag_matches;

    #[test]
    fn matches_hapi_source_with_request_id_fragment() {
        // Observed from hapiproject/hapi:v8.10.0-3
        assert!(source_tag_matches(
            "urn:arsmedicatech:fhir-sync:node-a#FRkkUAMqHdNorySm",
            "urn:arsmedicatech:fhir-sync:node-a"
        ));
    }

    #[test]
    fn matches_bare_tag_without_fragment() {
        assert!(source_tag_matches(
            "urn:arsmedicatech:fhir-sync:node-a",
            "urn:arsmedicatech:fhir-sync:node-a"
        ));
    }

    #[test]
    fn does_not_match_other_node() {
        assert!(!source_tag_matches(
            "urn:arsmedicatech:fhir-sync:node-b#FRkkUAMqHdNorySm",
            "urn:arsmedicatech:fhir-sync:node-a"
        ));
    }

    #[test]
    fn does_not_match_foreign_source() {
        assert!(!source_tag_matches(
            "http://some-other-system/ehr#abc",
            "urn:arsmedicatech:fhir-sync:node-a"
        ));
    }
}
