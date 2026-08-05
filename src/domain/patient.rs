use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum AddressUse {
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "temp")]
    Temp,
}

impl AddressUse {
    pub fn as_str(&self) -> &'static str {
        match self {
            AddressUse::Home => "home",
            AddressUse::Temp => "temp",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum AddressKind {
    #[serde(rename = "postal")]
    Postal,
    #[serde(rename = "physical")]
    Physical,
}

impl AddressKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            AddressKind::Postal => "postal",
            AddressKind::Physical => "physical",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DomainAddress {
    pub line:     Option<String>,
    pub city:     Option<String>,
    pub province: Option<String>,
    pub postal:   Option<String>,
    #[serde(rename = "use")]
    pub use_:     AddressUse,
    #[serde(rename = "type")]
    pub kind:     AddressKind,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainPatient {
    pub demographic_no: String,
    pub first_name:    Option<String>,
    pub last_name:     Option<String>,
    pub date_of_birth: Option<String>, // ISO "YYYY-MM-DD"
    pub addresses:     Vec<DomainAddress>,
    pub sex:           Option<String>,
    pub phone:         Option<String>,
    pub email:         Option<String>,
    /// Provincial health insurance number
    pub hin:           Option<String>,
    /// Oscar `patient_status`: AC, IN, DE, etc.
    pub patient_status: Option<String>,
    /// From `demographic_merged.merged_to`; when present this is a merge-loser record.
    pub merged_to:     Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_patient_deserialization() {
        let json = r#"{
            "demographic_no": "12345",
            "first_name": "John",
            "last_name": "Doe",
            "date_of_birth": "1990-01-01",
            "addresses": [
                {
                    "line": "123 Main St",
                    "city": "Toronto",
                    "province": "ON",
                    "postal": "M5V1A1",
                    "use": "home",
                    "type": "postal"
                }
            ],
            "sex": "male",
            "phone": "+1-555-123-4567",
            "email": "john.doe@example.com"
        }"#;

        let patient: DomainPatient = serde_json::from_str(json).unwrap();

        assert_eq!(patient.demographic_no, "12345");
        assert_eq!(patient.first_name, Some("John".to_string()));
        assert_eq!(patient.last_name, Some("Doe".to_string()));
        assert_eq!(patient.date_of_birth, Some("1990-01-01".to_string()));
        assert_eq!(patient.addresses.len(), 1);
        assert_eq!(patient.addresses[0].city, Some("Toronto".to_string()));
        assert_eq!(patient.addresses[0].use_, AddressUse::Home);
        assert_eq!(patient.addresses[0].kind, AddressKind::Postal);
        assert_eq!(patient.sex, Some("male".to_string()));
        assert_eq!(patient.phone, Some("+1-555-123-4567".to_string()));
        assert_eq!(patient.email, Some("john.doe@example.com".to_string()));
    }

    #[test]
    fn test_domain_patient_minimal_deserialization() {
        let json = r#"{
            "demographic_no": "67890"
        }"#;

        let patient: DomainPatient = serde_json::from_str(json).unwrap();

        assert_eq!(patient.demographic_no, "67890");
        assert_eq!(patient.first_name, None);
        assert_eq!(patient.last_name, None);
        assert_eq!(patient.date_of_birth, None);
        assert!(patient.addresses.is_empty());
        assert_eq!(patient.sex, None);
        assert_eq!(patient.phone, None);
        assert_eq!(patient.email, None);
        assert_eq!(patient.patient_status, None);
        assert_eq!(patient.merged_to, None);
    }

    #[test]
    fn test_domain_patient_partial_deserialization() {
        let json = r#"{
            "demographic_no": "99999",
            "first_name": "Jane",
            "date_of_birth": "1985-05-15",
            "sex": "female",
            "email": "jane@example.com"
        }"#;

        let patient: DomainPatient = serde_json::from_str(json).unwrap();

        assert_eq!(patient.demographic_no, "99999");
        assert_eq!(patient.first_name, Some("Jane".to_string()));
        assert_eq!(patient.last_name, None);
        assert_eq!(patient.date_of_birth, Some("1985-05-15".to_string()));
        assert!(patient.addresses.is_empty());
        assert_eq!(patient.sex, Some("female".to_string()));
        assert_eq!(patient.phone, None);
        assert_eq!(patient.email, Some("jane@example.com".to_string()));
    }

    #[test]
    fn test_domain_address_serde_roundtrip() {
        let address = DomainAddress {
            line: Some("456 Oak Ave".to_string()),
            city: Some("Vancouver".to_string()),
            province: Some("BC".to_string()),
            postal: Some("V6B1A1".to_string()),
            use_: AddressUse::Home,
            kind: AddressKind::Physical,
        };

        let json = serde_json::to_string(&address).unwrap();
        assert!(json.contains("\"use\":\"home\""));
        assert!(json.contains("\"type\":\"physical\""));

        let parsed: DomainAddress = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, address);
    }

    #[test]
    fn test_domain_patient_invalid_json() {
        let json = r#"{
            "demographic_no": "12345",
            "invalid_field": "value"
        }"#;

        // Should still deserialize successfully, ignoring unknown fields
        let patient: DomainPatient = serde_json::from_str(json).unwrap();
        assert_eq!(patient.demographic_no, "12345");
    }

    #[test]
    fn test_domain_patient_missing_required_field() {
        let json = r#"{
            "first_name": "John"
        }"#;

        // This should fail because demographic_no is required
        let result: Result<DomainPatient, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
