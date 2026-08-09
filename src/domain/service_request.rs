use serde::{Deserialize, Serialize};

/// Domain model for a FHIR `ServiceRequest` status update sourced from
/// Oscar's `consultationRequests` table.
///
/// AMT originates the `ServiceRequest` via the write-back path
/// (`writeback/mappers/service_request.rs`); this is the reverse direction —
/// Oscar's own consult workflow (a human clicking "Update Consultation
/// Request" and changing the Status radio) is resynced back into AMT so the
/// task shows as progressed/completed there too.
///
/// Identity is carried by `request_id`, matched against the same
/// `oscar_consult_request_system` identifier AMT used when it created the
/// resource, so this updates the existing `ServiceRequest` in place rather
/// than creating a new one.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainServiceRequest {
    pub request_id:     String,
    pub demographic_no: Option<String>,
    pub provider_no:    Option<String>,
    pub reason:         Option<String>,
    pub clinical_info:  Option<String>,
    pub referal_date:   Option<String>, // ISO 8601 date, "YYYY-MM-DD"
    pub urgency:        Option<String>, // raw Oscar urgency code
    pub status:         Option<String>, // raw Oscar status code (case-sensitive)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_service_request_minimal() {
        let r = DomainServiceRequest {
            request_id: "7".to_string(),
            demographic_no: None,
            provider_no: None,
            reason: None,
            clinical_info: None,
            referal_date: None,
            urgency: None,
            status: None,
        };
        assert_eq!(r.request_id, "7");
    }
}
