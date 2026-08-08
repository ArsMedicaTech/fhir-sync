//! HAPI `_history` poller for the AMT → Oscar write-back path.
//!
//! Polls the local HAPI server for `Patient`, `Appointment`, and
//! `DocumentReference` changes, suppresses Oscar-originated and write-back
//! echoes using `meta.source`, maps AMT-authored resources to Oscar rows, and
//! writes them through `OscarSink`.
//!
//! Each successful write is committed only after the generated Oscar
//! identifier has been written back to HAPI, so an orphan Oscar row cannot be
//! created by a failed HAPI update.

use std::collections::HashMap;
use std::time::Duration;

use anyhow::{Context, Result};
use chrono::SecondsFormat;
use chrono_tz::Tz;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use tokio::time::sleep;
use tracing::{info, warn};

use crate::auth::TokenProvider;
use crate::config::{Config, FhirConfig};
use crate::writeback::authorship::{is_oscar_origin, is_writeback_source, WRITE_BACK_SOURCE};
use crate::writeback::deadletter::{write as write_dead_letter, DeadLetter};
use crate::writeback::mappers::{
    fhir_appointment_to_row, fhir_document_reference_to_row, fhir_patient_to_row,
    fhir_service_request_to_row, MappingError,
};
use crate::writeback::oscar_sink::{OscarSink, OscarTx};

#[derive(Debug, Default, Serialize, Deserialize)]
struct WritebackCheckpoint {
    since: String,
    #[serde(default)]
    last_versionids_seen: HashMap<String, String>,
}

#[derive(Debug)]
enum HistoryOp {
    Create,
    Update,
    Delete,
}

#[derive(Debug)]
struct HistoryEvent {
    resource_type: String,
    id: String,
    version_id: String,
    last_updated: String,
    op: HistoryOp,
    resource: Value,
}

pub async fn run(cfg: Config) -> anyhow::Result<()> {
    let client = Client::new();
    let sink = OscarSink::new(&cfg.writeback.db, &cfg.writeback.sentinel_update_user);
    let tz: Tz = cfg
        .oscar
        .timezone
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("[oscar] timezone is required for writeback"))?
        .parse()
        .context("parsing [oscar].timezone")?;
    let hapi_base = cfg.fhir.base_url.trim_end_matches('/').to_string();

    let state_dir = &cfg.writeback.state_dir;
    tokio::fs::create_dir_all(state_dir).await.ok();
    let checkpoint_path = format!("{state_dir}/writeback_checkpoint.json");
    let mut checkpoint = load_checkpoint(&checkpoint_path).await;
    if checkpoint.since.is_empty() {
        // A fresh checkpoint starts at "now" so pre-existing HAPI resources are
        // not retroactively treated as AMT-authored writes on first run.
        checkpoint.since = chrono::Utc::now()
            .to_rfc3339_opts(SecondsFormat::Millis, true);
    }

    let poll_interval = Duration::from_millis(cfg.writeback.poll_interval_ms);
    let page_size = cfg.writeback.page_size;

    loop {
        match poll_one_cycle(
            &client,
            &cfg,
            &hapi_base,
            &sink,
            &tz,
            &mut checkpoint,
            &checkpoint_path,
            page_size,
        )
        .await
        {
            Ok(processed) if processed > 0 => {
                info!("writeback: processed {processed} resources this cycle");
            }
            Ok(_) => {}
            Err(e) => {
                warn!("writeback poll cycle failed: {e:?}; retrying after interval");
            }
        }
        sleep(poll_interval).await;
    }
}

async fn poll_one_cycle(
    client: &Client,
    cfg: &Config,
    hapi_base: &str,
    sink: &OscarSink,
    tz: &Tz,
    checkpoint: &mut WritebackCheckpoint,
    checkpoint_path: &str,
    page_size: usize,
) -> Result<usize> {
    let token = hapi_token(client, &cfg.fhir).await?;
    let mut next_url = Some(build_history_url(hapi_base, &checkpoint.since, page_size));
    let mut processed = 0;

    while let Some(url) = next_url {
        let mut req = client.get(&url).header("Accept", "application/fhir+json");
        if let Some(t) = &token {
            req = req.bearer_auth(t);
        }

        let resp = req.send().await.with_context(|| format!("GET {url}"))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("HAPI history request failed ({status}): {text}");
        }

        let bundle: Value = resp.json().await?;
        let entries = bundle
            .get("entry")
            .and_then(Value::as_array)
            .map(|a| a.as_slice())
            .unwrap_or(&[]);

        for entry in entries {
            let Some(event) = parse_history_entry(entry) else { continue };

            if !cfg
                .writeback
                .hapi_resource_types
                .iter()
                .any(|r| r == &event.resource_type)
            {
                continue;
            }

            if is_oscar_origin(&event.resource) {
                info!(
                    "writeback: skipping Oscar-originated/sourceless {} {}",
                    event.resource_type, event.id
                );
                continue;
            }

            if is_writeback_source(&event.resource) {
                info!(
                    "writeback: skipping write-back echo {} {}",
                    event.resource_type, event.id
                );
                continue;
            }

            let key = format!("{}/{}", event.resource_type, event.id);
            if checkpoint
                .last_versionids_seen
                .get(&key)
                .map(|v| v == &event.version_id)
                .unwrap_or(false)
            {
                info!(
                    "writeback: skipping already-processed {} {} version {}",
                    event.resource_type, event.id, event.version_id
                );
                continue;
            }

            match event.op {
                HistoryOp::Delete => {
                    warn!(
                        "writeback: ignoring DELETE for {}/{} (no destructive writes)",
                        event.resource_type, event.id
                    );
                    continue;
                }
                HistoryOp::Create | HistoryOp::Update => {}
            }

            let mut tx = sink.begin().await?;
            if let Err(e) = process_resource(client, cfg, token.as_deref(), &mut tx, tz, &event).await {
                warn!(
                    "writeback: dead-lettering {}/{}: {e:?}",
                    event.resource_type, event.id
                );
                tx.rollback().await.ok();
                let dl = DeadLetter {
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    id: event.id.clone(),
                    resource_type: event.resource_type.clone(),
                    version_id: Some(event.version_id.clone()),
                    error: format!("{e}"),
                    payload: Some(event.resource.clone()),
                };
                write_dead_letter(&cfg.writeback.dead_letter_path, &dl).await?;
            } else {
                tx.commit().await?;
                info!("writeback: committed {}/{}" , event.resource_type, event.id);
                checkpoint
                    .last_versionids_seen
                    .insert(key, event.version_id.clone());
            }

            checkpoint.since = event.last_updated.clone();
            save_checkpoint(checkpoint_path, checkpoint).await?;
            processed += 1;
        }

        next_url = next_page_url(&bundle);
    }

    Ok(processed)
}

async fn hapi_token(client: &Client, fhir: &FhirConfig) -> Result<Option<String>> {
    if let Some(kc) = &fhir.keycloak {
        let provider = TokenProvider::new(kc, client.clone())?;
        return Ok(Some(provider.token().await?));
    }
    if let Some(env) = &fhir.token_env {
        return Ok(std::env::var(env).ok());
    }
    Ok(None)
}

fn build_history_url(base: &str, since: &str, _count: usize) -> String {
    format!("{base}/_history?_since={since}&_count={_count}")
}

fn next_page_url(bundle: &Value) -> Option<String> {
    let links = bundle.get("link").and_then(Value::as_array)?;
    for link in links {
        if link.get("relation").and_then(Value::as_str) == Some("next") {
            return link.get("url").and_then(Value::as_str).map(String::from);
        }
    }
    None
}

fn parse_history_entry(entry: &Value) -> Option<HistoryEvent> {
    let resource = entry.get("resource").cloned()?;
    let request = entry.get("request")?;
    let method = request.get("method")?.as_str()?;
    let op = match method {
        "POST" => HistoryOp::Create,
        "PUT" | "PATCH" => HistoryOp::Update,
        "DELETE" => HistoryOp::Delete,
        _ => return None,
    };

    let resource_type = resource.get("resourceType")?.as_str()?.to_string();
    let id = resource.get("id")?.as_str()?.to_string();
    let version_id = resource
        .get("meta")
        .and_then(|m| m.get("versionId"))
        .and_then(Value::as_str)?
        .to_string();
    let last_updated = resource
        .get("meta")
        .and_then(|m| m.get("lastUpdated"))
        .and_then(Value::as_str)?
        .to_string();

    Some(HistoryEvent {
        resource_type,
        id,
        version_id,
        last_updated,
        op,
        resource,
    })
}

async fn process_resource(
    client: &Client,
    cfg: &Config,
    token: Option<&str>,
    tx: &mut OscarTx,
    tz: &Tz,
    event: &HistoryEvent,
) -> Result<()> {
    match event.resource_type.as_str() {
        "Patient" => {
            let (existing_id, row) = match fhir_patient_to_row(
                &event.resource,
                &cfg.fhir.oscar_demographic_system,
            ) {
                Ok(v) => v,
                Err(MappingError::MergeTombstone) => {
                    info!(
                        "writeback: skipping merge tombstone Patient {}",
                        event.id
                    );
                    return Ok(());
                }
                Err(e) => return Err(anyhow::anyhow!("{e}")),
            };
            let new_id = tx.write_demographic(existing_id.as_deref(), &row, tz).await?;
            if existing_id.is_none() {
                hapi_update_identifier(client, cfg, token, event, &new_id).await?;
            }
        }
        "Appointment" => {
            let mut demographic_no: Option<String> = None;
            let mut provider_no: Option<String> = None;
            if let Some(parts) = event.resource.get("participant").and_then(Value::as_array) {
                for p in parts {
                    if let Some(actor_ref) = p
                        .get("actor")
                        .and_then(|v| v.get("reference"))
                        .and_then(Value::as_str)
                    {
                        if demographic_no.is_none() {
                            demographic_no = resolve_identifier(
                                client,
                                cfg,
                                token,
                                actor_ref,
                                &cfg.fhir.oscar_demographic_system,
                            )
                            .await
                            .map_err(|e| anyhow::anyhow!("{e}"))?;
                        }
                        if provider_no.is_none() {
                            provider_no = resolve_identifier(
                                client,
                                cfg,
                                token,
                                actor_ref,
                                &cfg.fhir.oscar_provider_system,
                            )
                            .await
                            .map_err(|e| anyhow::anyhow!("{e}"))?;
                        }
                    }
                }
            }
            let (existing_id, row) = fhir_appointment_to_row(
                &event.resource,
                demographic_no,
                provider_no,
                &cfg.fhir.oscar_appointment_system,
                &cfg.writeback.appointment_status_map,
                tz,
            )
            .map_err(|e| anyhow::anyhow!("{e}"))?;
            let new_id = tx.write_appointment(existing_id.as_deref(), &row).await?;
            if existing_id.is_none() {
                hapi_update_identifier(client, cfg, token, event, &new_id).await?;
            }
        }
        "DocumentReference" => {
            let subject_ref = event
                .resource
                .get("subject")
                .and_then(|v| v.get("reference"))
                .and_then(Value::as_str)
                .ok_or_else(|| anyhow::anyhow!("missing subject reference"))?;
            let demographic_no = resolve_identifier(
                client,
                cfg,
                token,
                subject_ref,
                &cfg.fhir.oscar_demographic_system,
            )
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
            let author_ref = event
                .resource
                .get("author")
                .and_then(Value::as_array)
                .and_then(|a| a.first())
                .and_then(|v| v.get("reference"))
                .and_then(Value::as_str);
            let provider_no = if let Some(r) = author_ref {
                resolve_identifier(client, cfg, token, r, &cfg.fhir.oscar_provider_system)
                    .await
                    .map_err(|e| anyhow::anyhow!("{e}"))?
            } else {
                None
            };
            let authenticator_ref = event
                .resource
                .get("authenticator")
                .and_then(|v| v.get("reference"))
                .and_then(Value::as_str);
            let signing_provider_no = if let Some(r) = authenticator_ref {
                resolve_identifier(client, cfg, token, r, &cfg.fhir.oscar_provider_system)
                    .await
                    .map_err(|e| anyhow::anyhow!("{e}"))?
            } else {
                None
            };
            let encounter_ref = event
                .resource
                .get("context")
                .and_then(|c| c.get("encounter"))
                .and_then(Value::as_array)
                .and_then(|a| a.first())
                .and_then(|v| v.get("reference"))
                .and_then(Value::as_str);
            let appointment_no = if let Some(r) = encounter_ref {
                resolve_identifier(client, cfg, token, r, &cfg.fhir.oscar_appointment_system)
                    .await
                    .map_err(|e| anyhow::anyhow!("{e}"))?
            } else {
                None
            };
            let (existing_id, row) = fhir_document_reference_to_row(
                &event.resource,
                &cfg.fhir.oscar_note_document_system,
                demographic_no,
                provider_no,
                signing_provider_no,
                appointment_no,
                tz,
            )
            .map_err(|e| anyhow::anyhow!("{e}"))?;
            let new_id = tx.write_note(existing_id.as_deref(), &row, tz).await?;
            if existing_id.is_none() {
                hapi_update_identifier(client, cfg, token, event, &new_id).await?;
            }
        }
        "ServiceRequest" => {
            let subject_ref = event
                .resource
                .get("subject")
                .and_then(|v| v.get("reference"))
                .and_then(Value::as_str)
                .ok_or_else(|| anyhow::anyhow!("missing subject reference"))?;
            let demographic_no = resolve_identifier(
                client,
                cfg,
                token,
                subject_ref,
                &cfg.fhir.oscar_demographic_system,
            )
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
            let requester_ref = event
                .resource
                .get("requester")
                .and_then(|v| v.get("reference"))
                .and_then(Value::as_str);
            let provider_no = if let Some(r) = requester_ref {
                resolve_identifier(client, cfg, token, r, &cfg.fhir.oscar_provider_system)
                    .await
                    .map_err(|e| anyhow::anyhow!("{e}"))?
            } else {
                None
            };
            let (existing_id, row) = fhir_service_request_to_row(
                &event.resource,
                &cfg.fhir.oscar_consult_request_system,
                &cfg.writeback.consult_service_map,
                &cfg.writeback.default_consult_provider_no,
                demographic_no,
                provider_no,
                tz,
            )
            .map_err(|e| anyhow::anyhow!("{e}"))?;
            let new_id = tx
                .write_consultation_request(existing_id.as_deref(), &row, tz)
                .await?;
            let source_node = event
                .resource
                .get("meta")
                .and_then(|m| m.get("source"))
                .and_then(Value::as_str)
                .unwrap_or("");
            if let Some(placer) = &row.placer_order_id {
                tx.upsert_consultation_request_ext(&new_id, "amt.placerOrderId", placer)
                    .await?;
            }
            tx.upsert_consultation_request_ext(&new_id, "amt.fhirServiceRequestId", &event.id)
                .await?;
            tx.upsert_consultation_request_ext(&new_id, "amt.sourceNode", source_node)
                .await?;
            if existing_id.is_none() {
                hapi_update_identifier(client, cfg, token, event, &new_id).await?;
            }
        }
        "Encounter" => {
            info!(
                "writeback: skipping Encounter {} (handled via DocumentReference)",
                event.id
            );
        }
        other => {
            warn!("writeback: unhandled resource type {other}");
        }
    }
    Ok(())
}

async fn hapi_update_identifier(
    client: &Client,
    cfg: &Config,
    token: Option<&str>,
    event: &HistoryEvent,
    oscar_id: &str,
) -> Result<()> {
    let system = match event.resource_type.as_str() {
        "Patient" => &cfg.fhir.oscar_demographic_system,
        "Appointment" => &cfg.fhir.oscar_appointment_system,
        "DocumentReference" => &cfg.fhir.oscar_note_document_system,
        "ServiceRequest" => &cfg.fhir.oscar_consult_request_system,
        other => anyhow::bail!("cannot write Oscar identifier for resource type {other}"),
    };

    let mut resource = event.resource.clone();
    set_identifier(&mut resource, system, oscar_id);
    set_meta_source(&mut resource, WRITE_BACK_SOURCE);

    let put_url = format!(
        "{}/{}/{}",
        cfg.fhir.base_url.trim_end_matches('/'),
        event.resource_type,
        event.id
    );

    let mut req = client
        .put(&put_url)
        .header("Accept", "application/fhir+json")
        .header("Content-Type", "application/fhir+json")
        .json(&resource);
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    let resp = req.send().await.with_context(|| format!("PUT {put_url}"))?;
    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        anyhow::bail!("HAPI identifier write-back failed ({status}): {text}");
    }

    info!(
        "writeback: wrote {}/{} identifier {}",
        event.resource_type, event.id, oscar_id
    );
    Ok(())
}

fn set_identifier(resource: &mut Value, system: &str, value: &str) {
    if let Some(arr) = resource
        .get_mut("identifier")
        .and_then(Value::as_array_mut)
    {
        if let Some(existing) = arr.iter_mut().find(|i| {
            i.get("system").and_then(Value::as_str) == Some(system)
        }) {
            existing["value"] = value.into();
            return;
        }
        arr.push(json!({"system": system, "value": value}));
    } else {
        resource["identifier"] = json!([{"system": system, "value": value}]);
    }
}

fn set_meta_source(resource: &mut Value, source: &str) {
    resource["meta"] = json!({"source": source});
}

fn identifier_value(resource: &Value, system: &str) -> Option<String> {
    resource
        .get("identifier")
        .and_then(Value::as_array)
        .and_then(|ids| {
            ids.iter()
                .find(|i| i.get("system").and_then(Value::as_str) == Some(system))
                .and_then(|i| i.get("value").and_then(Value::as_str))
        })
        .map(String::from)
}

/// Fetches a referenced resource from HAPI and returns the value of the
/// identifier matching `system`, if present. Returns `Ok(None)` when the
/// resource exists but has no matching identifier (a legitimate "not linked
/// to this EMR" state — not an error). Returns `Err` only for transport/HTTP
/// failures or a 404 (the reference itself is broken).
async fn resolve_identifier(
    client: &Client,
    cfg: &Config,
    token: Option<&str>,
    reference: &str,
    system: &str,
) -> std::result::Result<Option<String>, MappingError> {
    let (resource_type, id) = reference
        .split_once('/')
        .ok_or_else(|| MappingError::ReferenceNotFound {
            reference: reference.to_string(),
        })?;
    if id.is_empty() {
        return Err(MappingError::ReferenceNotFound {
            reference: reference.to_string(),
        });
    }
    let url = format!(
        "{}/{}/{}",
        cfg.fhir.base_url.trim_end_matches('/'),
        resource_type,
        id
    );
    let mut req = client.get(&url).header("Accept", "application/fhir+json");
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }
    let resp = req.send().await.map_err(|e| MappingError::ReferenceFetchFailed {
        reference: reference.to_string(),
        detail: e.to_string(),
    })?;
    if resp.status() == 404 {
        return Err(MappingError::ReferenceNotFound {
            reference: reference.to_string(),
        });
    }
    if !resp.status().is_success() {
        return Err(MappingError::ReferenceFetchFailed {
            reference: reference.to_string(),
            detail: resp.status().to_string(),
        });
    }
    let resource: Value = resp.json().await.map_err(|e| MappingError::ReferenceFetchFailed {
        reference: reference.to_string(),
        detail: e.to_string(),
    })?;
    Ok(identifier_value(&resource, system))
}

async fn load_checkpoint(path: &str) -> WritebackCheckpoint {
    match tokio::fs::read_to_string(path).await {
        Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
        Err(_) => WritebackCheckpoint::default(),
    }
}

async fn save_checkpoint(path: &str, cp: &WritebackCheckpoint) -> Result<()> {
    let tmp = format!("{path}.tmp");
    let json = serde_json::to_string_pretty(cp)?;
    tokio::fs::write(&tmp, json)
        .await
        .with_context(|| format!("writing checkpoint temp {tmp}"))?;
    tokio::fs::rename(&tmp, path)
        .await
        .with_context(|| format!("renaming checkpoint to {path}"))?;
    Ok(())
}
