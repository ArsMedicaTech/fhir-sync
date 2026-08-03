//! Conflict detection for HAPI → HAPI replication.

use anyhow::Context;
use serde_json::Value;
use tracing::warn;

use crate::config::{ConflictPolicy, ReplicationLink, ReplicationMode, ReplicationNode};

use super::util::{find_identifier_value, write_conflict, ConflictRecord};
use super::writer::ReplicateError;

/// Checks whether the target has been modified independently since the last
/// replicated version. Returns `true` if the write may proceed, `false` if it
/// was dead-lettered under the `dead_letter` policy.
pub async fn check_and_resolve(
    client: &reqwest::Client,
    link: &ReplicationLink,
    target_node: &ReplicationNode,
    token: Option<&str>,
    resource_type: &str,
    source_id: &str,
    source_version_id: &str,
    source_resource: &Value,
    recorded_version: Option<&str>,
    conflicts_path: &str,
) -> Result<bool, ReplicateError> {
    let (target_id, target_resource) = match link.mode {
        ReplicationMode::Mirror => {
            let url = format!(
                "{}/{}/{}",
                target_node.base_url.trim_end_matches('/'),
                resource_type,
                source_id
            );
            fetch_target(client, &url, token).await?
        }
        ReplicationMode::Federate => {
            let system = link
                .federate_identifier_system
                .as_ref()
                .expect("federate mode validated");
            let value = match find_identifier_value(source_resource, system) {
                Some(v) => v,
                None => {
                    // The writer will dead-letter for no_federation_identifier.
                    return Ok(true);
                }
            };
            let base = format!("{}/{}", target_node.base_url.trim_end_matches('/'), resource_type);
            let ident = format!("{}|{}", system, value);
            let mut req = client.get(&base).query(&[
                ("identifier", ident.as_str()),
                ("_summary", "true"),
                ("_count", "1"),
            ]);
            if let Some(t) = token {
                req = req.bearer_auth(t);
            }
            let resp = req
                .send()
                .await
                .with_context(|| format!("conflict search {base}"))?;
            if !resp.status().is_success() {
                let status = resp.status();
                let text = resp.text().await.unwrap_or_default();
                return Err(ReplicateError::Retryable(anyhow::anyhow!(
                    "conflict search failed ({status}): {text}"
                )));
            }
            let bundle: Value = resp.json::<Value>().await.context("parsing conflict search bundle")?;
            bundle
                .get("entry")
                .and_then(Value::as_array)
                .and_then(|e| e.first())
                .and_then(|e| e.get("resource"))
                .and_then(Value::as_object)
                .and_then(|r| {
                    let id = r.get("id").and_then(Value::as_str)?;
                    Some((id.to_string(), Value::Object(r.clone())))
                })
                .unwrap_or((String::new(), Value::Null))
        }
    };

    if target_id.is_empty() || target_resource.is_null() {
        return Ok(true);
    }

    let target_version = target_resource
        .get("meta")
        .and_then(|m| m.get("versionId"))
        .and_then(Value::as_str)
        .unwrap_or("");
    let target_source = target_resource
        .get("meta")
        .and_then(|m| m.get("source"))
        .and_then(Value::as_str)
        .unwrap_or("");
    let is_fabric = target_source.starts_with("urn:arsmedicatech:fhir-sync:");

    // No conflict if the current target version is the one we last wrote,
    // or if it is a version written by this fabric (another link from us).
    if let Some(recorded) = recorded_version {
        if target_version == recorded {
            return Ok(true);
        }
    }
    if is_fabric {
        return Ok(true);
    }

    // Conflict: independent edit on target.
    warn!(
        "replication link {}: conflict detected for {}/{} (target version {})",
        link.name, resource_type, target_id, target_version
    );

    if matches!(link.conflict_policy, ConflictPolicy::SourceWins) {
        warn!("replication link {}: conflict policy is source_wins; overwriting {}/{}", link.name, resource_type, target_id);
        return Ok(true);
    }

    record_conflict(
        link,
        resource_type,
        source_id,
        source_version_id,
        &target_id,
        source_resource,
        &target_resource,
        conflicts_path,
    )?;
    Ok(false)
}

async fn fetch_target(
    client: &reqwest::Client,
    url: &str,
    token: Option<&str>,
) -> Result<(String, Value), ReplicateError> {
    let mut req = client.get(url).header("Accept", "application/fhir+json");
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }
    let resp = req
        .send()
        .await
        .with_context(|| format!("conflict GET {url}"))?;
    let status = resp.status();
    if status.as_u16() == 404 || status.as_u16() == 410 {
        return Ok((String::new(), Value::Null));
    }
    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        return Err(ReplicateError::Retryable(anyhow::anyhow!(
            "conflict GET failed ({status}): {text}"
        )));
    }
    let resource: Value = resp.json::<Value>().await.context("parsing target resource")?;
    let id = resource
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    Ok((id, resource))
}

fn record_conflict(
    link: &ReplicationLink,
    resource_type: &str,
    source_id: &str,
    source_version_id: &str,
    target_id: &str,
    source_resource: &Value,
    target_resource: &Value,
    path: &str,
) -> Result<(), ReplicateError> {
    let source_meta = source_resource.get("meta").cloned();
    let target_meta = target_resource.get("meta").cloned();
    let identifiers = source_resource.get("identifier").cloned().unwrap_or(Value::Null);

    write_conflict(
        path,
        &ConflictRecord {
            timestamp: chrono::Utc::now().to_rfc3339(),
            link: &link.name,
            resource_type,
            source_id,
            target_id,
            source_version_id,
            source_meta,
            target_meta,
            policy: match link.conflict_policy {
                ConflictPolicy::DeadLetter => "dead_letter",
                ConflictPolicy::SourceWins => "source_wins",
            },
            identifiers,
        },
    )
    .map_err(|e| ReplicateError::Retryable(anyhow::anyhow!("{e:?}")))?;
    Ok(())
}
