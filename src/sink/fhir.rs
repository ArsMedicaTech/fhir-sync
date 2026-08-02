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
use tokio::sync::mpsc::Receiver;
use tracing::{error, info, warn};

use crate::config::{Config, FhirConfig};
use crate::domain::patient::DomainPatient;
use crate::event::{Op, SyncEvent};
use crate::metrics::SharedMetrics;

const META_SOURCE: &str = "urn:arsmedicatech:fhir-sync:oscar";

/// Runs the sink to completion (until the channel closes).
pub async fn run(cfg: Config, mut rx: Receiver<SyncEvent>, metrics: SharedMetrics) -> Result<()> {
    let client = reqwest::Client::new();
    let token = cfg
        .fhir
        .token_env
        .as_ref()
        .and_then(|key| std::env::var(key).ok());

    while let Some(event) = rx.recv().await {
        let key = event.idempotency_key.clone();

        if let Err(e) = sync_with_retry(&client, &cfg, token.as_deref(), &event).await {
            error!("fhir sink: exhausted retries for {key}: {e:?}");
            if let Err(dl_err) = write_dead_letter(&cfg.sync.dead_letter_path, &event, &e) {
                // PHI note (spec §8): never let a dead-letter write failure crash the
                // stream either — log identifier only and move on.
                error!("fhir sink: failed to write dead letter for {key}: {dl_err:?}");
            }
        }
    }

    Ok(())
}

/// Retries `sync_one` with exponential backoff, doubling `retry_base_ms`
/// each attempt (capped to avoid overflow), up to `retry_max_attempts`.
async fn sync_with_retry(
    client: &reqwest::Client,
    cfg: &Config,
    token: Option<&str>,
    event: &SyncEvent,
    metrics: &SharedMetrics,
) -> Result<()> {
    let max_attempts = cfg.sync.retry_max_attempts.max(1);
    let base_ms = cfg.sync.retry_base_ms;

    let mut last_err = None;
    for attempt in 0..max_attempts {
        match sync_one(client, &cfg.fhir, token, event).await {
            Ok(()) => return Ok(()),
            Err(e) => {
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

async fn sync_one(
    client: &reqwest::Client,
    fhir_cfg: &FhirConfig,
    token: Option<&str>,
    event: &SyncEvent,
) -> Result<()> {
    let mut patient = build_patient(&event.payload, fhir_cfg);
    if event.op == Op::Delete {
        patient.active = Some(false.into());
    }

    let body = fhirbolt::json::to_string(&patient, None).context("serializing FHIR Patient")?;

    let url = format!(
        "{}/Patient?identifier={}|{}",
        fhir_cfg.base_url.trim_end_matches('/'),
        fhir_cfg.oscar_demographic_system,
        event.payload.demographic_no,
    );

    let mut req = client
        .put(&url)
        .header("Content-Type", "application/fhir+json")
        .body(body);

    if let Some(token) = token {
        req = req.bearer_auth(token);
    }

    let resp = req.send().await.context("sending conditional PUT to HAPI")?;
    let status = resp.status();

    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        anyhow::bail!("HAPI conditional PUT failed ({status}): {text}");
    }

    info!("fhir sink: synced {} -> {}", event.idempotency_key, url);
    Ok(())
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
}
