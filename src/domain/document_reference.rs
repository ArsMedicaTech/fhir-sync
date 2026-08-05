use serde::{Deserialize, Serialize};

/// A single link from `casemgmt_note_link` carrying the target table/id pair.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoteLink {
    pub table_name: String,
    pub table_id: String,
}

/// Domain model for a FHIR `DocumentReference` sourced from Oscar's
/// `casemgmt_note` table.  Carries the narrative body and links; metadata lives
/// on the paired `DomainEncounter` (D2).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainDocumentReference {
    pub note_id: String,
    pub uuid: Option<String>,
    pub demographic_no: String,
    pub provider_no: Option<String>,
    pub signing_provider_no: Option<String>,
    pub observation_date: Option<String>,
    pub encounter_type: Option<String>,
    pub signed: bool,
    pub archived: bool,
    pub note: String,
    pub links: Vec<NoteLink>,
}
