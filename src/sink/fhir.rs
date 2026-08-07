//! Sink task: consumes `SyncEvent`s and conditionally upserts a FHIR R4B
//! `Patient` into HAPI (D5). Owns the `rx` end of the channel — there is
//! exactly one consumer (D4).
//!
//! Failed syncs are retried with exponential backoff
//! (`cfg.sync.retry_max_attempts` / `retry_base_ms`); on exhaustion the
//! event is appended to `cfg.sync.dead_letter_path` and the stream keeps
//! running — one bad record must never take down the process.

use std::collections::HashSet;
use std::io::Write;
use std::time::Duration;

use anyhow::{Context, Result};
use fhirbolt::model::r4b::resources::{
    Appointment, AppointmentParticipant, Bundle, BundleEntry, BundleEntryRequest, CareTeam,
    CareTeamParticipant, Patient, PatientDeceased, PatientLink, Practitioner,
};
use fhirbolt::model::r4b::Resource as FhirResource;
use fhirbolt::model::r4b::types::{
    Address, CodeableConcept, Coding, ContactPoint, Extension, ExtensionValue, HumanName,
    Identifier, Meta, Reference,
};
use chrono::{LocalResult, NaiveDate, NaiveTime, TimeZone};
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{error, info, warn};

use crate::auth::TokenProvider;
use crate::config::{Config, FhirConfig, OscarConfig, WritebackConfig};
use crate::dispatch::DispatchNotification;
use crate::domain::appointment::DomainAppointment;
use crate::domain::care_team::DomainCareTeam;
use crate::domain::patient::{AddressKind, AddressUse, DomainAddress, DomainPatient};
use crate::domain::practitioner::DomainPractitioner;
use crate::domain::resource::DomainResource;
use crate::event::{Op, ResourceType, Source, SyncEvent};

mod oscar2;
use crate::metrics::SharedMetrics;

pub(crate) const META_SOURCE: &str = "urn:arsmedicatech:fhir-sync:oscar";

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
        match sync_one(client, cfg, token_provider, event).await {
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

    Err(SyncFailure::Permanent(last_err.unwrap_or_else(|| anyhow::anyhow!("unknown sync failure"))))
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

impl std::fmt::Display for SyncFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncFailure::Retryable(e) | SyncFailure::Permanent(e) => write!(f, "{e}"),
        }
    }
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

    let (identifier_system, source_id) = match event.payload() {
        DomainResource::Patient(p) => (&fhir_cfg.oscar_demographic_system, p.demographic_no.as_str()),
        DomainResource::Practitioner(p) => (&fhir_cfg.oscar_provider_system, p.provider_no.as_str()),
        DomainResource::Appointment(a) => (&fhir_cfg.oscar_appointment_system, a.appointment_no.as_str()),
        DomainResource::Encounter(e) => (&fhir_cfg.oscar_note_system, e.uuid.as_deref().unwrap_or(&e.note_id)),
        DomainResource::DocumentReference(d) => (&fhir_cfg.oscar_note_document_system, d.uuid.as_deref().unwrap_or(&d.note_id)),
        DomainResource::Condition(c) => {
            let sys = if c.source_table == "dxresearch" {
                &fhir_cfg.oscar_dxresearch_system
            } else {
                &fhir_cfg.oscar_cpp_condition_system
            };
            (sys, c.source_id.as_str())
        }
        DomainResource::FamilyMemberHistory(f) => (&fhir_cfg.oscar_cpp_condition_system, f.note_id.as_str()),
        DomainResource::CareTeam(c) => (&fhir_cfg.oscar_care_team_system, c.demographic_no.as_str()),
    };
    let identifier = format!("{}|{}", identifier_system, source_id);

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
    cfg: &Config,
    token_provider: Option<&TokenProvider>,
    event: &SyncEvent,
) -> Result<FhirResult, SyncFailure> {
    let fhir_cfg = &cfg.fhir;
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
        DomainResource::Practitioner(practitioner) => {
            sync_practitioner(client, fhir_cfg, token, event, practitioner).await
        }
        DomainResource::Appointment(appointment) => {
            sync_appointment(client, fhir_cfg, token, event, appointment, &cfg.oscar).await
        }
        DomainResource::Encounter(encounter) => {
            oscar2::sync_encounter(client, fhir_cfg, token, event, encounter, &cfg.oscar).await
        }
        DomainResource::DocumentReference(doc) => {
            oscar2::sync_document_reference(client, fhir_cfg, token, event, doc, &cfg.oscar).await
        }
        DomainResource::Condition(condition) => {
            oscar2::sync_condition(client, fhir_cfg, token, event, condition).await
        }
        DomainResource::FamilyMemberHistory(fh) => {
            oscar2::sync_family_member_history(client, fhir_cfg, token, event, fh).await
        }
        DomainResource::CareTeam(care_team) => {
            sync_care_team(client, fhir_cfg, token, event, care_team).await
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

    let req = build_put_request(client, fhir_cfg, token.as_deref(), event).body(body);

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

async fn sync_practitioner(
    client: &reqwest::Client,
    fhir_cfg: &FhirConfig,
    token: Option<String>,
    event: &SyncEvent,
    practitioner: &DomainPractitioner,
) -> Result<FhirResult, SyncFailure> {
    let mut fhir_practitioner = build_practitioner(practitioner, fhir_cfg);
    if event.op() == Op::Delete {
        fhir_practitioner.active = Some(false.into());
    }

    let body = fhirbolt::json::to_string(&fhir_practitioner, None)
        .context("serializing FHIR Practitioner")
        .map_err(SyncFailure::Permanent)?;

    let req = build_put_request(client, fhir_cfg, token.as_deref(), event).body(body);

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

    let identifier = format!("{}|{}", fhir_cfg.oscar_provider_system, practitioner.provider_no);
    info!(
        "fhir sink: synced {} -> {} (fhir_id={} version_id={:?})",
        event.idempotency_key(), identifier, result.fhir_id, result.version_id
    );
    Ok(result)
}

const OSCAR_APPOINTMENT_STATUS_SYSTEM: &str = "https://arsmedicatech.com/fhir/sid/oscar-appointment-status";
const OSCAR_BOOKING_SOURCE_URL: &str = "https://arsmedicatech.com/fhir/StructureDefinition/oscar-booking-source";

async fn sync_appointment(
    client: &reqwest::Client,
    fhir_cfg: &FhirConfig,
    token: Option<String>,
    event: &SyncEvent,
    appointment: &DomainAppointment,
    oscar_cfg: &OscarConfig,
) -> Result<FhirResult, SyncFailure> {
    let fhir_appointment = build_appointment(appointment, fhir_cfg, oscar_cfg, event.op())?;
    let bundle = build_appointment_bundle(&fhir_appointment, fhir_cfg, event);

    let body = fhirbolt::json::to_string(&bundle, None)
        .context("serializing FHIR transaction Bundle")
        .map_err(SyncFailure::Permanent)?;

    let base = fhir_cfg.base_url.trim_end_matches('/');
    let mut req = client
        .post(base)
        .header("Content-Type", "application/fhir+json");

    if let Some(token) = token {
        req = req.bearer_auth(token);
    }

    let resp = req
        .body(body)
        .send()
        .await
        .with_context(|| "sending transaction Bundle to HAPI")
        .map_err(SyncFailure::Retryable)?;
    let status = resp.status();

    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        let err = anyhow::anyhow!("HAPI transaction Bundle failed ({status}): {text}");
        if status.is_client_error() && status.as_u16() != 429 {
            return Err(SyncFailure::Permanent(err));
        }
        return Err(SyncFailure::Retryable(err));
    }

    let body_text = resp.text().await.unwrap_or_default();
    let mut result = FhirResult {
        fhir_id: String::new(),
        version_id: None,
    };

    if !body_text.trim().is_empty() {
        if let Ok(response_bundle) = fhirbolt::json::from_str::<Bundle>(&body_text, None) {
            if let Some(entry) = response_bundle.entry.first() {
                if let Some(response) = &entry.response {
                    let entry_status = response
                        .status
                        .value
                        .as_deref()
                        .unwrap_or("")
                        .to_ascii_lowercase();
                    if !entry_status.starts_with('2') {
                        return Err(SyncFailure::Permanent(anyhow::anyhow!(
                            "HAPI transaction Bundle entry failed with status '{entry_status}'"
                        )));
                    }

                    if let Some(loc) = response
                        .location
                        .as_ref()
                        .and_then(|l| l.value.as_deref())
                    {
                        result.fhir_id = parse_location_id(loc).unwrap_or_default();
                        result.version_id = parse_location_version_id(loc);
                    }

                    if result.fhir_id.is_empty() {
                        if let Some(etag) = response.etag.as_ref().and_then(|e| e.value.as_deref()) {
                            // W/"Appointment/123/_history/2" or just the id.
                            result.fhir_id = parse_location_id(etag).unwrap_or_default();
                            result.version_id = parse_location_version_id(etag);
                        }
                    }
                }
            }
        }
    }

    let identifier = format!("{}|{}", fhir_cfg.oscar_appointment_system, appointment.appointment_no);
    info!(
        "fhir sink: synced {} -> {} (fhir_id={} version_id={:?})",
        event.idempotency_key(), identifier, result.fhir_id, result.version_id
    );
    Ok(result)
}

async fn sync_care_team(
    client: &reqwest::Client,
    fhir_cfg: &FhirConfig,
    token: Option<String>,
    event: &SyncEvent,
    care_team: &DomainCareTeam,
) -> Result<FhirResult, SyncFailure> {
    // Deletions on the demographic row do not alter CareTeam membership:
    // removal is a human decision in AMT-Social (D2).
    if event.op() == Op::Delete {
        return Ok(FhirResult {
            fhir_id: String::new(),
            version_id: None,
        });
    }

    let base = fhir_cfg.base_url.trim_end_matches('/');
    let identifier = format!(
        "{}|{}",
        fhir_cfg.oscar_care_team_system, care_team.demographic_no
    );

    let (initial_id, initial_version, initial_resource) =
        match find_existing_care_team(client, fhir_cfg, token.as_deref(), base, &identifier).await? {
            Some(found) => found,
            None => {
                let fhir_ct = build_care_team(care_team, fhir_cfg);
                let bundle = build_care_team_create_bundle(&fhir_ct, fhir_cfg, event);
                let body = fhirbolt::json::to_string(&bundle, None)
                    .context("serializing CareTeam transaction Bundle")
                    .map_err(SyncFailure::Permanent)?;

                let mut req = client
                    .post(base)
                    .header("Content-Type", "application/fhir+json")
                    .body(body);
                if let Some(t) = token.as_deref() {
                    req = req.bearer_auth(t);
                }

                let resp = req
                    .send()
                    .await
                    .with_context(|| "sending CareTeam transaction Bundle")
                    .map_err(SyncFailure::Retryable)?;
                let status = resp.status();

                if !status.is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    return Err(classify_http_error(status, &text, "CareTeam create Bundle"));
                }

                let body_text = resp.text().await.unwrap_or_default();
                let result = parse_bundle_response(&body_text)?;
                info!(
                    "fhir sink: synced {} -> CareTeam {} (version_id={:?})",
                    event.idempotency_key(),
                    result.fhir_id,
                    result.version_id
                );
                return Ok(result);
            }
        };

    // Existing CareTeam: ensure the current MRP is present (D2).
    // We resolve the Oscar identifier to the set of matching Practitioner IDs, because HAPI may
    // store the participant as a literal `Practitioner/<id>` after resolving the conditional
    // reference. Without this, an exact-string compare would re-append the same MRP every sync.
    let prov_sys: String =
        url::form_urlencoded::byte_serialize(fhir_cfg.oscar_provider_system.as_bytes()).collect();
    let prov_val: String =
        url::form_urlencoded::byte_serialize(care_team.provider_no.as_bytes()).collect();
    let expected_member = format!("Practitioner?identifier={prov_sys}|{prov_val}");
    let practitioner_ids = resolve_practitioner_ids(
        client,
        token.as_deref(),
        base,
        &fhir_cfg.oscar_provider_system,
        &care_team.provider_no,
    )
    .await?;
    let already_present = initial_resource
        .participant
        .iter()
        .any(|p| participant_has_member(p, &expected_member, &practitioner_ids));

    if already_present {
        info!(
            "fhir sink: {} CareTeam {} already contains MRP {}; no-op",
            event.idempotency_key(),
            initial_id,
            care_team.provider_no
        );
        return Ok(FhirResult {
            fhir_id: initial_id,
            version_id: Some(initial_version),
        });
    }

    // Append the MRP and PUT with If-Match. On 409/412, re-read once and retry.
    let mut current_id = initial_id;
    let mut current_version = initial_version;
    let mut current_resource = initial_resource;

    for attempt in 0..2 {
        current_resource
            .participant
            .push(build_care_team_participant(care_team, fhir_cfg));

        let body = fhirbolt::json::to_string(&current_resource, None)
            .context("serializing updated CareTeam")
            .map_err(SyncFailure::Permanent)?;

        let url = format!("{}/CareTeam/{}", base, current_id);
        let mut req = client
            .put(&url)
            .header("Content-Type", "application/fhir+json")
            .header("If-Match", format!("W/\"{}\"", current_version))
            .body(body);
        if let Some(t) = token.as_deref() {
            req = req.bearer_auth(t);
        }

        let resp = req
            .send()
            .await
            .with_context(|| format!("PUT CareTeam/{}", current_id))
            .map_err(SyncFailure::Retryable)?;
        let status = resp.status();

        if status.is_success() {
            let body_text = resp.text().await.unwrap_or_default();
            let result = parse_care_team_put_response(&body_text, &current_id)?;
            info!(
                "fhir sink: synced {} -> CareTeam {} appended MRP {} (version_id={:?})",
                event.idempotency_key(),
                result.fhir_id,
                care_team.provider_no,
                result.version_id
            );
            return Ok(result);
        }

        let text = resp.text().await.unwrap_or_default();
        let code = status.as_u16();
        if attempt == 0 && (code == 409 || code == 412) {
            warn!(
                "fhir sink: CareTeam/{current_id} conflict on attempt {attempt}; re-reading and retrying"
            );
            if let Some((id, version, resource)) =
                find_existing_care_team(client, fhir_cfg, token.as_deref(), base, &identifier)
                    .await?
            {
                current_id = id;
                current_version = version;
                current_resource = resource;
                // If the winner already has the MRP, this is now a no-op.
                let practitioner_ids = resolve_practitioner_ids(
                    client,
                    token.as_deref(),
                    base,
                    &fhir_cfg.oscar_provider_system,
                    &care_team.provider_no,
                )
                .await?;
                if current_resource
                    .participant
                    .iter()
                    .any(|p| participant_has_member(p, &expected_member, &practitioner_ids))
                {
                    return Ok(FhirResult {
                        fhir_id: current_id,
                        version_id: Some(current_version),
                    });
                }
                continue;
            }
        }
        return Err(classify_http_error(status, &text, "CareTeam update"));
    }

    Err(SyncFailure::Retryable(anyhow::anyhow!(
        "CareTeam/{current_id} update conflict persisted after retry"
    )))
}

fn build_care_team_participant(care_team: &DomainCareTeam, fhir_cfg: &FhirConfig) -> CareTeamParticipant {
    let sys: String =
        url::form_urlencoded::byte_serialize(fhir_cfg.oscar_provider_system.as_bytes()).collect();
    let val: String =
        url::form_urlencoded::byte_serialize(care_team.provider_no.as_bytes()).collect();

    CareTeamParticipant {
        role: vec![CodeableConcept {
            coding: vec![Coding {
                system: Some("http://snomed.info/sct".to_string().into()),
                code: Some("446050000".to_string().into()),
                display: Some("Primary care physician".to_string().into()),
                ..Default::default()
            }],
            ..Default::default()
        }],
        member: Some(Box::new(Reference {
            reference: Some(format!("Practitioner?identifier={sys}|{val}").into()),
            ..Default::default()
        })),
        ..Default::default()
    }
}

fn build_care_team(care_team: &DomainCareTeam, fhir_cfg: &FhirConfig) -> CareTeam {
    let demo_sys: String =
        url::form_urlencoded::byte_serialize(fhir_cfg.oscar_demographic_system.as_bytes()).collect();
    let demo_val: String =
        url::form_urlencoded::byte_serialize(care_team.demographic_no.as_bytes()).collect();

    CareTeam {
        meta: Some(Box::new(Meta {
            source: Some(META_SOURCE.into()),
            ..Default::default()
        })),
        identifier: vec![Identifier {
            system: Some(fhir_cfg.oscar_care_team_system.clone().into()),
            value: Some(care_team.demographic_no.clone().into()),
            ..Default::default()
        }],
        status: Some("active".into()),
        category: vec![CodeableConcept {
            coding: vec![Coding {
                system: Some("http://loinc.org".to_string().into()),
                code: Some("LA28865-6".to_string().into()),
                display: Some("Longitudinal care-coordination focused care team".to_string().into()),
                ..Default::default()
            }],
            ..Default::default()
        }],
        subject: Some(Box::new(Reference {
            reference: Some(format!("Patient?identifier={demo_sys}|{demo_val}").into()),
            ..Default::default()
        })),
        participant: vec![build_care_team_participant(care_team, fhir_cfg)],
        ..Default::default()
    }
}

fn build_care_team_create_bundle(
    fhir_care_team: &CareTeam,
    fhir_cfg: &FhirConfig,
    event: &SyncEvent,
) -> Bundle {
    let sys: String =
        url::form_urlencoded::byte_serialize(fhir_cfg.oscar_care_team_system.as_bytes()).collect();
    let val: String =
        url::form_urlencoded::byte_serialize(event.payload().source_id().as_bytes()).collect();
    let conditional_url = format!("{}?identifier={sys}|{val}", event.resource_type().as_path());

    let mut bundle = Bundle::default();
    bundle.r#type = "transaction".into();
    bundle.entry.push(BundleEntry {
        full_url: Some(format!("urn:uuid:{}", event.idempotency_key()).into()),
        resource: Some(FhirResource::CareTeam(Box::new(fhir_care_team.clone()))),
        request: Some(BundleEntryRequest {
            method: "PUT".into(),
            url: conditional_url.into(),
            ..Default::default()
        }),
        ..Default::default()
    });
    bundle
}

async fn find_existing_care_team(
    client: &reqwest::Client,
    fhir_cfg: &FhirConfig,
    token: Option<&str>,
    base: &str,
    identifier: &str,
) -> Result<Option<(String, String, CareTeam)>, SyncFailure> {
    let mut req = client
        .get(format!("{}/CareTeam", base))
        .query(&[("identifier", identifier)])
        .header("Accept", "application/fhir+json");
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    let resp = req
        .send()
        .await
        .with_context(|| "searching CareTeam")
        .map_err(SyncFailure::Retryable)?;
    let status = resp.status();

    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        return Err(classify_http_error(status, &text, "CareTeam search"));
    }

    let body_text = resp.text().await.unwrap_or_default();
    // NOTE: this is a *search*-result Bundle (entry.search.mode = "match"/etc.),
    // not a transaction-response Bundle (entry.response = ...) like the other
    // fhirbolt::json::from_str::<Bundle> call sites. fhirbolt's Bundle model fails
    // on entry.search.mode as a bare FHIR `code` string, so walk the envelope as
    // plain JSON and hand fhirbolt only the `resource` object, which has no `search`
    // field to trip on.
    let raw: serde_json::Value = serde_json::from_str(&body_text)
        .with_context(|| "parsing CareTeam search Bundle as JSON")
        .map_err(SyncFailure::Retryable)?;
    let raw_entries = raw
        .get("entry")
        .and_then(|e| e.as_array())
        .cloned()
        .unwrap_or_default();

    for raw_entry in &raw_entries {
        let Some(resource_json) = raw_entry.get("resource") else {
            continue;
        };
        if resource_json.get("resourceType").and_then(|v| v.as_str()) != Some("CareTeam") {
            continue;
        }
        let ct: CareTeam = fhirbolt::json::from_str(&resource_json.to_string(), None)
            .with_context(|| "parsing CareTeam resource from search entry")
            .map_err(SyncFailure::Retryable)?;

        let Some(id) = ct.id.as_ref().and_then(|i| i.value.clone()) else {
            continue;
        };
        let Some(version_id) = ct
            .meta
            .as_ref()
            .and_then(|m| m.version_id.as_ref())
            .and_then(|v| v.value.clone())
        else {
            continue;
        };
        // D5: only touch CareTeams carrying our Oscar identifier and source stamp.
        let has_oscar_identifier = ct.identifier.iter().any(|i| {
            i.system
                .as_ref()
                .and_then(|s| s.value.as_deref())
                == Some(fhir_cfg.oscar_care_team_system.as_str())
                && i.value.as_ref().and_then(|v| v.value.as_deref()).is_some()
        });
        let has_sync_source = ct
            .meta
            .as_ref()
            .and_then(|m| m.source.as_ref())
            .and_then(|s| s.value.as_deref())
            .map(|s| s.starts_with(META_SOURCE))
            .unwrap_or(false);
        if has_oscar_identifier && has_sync_source {
            return Ok(Some((id, version_id, ct)));
        }
    }

    Ok(None)
}

/// Searches HAPI for Practitioners carrying the given Oscar identifier and returns their
/// literal resource IDs. This is used for identity-aware participant matching: HAPI may store
/// the participant reference as `Practitioner/<id>` after resolving the conditional reference
/// server-side, so an exact string compare against `Practitioner?identifier=...` would miss it.
/// Parsed as plain JSON because search-result Bundles carry `entry.search.mode`, which fhirbolt's
/// Bundle model fails on (see `find_existing_care_team`).
async fn resolve_practitioner_ids(
    client: &reqwest::Client,
    token: Option<&str>,
    base: &str,
    system: &str,
    value: &str,
) -> Result<HashSet<String>, SyncFailure> {
    let identifier = format!("{}|{}", system, value);
    let mut req = client
        .get(format!("{}/Practitioner", base))
        .query(&[("identifier", identifier.as_str())])
        .header("Accept", "application/fhir+json");
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    let resp = req
        .send()
        .await
        .with_context(|| "searching Practitioner by Oscar identifier")
        .map_err(SyncFailure::Retryable)?;
    let status = resp.status();

    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        return Err(classify_http_error(status, &text, "Practitioner identity search"));
    }

    let body_text = resp.text().await.unwrap_or_default();
    let raw: serde_json::Value = serde_json::from_str(&body_text)
        .with_context(|| "parsing Practitioner search Bundle as JSON")
        .map_err(SyncFailure::Retryable)?;
    let raw_entries = raw
        .get("entry")
        .and_then(|e| e.as_array())
        .cloned()
        .unwrap_or_default();

    let mut ids = HashSet::new();
    for raw_entry in raw_entries {
        let Some(resource_json) = raw_entry.get("resource") else {
            continue;
        };
        if resource_json.get("resourceType").and_then(|v| v.as_str()) != Some("Practitioner") {
            continue;
        }
        if let Some(id) = resource_json.get("id").and_then(|v| v.as_str()) {
            ids.insert(id.to_string());
        }
    }

    Ok(ids)
}

fn participant_has_member(
    participant: &CareTeamParticipant,
    expected_conditional: &str,
    practitioner_ids: &HashSet<String>,
) -> bool {
    let Some(reference) = participant
        .member
        .as_ref()
        .and_then(|m| m.reference.as_ref())
        .and_then(|r| r.value.as_deref())
    else {
        return false;
    };

    if reference == expected_conditional {
        return true;
    }

    // HAPI may rewrite a conditional reference to a literal reference after resolving
    // it server-side. Accept literal references whose target Practitioner carries the
    // expected Oscar identifier.
    if let Some(id) = reference.rsplit_once("/Practitioner/").map(|(_, id)| id) {
        return practitioner_ids.contains(id);
    }
    if let Some(id) = reference.strip_prefix("Practitioner/") {
        return practitioner_ids.contains(id);
    }

    false
}

fn classify_http_error(status: reqwest::StatusCode, text: &str, context: &str) -> SyncFailure {
    let err = anyhow::anyhow!("{context} failed ({status}): {text}");
    if status.is_client_error() && status.as_u16() != 429 {
        SyncFailure::Permanent(err)
    } else {
        SyncFailure::Retryable(err)
    }
}

fn parse_bundle_response(body_text: &str) -> Result<FhirResult, SyncFailure> {
    let mut result = FhirResult {
        fhir_id: String::new(),
        version_id: None,
    };

    if !body_text.trim().is_empty() {
        if let Ok(response_bundle) = fhirbolt::json::from_str::<Bundle>(body_text, None) {
            if let Some(entry) = response_bundle.entry.first() {
                if let Some(response) = &entry.response {
                    let entry_status = response
                        .status
                        .value
                        .as_deref()
                        .unwrap_or("")
                        .to_ascii_lowercase();
                    if !entry_status.starts_with('2') {
                        return Err(SyncFailure::Permanent(anyhow::anyhow!(
                            "CareTeam transaction Bundle entry failed with status '{entry_status}'"
                        )));
                    }

                    if let Some(loc) = response
                        .location
                        .as_ref()
                        .and_then(|l| l.value.as_deref())
                    {
                        result.fhir_id = parse_location_id(loc).unwrap_or_default();
                        result.version_id = parse_location_version_id(loc);
                    }

                    if result.fhir_id.is_empty() {
                        if let Some(etag) = response.etag.as_ref().and_then(|e| e.value.as_deref()) {
                            result.fhir_id = parse_location_id(etag).unwrap_or_default();
                            result.version_id = parse_location_version_id(etag);
                        }
                    }
                }
            }
        }
    }

    Ok(result)
}

fn parse_care_team_put_response(body_text: &str, fallback_id: &str) -> Result<FhirResult, SyncFailure> {
    let mut result = FhirResult {
        fhir_id: fallback_id.to_string(),
        version_id: None,
    };

    if !body_text.trim().is_empty() {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(body_text) {
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

fn build_practitioner(payload: &DomainPractitioner, cfg: &FhirConfig) -> Practitioner {
    let mut practitioner = Practitioner::default();

    practitioner.meta = Some(Box::new(Meta {
        source: Some(META_SOURCE.into()),
        ..Default::default()
    }));

    practitioner.identifier.push(Identifier {
        system: Some(cfg.oscar_provider_system.clone().into()),
        value: Some(payload.provider_no.clone().into()),
        ..Default::default()
    });

    if let Some(billing) = &payload.billing_no {
        practitioner.identifier.push(Identifier {
            system: Some(cfg.bc_msp_practitioner_system.clone().into()),
            value: Some(billing.clone().into()),
            ..Default::default()
        });
    }

    if let (Some(no), Some(type_)) = (&payload.practitioner_no, &payload.practitioner_no_type) {
        if !no.is_empty() && !type_.is_empty() {
            practitioner.identifier.push(Identifier {
                r#type: Some(Box::new(CodeableConcept {
                    text: Some(type_.clone().into()),
                    ..Default::default()
                })),
                value: Some(no.clone().into()),
                ..Default::default()
            });
        }
    }

    if payload.first_name.is_some() || payload.last_name.is_some() || payload.title.is_some() {
        practitioner.name.push(HumanName {
            prefix: payload
                .title
                .clone()
                .map(|t| vec![t.into()])
                .unwrap_or_default(),
            given: payload
                .first_name
                .clone()
                .map(|g| vec![g.into()])
                .unwrap_or_default(),
            family: payload.last_name.clone().map(Into::into),
            ..Default::default()
        });
    }

    practitioner.gender = Some(map_gender(payload.sex.as_deref()).into());

    if let Some(dob) = &payload.date_of_birth {
        practitioner.birth_date = Some(dob.clone().into());
    }

    if let Some(email) = &payload.email {
        practitioner.telecom.push(ContactPoint {
            system: Some("email".into()),
            value: Some(email.clone().into()),
            ..Default::default()
        });
    }

    if let Some(phone) = &payload.phone {
        practitioner.telecom.push(ContactPoint {
            r#use: Some("home".into()),
            system: Some("phone".into()),
            value: Some(phone.clone().into()),
            ..Default::default()
        });
    }

    if let Some(work_phone) = &payload.work_phone {
        practitioner.telecom.push(ContactPoint {
            r#use: Some("work".into()),
            system: Some("phone".into()),
            value: Some(work_phone.clone().into()),
            ..Default::default()
        });
    }

    if let Some(addr) = &payload.address {
        practitioner.address.push(Address {
            text: Some(addr.clone().into()),
            ..Default::default()
        });
    }

    practitioner.active = Some(practitioner_active(payload).into());

    practitioner
}

fn practitioner_active(payload: &DomainPractitioner) -> bool {
    match payload.status.as_deref().map(|s| s.trim()) {
        Some("1") => true,
        Some("0") => false,
        Some(other) => {
            warn!(
                "build_practitioner: unexpected status '{}' for provider_no {}",
                other, payload.provider_no
            );
            true
        }
        None => true,
    }
}

/// Converts a naive local `appointment_date` + `start_time`/`end_time` into an
/// RFC 3339 string using the configured IANA timezone (D5).
///
/// Returns `SyncFailure::Permanent` for nonexistent local times. For ambiguous
/// times (fall-back) the first (DST) candidate is chosen and a `warn!` names
/// both candidates.
fn to_appointment_instant(
    date: &str,
    time: &str,
    tz_name: &str,
) -> Result<String, SyncFailure> {
    let naive_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|e| SyncFailure::Permanent(anyhow::anyhow!("invalid appointment_date '{date}': {e}")))?;
    let naive_time = NaiveTime::parse_from_str(time, "%H:%M:%S%.f")
    .map_err(|e| SyncFailure::Permanent(anyhow::anyhow!("invalid start/end_time '{time}': {e}")))?;
    let naive = naive_date.and_time(naive_time);

    let tz: chrono_tz::Tz = tz_name
        .parse()
        .map_err(|_| SyncFailure::Permanent(anyhow::anyhow!("invalid IANA timezone '{tz_name}'")))?;

    let dt = match tz.from_local_datetime(&naive) {
        LocalResult::Single(dt) => dt,
        LocalResult::Ambiguous(dt1, dt2) => {
            warn!(
                "appointment time {naive} is ambiguous in {tz_name}: candidates {} and {}; taking first",
                dt1.to_rfc3339(), dt2.to_rfc3339()
            );
            dt1
        }
        LocalResult::None => {
            return Err(SyncFailure::Permanent(anyhow::anyhow!(
                "nonexistent_local_time: {naive} {tz_name}"
            )));
        }
    };

    Ok(dt.to_rfc3339())
}

/// Computes `minutesDuration` from the naive local start and end times (D5).
fn appointment_minutes_duration(start: &str, end: &str) -> Option<u32> {
    let start = NaiveTime::parse_from_str(start, "%H:%M:%S%.f").ok()?;
    let end = NaiveTime::parse_from_str(end, "%H:%M:%S%.f").ok()?;
    let seconds = (end - start).num_seconds();
    if seconds > 0 { Some((seconds / 60) as u32) } else { None }
}

fn build_appointment(
    payload: &DomainAppointment,
    fhir_cfg: &FhirConfig,
    oscar_cfg: &OscarConfig,
    op: Op,
) -> Result<Appointment, SyncFailure> {
    let mut appt = Appointment::default();

    appt.meta = Some(Box::new(Meta {
        source: Some(META_SOURCE.into()),
        ..Default::default()
    }));

    appt.identifier.push(Identifier {
        system: Some(fhir_cfg.oscar_appointment_system.clone().into()),
        value: Some(payload.appointment_no.clone().into()),
        ..Default::default()
    });

    let raw_status = payload.status.as_deref().map(|s| s.trim());

    if let Some(code) = raw_status {
        appt.identifier.push(Identifier {
            system: Some(OSCAR_APPOINTMENT_STATUS_SYSTEM.into()),
            value: Some(code.into()),
            ..Default::default()
        });
    }

    let fhir_status = if op == Op::Delete {
        "cancelled"
    } else {
        match raw_status {
            Some(code) => oscar_cfg
                .appointment_status_map
                .get(code)
                .map(String::as_str)
                .ok_or_else(|| SyncFailure::Permanent(anyhow::anyhow!("unmapped_appointment_status: {code}")))?,
            None => {
                return Err(SyncFailure::Permanent(anyhow::anyhow!(
                    "appointment_no={} has no status",
                    payload.appointment_no
                )));
            }
        }
    };
    appt.status = fhir_status.into();

    let tz = oscar_cfg
        .timezone
        .as_deref()
        .ok_or_else(|| SyncFailure::Permanent(anyhow::anyhow!("missing oscar timezone")))?;

    if let (Some(date), Some(start)) = (payload.appointment_date.as_deref(), payload.start_time.as_deref()) {
        appt.start = Some(to_appointment_instant(date, start, tz)?.into());
    }

    if let (Some(date), Some(end)) = (payload.appointment_date.as_deref(), payload.end_time.as_deref()) {
        appt.end = Some(to_appointment_instant(date, end, tz)?.into());
    }

    if let (Some(start), Some(end)) = (payload.start_time.as_deref(), payload.end_time.as_deref()) {
        if let Some(minutes) = appointment_minutes_duration(start, end) {
            appt.minutes_duration = Some(minutes.into());
        }
    }

    if let Some(type_) = &payload.type_ {
        appt.appointment_type = Some(Box::new(CodeableConcept {
            text: Some(type_.clone().into()),
            ..Default::default()
        }));
    }

    if let Some(reason) = &payload.reason {
        appt.reason_code.push(CodeableConcept {
            text: Some(reason.clone().into()),
            ..Default::default()
        });
    }

    match (&payload.notes, &payload.remarks) {
        (Some(notes), Some(remarks)) => {
            appt.comment = Some(format!("{notes} / {remarks}").into());
        }
        (Some(notes), None) => appt.comment = Some(notes.clone().into()),
        (None, Some(remarks)) => appt.comment = Some(remarks.clone().into()),
        (None, None) => {}
    }

    if let Some(urgency) = &payload.urgency {
        if let Ok(value) = urgency.trim().parse::<u32>() {
            appt.priority = Some(value.into());
        } else {
            warn!(
                "build_appointment: non-numeric urgency '{}' for appointment_no={}",
                urgency, payload.appointment_no
            );
        }
    }

    if let Some(created) = &payload.createdatetime {
        appt.created = Some(created.replace(' ', "T").into());
    }

    if let Some(source) = &payload.booking_source {
        appt.extension.push(Extension {
            url: OSCAR_BOOKING_SOURCE_URL.to_string(),
            value: Some(ExtensionValue::String(source.clone().into())),
            ..Default::default()
        });
    }

    let mut participants = Vec::new();

    if let Some(demographic_no) = &payload.demographic_no {
        let sys: String = url::form_urlencoded::byte_serialize(fhir_cfg.oscar_demographic_system.as_bytes()).collect();
        let val: String = url::form_urlencoded::byte_serialize(demographic_no.as_bytes()).collect();
        participants.push(AppointmentParticipant {
            actor: Some(Box::new(Reference {
                reference: Some(format!("Patient?identifier={sys}|{val}").into()),
                ..Default::default()
            })),
            required: Some("required".into()),
            status: "accepted".into(),
            ..Default::default()
        });
    }

    if let Some(provider_no) = &payload.provider_no {
        let sys: String = url::form_urlencoded::byte_serialize(fhir_cfg.oscar_provider_system.as_bytes()).collect();
        let val: String = url::form_urlencoded::byte_serialize(provider_no.as_bytes()).collect();
        participants.push(AppointmentParticipant {
            actor: Some(Box::new(Reference {
                reference: Some(format!("Practitioner?identifier={sys}|{val}").into()),
                ..Default::default()
            })),
            required: Some("required".into()),
            status: "accepted".into(),
            ..Default::default()
        });
    }

    if participants.is_empty() {
        return Err(SyncFailure::Permanent(anyhow::anyhow!(
            "unmappable_appointment: appointment_no={} has no participants",
            payload.appointment_no
        )));
    }
    appt.participant = participants;

    Ok(appt)
}

/// Builds a transaction Bundle containing a single conditional PUT for the
/// supplied Appointment (D4). Using a transaction lets HAPI resolve the
/// conditional `Patient?identifier=...` and `Practitioner?identifier=...`
/// references inside the `Appointment.participant` list.
fn build_appointment_bundle(
    fhir_appointment: &Appointment,
    fhir_cfg: &FhirConfig,
    event: &SyncEvent,
) -> Bundle {
    let sys: String =
        url::form_urlencoded::byte_serialize(fhir_cfg.oscar_appointment_system.as_bytes()).collect();
    let val: String =
        url::form_urlencoded::byte_serialize(event.payload().source_id().as_bytes()).collect();
    let conditional_url = format!("{}?identifier={sys}|{val}", event.resource_type().as_path());

    let mut bundle = Bundle::default();
    bundle.r#type = "transaction".into();
    bundle.entry.push(BundleEntry {
        full_url: Some(format!("urn:uuid:{}", event.idempotency_key()).into()),
        resource: Some(FhirResource::Appointment(Box::new(fhir_appointment.clone()))),
        request: Some(BundleEntryRequest {
            method: "PUT".into(),
            url: conditional_url.into(),
            ..Default::default()
        }),
        ..Default::default()
    });
    bundle
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
            oscar_demographic_system: "https://arsmedicatech.com/fhir/sid/oscar-demographic"
                .to_string(),
            oscar_provider_system: "https://arsmedicatech.com/fhir/sid/oscar-provider".to_string(),
            oscar_appointment_system: "https://arsmedicatech.com/fhir/sid/oscar-appointment".to_string(),
            bc_phn_system: "https://fhir.infoway-inforoute.ca/NamingSystem/ca-bc-patient-healthcare-id".to_string(),
            bc_msp_practitioner_system: "https://fhir.infoway-inforoute.ca/NamingSystem/ca-bc-provider-billing-number".to_string(),
            token_env: None,
            keycloak: None,
            ..Default::default()
        }
    }

    fn oscar_cfg() -> OscarConfig {
        OscarConfig {
            timezone: Some("America/Vancouver".to_string()),
            region: Some("BC".to_string()),
            appointment_status_map: crate::config::OscarConfig::default().appointment_status_map,
            default_mrp_provider_no: None,
            care_team_enabled: true,
        }
    }

    fn appt_payload() -> DomainAppointment {
        DomainAppointment {
            appointment_no: "1".to_string(),
            demographic_no: Some("101".to_string()),
            provider_no: Some("100001".to_string()),
            appointment_date: Some("2026-08-10".to_string()),
            start_time: Some("09:00:00".to_string()),
            end_time: Some("09:15:00".to_string()),
            status: Some("t".to_string()),
            reason: Some("Follow-up".to_string()),
            notes: Some("note one".to_string()),
            remarks: Some("note two".to_string()),
            urgency: Some("3".to_string()),
            createdatetime: Some("2026-08-09 16:30:00".to_string()),
            location: Some("Room A".to_string()),
            booking_source: Some("online".to_string()),
            type_: Some("Regular".to_string()),
        }
    }

    #[test]
    fn build_appointment_maps_all_fields() {
        let appt = build_appointment(&appt_payload(), &fhir_cfg(), &oscar_cfg(), Op::Upsert).unwrap();

        assert_eq!(appt.identifier.len(), 2);
        assert_eq!(appt.identifier[0].value, Some("1".to_string().into()));
        assert_eq!(appt.identifier[1].value, Some("t".to_string().into()));
        assert_eq!(appt.status.value, Some("booked".to_string()));
        assert_eq!(appt.start.as_ref().map(|i| i.value.clone()), Some(Some("2026-08-10T09:00:00-07:00".to_string())));
        assert_eq!(appt.end.as_ref().map(|i| i.value.clone()), Some(Some("2026-08-10T09:15:00-07:00".to_string())));
        assert_eq!(appt.minutes_duration.as_ref().map(|m| m.value), Some(Some(15)));
        assert_eq!(appt.appointment_type.as_ref().and_then(|c| c.text.as_ref()).and_then(|s| s.value.clone()), Some("Regular".to_string()));
        assert_eq!(appt.reason_code.len(), 1);
        assert_eq!(appt.comment.as_ref().map(|s| s.value.clone()), Some(Some("note one / note two".to_string())));
        assert_eq!(appt.priority.as_ref().map(|u| u.value), Some(Some(3)));
        assert_eq!(appt.created.as_ref().map(|d| d.value.clone()), Some(Some("2026-08-09T16:30:00".to_string())));
        assert_eq!(appt.extension.len(), 1);
        assert_eq!(appt.participant.len(), 2);
        assert!(appt.participant[0].actor.as_ref().unwrap().reference.as_ref().unwrap().value.as_ref().unwrap().starts_with("Patient?identifier"));
        assert!(appt.participant[1].actor.as_ref().unwrap().reference.as_ref().unwrap().value.as_ref().unwrap().starts_with("Practitioner?identifier"));
    }

    #[test]
    fn build_appointment_bundle_contains_conditional_put() {
        let cfg = fhir_cfg();
        let payload = appt_payload();
        let event = SyncEvent::new(
            Source::OscarBinlog { table: "appointment".to_string() },
            Op::Upsert,
            DomainResource::Appointment(payload.clone()),
            chrono::Utc::now(),
        );

        let appt = build_appointment(&payload, &cfg, &oscar_cfg(), Op::Upsert).unwrap();
        let bundle = build_appointment_bundle(&appt, &cfg, &event);

        assert_eq!(bundle.r#type.value.as_deref(), Some("transaction"));
        assert_eq!(bundle.entry.len(), 1);

        let request = bundle.entry[0].request.as_ref().unwrap();
        assert_eq!(request.method.value.as_deref(), Some("PUT"));
        let url = request.url.value.as_deref().unwrap();
        assert!(url.starts_with("Appointment?identifier="));
        assert!(url.contains("oscar-appointment"));
        assert!(url.contains("1"));

        assert!(matches!(
            bundle.entry[0].resource,
            Some(FhirResource::Appointment(_))
        ));
    }

    #[test]
    fn build_appointment_delete_sets_cancelled() {
        let appt = build_appointment(&appt_payload(), &fhir_cfg(), &oscar_cfg(), Op::Delete).unwrap();
        assert_eq!(appt.status.value, Some("cancelled".to_string()));
    }

    #[test]
    fn build_appointment_unmapped_status_dead_letters() {
        let mut payload = appt_payload();
        payload.status = Some("a".to_string());
        let err = build_appointment(&payload, &fhir_cfg(), &oscar_cfg(), Op::Upsert).unwrap_err();
        assert!(err.to_string().contains("unmapped_appointment_status"));
    }

    #[test]
    fn build_appointment_nonexistent_time_dead_letters() {
        let mut payload = appt_payload();
        payload.appointment_date = Some("2026-03-08".to_string());
        payload.start_time = Some("02:30:00".to_string());
        payload.end_time = Some("02:45:00".to_string());
        let err = build_appointment(&payload, &fhir_cfg(), &oscar_cfg(), Op::Upsert).unwrap_err();
        assert!(err.to_string().contains("nonexistent_local_time"));
    }

    #[test]
    fn build_appointment_ambiguous_time_takes_first_candidate() {
        let mut payload = appt_payload();
        payload.appointment_date = Some("2026-11-01".to_string());
        payload.start_time = Some("01:30:00".to_string());
        payload.end_time = Some("01:45:00".to_string());
        let appt = build_appointment(&payload, &fhir_cfg(), &oscar_cfg(), Op::Upsert).unwrap();
        assert!(appt.start.as_ref().unwrap().value.as_ref().unwrap().contains("-07:00"));
        assert!(appt.end.as_ref().unwrap().value.as_ref().unwrap().contains("-07:00"));
    }

    #[test]
    fn build_appointment_omits_practitioner_participant() {
        let mut payload = appt_payload();
        payload.provider_no = None;
        let appt = build_appointment(&payload, &fhir_cfg(), &oscar_cfg(), Op::Upsert).unwrap();
        assert_eq!(appt.participant.len(), 1);
        assert!(appt.participant[0].actor.as_ref().unwrap().reference.as_ref().unwrap().value.as_ref().unwrap().starts_with("Patient"));
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
    fn build_practitioner_maps_all_in_scope_fields() {
        let payload = DomainPractitioner {
            provider_no: "1001".to_string(),
            billing_no: Some("B1001".to_string()),
            practitioner_no: Some("PN-1".to_string()),
            practitioner_no_type: Some("College".to_string()),
            ohip_no: None,
            title: Some("Dr".to_string()),
            first_name: Some("Alice".to_string()),
            last_name: Some("Ng".to_string()),
            sex: Some("F".to_string()),
            date_of_birth: Some("1980-04-15".to_string()),
            phone: Some("604-555-0100".to_string()),
            email: Some("alice@example.com".to_string()),
            work_phone: Some("604-555-0200".to_string()),
            address: Some("123 Main St".to_string()),
            status: Some("1".to_string()),
        };

        let p = build_practitioner(&payload, &fhir_cfg());
        assert_eq!(p.identifier.len(), 3);
        assert_eq!(p.identifier[0].system.as_ref().and_then(|s| s.value.as_deref()), Some("https://arsmedicatech.com/fhir/sid/oscar-provider"));
        assert_eq!(p.identifier[0].value.as_ref().and_then(|s| s.value.as_deref()), Some("1001"));
        assert_eq!(p.identifier[1].system.as_ref().and_then(|s| s.value.as_deref()), Some("https://fhir.infoway-inforoute.ca/NamingSystem/ca-bc-provider-billing-number"));
        assert_eq!(p.identifier[1].value.as_ref().and_then(|s| s.value.as_deref()), Some("B1001"));
        assert_eq!(p.identifier[2].value.as_ref().and_then(|s| s.value.as_deref()), Some("PN-1"));
        assert_eq!(p.name.len(), 1);
        assert_eq!(p.name[0].prefix.first().and_then(|s| s.value.as_deref()), Some("Dr"));
        assert_eq!(p.name[0].given.first().and_then(|s| s.value.as_deref()), Some("Alice"));
        assert_eq!(p.name[0].family.as_ref().and_then(|s| s.value.as_deref()), Some("Ng"));
        assert_eq!(p.gender.as_ref().and_then(|c| c.value.as_deref()), Some("female"));
        assert_eq!(p.birth_date.as_ref().and_then(|d| d.value.as_deref()), Some("1980-04-15"));
        assert_eq!(p.telecom.len(), 3);
        assert_eq!(p.address.len(), 1);
        assert_eq!(p.active.as_ref().map(|b| b.value).flatten(), Some(true));
    }

    #[test]
    fn build_practitioner_marks_inactive_for_status_zero() {
        let payload = DomainPractitioner {
            provider_no: "1003".to_string(),
            billing_no: None,
            practitioner_no: None,
            practitioner_no_type: None,
            ohip_no: None,
            title: None,
            first_name: None,
            last_name: None,
            sex: None,
            date_of_birth: None,
            phone: None,
            email: None,
            work_phone: None,
            address: None,
            status: Some("0".to_string()),
        };

        let p = build_practitioner(&payload, &fhir_cfg());
        assert_eq!(p.active.as_ref().map(|b| b.value).flatten(), Some(false));
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
                    "https://arsmedicatech.com/fhir/sid/oscar-demographic".into(),
                oscar_provider_system: "https://arsmedicatech.com/fhir/sid/oscar-provider".into(),
                oscar_appointment_system: "https://arsmedicatech.com/fhir/sid/oscar-appointment".into(),
                bc_phn_system: "https://fhir.infoway-inforoute.ca/NamingSystem/ca-bc-patient-healthcare-id".into(),
                bc_msp_practitioner_system: "https://fhir.infoway-inforoute.ca/NamingSystem/ca-bc-provider-billing-number".into(),
                token_env: None,
                keycloak: None,
                ..Default::default()
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
            writeback: WritebackConfig::default(),
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

    #[test]
    fn build_care_team_has_required_shape() {
        let cfg = fhir_cfg();
        let ct = build_care_team(
            &DomainCareTeam {
                demographic_no: "101".to_string(),
                provider_no: "P-001".to_string(),
            },
            &cfg,
        );

        assert_eq!(ct.identifier.len(), 1);
        assert_eq!(
            ct.identifier[0].system.as_ref().and_then(|s| s.value.as_deref()),
            Some(cfg.oscar_care_team_system.as_str())
        );
        assert_eq!(
            ct.identifier[0].value.as_ref().and_then(|v| v.value.as_deref()),
            Some("101")
        );
        assert_eq!(
            ct.status.as_ref().and_then(|s| s.value.as_deref()),
            Some("active")
        );
        assert_eq!(ct.category.len(), 1);
        let coding = ct.category[0].coding.first().unwrap();
        assert_eq!(
            coding.system.as_ref().and_then(|s| s.value.as_deref()),
            Some("http://loinc.org")
        );
        assert_eq!(
            coding.code.as_ref().and_then(|c| c.value.as_deref()),
            Some("LA28865-6")
        );
        let subject_ref = ct
            .subject
            .as_ref()
            .and_then(|s| s.reference.as_ref())
            .and_then(|r| r.value.as_deref())
            .unwrap_or("");
        assert!(subject_ref.starts_with("Patient?identifier="), "subject must be a Patient conditional reference: {subject_ref}");
        assert!(subject_ref.contains("101"));

        assert_eq!(ct.participant.len(), 1);
        let member_ref = ct.participant[0]
            .member
            .as_ref()
            .and_then(|m| m.reference.as_ref())
            .and_then(|r| r.value.as_deref())
            .unwrap_or("");
        assert!(member_ref.starts_with("Practitioner?identifier="));
        assert!(member_ref.contains("P-001"));

        assert_eq!(
            ct.meta.as_ref()
                .and_then(|m| m.source.as_ref())
                .and_then(|s| s.value.as_deref()),
            Some(META_SOURCE)
        );
    }

    #[test]
    fn care_team_idempotency_key_differs_from_patient_for_same_row() {
        let now = chrono::Utc::now();
        let patient_event = SyncEvent::new(
            Source::OscarBinlog { table: "demographic".to_string() },
            Op::Upsert,
            DomainResource::Patient(DomainPatient {
                demographic_no: "101".to_string(),
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
            now,
        );
        let care_team_event = SyncEvent::new(
            Source::OscarBinlog { table: "demographic".to_string() },
            Op::Upsert,
            DomainResource::CareTeam(DomainCareTeam {
                demographic_no: "101".to_string(),
                provider_no: "P-001".to_string(),
            }),
            now,
        );
        assert_ne!(patient_event.idempotency_key(), care_team_event.idempotency_key());
    }
}
