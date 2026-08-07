use std::collections::HashMap;

use serde_json::Value;

use super::MappingError;

#[derive(Debug, Default)]
pub struct AppointmentRow {
    pub demographic_no: Option<String>,
    pub provider_no: Option<String>,
    pub appointment_date: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub status: Option<String>,
    pub reason: Option<String>,
    pub booking_source: Option<String>,
    pub lastupdateuser: Option<String>,
}

pub fn fhir_appointment_to_row(
    _appointment: &Value,
    _oscar_demographic_system: &str,
    _oscar_appointment_system: &str,
    _status_map: &HashMap<String, String>,
    _timezone: &str,
) -> Result<(Option<String>, AppointmentRow), MappingError> {
    unimplemented!("appointment writeback mapper is not yet implemented")
}
