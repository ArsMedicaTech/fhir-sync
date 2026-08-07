use serde_json::Value;

use super::MappingError;

#[derive(Debug, Default)]
pub struct NoteRow {
    pub uuid: Option<String>,
    pub demographic_no: Option<String>,
    pub provider_no: Option<String>,
    pub signing_provider_no: Option<String>,
    pub observation_date: Option<String>,
    pub update_date: Option<String>,
    pub encounter_type: Option<String>,
    pub note: Option<String>,
    pub signed: bool,
    pub archived: bool,
    pub appointment_no: Option<String>,
}

pub fn fhir_document_reference_to_row(
    _doc_ref: &Value,
    _oscar_note_document_system: &str,
    _oscar_demographic_system: &str,
) -> Result<(Option<String>, NoteRow), MappingError> {
    unimplemented!("note writeback mapper is not yet implemented")
}
