use serde::{Deserialize, Serialize};

/// Domain model for a FHIR `Condition`.
///
/// Used for both coded problem-list rows (`dxresearch`) and narrative
/// CPP entries (`casemgmt_note` + `casemgmt_note_ext`).  The `source_table`
/// field lets the sink choose the correct identifier system.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainCondition {
    pub source_id: String,
    pub source_table: String,
    pub demographic_no: String,
    pub code: Option<String>,
    pub coding_system: Option<String>,
    pub display: Option<String>,
    pub clinical_status: Option<String>,
    pub verification_status: Option<String>,
    pub onset_date: Option<String>,
    pub abatement_date: Option<String>,
    pub onset_age: Option<String>,
    pub note: Option<String>,
    pub recorded_date: Option<String>,
    pub recorder: Option<String>,
    pub problem_description: Option<String>,
    pub treatment: Option<String>,
    pub exposure_details: Option<String>,
    pub hide_cpp: Option<String>,
}

/// Domain model for a FHIR `FamilyMemberHistory` sourced from
/// `casemgmt_note_ext` rows that carry a `Relationship` key.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainFamilyMemberHistory {
    pub note_id: String,
    pub demographic_no: String,
    pub relationship: Option<String>,
    pub problem_description: Option<String>,
    pub onset_age: Option<String>,
    pub life_stage: Option<String>,
    pub observation_date: Option<String>,
}
