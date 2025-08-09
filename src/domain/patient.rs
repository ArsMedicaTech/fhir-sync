use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainPatient {
    pub demographic_no: String,
    pub first_name:    Option<String>,
    pub last_name:     Option<String>,
    pub date_of_birth: Option<String>, // ISO "YYYY-MM-DD"
    pub location:      Option<(String, String, String, String)>, // city, province, country, postal
    pub sex:           Option<String>, // "male" | "female" | "other" | ...
    pub phone:         Option<String>,
    pub email:         Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_patient_deserialization() {
        let json = r#"{
            "demographic_no": "12345",
            "first_name": "John",
            "last_name": "Doe",
            "date_of_birth": "1990-01-01",
            "location": ["Toronto", "ON", "Canada", "M5V1A1"],
            "sex": "male",
            "phone": "+1-555-123-4567",
            "email": "john.doe@example.com"
        }"#;

        let patient: DomainPatient = serde_json::from_str(json).unwrap();
        
        assert_eq!(patient.demographic_no, "12345");
        assert_eq!(patient.first_name, Some("John".to_string()));
        assert_eq!(patient.last_name, Some("Doe".to_string()));
        assert_eq!(patient.date_of_birth, Some("1990-01-01".to_string()));
        assert_eq!(patient.location, Some(("Toronto".to_string(), "ON".to_string(), "Canada".to_string(), "M5V1A1".to_string())));
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
        assert_eq!(patient.location, None);
        assert_eq!(patient.sex, None);
        assert_eq!(patient.phone, None);
        assert_eq!(patient.email, None);
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
        assert_eq!(patient.location, None);
        assert_eq!(patient.sex, Some("female".to_string()));
        assert_eq!(patient.phone, None);
        assert_eq!(patient.email, Some("jane@example.com".to_string()));
    }

    #[test]
    fn test_domain_patient_location_tuple() {
        let json = r#"{
            "demographic_no": "11111",
            "location": ["Vancouver", "BC", "Canada", "V6B1A1"]
        }"#;

        let patient: DomainPatient = serde_json::from_str(json).unwrap();
        
        assert_eq!(patient.location, Some(("Vancouver".to_string(), "BC".to_string(), "Canada".to_string(), "V6B1A1".to_string())));
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
