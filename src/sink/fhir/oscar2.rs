use anyhow::Context;
use base64ct::{Base64, Encoding};
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use fhirbolt::model::r4b::resources::{
    Bundle, BundleEntry, BundleEntryRequest, Condition, ConditionAbatement, ConditionOnset,
    DocumentReference, DocumentReferenceContent, DocumentReferenceContext, Encounter,
    EncounterParticipant, FamilyMemberHistory, FamilyMemberHistoryCondition,
    FamilyMemberHistoryConditionOnset,
};
use fhirbolt::model::r4b::Resource as FhirResource;
use fhirbolt::model::r4b::types::{
    Age, Annotation, Attachment, CodeableConcept, Coding, Identifier, Meta, Period, Reference,
};
use tracing::{info, warn};

use crate::domain::condition::{DomainCondition, DomainFamilyMemberHistory};
use crate::domain::document_reference::DomainDocumentReference;
use crate::domain::encounter::DomainEncounter;
use crate::event::ResourceType;

use super::{FhirConfig, FhirResult, OscarConfig, SyncEvent, SyncFailure, parse_location_id, parse_location_version_id, META_SOURCE};

const ACT_CODE_SYSTEM: &str = "http://terminology.hl7.org/CodeSystem/v3-ActCode";
const PARTICIPATION_TYPE_SYSTEM: &str = "http://terminology.hl7.org/CodeSystem/v3-ParticipationType";
const CONDITION_CLINICAL_SYSTEM: &str = "http://terminology.hl7.org/CodeSystem/condition-clinical";
const CONDITION_VERIFICATION_SYSTEM: &str = "http://terminology.hl7.org/CodeSystem/condition-ver-status";
const UOM_SYSTEM: &str = "http://unitsofmeasure.org";

// ---------------------------------------------------------------------------
// Public sync entry points called from `fhir.rs` `sync_one`.
// ---------------------------------------------------------------------------

pub(super) async fn sync_encounter(
    client: &reqwest::Client,
    fhir_cfg: &FhirConfig,
    token: Option<String>,
    event: &SyncEvent,
    encounter: &DomainEncounter,
    oscar_cfg: &OscarConfig,
) -> Result<FhirResult, SyncFailure> {
    let fhir_encounter = build_encounter(encounter, fhir_cfg, oscar_cfg)?;
    let bundle = build_conditional_put_bundle(
        FhirResource::Encounter(Box::new(fhir_encounter)),
        &fhir_cfg.oscar_note_system,
        event,
    );
    let result = send_transaction_bundle(client, fhir_cfg, token, &bundle).await?;
    let identifier = format!("{}|{}", fhir_cfg.oscar_note_system, event.payload().source_id());
    info!(
        "fhir sink: synced {} -> {} (fhir_id={} version_id={:?})",
        event.idempotency_key(), identifier, result.fhir_id, result.version_id
    );
    Ok(result)
}

pub(super) async fn sync_document_reference(
    client: &reqwest::Client,
    fhir_cfg: &FhirConfig,
    token: Option<String>,
    event: &SyncEvent,
    doc: &DomainDocumentReference,
    oscar_cfg: &OscarConfig,
) -> Result<FhirResult, SyncFailure> {
    let fhir_doc = build_document_reference(doc, fhir_cfg, oscar_cfg)?;
    let bundle = build_conditional_put_bundle(
        FhirResource::DocumentReference(Box::new(fhir_doc)),
        &fhir_cfg.oscar_note_document_system,
        event,
    );
    let result = send_transaction_bundle(client, fhir_cfg, token, &bundle).await?;
    let identifier = format!("{}|{}", fhir_cfg.oscar_note_document_system, event.payload().source_id());
    info!(
        "fhir sink: synced {} -> {} (fhir_id={} version_id={:?})",
        event.idempotency_key(), identifier, result.fhir_id, result.version_id
    );
    Ok(result)
}

pub(super) async fn sync_condition(
    client: &reqwest::Client,
    fhir_cfg: &FhirConfig,
    token: Option<String>,
    event: &SyncEvent,
    condition: &DomainCondition,
) -> Result<FhirResult, SyncFailure> {
    let fhir_condition = build_condition(condition, fhir_cfg)?;
    let identifier_system = if condition.source_table == "dxresearch" {
        &fhir_cfg.oscar_dxresearch_system
    } else {
        &fhir_cfg.oscar_cpp_condition_system
    };
    let bundle = build_conditional_put_bundle(
        FhirResource::Condition(Box::new(fhir_condition)),
        identifier_system,
        event,
    );
    let result = send_transaction_bundle(client, fhir_cfg, token, &bundle).await?;
    let identifier = format!("{}|{}", identifier_system, event.payload().source_id());
    info!(
        "fhir sink: synced {} -> {} (fhir_id={} version_id={:?})",
        event.idempotency_key(), identifier, result.fhir_id, result.version_id
    );
    Ok(result)
}

pub(super) async fn sync_family_member_history(
    client: &reqwest::Client,
    fhir_cfg: &FhirConfig,
    token: Option<String>,
    event: &SyncEvent,
    fh: &DomainFamilyMemberHistory,
) -> Result<FhirResult, SyncFailure> {
    let fhir_fh = build_family_member_history(fh, fhir_cfg)?;
    let bundle = build_conditional_put_bundle(
        FhirResource::FamilyMemberHistory(Box::new(fhir_fh)),
        &fhir_cfg.oscar_cpp_condition_system,
        event,
    );
    let result = send_transaction_bundle(client, fhir_cfg, token, &bundle).await?;
    let identifier = format!("{}|{}", fhir_cfg.oscar_cpp_condition_system, event.payload().source_id());
    info!(
        "fhir sink: synced {} -> {} (fhir_id={} version_id={:?})",
        event.idempotency_key(), identifier, result.fhir_id, result.version_id
    );
    Ok(result)
}

// ---------------------------------------------------------------------------
// Shared transaction helpers
// ---------------------------------------------------------------------------

fn build_conditional_put_bundle(
    fhir_resource: FhirResource,
    identifier_system: &str,
    event: &SyncEvent,
) -> Bundle {
    let sys: String = url::form_urlencoded::byte_serialize(identifier_system.as_bytes()).collect();
    let val: String = url::form_urlencoded::byte_serialize(event.payload().source_id().as_bytes()).collect();
    let conditional_url = format!("{}?identifier={sys}|{val}", event.resource_type().as_path());

    let mut bundle = Bundle::default();
    bundle.r#type = "transaction".into();
    bundle.entry.push(BundleEntry {
        full_url: Some(format!("urn:uuid:{}", event.idempotency_key()).into()),
        resource: Some(fhir_resource),
        request: Some(BundleEntryRequest {
            method: "PUT".into(),
            url: conditional_url.into(),
            ..Default::default()
        }),
        ..Default::default()
    });
    bundle
}

async fn send_transaction_bundle(
    client: &reqwest::Client,
    fhir_cfg: &FhirConfig,
    token: Option<String>,
    bundle: &Bundle,
) -> Result<FhirResult, SyncFailure> {
    let body = fhirbolt::json::to_string(bundle, None)
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

// ---------------------------------------------------------------------------
// Reference builders
// ---------------------------------------------------------------------------

fn conditional_reference(system: &str, value: &str, resource: &str) -> Reference {
    let sys: String = url::form_urlencoded::byte_serialize(system.as_bytes()).collect();
    let val: String = url::form_urlencoded::byte_serialize(value.as_bytes()).collect();
    Reference {
        reference: Some(format!("{resource}?identifier={sys}|{val}").into()),
        ..Default::default()
    }
}

fn patient_ref(fhir_cfg: &FhirConfig, demographic_no: &str) -> Reference {
    conditional_reference(&fhir_cfg.oscar_demographic_system, demographic_no, "Patient")
}

fn practitioner_ref(fhir_cfg: &FhirConfig, provider_no: &str) -> Reference {
    conditional_reference(&fhir_cfg.oscar_provider_system, provider_no, "Practitioner")
}

fn appointment_ref(fhir_cfg: &FhirConfig, appointment_no: &str) -> Reference {
    conditional_reference(&fhir_cfg.oscar_appointment_system, appointment_no, "Appointment")
}

fn encounter_ref(fhir_cfg: &FhirConfig, note_id_value: &str) -> Reference {
    conditional_reference(&fhir_cfg.oscar_note_system, note_id_value, "Encounter")
}

fn participant_type(code: &str, display: &str) -> CodeableConcept {
    CodeableConcept {
        coding: vec![Coding {
            system: Some(PARTICIPATION_TYPE_SYSTEM.into()),
            code: Some(code.into()),
            display: Some(display.into()),
            ..Default::default()
        }],
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// Resource builders
// ---------------------------------------------------------------------------

fn build_encounter(
    enc: &DomainEncounter,
    fhir_cfg: &FhirConfig,
    oscar_cfg: &OscarConfig,
) -> Result<Encounter, SyncFailure> {
    let class = enc
        .billing_visit_type
        .as_deref()
        .and_then(billing_class)
        .or_else(|| crate::mapping::casemgmt_note::resolve_class(enc))
        .ok_or_else(|| SyncFailure::Permanent(anyhow::anyhow!(
            "unmappable encounter for note_id={}", enc.note_id
        )))?;

    let mut encounter = Encounter::default();

    encounter.meta = Some(Box::new(Meta {
        source: Some(META_SOURCE.into()),
        ..Default::default()
    }));

    // Identifier[0] = uuid (or note-id fallback), identifier[1] = revision.
    let primary_value = enc.uuid.clone().unwrap_or_else(|| format!("oscar-note-{}", enc.note_id));
    encounter.identifier.push(Identifier {
        system: Some(fhir_cfg.oscar_note_system.clone().into()),
        value: Some(primary_value.into()),
        ..Default::default()
    });
    encounter.identifier.push(Identifier {
        system: Some(fhir_cfg.oscar_note_revision_system.clone().into()),
        value: Some(enc.note_id.clone().into()),
        ..Default::default()
    });

    encounter.status = if enc.archived {
        "entered-in-error".into()
    } else if enc.signed {
        "finished".into()
    } else {
        "in-progress".into()
    };

    encounter.class = Box::new(Coding {
        system: Some(ACT_CODE_SYSTEM.into()),
        code: Some(class.into()),
        ..Default::default()
    });

    encounter.subject = Some(Box::new(patient_ref(fhir_cfg, &enc.demographic_no)));

    let mut participants = Vec::new();
    if let Some(provider_no) = &enc.provider_no {
        participants.push(EncounterParticipant {
            r#type: vec![participant_type("PPRF", "primary performer")],
            individual: Some(Box::new(practitioner_ref(fhir_cfg, provider_no))),
            ..Default::default()
        });
    }
    if enc.signed {
        if let Some(signing) = &enc.signing_provider_no {
            if signing != enc.provider_no.as_deref().unwrap_or("") {
                participants.push(EncounterParticipant {
                    r#type: vec![participant_type("ATND", "attender")],
                    individual: Some(Box::new(practitioner_ref(fhir_cfg, signing))),
                    ..Default::default()
                });
            }
        }
    }
    encounter.participant = participants;

    if let Some(appt) = &enc.appointment_no {
        encounter.appointment.push(appointment_ref(fhir_cfg, appt));
    }

    if let Some(billing_code) = &enc.billing_code {
        encounter.r#type.push(CodeableConcept {
            coding: vec![Coding {
                system: Some(fhir_cfg.msp_service_code_system.clone().into()),
                code: Some(billing_code.clone().into()),
                ..Default::default()
            }],
            ..Default::default()
        });
    }

    if let Some(obs) = &enc.observation_date {
        let (start, end) = encounter_period(obs, enc.hour, enc.minute, oscar_cfg)?;
        if start.is_some() || end.is_some() {
            encounter.period = Some(Box::new(Period {
                r#start: start.map(|s| s.into()),
                r#end: end.map(|s| s.into()),
                ..Default::default()
            }));
        }
    }

    Ok(encounter)
}

fn build_document_reference(
    doc: &DomainDocumentReference,
    fhir_cfg: &FhirConfig,
    oscar_cfg: &OscarConfig,
) -> Result<DocumentReference, SyncFailure> {
    let mut dr = DocumentReference::default();

    dr.meta = Some(Box::new(Meta {
        source: Some(META_SOURCE.into()),
        ..Default::default()
    }));

    let primary_value = doc.uuid.clone().unwrap_or_else(|| format!("oscar-note-{}", doc.note_id));
    dr.identifier.push(Identifier {
        system: Some(fhir_cfg.oscar_note_document_system.clone().into()),
        value: Some(primary_value.into()),
        ..Default::default()
    });
    dr.identifier.push(Identifier {
        system: Some(fhir_cfg.oscar_note_revision_system.clone().into()),
        value: Some(doc.note_id.clone().into()),
        ..Default::default()
    });

    dr.status = if doc.archived { "entered-in-error".into() } else { "current".into() };
    dr.doc_status = if doc.signed { Some("final".into()) } else { Some("preliminary".into()) };

    dr.r#type = Some(Box::new(CodeableConcept {
        text: Some(document_reference_type(&doc.encounter_type).into()),
        ..Default::default()
    }));

    dr.subject = Some(Box::new(patient_ref(fhir_cfg, &doc.demographic_no)));

    if let Some(provider_no) = &doc.provider_no {
        dr.author.push(practitioner_ref(fhir_cfg, provider_no));
    }
    if doc.signed {
        if let Some(signing) = &doc.signing_provider_no {
            dr.authenticator = Some(Box::new(practitioner_ref(fhir_cfg, signing)));
        }
    }

    if let Some(obs) = &doc.observation_date {
        // DocumentReference.date is an instant; only populate when we have a
        // complete timestamp.
        if let Some(dt) = to_instant(obs, oscar_cfg)? {
            dr.date = Some(dt.into());
        }
    }

    let encoded = Base64::encode_string(doc.note.as_bytes());
    dr.content.push(DocumentReferenceContent {
        attachment: Box::new(Attachment {
            content_type: Some("text/plain".into()),
            data: Some(encoded.into()),
            ..Default::default()
        }),
        ..Default::default()
    });

    let is_admin = doc.encounter_type.as_deref() == Some("encounter without client");
    if !is_admin {
        let note_id_value = doc.uuid.as_deref().unwrap_or(&doc.note_id);
        dr.context = Some(DocumentReferenceContext {
            encounter: vec![encounter_ref(fhir_cfg, note_id_value)],
            ..Default::default()
        });
    }

    Ok(dr)
}

fn build_condition(condition: &DomainCondition, fhir_cfg: &FhirConfig) -> Result<Condition, SyncFailure> {
    let mut cond = Condition::default();

    cond.meta = Some(Box::new(Meta {
        source: Some(META_SOURCE.into()),
        ..Default::default()
    }));

    let identifier_system = if condition.source_table == "dxresearch" {
        &fhir_cfg.oscar_dxresearch_system
    } else {
        &fhir_cfg.oscar_cpp_condition_system
    };
    cond.identifier.push(Identifier {
        system: Some(identifier_system.clone().into()),
        value: Some(condition.source_id.clone().into()),
        ..Default::default()
    });

    if let Some(cs) = &condition.clinical_status {
        cond.clinical_status = Some(Box::new(CodeableConcept {
            coding: vec![Coding {
                system: Some(CONDITION_CLINICAL_SYSTEM.into()),
                code: Some(cs.clone().into()),
                ..Default::default()
            }],
            ..Default::default()
        }));
    }

    if let Some(vs) = &condition.verification_status {
        cond.verification_status = Some(Box::new(CodeableConcept {
            coding: vec![Coding {
                system: Some(CONDITION_VERIFICATION_SYSTEM.into()),
                code: Some(vs.clone().into()),
                ..Default::default()
            }],
            ..Default::default()
        }));
    }

    if condition.coding_system.as_deref() == Some("icd9") && condition.code.is_some() {
        let code = condition.code.as_deref().unwrap();
        cond.code = Some(Box::new(CodeableConcept {
            text: condition.problem_description.clone().map(|t| t.into()),
            coding: vec![Coding {
                system: Some(fhir_cfg.icd9_system.clone().into()),
                code: Some(code.into()),
                display: condition.display.clone().map(|d| d.into()),
                ..Default::default()
            }],
            ..Default::default()
        }));
    } else if condition.problem_description.is_some() {
        cond.code = Some(Box::new(CodeableConcept {
            text: condition.problem_description.clone().map(|t| t.into()),
            ..Default::default()
        }));
    } else if condition.code.is_some() {
        // Unknown coding_system but we have a code; emit text only.
        cond.code = Some(Box::new(CodeableConcept {
            text: condition.code.clone().map(|c| c.into()),
            ..Default::default()
        }));
    }

    cond.subject = Box::new(patient_ref(fhir_cfg, &condition.demographic_no));

    if let Some(date) = &condition.onset_date {
        cond.onset = Some(ConditionOnset::DateTime(date.replace(' ', "T").into()));
    } else if let Some(age) = &condition.onset_age {
        cond.onset = Some(ConditionOnset::Age(Box::new(Age {
            value: Some(age.clone().into()),
            unit: Some("years".into()),
            system: Some(UOM_SYSTEM.into()),
            code: Some("a".into()),
            ..Default::default()
        })));
    }

    if let Some(date) = &condition.abatement_date {
        cond.abatement = Some(ConditionAbatement::DateTime(date.replace(' ', "T").into()));
    }

    if let Some(date) = &condition.recorded_date {
        cond.recorded_date = Some(date.replace(' ', "T").into());
    }

    if let Some(recorder) = &condition.recorder {
        cond.recorder = Some(Box::new(practitioner_ref(fhir_cfg, recorder)));
    }

    let mut note_text = String::new();
    if let Some(treatment) = &condition.treatment {
        note_text.push_str(treatment);
    }
    if let Some(exposure) = &condition.exposure_details {
        if !note_text.is_empty() {
            note_text.push('\n');
        }
        note_text.push_str(exposure);
    }
    if !note_text.is_empty() {
        cond.note.push(Annotation {
            text: note_text.into(),
            ..Default::default()
        });
    }

    if let Some(hide) = &condition.hide_cpp {
        if hide.trim() == "1" {
            cond.meta.as_mut().unwrap().tag.push(Coding {
                system: Some("https://arsmedicatech.com/fhir/sid/oscar-cpp-display".into()),
                code: Some("hide-cpp".into()),
                ..Default::default()
            });
        }
    }

    Ok(cond)
}

fn build_family_member_history(
    fh: &DomainFamilyMemberHistory,
    fhir_cfg: &FhirConfig,
) -> Result<FamilyMemberHistory, SyncFailure> {
    let mut f = FamilyMemberHistory::default();

    f.meta = Some(Box::new(Meta {
        source: Some(META_SOURCE.into()),
        ..Default::default()
    }));

    f.identifier.push(Identifier {
        system: Some(fhir_cfg.oscar_cpp_condition_system.clone().into()),
        value: Some(fh.note_id.clone().into()),
        ..Default::default()
    });

    f.status = "completed".into();
    f.patient = Box::new(patient_ref(fhir_cfg, &fh.demographic_no));

    if let Some(rel) = &fh.relationship {
        f.relationship = Box::new(CodeableConcept {
            text: Some(rel.clone().into()),
            ..Default::default()
        });
    }

    if let Some(problem) = &fh.problem_description {
        let onset = fh.onset_age.as_deref().map(|age| {
            FamilyMemberHistoryConditionOnset::Age(Box::new(Age {
                value: Some(age.into()),
                unit: Some("years".into()),
                system: Some(UOM_SYSTEM.into()),
                code: Some("a".into()),
                ..Default::default()
            }))
        });

        let mut notes = Vec::new();
        if let Some(stage) = &fh.life_stage {
            notes.push(Annotation {
                text: stage.clone().into(),
                ..Default::default()
            });
        }

        f.condition.push(FamilyMemberHistoryCondition {
            code: Box::new(CodeableConcept {
                text: Some(problem.clone().into()),
                ..Default::default()
            }),
            onset,
            note: notes,
            ..Default::default()
        });
    }

    Ok(f)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn billing_class(vt: &str) -> Option<String> {
    match vt {
        "A" | "T" | "P" | "G" | "M" | "D" | "F" => Some("AMB".to_string()),
        "I" | "C" => Some("IMP".to_string()),
        "E" => Some("EMER".to_string()),
        "R" => Some("HH".to_string()),
        "Z" => Some("AMB".to_string()),
        _ => None,
    }
}

fn document_reference_type(encounter_type: &Option<String>) -> String {
    match encounter_type.as_deref() {
        Some("face to face encounter with client") => "in-person note".to_string(),
        Some("telephone encounter with client") => "telephone note".to_string(),
        Some("email encounter with client") => "email note".to_string(),
        Some("encounter without client") => "administrative note".to_string(),
        Some(_) | None => "in-person note".to_string(),
    }
}

fn encounter_period(
    obs: &str,
    hour: Option<u32>,
    minute: Option<u32>,
    oscar_cfg: &OscarConfig,
) -> Result<(Option<String>, Option<String>), SyncFailure> {
    if let Some((date, time)) = obs.split_once(' ') {
        let start = to_appointment_instant_rfc3339(date, time, oscar_cfg)?;
        let end = if let (Some(h), Some(m)) = (hour, minute) {
            if h > 0 || m > 0 {
                let naive = NaiveDateTime::new(
                    NaiveDate::parse_from_str(date, "%Y-%m-%d")
                        .map_err(|e| SyncFailure::Permanent(anyhow::anyhow!("invalid observation_date '{date}': {e}")))?,
                    NaiveTime::parse_from_str(time, "%H:%M:%S")
                        .map_err(|e| SyncFailure::Permanent(anyhow::anyhow!("invalid observation time '{time}': {e}")))?,
                );
                let end_naive = naive + chrono::Duration::minutes((h * 60 + m).into());
                let end_date = end_naive.format("%Y-%m-%d").to_string();
                let end_time = end_naive.format("%H:%M:%S").to_string();
                Some(to_appointment_instant_rfc3339(&end_date, &end_time, oscar_cfg)?)
            } else {
                None
            }
        } else {
            None
        };
        return Ok((Some(start), end));
    }

    if NaiveDate::parse_from_str(obs, "%Y-%m-%d").is_ok() {
        // date-only precision; ignore any hour/minute duration because we have
        // no start clock time.
        return Ok((Some(obs.to_string()), None));
    }

    warn!("encounter_period: unparseable observation_date '{obs}'");
    Ok((None, None))
}

fn to_instant(
    obs: &str,
    oscar_cfg: &OscarConfig,
) -> Result<Option<String>, SyncFailure> {
    if let Some((date, time)) = obs.split_once(' ') {
        return Ok(Some(to_appointment_instant_rfc3339(date, time, oscar_cfg)?));
    }
    Ok(None)
}

fn to_appointment_instant_rfc3339(
    date: &str,
    time: &str,
    oscar_cfg: &OscarConfig,
) -> Result<String, SyncFailure> {
    let tz_name = oscar_cfg
        .timezone
        .as_deref()
        .ok_or_else(|| SyncFailure::Permanent(anyhow::anyhow!("missing oscar timezone")))?;
    super::to_appointment_instant(date, time, tz_name)
}
