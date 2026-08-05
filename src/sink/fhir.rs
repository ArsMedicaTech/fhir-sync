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
use fhirbolt::model::r4b::resources::{Patient, PatientDeceased, PatientLink};
use fhirbolt::model::r4b::types::{Address, ContactPoint, HumanName, Identifier, Meta, Reference};
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{error, info, warn};

use crate::auth::TokenProvider;
use crate::config::{Config, FhirConfig};
use crate::dispatch::DispatchNotification;
use crate::domain::patient::{AddressKind, AddressUse, DomainAddress, DomainPatient};
use crate::domain::resource::DomainResource;
use crate::event::{Op, ResourceType, Source, SyncEvent};
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
    let token_provider = match cfg.fhir.keycloak.as_ref() {
        Some(kc) => Some(TokenProvider::new(kc, client.clone())?),
        None => None,
    };

    while let Some(event) = rx.recv().await {
        let key = event.idempotency_key().to_string();

        match sync_with_retry(&client, &cfg, token_provider.as_ref(), &event, &metrics).await {
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
        resource_type: event.resource_type().as_path().to_string(),
        fhir_id: result.fhir_id.clone(),
        fhir_version_id: result.version_id.clone(),
        op: event.op(),
        source: event.source().clone(),
        idempotency_key: event.idempotency_key().to_string(),
        occurred_at: event.occurred_at(),
        fhir_base_url: cfg.fhir.base_url.clone(),
    }
}

/// Retries `sync_one` with exponential backoff, doubling `retry_base_ms`
/// each attempt (capped to avoid overflow), up to `retry_max_attempts`.
/// Permanent client errors are dead-lettered immediately with no retries.
async fn sync_with_retry(
    client: &reqwest::Client,
    cfg: &Config,
    token_provider: Option<&TokenProvider>,
    event: &SyncEvent,
    metrics: &SharedMetrics,
) -> Result<FhirResult, SyncFailure> {
    let max_attempts = cfg.sync.retry_max_attempts.max(1);
    let base_ms = cfg.sync.retry_base_ms;

    let mut last_err = None;
    for attempt in 0..max_attempts {
        match sync_one(client, &cfg.fhir, token_provider, event).await {
            Ok(res) => return Ok(res),
            Err(SyncFailure::Permanent(e)) => {
                warn!(
                    "fhir sink: attempt {}/{} failed permanently for {}: {e:?}",
                    attempt + 1,
                    max_attempts,
                    event.idempotency_key()
                );
                last_err = Some(e);
                break;
            }
            Err(SyncFailure::Retryable(e)) => {
                warn!(
                    "fhir sink: attempt {}/{} failed for {}: {e:?}",
                    attempt + 1,
                    max_attempts,
                    event.idempotency_key()
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
        "idempotency_key": event.idempotency_key(),
        "source": format!("{:?}", event.source()),
        "op": format!("{:?}", event.op()),
        "occurred_at": event.occurred_at().to_rfc3339(),
        "source_id": event.payload().source_id(),
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
    let resource_path = event.resource_type().as_path();
    let base = format!("{}/{resource_path}", fhir_cfg.base_url.trim_end_matches('/'));

    let identifier_system = match event.resource_type() {
        ResourceType::Patient => &fhir_cfg.oscar_demographic_system,
        ResourceType::Practitioner => &fhir_cfg.oscar_provider_system,
        ResourceType::Appointment => &fhir_cfg.oscar_appointment_system,
    };
    let identifier = format!("{}|{}", identifier_system, event.payload().source_id());

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
    token_provider: Option<&TokenProvider>,
    event: &SyncEvent,
) -> Result<FhirResult, SyncFailure> {
    let token: Option<String> = match token_provider {
        Some(tp) => Some(tp.token().await.map_err(SyncFailure::Retryable)?),
        None => fhir_cfg
            .token_env
            .as_ref()
            .and_then(|key| std::env::var(key).ok()),
    };

    match event.payload() {
        DomainResource::Patient(patient) => {
            sync_patient(client, fhir_cfg, token, event, patient).await
        }
        other => {
            warn!(
                "fhir sink: unsupported resource type {:?} for {}",
                event.resource_type(),
                event.idempotency_key()
            );
            Err(SyncFailure::Permanent(anyhow::anyhow!(
                "unsupported resource type {:?}",
                event.resource_type()
            )))
        }
    }
}

async fn sync_patient(
    client: &reqwest::Client,
    fhir_cfg: &FhirConfig,
    token: Option<String>,
    event: &SyncEvent,
    patient: &DomainPatient,
) -> Result<FhirResult, SyncFailure> {
    let mut fhir_patient = build_patient(patient, fhir_cfg);
    if event.op() == Op::Delete {
        fhir_patient.active = Some(false.into());
    }

    let body = fhirbolt::json::to_string(&fhir_patient, None)
        .context("serializing FHIR Patient")
        .map_err(SyncFailure::Permanent)?;

    let mut req = build_put_request(client, fhir_cfg, token.as_deref(), event).body(body);

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

    let identifier = format!("{}|{}", fhir_cfg.oscar_demographic_system, patient.demographic_no);
    info!(
        "fhir sink: synced {} -> {} (fhir_id={} version_id={:?})",
        event.idempotency_key(), identifier, result.fhir_id, result.version_id
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

/// Maps Oscar `sex` to FHIR R4 `AdministrativeGender`.
///
/// `M`/`MALE` -> male, `F`/`FEMALE` -> female, `O`/`T`/`I` -> other,
/// empty/`U`/NULL -> unknown. Anything else is logged and treated as unknown.
fn map_gender(sex: Option<&str>) -> &'static str {
    match sex.map(|s| s.trim().to_ascii_uppercase()).as_deref() {
        None | Some("") | Some("U") => "unknown",
        Some("M") | Some("MALE") => "male",
        Some("F") | Some("FEMALE") => "female",
        Some("O") | Some("T") | Some("I") => "other",
        Some(other) => {
            warn!("map_gender: unexpected sex value '{}'", other);
            "unknown"
        }
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
            system: Some(cfg.bc_phn_system.clone().into()),
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

    // Never omitted — falls back to "unknown".
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

    for addr in &payload.addresses {
        patient.address.push(build_address(addr));
    }

    // `patient_status` and `demographic_merged` both influence active/deceased/link.
    let (active, deceased, link) = patient_lifecycle(payload, cfg);
    patient.active = Some(active.into());
    patient.deceased = deceased;
    patient.link = link;

    patient
}

fn build_address(addr: &DomainAddress) -> Address {
    Address {
        r#use: Some(addr.use_.as_str().into()),
        r#type: Some(addr.kind.as_str().into()),
        r#line: addr.line.clone().map(|l| vec![l.into()]).unwrap_or_default(),
        city: addr.city.clone().map(Into::into),
        state: addr.province.clone().map(Into::into),
        country: Some("CA".into()),
        postal_code: addr.postal.clone().map(Into::into),
        ..Default::default()
    }
}

fn patient_lifecycle(
    payload: &DomainPatient,
    cfg: &FhirConfig,
) -> (bool, Option<PatientDeceased>, Vec<PatientLink>) {
    if let Some(merged_to) = &payload.merged_to {
        let sys: String =
            url::form_urlencoded::byte_serialize(cfg.oscar_demographic_system.as_bytes()).collect();
        let val: String = url::form_urlencoded::byte_serialize(merged_to.as_bytes()).collect();
        let reference = format!("Patient?identifier={sys}|{val}");
        let link = PatientLink {
            other: Box::new(Reference {
                reference: Some(reference.into()),
                ..Default::default()
            }),
            r#type: "replaced-by".into(),
            ..Default::default()
        };
        return (false, None, vec![link]);
    }

    match payload
        .patient_status
        .as_deref()
        .map(|s| s.trim().to_ascii_uppercase())
        .as_deref()
    {
        Some("AC") => (true, None, Vec::new()),
        Some("IN") => (false, None, Vec::new()),
        Some("DE") => (
            false,
            Some(PatientDeceased::Boolean(true.into())),
            Vec::new(),
        ),
        Some(other) => {
            warn!(
                "build_patient: unexpected patient_status '{}' for demographic_no {}",
                other, payload.demographic_no
            );
            (true, None, Vec::new())
        }
        None => (true, None, Vec::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        Config, DatabaseConfig, DispatchConfig, FhirConfig, OscarConfig, ReplicationConfig,
        ServerConfig, SyncConfig,
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
            oscar_provider_system: "https://arsmedicatech.com/fhir/sid/oscar-provider".to_string(),
            oscar_appointment_system: "https://arsmedicatech.com/fhir/sid/oscar-appointment".to_string(),
            bc_phn_system: "https://fhir.infoway-inforoute.ca/NamingSystem/ca-bc-patient-healthcare-id".to_string(),
            bc_msp_practitioner_system: "https://fhir.infoway-inforoute.ca/NamingSystem/ca-bc-msp-practitioner-id".to_string(),
            token_env: None,
            keycloak: None,
        }
    }

    #[test]
    fn gender_mapping_never_omits() {
        assert_eq!(map_gender(Some("M")), "male");
        assert_eq!(map_gender(Some("MALE")), "male");
        assert_eq!(map_gender(Some("female")), "female");
        assert_eq!(map_gender(Some("FEMALE")), "female");
        assert_eq!(map_gender(Some("O")), "other");
        assert_eq!(map_gender(Some("T")), "other");
        assert_eq!(map_gender(Some("I")), "other");
        assert_eq!(map_gender(Some("U")), "unknown");
        assert_eq!(map_gender(Some("")), "unknown");
        assert_eq!(map_gender(None), "unknown");
    }

    #[test]
    fn build_patient_omits_absent_telecom_and_address() {
        let payload = DomainPatient {
            demographic_no: "123".to_string(),
            first_name: Some("Alice".to_string()),
            last_name: Some("Smith".to_string()),
            date_of_birth: Some("1990-03-05".to_string()),
            addresses: Vec::new(),
            patient_status: None,
            merged_to: None,
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
            addresses: Vec::new(),
            patient_status: None,
            merged_to: None,
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
    fn build_patient_emits_two_addresses() {
        let payload = DomainPatient {
            demographic_no: "101".to_string(),
            first_name: Some("Bob".to_string()),
            last_name: Some("Whitfield".to_string()),
            date_of_birth: Some("1968-07-14".to_string()),
            addresses: vec![
                DomainAddress {
                    line: Some("123 Postal St".to_string()),
                    city: Some("Vancouver".to_string()),
                    province: Some("BC".to_string()),
                    postal: Some("V6C1V5".to_string()),
                    use_: AddressUse::Home,
                    kind: AddressKind::Postal,
                },
                DomainAddress {
                    line: Some("456 Physical Ave".to_string()),
                    city: Some("Burnaby".to_string()),
                    province: Some("BC".to_string()),
                    postal: Some("V5A2B3".to_string()),
                    use_: AddressUse::Home,
                    kind: AddressKind::Physical,
                },
            ],
            patient_status: Some("AC".to_string()),
            merged_to: None,
            sex: Some("M".to_string()),
            phone: None,
            email: None,
            hin: Some("9123456781".to_string()),
        };

        let patient = build_patient(&payload, &fhir_cfg());
        assert_eq!(patient.address.len(), 2);
        assert_eq!(patient.address[0].r#type.as_ref().map(|c| c.value.clone()), Some(Some("postal".to_string())));
        assert_eq!(patient.address[1].r#type.as_ref().map(|c| c.value.clone()), Some(Some("physical".to_string())));
        assert_eq!(patient.address[0].country.as_ref().map(|c| c.value.clone()), Some(Some("CA".to_string())));
        assert_eq!(patient.active.as_ref().map(|b| b.value).flatten(), Some(true));
        assert_eq!(patient.identifier.len(), 2); // no ver
    }

    #[test]
    fn build_patient_allows_null_address_line() {
        let payload = DomainPatient {
            demographic_no: "102".to_string(),
            first_name: Some("Kayode".to_string()),
            last_name: Some("Adeyemi".to_string()),
            date_of_birth: Some("1991-03-22".to_string()),
            addresses: vec![DomainAddress {
                line: None,
                city: Some("Vancouver".to_string()),
                province: Some("BC".to_string()),
                postal: Some("V5K0A1".to_string()),
                use_: AddressUse::Home,
                kind: AddressKind::Postal,
            }],
            patient_status: None,
            merged_to: None,
            sex: Some("O".to_string()),
            phone: None,
            email: None,
            hin: None,
        };

        let patient = build_patient(&payload, &fhir_cfg());
        assert_eq!(patient.address.len(), 1);
        assert!(patient.address[0].r#line.is_empty());
        assert_eq!(patient.gender.as_ref().map(|c| c.value.clone()).flatten().as_deref(), Some("other"));
    }

    #[test]
    fn build_patient_marks_deceased_and_inactive() {
        let payload = DomainPatient {
            demographic_no: "104".to_string(),
            first_name: Some("Luc".to_string()),
            last_name: Some("Tremblay".to_string()),
            date_of_birth: Some("1943-11-30".to_string()),
            addresses: Vec::new(),
            patient_status: Some("DE".to_string()),
            merged_to: None,
            sex: Some("M".to_string()),
            phone: None,
            email: None,
            hin: None,
        };

        let patient = build_patient(&payload, &fhir_cfg());
        assert_eq!(patient.active.as_ref().map(|b| b.value).flatten(), Some(false));
        assert!(matches!(patient.deceased, Some(PatientDeceased::Boolean(_))));
    }

    #[test]
    fn build_patient_marks_replaced_by_link() {
        let payload = DomainPatient {
            demographic_no: "106".to_string(),
            first_name: None,
            last_name: None,
            date_of_birth: None,
            addresses: Vec::new(),
            patient_status: None,
            merged_to: Some("101".to_string()),
            sex: None,
            phone: None,
            email: None,
            hin: None,
        };

        let patient = build_patient(&payload, &fhir_cfg());
        assert_eq!(patient.active.as_ref().map(|b| b.value).flatten(), Some(false));
        assert_eq!(patient.link.len(), 1);
        assert_eq!(patient.link[0].r#type.value.as_deref(), Some("replaced-by"));
        assert!(patient.link[0].other.reference.as_ref().map(|s| s.value.as_deref()).flatten().unwrap_or("").contains("101"));
    }

    #[test]
    fn dead_letter_never_contains_full_payload() {
        let dir = std::env::temp_dir().join(format!("fhir-sync-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("dead_letter.jsonl");
        let path_str = path.to_str().unwrap();

        let event = SyncEvent::new(
            Source::OscarBinlog { table: "demographic".to_string() },
            Op::Upsert,
            DomainResource::Patient(DomainPatient {
                demographic_no: "123".to_string(),
                first_name: Some("Alice".to_string()),
                last_name: Some("Smith".to_string()),
                date_of_birth: None,
                addresses: Vec::new(),
                patient_status: None,
                merged_to: None,
                sex: None,
                phone: None,
                email: Some("alice@example.com".to_string()),
                hin: None,
            }),
            chrono::Utc::now(),
        );

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
        let cfg = fhir_cfg();
        let event = SyncEvent::new(
            Source::OscarBinlog { table: "demographic".to_string() },
            Op::Upsert,
            DomainResource::Patient(DomainPatient {
                demographic_no: "121".to_string(),
                first_name: None,
                last_name: None,
                date_of_birth: None,
                addresses: Vec::new(),
                patient_status: None,
                merged_to: None,
                sex: None,
                phone: None,
                email: None,
                hin: None,
            }),
            chrono::Utc::now(),
        );

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
                oscar_provider_system: "https://arsmedicatech.com/fhir/sid/oscar-provider".into(),
                oscar_appointment_system: "https://arsmedicatech.com/fhir/sid/oscar-appointment".into(),
                bc_phn_system: "https://fhir.infoway-inforoute.ca/NamingSystem/ca-bc-patient-healthcare-id".into(),
                bc_msp_practitioner_system: "https://fhir.infoway-inforoute.ca/NamingSystem/ca-bc-msp-practitioner-id".into(),
                token_env: None,
                keycloak: None,
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
            oscar: OscarConfig::default(),
            debug: None,
        };

        let event = SyncEvent::new(
            Source::OscarBinlog { table: "demographic".to_string() },
            Op::Upsert,
            DomainResource::Patient(DomainPatient {
                demographic_no: "121".into(),
                first_name: None,
                last_name: None,
                date_of_birth: None,
                addresses: Vec::new(),
                patient_status: None,
                merged_to: None,
                sex: None,
                phone: None,
                email: None,
                hin: None,
            }),
            chrono::Utc::now(),
        );

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
