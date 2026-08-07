//! HAPI `_history` poller for the AMT → Oscar write-back path.
//!
//! Polls the local HAPI server for `Patient`, `Appointment`, and
//! `DocumentReference` changes, suppresses Oscar-originated echoes using
//! `meta.source`, maps AMT-authored resources to Oscar rows, and writes them
//! through `OscarSink`.

use std::collections::HashMap;
use std::time::Duration;

use anyhow::{Context, Result};
use chrono_tz::Tz;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::time::sleep;
use tracing::{info, warn};

use crate::auth::TokenProvider;
use crate::config::{Config, FhirConfig};
use crate::writeback::authorship::is_oscar_origin;
use crate::writeback::deadletter::{write as write_dead_letter, DeadLetter};
use crate::writeback::mappers::{
    fhir_appointment_to_row, fhir_document_reference_to_row, fhir_patient_to_row,
};
use crate::writeback::oscar_sink::OscarSink;

const SINCE_START: &str = "1900-01-01T00:00:00.000Z";

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
        checkpoint.since = SINCE_START.to_string();
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
                    "writeback: skipping Oscar-originated {} {}",
                    event.resource_type, event.id
                );
                continue;
            }

            if event.resource.get("meta").and_then(|m| m.get("source")).is_none() {
                warn!(
                    "writeback: {} {} has no meta.source; treating as AMT-authored",
                    event.resource_type, event.id
                );
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

            if let Err(e) = process_resource(cfg, sink, tz, &event).await {
                warn!(
                    "writeback: dead-lettering {}/{}: {e:?}",
                    event.resource_type, event.id
                );
                let dl = DeadLetter {
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    id: event.id.clone(),
                    resource_type: event.resource_type.clone(),
                    version_id: Some(event.version_id.clone()),
                    error: format!("{e}"),
                    payload: Some(event.resource.clone()),
                };
                write_dead_letter(&cfg.writeback.dead_letter_path, &dl).await?;
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
    cfg: &Config,
    sink: &OscarSink,
    tz: &Tz,
    event: &HistoryEvent,
) -> Result<()> {
    match event.resource_type.as_str() {
        "Patient" => {
            let (id, row) = fhir_patient_to_row(&event.resource, &cfg.fhir.oscar_demographic_system)
                .map_err(|e| anyhow::anyhow!("{e}"))?;
            sink.write_demographic(id.as_deref(), &row, tz).await?;
        }
        "Appointment" => {
            let (id, row) = fhir_appointment_to_row(
                &event.resource,
                &cfg.fhir.oscar_demographic_system,
                &cfg.fhir.oscar_provider_system,
                &cfg.fhir.oscar_appointment_system,
                &cfg.writeback.appointment_status_map,
                tz,
            )
            .map_err(|e| anyhow::anyhow!("{e}"))?;
            sink.write_appointment(id.as_deref(), &row).await?;
        }
        "DocumentReference" => {
            let (id, row) = fhir_document_reference_to_row(
                &event.resource,
                &cfg.fhir.oscar_note_document_system,
                &cfg.fhir.oscar_demographic_system,
                &cfg.fhir.oscar_provider_system,
                &cfg.fhir.oscar_appointment_system,
                tz,
            )
            .map_err(|e| anyhow::anyhow!("{e}"))?;
            sink.write_note(id.as_deref(), &row, tz).await?;
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


