use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainPractitioner {
    pub practitioner_id: String,
    
    // Basic practitioner information
    pub active: Option<bool>, // Whether this practitioner's record is in active use
    
    // Name information
    pub family_name: Option<String>, // Family name (surname)
    pub given_names: Option<Vec<String>>, // Given names (first names)
    pub prefix: Option<Vec<String>>, // Prefixes (Dr., Prof., etc.)
    pub suffix: Option<Vec<String>>, // Suffixes (Jr., Sr., III, etc.)
    pub use_code: Option<String>, // Name use (usual, official, temp, nickname, anonymous, old, maiden)
    pub text: Option<String>, // Text representation of the full name
    
    // Contact information
    pub telecom_system: Option<Vec<String>>, // Contact system (phone, fax, email, pager, url, sms, other)
    pub telecom_value: Option<Vec<String>>, // Contact value
    pub telecom_use: Option<Vec<String>>, // Contact use (home, work, temp, old, mobile)
    pub telecom_rank: Option<Vec<u32>>, // Specify preferred order of use (1 = highest)
    pub telecom_period_start: Option<Vec<String>>, // Time period when the contact point was/is in use (ISO datetime)
    pub telecom_period_end: Option<Vec<String>>, // Time period when the contact point was/is in use (ISO datetime)
    
    // Demographics
    pub gender: Option<String>, // "male" | "female" | "other" | "unknown"
    pub gender_code: Option<String>, // Code for gender
    pub gender_system: Option<String>, // Terminology system for gender
    pub gender_display: Option<String>, // Display name for gender
    pub birth_date: Option<String>, // ISO date string for birth date
    pub deceased: Option<bool>, // Indicates if the practitioner is deceased or not
    pub deceased_date: Option<String>, // ISO datetime string for death date
    
    // Address information
    pub address_use: Option<Vec<String>>, // Address use (home, work, temp, old, billing)
    pub address_type: Option<Vec<String>>, // Address type (postal, physical, both)
    pub address_text: Option<Vec<String>>, // Text representation of the address
    pub address_line: Option<Vec<Vec<String>>>, // Street address lines
    pub address_city: Option<Vec<String>>, // City name
    pub address_district: Option<Vec<String>>, // District name (sublocality)
    pub address_state: Option<Vec<String>>, // State name
    pub address_postal_code: Option<Vec<String>>, // Postal code
    pub address_country: Option<Vec<String>>, // Country name
    pub address_period_start: Option<Vec<String>>, // Time period when address was/is in use (ISO datetime)
    pub address_period_end: Option<Vec<String>>, // Time period when address was/is in use (ISO datetime)
    
    // Photo information
    pub photo_content_type: Option<Vec<String>>, // Mime type of the content
    pub photo_language: Option<Vec<String>>, // Human language of the content
    pub photo_data: Option<Vec<String>>, // Data inline, base64ed
    pub photo_url: Option<Vec<String>>, // Uri where the data can be found
    pub photo_size: Option<Vec<u64>>, // Number of bytes of content
    pub photo_hash: Option<Vec<String>>, // Hash of the data (sha-1, base64ed)
    pub photo_title: Option<Vec<String>>, // Label to display in place of the data
    pub photo_creation: Option<Vec<String>>, // Date attachment was first created (ISO datetime)
    
    // Qualifications
    pub qualification_identifiers: Option<Vec<Vec<String>>>, // An identifier for this qualification for the practitioner
    pub qualification_identifier_systems: Option<Vec<Vec<String>>>, // Terminology systems for qualification identifiers
    pub qualification_identifier_values: Option<Vec<Vec<String>>>, // Values for qualification identifiers
    pub qualification_identifier_uses: Option<Vec<Vec<String>>>, // Uses for qualification identifiers
    pub qualification_identifier_periods_start: Option<Vec<Vec<String>>>, // Periods for qualification identifiers (start)
    pub qualification_identifier_periods_end: Option<Vec<Vec<String>>>, // Periods for qualification identifiers (end)
    pub qualification_codes: Option<Vec<String>>, // Coded representation of the qualification
    pub qualification_code_codes: Option<Vec<String>>, // Codes for qualifications
    pub qualification_code_systems: Option<Vec<String>>, // Terminology systems for qualifications
    pub qualification_code_displays: Option<Vec<String>>, // Display names for qualifications
    pub qualification_periods_start: Option<Vec<String>>, // Period during which the qualification is valid (start)
    pub qualification_periods_end: Option<Vec<String>>, // Period during which the qualification is valid (end)
    pub qualification_issuer_ids: Option<Vec<String>>, // Organization that regulates and issues the qualification
    pub qualification_issuer_types: Option<Vec<String>>, // Types of qualification issuers
    
    // Communication
    pub communication_languages: Option<Vec<String>>, // The language code used to communicate with the practitioner
    pub communication_language_codes: Option<Vec<String>>, // Codes for communication languages
    pub communication_language_systems: Option<Vec<String>>, // Terminology systems for communication languages
    pub communication_language_displays: Option<Vec<String>>, // Display names for communication languages
    pub communication_preferred: Option<Vec<bool>>, // Language preference indicator
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_practitioner_deserialization() {
        let json = r#"{
            "practitioner_id": "prac_12345",
            "active": true,
            "family_name": "Smith",
            "given_names": ["John", "Michael"],
            "prefix": ["Dr."],
            "suffix": ["MD"],
            "use_code": "official",
            "text": "Dr. John Michael Smith, MD",
            "telecom_system": ["phone", "email"],
            "telecom_value": ["+1-555-123-4567", "john.smith@hospital.com"],
            "telecom_use": ["work", "work"],
            "telecom_rank": [1, 2],
            "gender": "male",
            "gender_code": "M",
            "gender_system": "http://hl7.org/fhir/administrative-gender",
            "gender_display": "Male",
            "birth_date": "1980-05-15",
            "deceased": false,
            "address_use": ["work"],
            "address_type": ["physical"],
            "address_text": "123 Main St, Suite 100, Anytown, ST 12345, USA",
            "address_line": [["123 Main St", "Suite 100"]],
            "address_city": ["Anytown"],
            "address_state": ["ST"],
            "address_postal_code": ["12345"],
            "address_country": ["USA"],
            "photo_content_type": ["image/jpeg"],
            "photo_url": ["https://hospital.com/photos/john_smith.jpg"],
            "photo_title": ["Dr. John Smith - Headshot"],
            "qualification_codes": ["MD", "Internal Medicine"],
            "qualification_code_codes": ["MD", "IM"],
            "qualification_code_systems": ["http://terminology.hl7.org/CodeSystem/v2-0360", "http://terminology.hl7.org/CodeSystem/v2-0402"],
            "qualification_code_displays": ["Doctor of Medicine", "Internal Medicine"],
            "qualification_periods_start": ["2010-06-01T00:00:00Z"],
            "qualification_periods_end": ["2030-06-01T00:00:00Z"],
            "qualification_issuer_ids": ["org_medical_board"],
            "qualification_issuer_types": ["Organization"],
            "communication_languages": ["English", "Spanish"],
            "communication_language_codes": ["en", "es"],
            "communication_language_systems": ["urn:ietf:bcp:47", "urn:ietf:bcp:47"],
            "communication_language_displays": ["English", "Spanish"],
            "communication_preferred": [true, false]
        }"#;

        let practitioner: DomainPractitioner = serde_json::from_str(json).unwrap();
        
        assert_eq!(practitioner.practitioner_id, "prac_12345");
        assert_eq!(practitioner.active, Some(true));
        assert_eq!(practitioner.family_name, Some("Smith".to_string()));
        assert_eq!(practitioner.given_names, Some(vec!["John".to_string(), "Michael".to_string()]));
        assert_eq!(practitioner.prefix, Some(vec!["Dr.".to_string()]));
        assert_eq!(practitioner.suffix, Some(vec!["MD".to_string()]));
        assert_eq!(practitioner.use_code, Some("official".to_string()));
        assert_eq!(practitioner.text, Some("Dr. John Michael Smith, MD".to_string()));
        assert_eq!(practitioner.telecom_system, Some(vec!["phone".to_string(), "email".to_string()]));
        assert_eq!(practitioner.telecom_value, Some(vec!["+1-555-123-4567".to_string(), "john.smith@hospital.com".to_string()]));
        assert_eq!(practitioner.telecom_use, Some(vec!["work".to_string(), "work".to_string()]));
        assert_eq!(practitioner.telecom_rank, Some(vec![1, 2]));
        assert_eq!(practitioner.gender, Some("male".to_string()));
        assert_eq!(practitioner.gender_code, Some("M".to_string()));
        assert_eq!(practitioner.gender_system, Some("http://hl7.org/fhir/administrative-gender".to_string()));
        assert_eq!(practitioner.gender_display, Some("Male".to_string()));
        assert_eq!(practitioner.birth_date, Some("1980-05-15".to_string()));
        assert_eq!(practitioner.deceased, Some(false));
        assert_eq!(practitioner.address_use, Some(vec!["work".to_string()]));
        assert_eq!(practitioner.address_type, Some(vec!["physical".to_string()]));
        assert_eq!(practitioner.address_text, Some(vec!["123 Main St, Suite 100, Anytown, ST 12345, USA".to_string()]));
        assert_eq!(practitioner.address_line, Some(vec![vec!["123 Main St".to_string(), "Suite 100".to_string()]]));
        assert_eq!(practitioner.address_city, Some(vec!["Anytown".to_string()]));
        assert_eq!(practitioner.address_state, Some(vec!["ST".to_string()]));
        assert_eq!(practitioner.address_postal_code, Some(vec!["12345".to_string()]));
        assert_eq!(practitioner.address_country, Some(vec!["USA".to_string()]));
        assert_eq!(practitioner.photo_content_type, Some(vec!["image/jpeg".to_string()]));
        assert_eq!(practitioner.photo_url, Some(vec!["https://hospital.com/photos/john_smith.jpg".to_string()]));
        assert_eq!(practitioner.photo_title, Some(vec!["Dr. John Smith - Headshot".to_string()]));
        assert_eq!(practitioner.qualification_codes, Some(vec!["MD".to_string(), "Internal Medicine".to_string()]));
        assert_eq!(practitioner.qualification_code_codes, Some(vec!["MD".to_string(), "IM".to_string()]));
        assert_eq!(practitioner.qualification_code_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/v2-0360".to_string(), "http://terminology.hl7.org/CodeSystem/v2-0402".to_string()]));
        assert_eq!(practitioner.qualification_code_displays, Some(vec!["Doctor of Medicine".to_string(), "Internal Medicine".to_string()]));
        assert_eq!(practitioner.qualification_periods_start, Some(vec!["2010-06-01T00:00:00Z".to_string()]));
        assert_eq!(practitioner.qualification_periods_end, Some(vec!["2030-06-01T00:00:00Z".to_string()]));
        assert_eq!(practitioner.qualification_issuer_ids, Some(vec!["org_medical_board".to_string()]));
        assert_eq!(practitioner.qualification_issuer_types, Some(vec!["Organization".to_string()]));
        assert_eq!(practitioner.communication_languages, Some(vec!["English".to_string(), "Spanish".to_string()]));
        assert_eq!(practitioner.communication_language_codes, Some(vec!["en".to_string(), "es".to_string()]));
        assert_eq!(practitioner.communication_language_systems, Some(vec!["urn:ietf:bcp:47".to_string(), "urn:ietf:bcp:47".to_string()]));
        assert_eq!(practitioner.communication_language_displays, Some(vec!["English".to_string(), "Spanish".to_string()]));
        assert_eq!(practitioner.communication_preferred, Some(vec![true, false]));
    }

    #[test]
    fn test_domain_practitioner_minimal_deserialization() {
        let json = r#"{
            "practitioner_id": "prac_67890"
        }"#;

        let practitioner: DomainPractitioner = serde_json::from_str(json).unwrap();
        
        assert_eq!(practitioner.practitioner_id, "prac_67890");
        assert_eq!(practitioner.active, None);
        assert_eq!(practitioner.family_name, None);
        assert_eq!(practitioner.given_names, None);
        assert_eq!(practitioner.prefix, None);
        assert_eq!(practitioner.suffix, None);
        assert_eq!(practitioner.use_code, None);
        assert_eq!(practitioner.text, None);
        assert_eq!(practitioner.telecom_system, None);
        assert_eq!(practitioner.telecom_value, None);
        assert_eq!(practitioner.telecom_use, None);
        assert_eq!(practitioner.telecom_rank, None);
        assert_eq!(practitioner.telecom_period_start, None);
        assert_eq!(practitioner.telecom_period_end, None);
        assert_eq!(practitioner.gender, None);
        assert_eq!(practitioner.gender_code, None);
        assert_eq!(practitioner.gender_system, None);
        assert_eq!(practitioner.gender_display, None);
        assert_eq!(practitioner.birth_date, None);
        assert_eq!(practitioner.deceased, None);
        assert_eq!(practitioner.deceased_date, None);
        assert_eq!(practitioner.address_use, None);
        assert_eq!(practitioner.address_type, None);
        assert_eq!(practitioner.address_text, None);
        assert_eq!(practitioner.address_line, None);
        assert_eq!(practitioner.address_city, None);
        assert_eq!(practitioner.address_district, None);
        assert_eq!(practitioner.address_state, None);
        assert_eq!(practitioner.address_postal_code, None);
        assert_eq!(practitioner.address_country, None);
        assert_eq!(practitioner.address_period_start, None);
        assert_eq!(practitioner.address_period_end, None);
        assert_eq!(practitioner.photo_content_type, None);
        assert_eq!(practitioner.photo_language, None);
        assert_eq!(practitioner.photo_data, None);
        assert_eq!(practitioner.photo_url, None);
        assert_eq!(practitioner.photo_size, None);
        assert_eq!(practitioner.photo_hash, None);
        assert_eq!(practitioner.photo_title, None);
        assert_eq!(practitioner.photo_creation, None);
        assert_eq!(practitioner.qualification_identifiers, None);
        assert_eq!(practitioner.qualification_identifier_systems, None);
        assert_eq!(practitioner.qualification_identifier_values, None);
        assert_eq!(practitioner.qualification_identifier_uses, None);
        assert_eq!(practitioner.qualification_identifier_periods_start, None);
        assert_eq!(practitioner.qualification_identifier_periods_end, None);
        assert_eq!(practitioner.qualification_codes, None);
        assert_eq!(practitioner.qualification_code_codes, None);
        assert_eq!(practitioner.qualification_code_systems, None);
        assert_eq!(practitioner.qualification_code_displays, None);
        assert_eq!(practitioner.qualification_periods_start, None);
        assert_eq!(practitioner.qualification_periods_end, None);
        assert_eq!(practitioner.qualification_issuer_ids, None);
        assert_eq!(practitioner.qualification_issuer_types, None);
        assert_eq!(practitioner.communication_languages, None);
        assert_eq!(practitioner.communication_language_codes, None);
        assert_eq!(practitioner.communication_language_systems, None);
        assert_eq!(practitioner.communication_language_displays, None);
        assert_eq!(practitioner.communication_preferred, None);
    }

    #[test]
    fn test_domain_practitioner_nurse() {
        let json = r#"{
            "practitioner_id": "prac_nurse_001",
            "active": true,
            "family_name": "Johnson",
            "given_names": ["Sarah", "Elizabeth"],
            "prefix": ["RN"],
            "suffix": ["BSN"],
            "use_code": "official",
            "text": "Sarah Elizabeth Johnson, RN, BSN",
            "telecom_system": ["phone", "email"],
            "telecom_value": ["+1-555-987-6543", "sarah.johnson@hospital.com"],
            "telecom_use": ["work", "work"],
            "telecom_rank": [1, 2],
            "gender": "female",
            "gender_code": "F",
            "gender_system": "http://hl7.org/fhir/administrative-gender",
            "gender_display": "Female",
            "birth_date": "1985-08-22",
            "deceased": false,
            "address_use": ["home"],
            "address_type": ["physical"],
            "address_text": "456 Oak Ave, Apartment 2B, Anytown, ST 12346, USA",
            "address_line": [["456 Oak Ave", "Apartment 2B"]],
            "address_city": ["Anytown"],
            "address_state": ["ST"],
            "address_postal_code": ["12346"],
            "address_country": ["USA"],
            "photo_content_type": ["image/jpeg"],
            "photo_url": ["https://hospital.com/photos/sarah_johnson.jpg"],
            "photo_title": ["Sarah Johnson - Professional Photo"],
            "qualification_codes": ["RN", "BSN", "Critical Care"],
            "qualification_code_codes": ["RN", "BSN", "CC"],
            "qualification_code_systems": ["http://terminology.hl7.org/CodeSystem/v2-0360", "http://terminology.hl7.org/CodeSystem/v2-0360", "http://terminology.hl7.org/CodeSystem/v2-0402"],
            "qualification_code_displays": ["Registered Nurse", "Bachelor of Science in Nursing", "Critical Care"],
            "qualification_periods_start": ["2008-05-01T00:00:00Z", "2008-05-01T00:00:00Z", "2015-01-01T00:00:00Z"],
            "qualification_periods_end": ["2028-05-01T00:00:00Z", "2028-05-01T00:00:00Z", "2025-01-01T00:00:00Z"],
            "qualification_issuer_ids": ["org_nursing_board", "org_nursing_board", "org_critical_care_cert"],
            "qualification_issuer_types": ["Organization", "Organization", "Organization"],
            "communication_languages": ["English", "French"],
            "communication_language_codes": ["en", "fr"],
            "communication_language_systems": ["urn:ietf:bcp:47", "urn:ietf:bcp:47"],
            "communication_language_displays": ["English", "French"],
            "communication_preferred": [true, false]
        }"#;

        let practitioner: DomainPractitioner = serde_json::from_str(json).unwrap();
        
        assert_eq!(practitioner.practitioner_id, "prac_nurse_001");
        assert_eq!(practitioner.active, Some(true));
        assert_eq!(practitioner.family_name, Some("Johnson".to_string()));
        assert_eq!(practitioner.given_names, Some(vec!["Sarah".to_string(), "Elizabeth".to_string()]));
        assert_eq!(practitioner.prefix, Some(vec!["RN".to_string()]));
        assert_eq!(practitioner.suffix, Some(vec!["BSN".to_string()]));
        assert_eq!(practitioner.use_code, Some("official".to_string()));
        assert_eq!(practitioner.text, Some("Sarah Elizabeth Johnson, RN, BSN".to_string()));
        assert_eq!(practitioner.telecom_system, Some(vec!["phone".to_string(), "email".to_string()]));
        assert_eq!(practitioner.telecom_value, Some(vec!["+1-555-987-6543".to_string(), "sarah.johnson@hospital.com".to_string()]));
        assert_eq!(practitioner.telecom_use, Some(vec!["work".to_string(), "work".to_string()]));
        assert_eq!(practitioner.telecom_rank, Some(vec![1, 2]));
        assert_eq!(practitioner.gender, Some("female".to_string()));
        assert_eq!(practitioner.gender_code, Some("F".to_string()));
        assert_eq!(practitioner.gender_system, Some("http://hl7.org/fhir/administrative-gender".to_string()));
        assert_eq!(practitioner.gender_display, Some("Female".to_string()));
        assert_eq!(practitioner.birth_date, Some("1985-08-22".to_string()));
        assert_eq!(practitioner.deceased, Some(false));
        assert_eq!(practitioner.address_use, Some(vec!["home".to_string()]));
        assert_eq!(practitioner.address_type, Some(vec!["physical".to_string()]));
        assert_eq!(practitioner.address_text, Some(vec!["456 Oak Ave, Apartment 2B, Anytown, ST 12346, USA".to_string()]));
        assert_eq!(practitioner.address_line, Some(vec![vec!["456 Oak Ave".to_string(), "Apartment 2B".to_string()]]));
        assert_eq!(practitioner.address_city, Some(vec!["Anytown".to_string()]));
        assert_eq!(practitioner.address_state, Some(vec!["ST".to_string()]));
        assert_eq!(practitioner.address_postal_code, Some(vec!["12346".to_string()]));
        assert_eq!(practitioner.address_country, Some(vec!["USA".to_string()]));
        assert_eq!(practitioner.photo_content_type, Some(vec!["image/jpeg".to_string()]));
        assert_eq!(practitioner.photo_url, Some(vec!["https://hospital.com/photos/sarah_johnson.jpg".to_string()]));
        assert_eq!(practitioner.photo_title, Some(vec!["Sarah Johnson - Professional Photo".to_string()]));
        assert_eq!(practitioner.qualification_codes, Some(vec!["RN".to_string(), "BSN".to_string(), "Critical Care".to_string()]));
        assert_eq!(practitioner.qualification_code_codes, Some(vec!["RN".to_string(), "BSN".to_string(), "CC".to_string()]));
        assert_eq!(practitioner.qualification_code_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/v2-0360".to_string(), "http://terminology.hl7.org/CodeSystem/v2-0360".to_string(), "http://terminology.hl7.org/CodeSystem/v2-0402".to_string()]));
        assert_eq!(practitioner.qualification_code_displays, Some(vec!["Registered Nurse".to_string(), "Bachelor of Science in Nursing".to_string(), "Critical Care".to_string()]));
        assert_eq!(practitioner.qualification_periods_start, Some(vec!["2008-05-01T00:00:00Z".to_string(), "2008-05-01T00:00:00Z".to_string(), "2015-01-01T00:00:00Z".to_string()]));
        assert_eq!(practitioner.qualification_periods_end, Some(vec!["2028-05-01T00:00:00Z".to_string(), "2028-05-01T00:00:00Z".to_string(), "2025-01-01T00:00:00Z".to_string()]));
        assert_eq!(practitioner.qualification_issuer_ids, Some(vec!["org_nursing_board".to_string(), "org_nursing_board".to_string(), "org_critical_care_cert".to_string()]));
        assert_eq!(practitioner.qualification_issuer_types, Some(vec!["Organization".to_string(), "Organization".to_string(), "Organization".to_string()]));
        assert_eq!(practitioner.communication_languages, Some(vec!["English".to_string(), "French".to_string()]));
        assert_eq!(practitioner.communication_language_codes, Some(vec!["en".to_string(), "fr".to_string()]));
        assert_eq!(practitioner.communication_language_systems, Some(vec!["urn:ietf:bcp:47".to_string(), "urn:ietf:bcp:47".to_string()]));
        assert_eq!(practitioner.communication_language_displays, Some(vec!["English".to_string(), "French".to_string()]));
        assert_eq!(practitioner.communication_preferred, Some(vec![true, false]));
    }

    #[test]
    fn test_domain_practitioner_missing_required_field() {
        let json = r#"{
            "telecom_system": ["phone"]
        }"#;

        // This should fail because practitioner_id is required
        let result: Result<DomainPractitioner, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
