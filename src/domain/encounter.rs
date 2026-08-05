use serde::{Deserialize, Serialize};

/// Domain model for a FHIR `Encounter` sourced from Oscar's `casemgmt_note`
/// table.  `uuid` (not `note_id`) is the encounter identity; multiple `note_id`
/// rows sharing a `uuid` are revisions of the same encounter (D1).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainEncounter {
    pub note_id: String,
    pub uuid: Option<String>,
    pub demographic_no: String,
    pub provider_no: Option<String>,
    pub signing_provider_no: Option<String>,
    pub observation_date: Option<String>,
    pub update_date: Option<String>,
    pub encounter_type: Option<String>,
    pub signed: bool,
    pub archived: bool,
    pub appointment_no: Option<String>,
    pub billing_code: Option<String>,
    pub billing_visit_type: Option<String>,
    pub hour: Option<u32>,
    pub minute: Option<u32>,
}
