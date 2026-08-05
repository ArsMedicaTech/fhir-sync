use serde::{Deserialize, Serialize};

/// Domain model for a FHIR `Practitioner` sourced from Oscar's `provider` table.
///
/// Role attributes (`provider_type`, `specialty`) are intentionally excluded
/// (D8); this struct models the person only.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainPractitioner {
    pub provider_no: String,
    pub first_name:  Option<String>,
    pub last_name:   Option<String>,
    pub phone:       Option<String>,
    pub email:       Option<String>,
    pub sex:         Option<String>,
    pub initials:    Option<String>,
    pub billing_no:  Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_practitioner_minimal() {
        let p = DomainPractitioner {
            provider_no: "1001".to_string(),
            first_name: None,
            last_name: None,
            phone: None,
            email: None,
            sex: None,
            initials: None,
            billing_no: None,
        };
        assert_eq!(p.provider_no, "1001");
    }
}
