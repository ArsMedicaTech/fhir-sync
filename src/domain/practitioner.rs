use serde::{Deserialize, Serialize};

/// Domain model for a FHIR `Practitioner` sourced from Oscar's `provider` table.
///
/// Role attributes (`provider_type`, `specialty`, `team`, `supervisor`,
/// `job_title`, `init`, `hso_no`, `rma_no`, `comments`, `provider_activity`,
/// `signed_confidentiality`) are intentionally excluded (D8); this struct
/// models the person only.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainPractitioner {
    pub provider_no: String,
    pub billing_no: Option<String>,
    pub practitioner_no: Option<String>,
    pub practitioner_no_type: Option<String>,
    pub ohip_no: Option<String>,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub sex: Option<String>,
    pub date_of_birth: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub work_phone: Option<String>,
    pub address: Option<String>,
    pub status: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_practitioner_minimal() {
        let p = DomainPractitioner {
            provider_no: "1001".to_string(),
            billing_no: None,
            practitioner_no: None,
            practitioner_no_type: None,
            ohip_no: None,
            title: None,
            first_name: None,
            last_name: None,
            sex: None,
            date_of_birth: None,
            phone: None,
            email: None,
            work_phone: None,
            address: None,
            status: None,
        };
        assert_eq!(p.provider_no, "1001");
    }
}
