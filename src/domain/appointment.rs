use serde::{Deserialize, Serialize};

/// Domain model for a FHIR `Appointment` sourced from Oscar's `appointment`
/// table.
///
/// Time columns are stored as naive local wall-clock strings; conversion to
/// a `chrono-tz` aware instant happens in the sink (D5).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainAppointment {
    pub appointment_no:  String,
    pub demographic_no:  Option<String>,
    pub provider_no:     Option<String>,
    pub appointment_date: Option<String>, // ISO 8601 date, "YYYY-MM-DD"
    pub start_time:      Option<String>,  // naive local wall-clock, "HH:MM:SS"
    pub end_time:        Option<String>,  // naive local wall-clock, "HH:MM:SS"
    pub status:          Option<String>,  // raw Oscar status code (case-sensitive)
    pub reason:          Option<String>,
    pub notes:           Option<String>,
    pub remarks:         Option<String>,
    pub urgency:         Option<String>,
    pub createdatetime:  Option<String>,  // ISO 8601 local-ish datetime, "YYYY-MM-DD HH:MM:SS"
    pub location:        Option<String>,  // deferred to Phase 3+; carried for logging/TODO
    pub booking_source:  Option<String>,
    #[serde(rename = "type")]
    pub type_:           Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_appointment_minimal() {
        let a = DomainAppointment {
            appointment_no: "1".to_string(),
            demographic_no: None,
            provider_no: None,
            appointment_date: None,
            start_time: None,
            end_time: None,
            status: None,
            reason: None,
            notes: None,
            remarks: None,
            urgency: None,
            createdatetime: None,
            location: None,
            booking_source: None,
            type_: None,
        };
        assert_eq!(a.appointment_no, "1");
    }
}
