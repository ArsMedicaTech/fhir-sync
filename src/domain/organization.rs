use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainOrganization {
    pub organization_id: String,
    
    // Basic organization information
    pub active: Option<bool>, // Whether the organization's record is still in active use
    
    // Type and identification
    pub types: Option<Vec<String>>, // Kind of organization
    pub type_codes: Option<Vec<String>>, // Codes for types
    pub type_systems: Option<Vec<String>>, // Terminology systems for types
    pub type_displays: Option<Vec<String>>, // Display names for types
    
    // Name information
    pub name: Option<String>, // Name used for the organization
    pub alias: Option<Vec<String>>, // A list of alternate names that the organization is known as
    pub description: Option<String>, // Additional details about the Organization
    
    // Contact information
    pub contact_purpose: Option<Vec<String>>, // The purpose of this contact
    pub contact_purpose_codes: Option<Vec<String>>, // Codes for contact purposes
    pub contact_purpose_systems: Option<Vec<String>>, // Terminology systems for contact purposes
    pub contact_purpose_displays: Option<Vec<String>>, // Display names for contact purposes
    pub contact_name: Option<Vec<String>>, // Name of an individual to contact
    pub contact_telecom_system: Option<Vec<Vec<String>>>, // Contact system (phone, fax, email, pager, url, sms, other)
    pub contact_telecom_value: Option<Vec<Vec<String>>>, // Contact value
    pub contact_telecom_use: Option<Vec<Vec<String>>>, // Contact use (work, temp, old, mobile)
    pub contact_telecom_rank: Option<Vec<Vec<u32>>>, // Specify preferred order of use (1 = highest)
    pub contact_address_use: Option<Vec<String>>, // Address use (work, temp, old, billing)
    pub contact_address_type: Option<Vec<String>>, // Address type (postal, physical, both)
    pub contact_address_text: Option<Vec<String>>, // Text representation of the address
    pub contact_address_line: Option<Vec<Vec<String>>>, // Street address lines
    pub contact_address_city: Option<Vec<String>>, // City name
    pub contact_address_district: Option<Vec<String>>, // District name (sublocality)
    pub contact_address_state: Option<Vec<String>>, // State name
    pub contact_address_postal_code: Option<Vec<String>>, // Postal code
    pub contact_address_country: Option<Vec<String>>, // Country name
    pub contact_organization_id: Option<Vec<String>>, // Organization associated with the contact
    pub contact_organization_type: Option<Vec<String>>, // Type of organization
    pub contact_period_start: Option<Vec<String>>, // Time period when the contact was/is in use (ISO datetime)
    pub contact_period_end: Option<Vec<String>>, // Time period when the contact was/is in use (ISO datetime)
    
    // Hierarchy
    pub part_of_id: Option<String>, // The organization of which this organization forms a part
    pub part_of_type: Option<String>, // Type of part of organization
    
    // Endpoints
    pub endpoint_ids: Option<Vec<String>>, // Technical endpoints providing access to services
    pub endpoint_types: Option<Vec<String>>, // Types of endpoints
    
    // Qualifications
    pub qualification_identifiers: Option<Vec<Vec<String>>>, // An identifier for this qualification for the organization
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_organization_deserialization() {
        let json = r#"{
            "organization_id": "org_12345",
            "active": true,
            "types": ["hospital", "healthcare-provider"],
            "type_codes": ["HOSP", "HCP"],
            "type_systems": ["http://terminology.hl7.org/CodeSystem/organization-type", "http://terminology.hl7.org/CodeSystem/organization-type"],
            "type_displays": ["Hospital", "Healthcare Provider"],
            "name": "Anytown General Hospital",
            "alias": ["AGH", "Anytown Hospital", "General Hospital"],
            "description": "A comprehensive healthcare facility providing emergency, inpatient, and outpatient services",
            "contact_purpose": ["general", "emergency", "billing"],
            "contact_purpose_codes": ["GENERAL", "EMERGENCY", "BILLING"],
            "contact_purpose_systems": ["http://terminology.hl7.org/CodeSystem/contactentity-type", "http://terminology.hl7.org/CodeSystem/contactentity-type", "http://terminology.hl7.org/CodeSystem/contactentity-type"],
            "contact_purpose_displays": ["General", "Emergency", "Billing"],
            "contact_name": ["Main Hospital", "Emergency Department", "Billing Department"],
            "contact_telecom_system": [["phone", "email"], ["phone", "pager"], ["phone", "email"]],
            "contact_telecom_value": [["+1-555-123-4567", "info@hospital.com"], ["+1-555-123-4568", "emergency@hospital.com"], ["+1-555-123-4569", "billing@hospital.com"]],
            "contact_telecom_use": [["work", "work"], ["work", "work"], ["work", "work"]],
            "contact_telecom_rank": [[1, 2], [1, 2], [1, 2]],
            "contact_address_use": ["work", "work", "work"],
            "contact_address_type": ["physical", "physical", "physical"],
            "contact_address_text": ["123 Main St, Anytown, ST 12345, USA", "123 Main St, Anytown, ST 12345, USA", "123 Main St, Anytown, ST 12345, USA"],
            "contact_address_line": [["123 Main St"], ["123 Main St"], ["123 Main St"]],
            "contact_address_city": ["Anytown", "Anytown", "Anytown"],
            "contact_address_state": ["ST", "ST", "ST"],
            "contact_address_postal_code": ["12345", "12345", "12345"],
            "contact_address_country": ["USA", "USA", "USA"],
            "contact_organization_id": ["org_hospital", "org_hospital", "org_hospital"],
            "contact_organization_type": ["Organization", "Organization", "Organization"],
            "contact_period_start": ["2020-01-01T00:00:00Z", "2020-01-01T00:00:00Z", "2020-01-01T00:00:00Z"],
            "contact_period_end": ["2030-01-01T00:00:00Z", "2030-01-01T00:00:00Z", "2030-01-01T00:00:00Z"],
            "part_of_id": "org_health_system",
            "part_of_type": "Organization",
            "endpoint_ids": ["endpoint_hospital_1", "endpoint_hospital_2"],
            "endpoint_types": ["Endpoint", "Endpoint"],
            "qualification_codes": ["JCAHO", "CMS", "State License"],
            "qualification_code_codes": ["JCAHO", "CMS", "STATE"],
            "qualification_code_systems": ["http://terminology.hl7.org/CodeSystem/organization-qualification", "http://terminology.hl7.org/CodeSystem/organization-qualification", "http://terminology.hl7.org/CodeSystem/organization-qualification"],
            "qualification_code_displays": ["Joint Commission Accreditation", "CMS Certification", "State Healthcare License"],
            "qualification_periods_start": ["2020-01-01T00:00:00Z", "2020-01-01T00:00:00Z", "2020-01-01T00:00:00Z"],
            "qualification_periods_end": ["2025-01-01T00:00:00Z", "2025-01-01T00:00:00Z", "2025-01-01T00:00:00Z"],
            "qualification_issuer_ids": ["org_jcaho", "org_cms", "org_state_health"],
            "qualification_issuer_types": ["Organization", "Organization", "Organization"]
        }"#;

        let organization: DomainOrganization = serde_json::from_str(json).unwrap();
        
        assert_eq!(organization.organization_id, "org_12345");
        assert_eq!(organization.active, Some(true));
        assert_eq!(organization.types, Some(vec!["hospital".to_string(), "healthcare-provider".to_string()]));
        assert_eq!(organization.type_codes, Some(vec!["HOSP".to_string(), "HCP".to_string()]));
        assert_eq!(organization.type_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/organization-type".to_string(), "http://terminology.hl7.org/CodeSystem/organization-type".to_string()]));
        assert_eq!(organization.type_displays, Some(vec!["Hospital".to_string(), "Healthcare Provider".to_string()]));
        assert_eq!(organization.name, Some("Anytown General Hospital".to_string()));
        assert_eq!(organization.alias, Some(vec!["AGH".to_string(), "Anytown Hospital".to_string(), "General Hospital".to_string()]));
        assert_eq!(organization.description, Some("A comprehensive healthcare facility providing emergency, inpatient, and outpatient services".to_string()));
        assert_eq!(organization.contact_purpose, Some(vec!["general".to_string(), "emergency".to_string(), "billing".to_string()]));
        assert_eq!(organization.contact_purpose_codes, Some(vec!["GENERAL".to_string(), "EMERGENCY".to_string(), "BILLING".to_string()]));
        assert_eq!(organization.contact_purpose_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/contactentity-type".to_string(), "http://terminology.hl7.org/CodeSystem/contactentity-type".to_string(), "http://terminology.hl7.org/CodeSystem/contactentity-type".to_string()]));
        assert_eq!(organization.contact_purpose_displays, Some(vec!["General".to_string(), "Emergency".to_string(), "Billing".to_string()]));
        assert_eq!(organization.contact_name, Some(vec!["Main Hospital".to_string(), "Emergency Department".to_string(), "Billing Department".to_string()]));
        assert_eq!(organization.contact_telecom_system, Some(vec![vec!["phone".to_string(), "email".to_string()], vec!["phone".to_string(), "pager".to_string()], vec!["phone".to_string(), "email".to_string()]]));
        assert_eq!(organization.contact_telecom_value, Some(vec![vec!["+1-555-123-4567".to_string(), "info@hospital.com".to_string()], vec!["+1-555-123-4568".to_string(), "emergency@hospital.com".to_string()], vec!["+1-555-123-4569".to_string(), "billing@hospital.com".to_string()]]));
        assert_eq!(organization.contact_telecom_use, Some(vec![vec!["work".to_string(), "work".to_string()], vec!["work".to_string(), "work".to_string()], vec!["work".to_string(), "work".to_string()]]));
        assert_eq!(organization.contact_telecom_rank, Some(vec![vec![1, 2], vec![1, 2], vec![1, 2]]));
        assert_eq!(organization.contact_address_use, Some(vec!["work".to_string(), "work".to_string(), "work".to_string()]));
        assert_eq!(organization.contact_address_type, Some(vec!["physical".to_string(), "physical".to_string(), "physical".to_string()]));
        assert_eq!(organization.contact_address_text, Some(vec!["123 Main St, Anytown, ST 12345, USA".to_string(), "123 Main St, Anytown, ST 12345, USA".to_string(), "123 Main St, Anytown, ST 12345, USA".to_string()]));
        assert_eq!(organization.contact_address_line, Some(vec![vec!["123 Main St".to_string()], vec!["123 Main St".to_string()], vec!["123 Main St".to_string()]]));
        assert_eq!(organization.contact_address_city, Some(vec!["Anytown".to_string(), "Anytown".to_string(), "Anytown".to_string()]));
        assert_eq!(organization.contact_address_state, Some(vec!["ST".to_string(), "ST".to_string(), "ST".to_string()]));
        assert_eq!(organization.contact_address_postal_code, Some(vec!["12345".to_string(), "12345".to_string(), "12345".to_string()]));
        assert_eq!(organization.contact_address_country, Some(vec!["USA".to_string(), "USA".to_string(), "USA".to_string()]));
        assert_eq!(organization.contact_organization_id, Some(vec!["org_hospital".to_string(), "org_hospital".to_string(), "org_hospital".to_string()]));
        assert_eq!(organization.contact_organization_type, Some(vec!["Organization".to_string(), "Organization".to_string(), "Organization".to_string()]));
        assert_eq!(organization.contact_period_start, Some(vec!["2020-01-01T00:00:00Z".to_string(), "2020-01-01T00:00:00Z".to_string(), "2020-01-01T00:00:00Z".to_string()]));
        assert_eq!(organization.contact_period_end, Some(vec!["2030-01-01T00:00:00Z".to_string(), "2030-01-01T00:00:00Z".to_string(), "2030-01-01T00:00:00Z".to_string()]));
        assert_eq!(organization.part_of_id, Some("org_health_system".to_string()));
        assert_eq!(organization.part_of_type, Some("Organization".to_string()));
        assert_eq!(organization.endpoint_ids, Some(vec!["endpoint_hospital_1".to_string(), "endpoint_hospital_2".to_string()]));
        assert_eq!(organization.endpoint_types, Some(vec!["Endpoint".to_string(), "Endpoint".to_string()]));
        assert_eq!(organization.qualification_codes, Some(vec!["JCAHO".to_string(), "CMS".to_string(), "State License".to_string()]));
        assert_eq!(organization.qualification_code_codes, Some(vec!["JCAHO".to_string(), "CMS".to_string(), "STATE".to_string()]));
        assert_eq!(organization.qualification_code_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/organization-qualification".to_string(), "http://terminology.hl7.org/CodeSystem/organization-qualification".to_string(), "http://terminology.hl7.org/CodeSystem/organization-qualification".to_string()]));
        assert_eq!(organization.qualification_code_displays, Some(vec!["Joint Commission Accreditation".to_string(), "CMS Certification".to_string(), "State Healthcare License".to_string()]));
        assert_eq!(organization.qualification_periods_start, Some(vec!["2020-01-01T00:00:00Z".to_string(), "2020-01-01T00:00:00Z".to_string(), "2020-01-01T00:00:00Z".to_string()]));
        assert_eq!(organization.qualification_periods_end, Some(vec!["2025-01-01T00:00:00Z".to_string(), "2025-01-01T00:00:00Z".to_string(), "2025-01-01T00:00:00Z".to_string()]));
        assert_eq!(organization.qualification_issuer_ids, Some(vec!["org_jcaho".to_string(), "org_cms".to_string(), "org_state_health".to_string()]));
        assert_eq!(organization.qualification_issuer_types, Some(vec!["Organization".to_string(), "Organization".to_string(), "Organization".to_string()]));
    }

    #[test]
    fn test_domain_organization_minimal_deserialization() {
        let json = r#"{
            "organization_id": "org_67890"
        }"#;

        let organization: DomainOrganization = serde_json::from_str(json).unwrap();
        
        assert_eq!(organization.organization_id, "org_67890");
        assert_eq!(organization.active, None);
        assert_eq!(organization.types, None);
        assert_eq!(organization.type_codes, None);
        assert_eq!(organization.type_systems, None);
        assert_eq!(organization.type_displays, None);
        assert_eq!(organization.name, None);
        assert_eq!(organization.alias, None);
        assert_eq!(organization.description, None);
        assert_eq!(organization.contact_purpose, None);
        assert_eq!(organization.contact_purpose_codes, None);
        assert_eq!(organization.contact_purpose_systems, None);
        assert_eq!(organization.contact_purpose_displays, None);
        assert_eq!(organization.contact_name, None);
        assert_eq!(organization.contact_telecom_system, None);
        assert_eq!(organization.contact_telecom_value, None);
        assert_eq!(organization.contact_telecom_use, None);
        assert_eq!(organization.contact_telecom_rank, None);
        assert_eq!(organization.contact_address_use, None);
        assert_eq!(organization.contact_address_type, None);
        assert_eq!(organization.contact_address_text, None);
        assert_eq!(organization.contact_address_line, None);
        assert_eq!(organization.contact_address_city, None);
        assert_eq!(organization.contact_address_district, None);
        assert_eq!(organization.contact_address_state, None);
        assert_eq!(organization.contact_address_postal_code, None);
        assert_eq!(organization.contact_address_country, None);
        assert_eq!(organization.contact_organization_id, None);
        assert_eq!(organization.contact_organization_type, None);
        assert_eq!(organization.contact_period_start, None);
        assert_eq!(organization.contact_period_end, None);
        assert_eq!(organization.part_of_id, None);
        assert_eq!(organization.part_of_type, None);
        assert_eq!(organization.endpoint_ids, None);
        assert_eq!(organization.endpoint_types, None);
        assert_eq!(organization.qualification_identifiers, None);
        assert_eq!(organization.qualification_identifier_systems, None);
        assert_eq!(organization.qualification_identifier_values, None);
        assert_eq!(organization.qualification_identifier_uses, None);
        assert_eq!(organization.qualification_identifier_periods_start, None);
        assert_eq!(organization.qualification_identifier_periods_end, None);
        assert_eq!(organization.qualification_codes, None);
        assert_eq!(organization.qualification_code_codes, None);
        assert_eq!(organization.qualification_code_systems, None);
        assert_eq!(organization.qualification_code_displays, None);
        assert_eq!(organization.qualification_periods_start, None);
        assert_eq!(organization.qualification_periods_end, None);
        assert_eq!(organization.qualification_issuer_ids, None);
        assert_eq!(organization.qualification_issuer_types, None);
    }

    #[test]
    fn test_domain_organization_missing_required_field() {
        let json = r#"{
            "active": true
        }"#;

        // This should fail because organization_id is required
        let result: Result<DomainOrganization, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
