use std::collections::HashMap;

use tracing::{info, warn};

use crate::domain::document_reference::{DomainDocumentReference, NoteLink};
use crate::domain::encounter::DomainEncounter;
use crate::domain::resource::DomainResource;
use crate::sources::RowChange;
use crate::mapping::syncable_provider;

pub type ColumnMap = HashMap<String, usize>;

fn lookup<'a>(change: &'a RowChange, columns: &ColumnMap, name: &str) -> Option<&'a str> {
    let idx = *columns.get(name)?;
    change
        .after
        .get(idx)
        .and_then(|v| v.as_deref())
        .filter(|s| !s.is_empty())
}

fn lookup_any<'a>(change: &'a RowChange, columns: &ColumnMap, names: &[&str]) -> Option<&'a str> {
    names.iter().find_map(|n| lookup(change, columns, n))
}

fn bool_flag(change: &RowChange, columns: &ColumnMap, name: &str) -> bool {
    lookup(change, columns, name).map(|s| s.trim() == "1").unwrap_or(false)
}

fn u32_opt(change: &RowChange, columns: &ColumnMap, name: &str) -> Option<u32> {
    lookup(change, columns, name).and_then(|s| s.trim().parse().ok())
}

/// Maps one `casemgmt_note` row into zero, one, or two `DomainResource`s.
///
/// - An `encounter_type` with no matching enum member (the group variants)
///   dead-letters the whole row.
/// - `encounter without client` produces a `DocumentReference` only.
/// - All other valid rows produce an `Encounter` + a `DocumentReference`.
/// - `billing_visit_type` is supplied by the caller after joining the
///   `billing` table (D3/E8); `None` falls back to `encounter_type`.
pub fn row_to_casemgmt_note_resources(
    change: &RowChange,
    columns: &ColumnMap,
    billing_visit_type: Option<&str>,
) -> Vec<DomainResource> {
    let Some(note_id) = lookup(change, columns, "note_id").map(str::to_string) else {
        warn!("casemgmt_note mapping: skipping row with no note_id");
        return Vec::new();
    };

    let uuid = lookup(change, columns, "uuid").map(str::to_string);
    if uuid.is_none() {
        warn!("casemgmt_note mapping: note_id={note_id} has NULL uuid; using note_id as identifier fallback");
    }

    let demographic_no = lookup(change, columns, "demographic_no")
        .map(str::to_string)
        .unwrap_or_default();
    
    let provider_no = syncable_provider(lookup_any(change, columns, &["provider_no", "providerNo"]));
    
    let signing_provider_no = syncable_provider(lookup_any(change, columns, &["signing_provider_no", "signingProviderNo"]));
    
    let observation_date = lookup_any(change, columns, &["observation_date", "observationDate"]).map(str::to_string);
    let update_date = lookup_any(change, columns, &["update_date", "updateDate"]).map(str::to_string);
    let encounter_type = lookup(change, columns, "encounter_type").map(str::to_string);
    let signed = bool_flag(change, columns, "signed");
    let archived = bool_flag(change, columns, "archived");
    let appointment_no = lookup_any(change, columns, &["appointmentNo", "appointment_no"]).map(str::to_string);
    let billing_code = lookup(change, columns, "billing_code").map(str::to_string);
    let hour = u32_opt(change, columns, "hourOfEncounterTime");
    let minute = u32_opt(change, columns, "minuteOfEncounterTime");
    let note = lookup(change, columns, "note").map(str::to_string).unwrap_or_default();

    let mut links = Vec::new();
    if let Some(table_name) = lookup(change, columns, "link_table_name") {
        if let Some(table_id) = lookup(change, columns, "link_table_id") {
            links.push(NoteLink {
                table_name: table_name.to_string(),
                table_id: table_id.to_string(),
            });
        }
    }

    let encounter = DomainEncounter {
        note_id: note_id.clone(),
        uuid: uuid.clone(),
        demographic_no,
        provider_no,
        signing_provider_no,
        observation_date,
        update_date,
        encounter_type,
        signed,
        archived,
        appointment_no,
        billing_code,
        billing_visit_type: billing_visit_type.map(str::to_string),
        hour,
        minute,
    };

    let doc_ref = DomainDocumentReference {
        note_id,
        uuid,
        demographic_no: encounter.demographic_no.clone(),
        provider_no: encounter.provider_no.clone(),
        signing_provider_no: encounter.signing_provider_no.clone(),
        observation_date: encounter.observation_date.clone(),
        encounter_type: encounter.encounter_type.clone(),
        signed,
        archived,
        note,
        links,
    };

    let note_id_for_info = doc_ref.note_id.clone();

    // Determine the FHIR Encounter.class to validate the encounter_type value.
    match resolve_class(&encounter) {
        Some(_) => {
            let mut out = Vec::with_capacity(2);
            // Encounter must precede DocumentReference: the DocumentReference's
            // `context.encounter` is a conditional reference that HAPI resolves
            // at write time, so the Encounter has to already exist (HAPI-1091).
            if !is_administrative_only(&encounter) {
                out.push(DomainResource::Encounter(encounter));
            } else {
                info!("casemgmt_note mapping: note_id={} is 'encounter without client'; emitting DocumentReference only", note_id_for_info);
            }
            out.push(DomainResource::DocumentReference(doc_ref));
            out
        }
        None => {
            // Unmapped encounter_type: dead-letter the row.  We still emit a
            // single resource so the sink writes the dead letter with the
            // unmapped_encounter_type reason.
            vec![DomainResource::DocumentReference(doc_ref)]
        }
    }
}

/// Returns `true` when the encounter type means no patient was present.
fn is_administrative_only(encounter: &DomainEncounter) -> bool {
    encounter.encounter_type.as_deref() == Some("encounter without client")
}

/// Resolves the FHIR `Encounter.class` from the note, preferring the billing
/// visit type when provided (D3).
pub fn resolve_class(encounter: &DomainEncounter) -> Option<String> {
    if let Some(vt) = &encounter.billing_visit_type {
        return match vt.as_str() {
            "A" | "T" | "P" | "G" | "M" | "D" | "F" => Some("AMB".to_string()),
            "I" | "C" => Some("IMP".to_string()),
            "E" => Some("EMER".to_string()),
            "R" => Some("HH".to_string()),
            "Z" => {
                warn!("billing visittype 'Z' (None of the above) for note_id={}; defaulting to AMB", encounter.note_id);
                Some("AMB".to_string())
            }
            _ => None,
        };
    }

    match encounter.encounter_type.as_deref() {
        Some("face to face encounter with client") => Some("AMB".to_string()),
        Some("telephone encounter with client") | Some("email encounter with client") => Some("VR".to_string()),
        Some("encounter without client") => Some("AMB".to_string()), // used only as fallback for DocumentReference.type
        Some(et) if et.starts_with("group ") => {
            warn!("unmapped_encounter_type: '{}' for note_id={}; group variants are not supported", et, encounter.note_id);
            None
        }
        Some(et) => {
            warn!("unmapped_encounter_type: '{}' for note_id={}", et, encounter.note_id);
            None
        }
        None => {
            warn!("empty encounter_type for note_id={}; defaulting to AMB", encounter.note_id);
            Some("AMB".to_string())
        }
    }
}
