//! Sink task: consumes `SyncEvent`s and conditionally upserts a FHIR R4B
//! `Patient` into HAPI (D5). Owns the `rx` end of the channel — there is
//! exactly one consumer (D4).
//!
//! Failed syncs are retried with exponential backoff
//! (`cfg.sync.retry_max_attempts` / `retry_base_ms`); on exhaustion the
//! event is appended to `cfg.sync.dead_letter_path` and the stream keeps
//! running — one bad record must never take down the process.

use std::io::Write;
use std::time::Duration;

use anyhow::{Context, Result};
use fhirbolt::model::r4b::resources::Patient;
use fhirbolt::model::r4b::types::{Address, ContactPoint, HumanName, Identifier, Meta};
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{error, info, warn};

use crate::config::{Config, FhirConfig};
use crate::dispatch::DispatchNotification;
use crate::domain::patient::DomainPatient;
use crate::event::{Op, SyncEvent};
use crate::metrics::SharedMetrics;

const META_SOURCE: &str = "urn:arsmedicatech:fhir-sync:oscar";

/// Runs the sink to completion (until the channel closes).
pub async fn run(
    cfg: Config,
    mut rx: Receiver<SyncEvent>,
    metrics: SharedMetrics,
    dispatch_tx: Option<Sender<DispatchNotification>>,
) -> Result<()> {
    let client = reqwest::Client::new();
    let token = cfg
        .fhir
        .token_env
        .as_ref()
        .and_then(|key| std::env::var(key).ok());

    while let Some(event) = rx.recv().await {
        let key = event.idempotency_key.clone();

        match sync_with_retry(&client, &cfg, token.as_deref(), &event, &metrics).await {
            Ok(result) => {
                metrics.inc_synced();
                if let Some(tx) = &dispatch_tx {
                    if !result.fhir_id.is_empty() {
                        let n = build_dispatch_notification(&event, &cfg, &result);
                        if tx.try_send(n).is_err() {
                            warn!("fhir sink: dispatch channel full or closed, dropping notification");
                            metrics.inc_dispatch_dropped();
                        }
                    } else {
                        warn!("fhir sink: HAPI success but no fhir_id for {key}; not dispatching");
                    }
                }
            }
            Err(e) => {
                let err = match e {
                    SyncFailure::Retryable(inner) | SyncFailure::Permanent(inner) => inner,
                };
                error!("fhir sink: exhausted retries for {key}: {err:?}");
                metrics.inc_dead_lettered();
                if let Err(dl_err) = write_dead_letter(&cfg.sync.dead_letter_path, &event, &err) {
                    // PHI note (spec §8): never let a dead-letter write failure crash the
                    // stream either — log identifier only and move on.
                    error!("fhir sink: failed to write dead letter for {key}: {dl_err:?}");
                }
            }
        }
    }

    Ok(())
}

fn build_dispatch_notification(
    event: &SyncEvent,
    cfg: &Config,
    result: &FhirResult,
) -> DispatchNotification {
    DispatchNotification {
        resource_type: match event.resource_type {
            crate::event::ResourceType::Patient => "Patient",
        }
        .to_string(),
        fhir_id: result.fhir_id.clone(),
        fhir_version_id: result.version_id.clone(),
        op: event.op,
        source: event.source,
        idempotency_key: event.idempotency_key.clone(),
        occurred_at: event.occurred_at,
        fhir_base_url: cfg.fhir.base_url.clone(),
    }
}

/// Retries `sync_one` with exponential backoff, doubling `retry_base_ms`
/// each attempt (capped to avoid overflow), up to `retry_max_attempts`.
/// Permanent client errors are dead-lettered immediately with no retries.
async fn sync_with_retry(
    client: &reqwest::Client,
    cfg: &Config,
    token: Option<&str>,
    event: &SyncEvent,
    metrics: &SharedMetrics,
) -> Result<FhirResult, SyncFailure> {
    let max_attempts = cfg.sync.retry_max_attempts.max(1);
    let base_ms = cfg.sync.retry_base_ms;

    let mut last_err = None;
    for attempt in 0..max_attempts {
        match sync_one(client, &cfg.fhir, token, event).await {
            Ok(res) => return Ok(res),
            Err(SyncFailure::Permanent(e)) => {
                warn!(
                    "fhir sink: attempt {}/{} failed permanently for {}: {e:?}",
                    attempt + 1,
                    max_attempts,
                    event.idempotency_key
                );
                last_err = Some(e);
                break;
            }
            Err(SyncFailure::Retryable(e)) => {
                warn!(
                    "fhir sink: attempt {}/{} failed for {}: {e:?}",
                    attempt + 1,
                    max_attempts,
                    event.idempotency_key
                );
                last_err = Some(e);
                if attempt + 1 < max_attempts {
                    metrics.inc_retried();
                    let backoff_ms = base_ms.saturating_mul(1u64 << attempt.min(10));
                    tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                }
            }
        }
    }

    Err(last_err.unwrap_or_else(|| anyhow::anyhow!("unknown sync failure")))
}

/// Appends a dead-letter record. Identifiers only — never the full payload
/// (spec §8: PHI must not land in dead-letter files).
fn write_dead_letter(path: &str, event: &SyncEvent, err: &anyhow::Error) -> Result<()> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("opening dead letter file {path}"))?;

    let record = serde_json::json!({
        "idempotency_key": event.idempotency_key,
        "source": format!("{:?}", event.source),
        "op": format!("{:?}", event.op),
        "occurred_at": event.occurred_at.to_rfc3339(),
        "demographic_no": event.payload.demographic_no,
        "error": err.to_string(),
    });

    writeln!(file, "{record}").context("writing dead letter record")?;
    Ok(())
}

/// Classifies a sink failure as retryable (network / 5xx / 429) or permanent
/// (any other 4xx). Keeps the decision out of string matching.
#[derive(Debug)]
enum SyncFailure {
    Retryable(anyhow::Error),
    Permanent(anyhow::Error),
}

/// The HAPI-assigned identifiers captured from a successful conditional PUT.
#[derive(Debug)]
struct FhirResult {
    fhir_id: String,
    version_id: Option<String>,
}

fn build_put_request(
    client: &reqwest::Client,
    fhir_cfg: &FhirConfig,
    token: Option<&str>,
    event: &SyncEvent,
) -> reqwest::RequestBuilder {
    let base = format!("{}/Patient", fhir_cfg.base_url.trim_end_matches('/'));
    let identifier = format!(
        "{}|{}",
        fhir_cfg.oscar_demographic_system, event.payload.demographic_no
    );

    let mut req = client
        .put(&base)
        .query(&[("identifier", identifier.as_str())])
        .header("Content-Type", "application/fhir+json");

    if let Some(token) = token {
        req = req.bearer_auth(token);
    }

    req
}

async fn sync_one(
    client: &reqwest::Client,
    fhir_cfg: &FhirConfig,
    token: Option<&str>,
    event: &SyncEvent,
) -> Result<FhirResult, SyncFailure> {
    let mut patient = build_patient(&event.payload, fhir_cfg);
    if event.op == Op::Delete {
        patient.active = Some(false.into());
    }

    let body = fhirbolt::json::to_string(&patient, None)
        .context("serializing FHIR Patient")
        .map_err(SyncFailure::Permanent)?;

    let mut req = build_put_request(client, fhir_cfg, token, event).body(body);

    let resp = req
        .send()
        .await
        .with_context(|| "sending conditional PUT to HAPI")
        .map_err(SyncFailure::Retryable)?;
    let status = resp.status();

    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        let err = anyhow::anyhow!("HAPI conditional PUT failed ({status}): {text}");
        if status.is_client_error() && status.as_u16() != 429 {
            return Err(SyncFailure::Permanent(err));
        }
        return Err(SyncFailure::Retryable(err));
    }

    let location = resp
        .headers()
        .get("location")
        .and_then(|v| v.to_str().ok())
        .map(String::from);
    let body_text = resp.text().await.unwrap_or_default();

    let mut result = FhirResult {
        fhir_id: String::new(),
        version_id: None,
    };

    if !body_text.trim().is_empty() {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&body_text) {
            if let Some(id) = value.get("id").and_then(|v| v.as_str()) {
                result.fhir_id = id.to_string();
            }
            if let Some(vid) = value
                .get("meta")
                .and_then(|m| m.get("versionId"))
                .and_then(|v| v.as_str())
            {
                result.version_id = Some(vid.to_string());
            }
        }
    }

    if result.fhir_id.is_empty() {
        if let Some(loc) = location {
            result.fhir_id = parse_location_id(&loc).unwrap_or_default();
            result.version_id = parse_location_version_id(&loc);
        }
    }

    let identifier = format!(
        "{}|{}",
        fhir_cfg.oscar_demographic_system, event.payload.demographic_no
    );
    info!(
        "fhir sink: synced {} -> {} (fhir_id={} version_id={:?})",
        event.idempotency_key, identifier, result.fhir_id, result.version_id
    );
    Ok(result)
}

/// Extracts the resource id from a `Location` header like
/// `Patient/123/_history/2` or `http://.../Patient/123/_history/2`.
fn parse_location_id(location: &str) -> Option<String> {
    let parts: Vec<&str> = location.trim_end_matches('/').split('/').collect();
    if parts.len() >= 3 && parts[parts.len() - 2] == "_history" {
        return Some(parts[parts.len() - 3].to_string());
    }
    if parts.len() >= 2 {
        return Some(parts.last().unwrap().to_string());
    }
    None
}

/// Extracts the `versionId` from a `Location` header ending in
/// `.../Patient/{id}/_history/{vid}`.
fn parse_location_version_id(location: &str) -> Option<String> {
    let parts: Vec<&str> = location.trim_end_matches('/').split('/').collect();
    if parts.len() >= 2 && parts[parts.len() - 2] == "_history" {
        return Some(parts.last().unwrap().to_string());
    }
    None
}

/// M/male -> male, F/female -> female, else unknown. Never omitted (D5).
fn map_gender(sex: Option<&str>) -> &'static str {
    match sex.map(|s| s.to_ascii_uppercase()) {
        Some(s) if s == "M" || s == "MALE" => "male",
        Some(s) if s == "F" || s == "FEMALE" => "female",
        _ => "unknown",
    }
}

fn build_patient(payload: &DomainPatient, cfg: &FhirConfig) -> Patient {
    let mut patient = Patient::default();

    patient.meta = Some(Box::new(Meta {
        source: Some(META_SOURCE.into()),
        ..Default::default()
    }));

    patient.identifier.push(Identifier {
        system: Some(cfg.oscar_demographic_system.clone().into()),
        value: Some(payload.demographic_no.clone().into()),
        ..Default::default()
    });

    // Provincial PHN, only if this Oscar instance has it.
    if let Some(hin) = &payload.hin {
        patient.identifier.push(Identifier {
            system: Some(cfg.oscar_hin_system.clone().into()),
            value: Some(hin.clone().into()),
            ..Default::default()
        });
    }

    if payload.first_name.is_some() || payload.last_name.is_some() {
        patient.name.push(HumanName {
            family: payload.last_name.clone().map(Into::into),
            given: payload
                .first_name
                .clone()
                .map(|g| vec![g.into()])
                .unwrap_or_default(),
            ..Default::default()
        });
    }

    if let Some(dob) = &payload.date_of_birth {
        patient.birth_date = Some(dob.clone().into());
    }

    // Never omitted — falls back to "unknown" (D5).
    patient.gender = Some(map_gender(payload.sex.as_deref()).into());

    if let Some(email) = &payload.email {
        patient.telecom.push(ContactPoint {
            system: Some("email".into()),
            value: Some(email.clone().into()),
            ..Default::default()
        });
    }

    if let Some(phone) = &payload.phone {
        patient.telecom.push(ContactPoint {
            system: Some("phone".into()),
            value: Some(phone.clone().into()),
            ..Default::default()
        });
    }

    if let Some((city, province, country, postal)) = &payload.location {
        patient.address.push(Address {
            city: Some(city.clone().into()),
            state: Some(province.clone().into()),
            country: Some(country.clone().into()),
            postal_code: Some(postal.clone().into()),
            ..Default::default()
        });
    }

    patient
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        Config, DatabaseConfig, DispatchConfig, FhirConfig, ReplicationConfig, ServerConfig,
        SyncConfig,
    };
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    fn fhir_cfg() -> FhirConfig {
        FhirConfig {
            base_url: "http://localhost:8082/fhir".to_string(),
            oscar_demographic_system: "https://arsmedicatech.com/fhir/sid/oscar-demographic-no"
                .to_string(),
            oscar_hin_system: "https://arsmedicatech.com/fhir/sid/oscar-hin".to_string(),
            token_env: None,
        }
    }

    #[test]
    fn gender_mapping_never_omits() {
        assert_eq!(map_gender(Some("M")), "male");
        assert_eq!(map_gender(Some("female")), "female");
        assert_eq!(map_gender(Some("other")), "unknown");
        assert_eq!(map_gender(None), "unknown");
    }

    #[test]
    fn build_patient_omits_absent_telecom_and_address() {
        let payload = DomainPatient {
            demographic_no: "123".to_string(),
            first_name: Some("Alice".to_string()),
            last_name: Some("Smith".to_string()),
            date_of_birth: Some("1990-03-05".to_string()),
            location: None,
            sex: Some("F".to_string()),
            phone: None,
            email: None,
            hin: None,
        };

        let patient = build_patient(&payload, &fhir_cfg());
        assert!(patient.telecom.is_empty());
        assert!(patient.address.is_empty());
        assert_eq!(patient.identifier.len(), 1);
        assert_eq!(patient.identifier[0].value, Some("123".to_string().into()));
    }

    #[test]
    fn build_patient_adds_hin_identifier_when_present() {
        let payload = DomainPatient {
            demographic_no: "123".to_string(),
            first_name: None,
            last_name: None,
            date_of_birth: None,
            location: None,
            sex: None,
            phone: None,
            email: None,
            hin: Some("9999888877".to_string()),
        };

        let patient = build_patient(&payload, &fhir_cfg());
        assert_eq!(patient.identifier.len(), 2);
        assert_eq!(
            patient.identifier[1].value,
            Some("9999888877".to_string().into())
        );
    }

    #[test]
    fn dead_letter_never_contains_full_payload() {
        use crate::event::{ResourceType, Source};

        let dir = std::env::temp_dir().join(format!("fhir-sync-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("dead_letter.jsonl");
        let path_str = path.to_str().unwrap();

        let event = SyncEvent {
            source: Source::OscarBinlog,
            op: Op::Upsert,
            resource_type: ResourceType::Patient,
            idempotency_key: "oscar:demographic:123:456".to_string(),
            payload: DomainPatient {
                demographic_no: "123".to_string(),
                first_name: Some("Alice".to_string()),
                last_name: Some("Smith".to_string()),
                date_of_birth: None,
                location: None,
                sex: None,
                phone: None,
                email: Some("alice@example.com".to_string()),
                hin: None,
            },
            occurred_at: chrono::Utc::now(),
        };

        write_dead_letter(path_str, &event, &anyhow::anyhow!("HAPI unreachable")).unwrap();

        let contents = std::fs::read_to_string(&path).unwrap();
        assert!(contents.contains("\"123\""));
        assert!(contents.contains("HAPI unreachable"));
        assert!(!contents.contains("Alice"));
        assert!(!contents.contains("alice@example.com"));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn conditional_put_url_percent_encodes_pipe() {
        use crate::event::{ResourceType, Source};

        let cfg = fhir_cfg();
        let event = SyncEvent {
            source: Source::OscarBinlog,
            op: Op::Upsert,
            resource_type: ResourceType::Patient,
            idempotency_key: "test".to_string(),
            payload: DomainPatient {
                demographic_no: "121".to_string(),
                first_name: None,
                last_name: None,
                date_of_birth: None,
                location: None,
                sex: None,
                phone: None,
                email: None,
                hin: None,
            },
            occurred_at: chrono::Utc::now(),
        };

        let req = build_put_request(&reqwest::Client::new(), &cfg, None, &event)
            .body("{}".to_string())
            .build()
            .unwrap();

        let url = req.url().as_str();
        assert!(
            url.contains("%7C"),
            "expected percent-encoded pipe in URL: {url}"
        );
        assert!(!url.contains('|'), "expected no literal pipe in URL: {url}");
    }

    async fn run_with_http_status(status_line: &str, max_attempts: u32) -> (Result<(), SyncFailure>, u32) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let attempts = Arc::new(AtomicU32::new(0));
        let a = attempts.clone();
        let response = format!(
            "HTTP/1.1 {}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            status_line
        );

        tokio::spawn(async move {
            loop {
                match tokio::time::timeout(Duration::from_millis(100), listener.accept()).await {
                    Ok(Ok((mut socket, _))) => {
                        let mut buf = [0u8; 4096];
                        let mut pos = 0;
                        let mut header_end = None;
                        while header_end.is_none() {
                            let n = socket.read(&mut buf[pos..]).await.unwrap();
                            pos += n;
                            if let Some(i) =
                                buf[..pos].windows(4).position(|w| w == b"\r\n\r\n")
                            {
                                header_end = Some(i + 4);
                            }
                        }
                        let header_end = header_end.unwrap();
                        let header = String::from_utf8_lossy(&buf[..header_end]);
                        let content_len = header
                            .to_lowercase()
                            .lines()
                            .find_map(|l| {
                                l.strip_prefix("content-length:")
                                    .and_then(|v| v.trim().parse::<usize>().ok())
                            });
                        if let Some(len) = content_len {
                            let body_have = pos.saturating_sub(header_end);
                            let mut remaining = len.saturating_sub(body_have);
                            while remaining > 0 {
                                let to_read = remaining.min(4096);
                                let n = socket.read(&mut buf[..to_read]).await.unwrap();
                                remaining -= n;
                            }
                        }

                        a.fetch_add(1, Ordering::Relaxed);
                        socket.write_all(response.as_bytes()).await.unwrap();
                    }
                    _ => break,
                }
            }
        });

        let client = reqwest::Client::new();
        let cfg = Config {
            database: DatabaseConfig {
                user: "".into(),
                password: "".into(),
                host: "".into(),
                port: 0,
                schema: "".into(),
                server_id: 1,
            },
            server: ServerConfig::default(),
            fhir: FhirConfig {
                base_url: format!("http://127.0.0.1:{port}/fhir"),
                oscar_demographic_system:
                    "https://arsmedicatech.com/fhir/sid/oscar-demographic-no".into(),
                oscar_hin_system: "https://arsmedicatech.com/fhir/sid/oscar-hin".into(),
                token_env: None,
            },
            sync: SyncConfig {
                checkpoint_path: "".into(),
                retry_max_attempts: max_attempts,
                retry_base_ms: 1,
                dead_letter_path: "".into(),
            },
            replication: ReplicationConfig::default(),
            dispatch: DispatchConfig::default(),
            oscar_enabled: true,
            debug: None,
        };

        let event = SyncEvent {
            source: crate::event::Source::OscarBinlog,
            op: Op::Upsert,
            resource_type: crate::event::ResourceType::Patient,
            idempotency_key: "test".into(),
            payload: DomainPatient {
                demographic_no: "121".into(),
                first_name: None,
                last_name: None,
                date_of_birth: None,
                location: None,
                sex: None,
                phone: None,
                email: None,
                hin: None,
            },
            occurred_at: chrono::Utc::now(),
        };

        let metrics = crate::metrics::Metrics::new();
        let result = sync_with_retry(&client, &cfg, None, &event, &metrics)
            .await
            .map(|_| ());
        tokio::time::sleep(Duration::from_millis(10)).await;
        (result, attempts.load(Ordering::Relaxed))
    }

    #[tokio::test]
    async fn status_400_is_not_retried() {
        let (result, attempts) = run_with_http_status("400 Bad Request", 3).await;
        assert!(result.is_err());
        assert_eq!(attempts, 1, "expected 1 attempt for 400, got {attempts}");
    }

    #[tokio::test]
    async fn status_503_is_retried_to_max() {
        let (result, attempts) = run_with_http_status("503 Service Unavailable", 3).await;
        assert!(result.is_err());
        assert_eq!(attempts, 3, "expected 3 attempts for 503, got {attempts}");
    }
}
