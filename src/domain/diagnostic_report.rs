use serde::{Deserialize, Serialize};

/// Domain model for a FHIR `DiagnosticReport` sourced from Oscar's
/// `consultationResponse` table.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainDiagnosticReport {
    pub response_id: String,
    pub demographic_no: String,
    pub provider_no: Option<String>,
    pub response_date: Option<String>,
    pub referral_date: Option<String>,
    pub status: Option<String>,
    pub examination: Option<String>,
    pub impression: Option<String>,
    pub plan: Option<String>,
    pub referral_reason: Option<String>,
    pub based_on: Option<String>,
}
