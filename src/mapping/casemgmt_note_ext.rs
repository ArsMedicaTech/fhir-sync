use std::collections::HashMap;

use tracing::debug;

use crate::domain::condition::{DomainCondition, DomainFamilyMemberHistory};

pub type ColumnMap = HashMap<String, usize>;

/// One row from `casemgmt_note_ext`, keyed by `note_id` (the revision id).
#[derive(Debug, Clone)]
pub struct NoteExt {
    pub key_val: String,
    pub value: Option<String>,
    pub date_value: Option<String>,
}

/// Parses all `casemgmt_note_ext` rows for a single note into either a
/// `Condition` or a `FamilyMemberHistory` (D8).
///
/// Returns `None` for both when the extension rows do not describe a
/// CPP problem/family-history entry (e.g. an unknown `key_val` only).
pub fn parse_note_ext(
    note_id: &str,
    demographic_no: &str,
    observation_date: Option<&str>,
    exts: &[NoteExt],
) -> (Option<DomainCondition>, Option<DomainFamilyMemberHistory>) {
    let mut map: HashMap<&str, &NoteExt> = HashMap::new();
    for ext in exts {
        map.insert(&ext.key_val, ext);
    }

    if map.contains_key("Relationship") {
        let relationship = map.get("Relationship").and_then(|e| e.value.as_deref());
        let problem = map
            .get("Problem Description")
            .and_then(|e| e.value.as_deref());
        let onset_age = map.get("Age at Onset").and_then(|e| e.value.as_deref());
        let life_stage = map.get("Life Stage").and_then(|e| e.value.as_deref());

        let fh = DomainFamilyMemberHistory {
            note_id: note_id.to_string(),
            demographic_no: demographic_no.to_string(),
            relationship: relationship.map(str::to_string),
            problem_description: problem.map(str::to_string),
            onset_age: onset_age.map(str::to_string),
            life_stage: life_stage.map(str::to_string),
            observation_date: observation_date.map(str::to_string),
        };
        return (None, Some(fh));
    }

    // Otherwise it is a narrative Condition.
    let problem = map
        .get("Problem Description")
        .and_then(|e| e.value.as_deref());
    let status = map.get("Problem Status").and_then(|e| e.value.as_deref());
    let start_date = map.get("Start Date").and_then(|e| e.date_value.as_deref());
    let resolution_date = map
        .get("Resolution Date")
        .and_then(|e| e.date_value.as_deref());
    let onset_age = map.get("Age at Onset").and_then(|e| e.value.as_deref());
    let treatment = map.get("Treatment").and_then(|e| e.value.as_deref());
    let exposure = map
        .get("Exposure Details")
        .and_then(|e| e.value.as_deref());
    let hide_cpp = map.get("Hide Cpp").and_then(|e| e.value.as_deref());

    // Only emit a Condition if at least one recognised problem key is present.
    if problem.is_none() && status.is_none() && start_date.is_none() && onset_age.is_none() {
        for key in map.keys() {
            debug!("casemgmt_note_ext: ignoring unknown key_val '{}' for note_id={note_id}", key);
        }
        return (None, None);
    }

    // Clinical / verification status uses the same A/C/D table as dxresearch.
    let (clinical_status, verification_status) = match status {
        Some("A") => (Some("active".to_string()), Some("confirmed".to_string())),
        Some("C") => (Some("resolved".to_string()), Some("confirmed".to_string())),
        Some("D") => (None, Some("entered-in-error".to_string())),
        _ => (Some("active".to_string()), Some("confirmed".to_string())),
    };

    let condition = DomainCondition {
        source_id: note_id.to_string(),
        source_table: "casemgmt_note_ext".to_string(),
        demographic_no: demographic_no.to_string(),
        code: None,
        coding_system: None,
        display: None,
        clinical_status,
        verification_status,
        onset_date: start_date.map(str::to_string),
        abatement_date: resolution_date.map(str::to_string),
        onset_age: if start_date.is_none() {
            onset_age.map(str::to_string)
        } else {
            None
        },
        note: None,
        recorded_date: observation_date.map(str::to_string),
        recorder: None,
        problem_description: problem.map(str::to_string),
        treatment: treatment.map(str::to_string),
        exposure_details: exposure.map(str::to_string),
        hide_cpp: hide_cpp.map(str::to_string),
    };
    (Some(condition), None)
}
