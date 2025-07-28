use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct OscarPatient {
    pub demographic_no: String,
    pub first_name:    Option<String>,
    pub last_name:     Option<String>,
    pub date_of_birth: Option<String>, // ISO "YYYY-MM-DD"
    pub location:      Option<(String, String, String, String)>, // city, province, country, postal
    pub sex:           Option<String>, // "male" | "female" | "other" | ...
    pub phone:         Option<String>,
    pub email:         Option<String>,
}


pub enum TableId {
    Demographic = 123, // <-- set this to the correct value
    // Add other table ids as needed
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_oscar_patient_deserialization() {
        let json = r#"{
            "demographic_no": "OSC12345",
            "first_name": "Alice",
            "last_name": "Johnson",
            "date_of_birth": "1982-03-15",
            "location": ["Montreal", "QC", "Canada", "H2Y1C6"],
            "sex": "female",
            "phone": "+1-514-555-0123",
            "email": "alice.johnson@example.com"
        }"#;

        let patient: OscarPatient = serde_json::from_str(json).unwrap();
        
        assert_eq!(patient.demographic_no, "OSC12345");
        assert_eq!(patient.first_name, Some("Alice".to_string()));
        assert_eq!(patient.last_name, Some("Johnson".to_string()));
        assert_eq!(patient.date_of_birth, Some("1982-03-15".to_string()));
        assert_eq!(patient.location, Some(("Montreal".to_string(), "QC".to_string(), "Canada".to_string(), "H2Y1C6".to_string())));
        assert_eq!(patient.sex, Some("female".to_string()));
        assert_eq!(patient.phone, Some("+1-514-555-0123".to_string()));
        assert_eq!(patient.email, Some("alice.johnson@example.com".to_string()));
    }

    #[test]
    fn test_oscar_patient_minimal_deserialization() {
        let json = r#"{
            "demographic_no": "OSC67890"
        }"#;

        let patient: OscarPatient = serde_json::from_str(json).unwrap();
        
        assert_eq!(patient.demographic_no, "OSC67890");
        assert_eq!(patient.first_name, None);
        assert_eq!(patient.last_name, None);
        assert_eq!(patient.date_of_birth, None);
        assert_eq!(patient.location, None);
        assert_eq!(patient.sex, None);
        assert_eq!(patient.phone, None);
        assert_eq!(patient.email, None);
    }

    #[test]
    fn test_table_id_demographic_value() {
        assert_eq!(TableId::Demographic as i32, 123);
    }

    #[test]
    fn test_oscar_patient_clone() {
        let patient = OscarPatient {
            demographic_no: "OSC99999".to_string(),
            first_name: Some("Bob".to_string()),
            last_name: Some("Smith".to_string()),
            date_of_birth: Some("1975-08-22".to_string()),
            location: Some(("Calgary".to_string(), "AB".to_string(), "Canada".to_string(), "T2P1J9".to_string())),
            sex: Some("male".to_string()),
            phone: Some("+1-403-555-0456".to_string()),
            email: Some("bob.smith@example.com".to_string()),
        };

        let cloned_patient = patient.clone();
        
        assert_eq!(cloned_patient.demographic_no, patient.demographic_no);
        assert_eq!(cloned_patient.first_name, patient.first_name);
        assert_eq!(cloned_patient.last_name, patient.last_name);
        assert_eq!(cloned_patient.date_of_birth, patient.date_of_birth);
        assert_eq!(cloned_patient.location, patient.location);
        assert_eq!(cloned_patient.sex, patient.sex);
        assert_eq!(cloned_patient.phone, patient.phone);
        assert_eq!(cloned_patient.email, patient.email);
    }

    #[test]
    fn test_oscar_patient_debug_format() {
        let patient = OscarPatient {
            demographic_no: "OSC11111".to_string(),
            first_name: Some("Carol".to_string()),
            last_name: None,
            date_of_birth: Some("1995-12-03".to_string()),
            location: None,
            sex: Some("other".to_string()),
            phone: None,
            email: Some("carol@example.com".to_string()),
        };

        let debug_output = format!("{:?}", patient);
        assert!(debug_output.contains("OSC11111"));
        assert!(debug_output.contains("Carol"));
        assert!(debug_output.contains("1995-12-03"));
        assert!(debug_output.contains("other"));
        assert!(debug_output.contains("carol@example.com"));
    }
}
