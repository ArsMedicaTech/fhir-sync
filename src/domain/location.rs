use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainLocation {
    pub location_id: String,
    
    // Basic location information
    pub status: Option<String>, // "active" | "suspended" | "inactive"
    pub operational_status: Option<String>, // The operational status of the location
    pub operational_status_code: Option<String>, // Code for operational status
    pub operational_status_system: Option<String>, // Terminology system for operational status
    pub operational_status_display: Option<String>, // Display name for operational status
    pub name: Option<String>, // Name of the location as used by humans
    pub alias: Option<Vec<String>>, // A list of alternate names that the location is known as
    pub description: Option<String>, // Additional details about the location
    pub mode: Option<String>, // "instance" | "kind"
    
    // Type and function information
    pub types: Option<Vec<String>>, // Type of function performed
    pub type_codes: Option<Vec<String>>, // Codes for types
    pub type_systems: Option<Vec<String>>, // Terminology systems for types
    pub type_displays: Option<Vec<String>>, // Display names for types
    
    // Contact information
    pub contact_purpose: Option<Vec<String>>, // The purpose of this contact
    pub contact_purpose_codes: Option<Vec<String>>, // Codes for contact purposes
    pub contact_purpose_systems: Option<Vec<String>>, // Terminology systems for contact purposes
    pub contact_purpose_displays: Option<Vec<String>>, // Display names for contact purposes
    pub contact_name: Option<Vec<String>>, // Name of an individual to contact
    pub contact_telecom_system: Option<Vec<Vec<String>>>, // Contact system (phone, fax, email, pager, url, sms, other)
    pub contact_telecom_value: Option<Vec<Vec<String>>>, // Contact value
    pub contact_telecom_use: Option<Vec<Vec<String>>>, // Contact use (home, work, temp, old, mobile)
    pub contact_telecom_rank: Option<Vec<Vec<u32>>>, // Specify preferred order of use (1 = highest)
    pub contact_address_use: Option<Vec<String>>, // Address use (home, work, temp, old, billing)
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
    
    // Physical location
    pub address_use: Option<String>, // Address use (home, work, temp, old, billing)
    pub address_type: Option<String>, // Address type (postal, physical, both)
    pub address_text: Option<String>, // Text representation of the address
    pub address_line: Option<Vec<String>>, // Street address lines
    pub address_city: Option<String>, // City name
    pub address_district: Option<String>, // District name (sublocality)
    pub address_state: Option<String>, // State name
    pub address_postal_code: Option<String>, // Postal code
    pub address_country: Option<String>, // Country name
    pub address_period_start: Option<String>, // Time period when address was/is in use (ISO datetime)
    pub address_period_end: Option<String>, // Time period when address was/is in use (ISO datetime)
    
    // Physical form
    pub form: Option<String>, // Physical form of the location
    pub form_code: Option<String>, // Code for form
    pub form_system: Option<String>, // Terminology system for form
    pub form_display: Option<String>, // Display name for form
    
    // Geographic position
    pub longitude: Option<f64>, // Longitude with WGS84 datum
    pub latitude: Option<f64>, // Latitude with WGS84 datum
    pub altitude: Option<f64>, // Altitude with WGS84 datum
    
    // Organization and hierarchy
    pub managing_organization_id: Option<String>, // Organization responsible for provisioning and upkeep
    pub managing_organization_type: Option<String>, // Type of managing organization
    pub part_of_id: Option<String>, // Another Location this one is physically a part of
    pub part_of_type: Option<String>, // Type of part of location
    
    // Characteristics
    pub characteristics: Option<Vec<String>>, // Collection of characteristics (attributes)
    pub characteristic_codes: Option<Vec<String>>, // Codes for characteristics
    pub characteristic_systems: Option<Vec<String>>, // Terminology systems for characteristics
    pub characteristic_displays: Option<Vec<String>>, // Display names for characteristics
    
    // Hours of operation
    pub hours_of_operation_days_of_week: Option<Vec<Vec<String>>>, // mon | tue | wed | thu | fri | sat | sun
    pub hours_of_operation_all_day: Option<Vec<bool>>, // The location is open all day
    pub hours_of_operation_opening_time: Option<Vec<String>>, // Time that the Location opens (ISO time)
    pub hours_of_operation_closing_time: Option<Vec<String>>, // Time that the Location closes (ISO time)
    
    // Virtual service
    pub virtual_service_channel_type: Option<Vec<String>>, // Channel type for virtual service
    pub virtual_service_channel_type_codes: Option<Vec<String>>, // Codes for channel types
    pub virtual_service_channel_type_systems: Option<Vec<String>>, // Terminology systems for channel types
    pub virtual_service_channel_type_displays: Option<Vec<String>>, // Display names for channel types
    pub virtual_service_address_url: Option<Vec<String>>, // Address for virtual service
    pub virtual_service_address_extension: Option<Vec<String>>, // Extension for virtual service address
    pub virtual_service_extension: Option<Vec<String>>, // Extension for virtual service
    pub virtual_service_extension_url: Option<Vec<String>>, // URL for virtual service extension
    pub virtual_service_extension_value: Option<Vec<String>>, // Value for virtual service extension
    
    // Endpoints
    pub endpoint_ids: Option<Vec<String>>, // Technical endpoints providing access to services
    pub endpoint_types: Option<Vec<String>>, // Types of endpoints
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_location_deserialization() {
        let json = r#"{
            "location_id": "loc_12345",
            "status": "active",
            "operational_status": "occupied",
            "operational_status_code": "O",
            "operational_status_system": "http://terminology.hl7.org/CodeSystem/v2-0116",
            "operational_status_display": "Occupied",
            "name": "Main Hospital - Emergency Department",
            "alias": ["ED", "Emergency Room", "ER"],
            "description": "24/7 emergency department with 20 beds and trauma center",
            "mode": "instance",
            "types": ["emergency-department", "trauma-center"],
            "type_codes": ["ED", "TC"],
            "type_systems": ["http://terminology.hl7.org/CodeSystem/v3-RoleCode", "http://terminology.hl7.org/CodeSystem/v3-RoleCode"],
            "type_displays": ["Emergency Department", "Trauma Center"],
            "contact_purpose": ["general", "emergency"],
            "contact_purpose_codes": ["GENERAL", "EMERGENCY"],
            "contact_purpose_systems": ["http://terminology.hl7.org/CodeSystem/contactentity-type", "http://terminology.hl7.org/CodeSystem/contactentity-type"],
            "contact_purpose_displays": ["General", "Emergency"],
            "contact_name": ["Emergency Department", "Trauma Team"],
            "contact_telecom_system": [["phone", "email"], ["phone", "pager"]],
            "contact_telecom_value": [["+1-555-123-4567", "ed@hospital.com"], ["+1-555-123-4568", "trauma@hospital.com"]],
            "contact_telecom_use": [["work", "work"], ["work", "work"]],
            "contact_telecom_rank": [[1, 2], [1, 2]],
            "contact_address_use": ["work", "work"],
            "contact_address_type": ["physical", "physical"],
            "contact_address_text": ["123 Main St, Anytown, ST 12345, USA", "123 Main St, Anytown, ST 12345, USA"],
            "contact_address_line": [["123 Main St"], ["123 Main St"]],
            "contact_address_city": ["Anytown", "Anytown"],
            "contact_address_state": ["ST", "ST"],
            "contact_address_postal_code": ["12345", "12345"],
            "contact_address_country": ["USA", "USA"],
            "contact_organization_id": ["org_hospital", "org_hospital"],
            "contact_organization_type": ["Organization", "Organization"],
            "contact_period_start": ["2020-01-01T00:00:00Z", "2020-01-01T00:00:00Z"],
            "contact_period_end": ["2030-01-01T00:00:00Z", "2030-01-01T00:00:00Z"],
            "address_use": "work",
            "address_type": "physical",
            "address_text": "123 Main St, Anytown, ST 12345, USA",
            "address_line": ["123 Main St"],
            "address_city": "Anytown",
            "address_state": "ST",
            "address_postal_code": "12345",
            "address_country": "USA",
            "address_period_start": "2020-01-01T00:00:00Z",
            "address_period_end": "2030-01-01T00:00:00Z",
            "form": "building",
            "form_code": "bu",
            "form_system": "http://terminology.hl7.org/CodeSystem/location-physical-type",
            "form_display": "Building",
            "longitude": -122.4194,
            "latitude": 37.7749,
            "altitude": 10.5,
            "managing_organization_id": "org_hospital",
            "managing_organization_type": "Organization",
            "part_of_id": "loc_main_hospital",
            "part_of_type": "Location",
            "characteristics": ["wheelchair-accessible", "24-7-access"],
            "characteristic_codes": ["WHEEL", "24/7"],
            "characteristic_systems": ["http://terminology.hl7.org/CodeSystem/location-characteristic", "http://terminology.hl7.org/CodeSystem/location-characteristic"],
            "characteristic_displays": ["Wheelchair Accessible", "24/7 Access"],
            "hours_of_operation_days_of_week": [["mon", "tue", "wed", "thu", "fri", "sat", "sun"]],
            "hours_of_operation_all_day": [true],
            "hours_of_operation_opening_time": ["00:00:00"],
            "hours_of_operation_closing_time": ["23:59:59"],
            "virtual_service_channel_type": ["video", "audio"],
            "virtual_service_channel_type_codes": ["video", "audio"],
            "virtual_service_channel_type_systems": ["http://terminology.hl7.org/CodeSystem/v3-EncounterChannel", "http://terminology.hl7.org/CodeSystem/v3-EncounterChannel"],
            "virtual_service_channel_type_displays": ["Video", "Audio"],
            "virtual_service_address_url": ["https://hospital.com/ed-video", "https://hospital.com/ed-audio"],
            "endpoint_ids": ["endpoint_ed_1", "endpoint_ed_2"],
            "endpoint_types": ["Endpoint", "Endpoint"]
        }"#;

        let location: DomainLocation = serde_json::from_str(json).unwrap();
        
        assert_eq!(location.location_id, "loc_12345");
        assert_eq!(location.status, Some("active".to_string()));
        assert_eq!(location.operational_status, Some("occupied".to_string()));
        assert_eq!(location.operational_status_code, Some("O".to_string()));
        assert_eq!(location.operational_status_system, Some("http://terminology.hl7.org/CodeSystem/v2-0116".to_string()));
        assert_eq!(location.operational_status_display, Some("Occupied".to_string()));
        assert_eq!(location.name, Some("Main Hospital - Emergency Department".to_string()));
        assert_eq!(location.alias, Some(vec!["ED".to_string(), "Emergency Room".to_string(), "ER".to_string()]));
        assert_eq!(location.description, Some("24/7 emergency department with 20 beds and trauma center".to_string()));
        assert_eq!(location.mode, Some("instance".to_string()));
        assert_eq!(location.types, Some(vec!["emergency-department".to_string(), "trauma-center".to_string()]));
        assert_eq!(location.type_codes, Some(vec!["ED".to_string(), "TC".to_string()]));
        assert_eq!(location.type_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string(), "http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string()]));
        assert_eq!(location.type_displays, Some(vec!["Emergency Department".to_string(), "Trauma Center".to_string()]));
        assert_eq!(location.contact_purpose, Some(vec!["general".to_string(), "emergency".to_string()]));
        assert_eq!(location.contact_purpose_codes, Some(vec!["GENERAL".to_string(), "EMERGENCY".to_string()]));
        assert_eq!(location.contact_purpose_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/contactentity-type".to_string(), "http://terminology.hl7.org/CodeSystem/contactentity-type".to_string()]));
        assert_eq!(location.contact_purpose_displays, Some(vec!["General".to_string(), "Emergency".to_string()]));
        assert_eq!(location.contact_name, Some(vec!["Emergency Department".to_string(), "Trauma Team".to_string()]));
        assert_eq!(location.contact_telecom_system, Some(vec![vec!["phone".to_string(), "email".to_string()], vec!["phone".to_string(), "pager".to_string()]]));
        assert_eq!(location.contact_telecom_value, Some(vec![vec!["+1-555-123-4567".to_string(), "ed@hospital.com".to_string()], vec!["+1-555-123-4568".to_string(), "trauma@hospital.com".to_string()]]));
        assert_eq!(location.contact_telecom_use, Some(vec![vec!["work".to_string(), "work".to_string()], vec!["work".to_string(), "work".to_string()]]));
        assert_eq!(location.contact_telecom_rank, Some(vec![vec![1, 2], vec![1, 2]]));
        assert_eq!(location.contact_address_use, Some(vec!["work".to_string(), "work".to_string()]));
        assert_eq!(location.contact_address_type, Some(vec!["physical".to_string(), "physical".to_string()]));
        assert_eq!(location.contact_address_text, Some(vec!["123 Main St, Anytown, ST 12345, USA".to_string(), "123 Main St, Anytown, ST 12345, USA".to_string()]));
        assert_eq!(location.contact_address_line, Some(vec![vec!["123 Main St".to_string()], vec!["123 Main St".to_string()]]));
        assert_eq!(location.contact_address_city, Some(vec!["Anytown".to_string(), "Anytown".to_string()]));
        assert_eq!(location.contact_address_state, Some(vec!["ST".to_string(), "ST".to_string()]));
        assert_eq!(location.contact_address_postal_code, Some(vec!["12345".to_string(), "12345".to_string()]));
        assert_eq!(location.contact_address_country, Some(vec!["USA".to_string(), "USA".to_string()]));
        assert_eq!(location.contact_organization_id, Some(vec!["org_hospital".to_string(), "org_hospital".to_string()]));
        assert_eq!(location.contact_organization_type, Some(vec!["Organization".to_string(), "Organization".to_string()]));
        assert_eq!(location.contact_period_start, Some(vec!["2020-01-01T00:00:00Z".to_string(), "2020-01-01T00:00:00Z".to_string()]));
        assert_eq!(location.contact_period_end, Some(vec!["2030-01-01T00:00:00Z".to_string(), "2030-01-01T00:00:00Z".to_string()]));
        assert_eq!(location.address_use, Some("work".to_string()));
        assert_eq!(location.address_type, Some("physical".to_string()));
        assert_eq!(location.address_text, Some("123 Main St, Anytown, ST 12345, USA".to_string()));
        assert_eq!(location.address_line, Some(vec!["123 Main St".to_string()]));
        assert_eq!(location.address_city, Some("Anytown".to_string()));
        assert_eq!(location.address_state, Some("ST".to_string()));
        assert_eq!(location.address_postal_code, Some("12345".to_string()));
        assert_eq!(location.address_country, Some("USA".to_string()));
        assert_eq!(location.address_period_start, Some("2020-01-01T00:00:00Z".to_string()));
        assert_eq!(location.address_period_end, Some("2030-01-01T00:00:00Z".to_string()));
        assert_eq!(location.form, Some("building".to_string()));
        assert_eq!(location.form_code, Some("bu".to_string()));
        assert_eq!(location.form_system, Some("http://terminology.hl7.org/CodeSystem/location-physical-type".to_string()));
        assert_eq!(location.form_display, Some("Building".to_string()));
        assert_eq!(location.longitude, Some(-122.4194));
        assert_eq!(location.latitude, Some(37.7749));
        assert_eq!(location.altitude, Some(10.5));
        assert_eq!(location.managing_organization_id, Some("org_hospital".to_string()));
        assert_eq!(location.managing_organization_type, Some("Organization".to_string()));
        assert_eq!(location.part_of_id, Some("loc_main_hospital".to_string()));
        assert_eq!(location.part_of_type, Some("Location".to_string()));
        assert_eq!(location.characteristics, Some(vec!["wheelchair-accessible".to_string(), "24-7-access".to_string()]));
        assert_eq!(location.characteristic_codes, Some(vec!["WHEEL".to_string(), "24/7".to_string()]));
        assert_eq!(location.characteristic_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/location-characteristic".to_string(), "http://terminology.hl7.org/CodeSystem/location-characteristic".to_string()]));
        assert_eq!(location.characteristic_displays, Some(vec!["Wheelchair Accessible".to_string(), "24/7 Access".to_string()]));
        assert_eq!(location.hours_of_operation_days_of_week, Some(vec![vec!["mon".to_string(), "tue".to_string(), "wed".to_string(), "thu".to_string(), "fri".to_string(), "sat".to_string(), "sun".to_string()]]));
        assert_eq!(location.hours_of_operation_all_day, Some(vec![true]));
        assert_eq!(location.hours_of_operation_opening_time, Some(vec!["00:00:00".to_string()]));
        assert_eq!(location.hours_of_operation_closing_time, Some(vec!["23:59:59".to_string()]));
        assert_eq!(location.virtual_service_channel_type, Some(vec!["video".to_string(), "audio".to_string()]));
        assert_eq!(location.virtual_service_channel_type_codes, Some(vec!["video".to_string(), "audio".to_string()]));
        assert_eq!(location.virtual_service_channel_type_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/v3-EncounterChannel".to_string(), "http://terminology.hl7.org/CodeSystem/v3-EncounterChannel".to_string()]));
        assert_eq!(location.virtual_service_channel_type_displays, Some(vec!["Video".to_string(), "Audio".to_string()]));
        assert_eq!(location.virtual_service_address_url, Some(vec!["https://hospital.com/ed-video".to_string(), "https://hospital.com/ed-audio".to_string()]));
        assert_eq!(location.endpoint_ids, Some(vec!["endpoint_ed_1".to_string(), "endpoint_ed_2".to_string()]));
        assert_eq!(location.endpoint_types, Some(vec!["Endpoint".to_string(), "Endpoint".to_string()]));
    }

    #[test]
    fn test_domain_location_minimal_deserialization() {
        let json = r#"{
            "location_id": "loc_67890"
        }"#;

        let location: DomainLocation = serde_json::from_str(json).unwrap();
        
        assert_eq!(location.location_id, "loc_67890");
        assert_eq!(location.status, None);
        assert_eq!(location.operational_status, None);
        assert_eq!(location.operational_status_code, None);
        assert_eq!(location.operational_status_system, None);
        assert_eq!(location.operational_status_display, None);
        assert_eq!(location.name, None);
        assert_eq!(location.alias, None);
        assert_eq!(location.description, None);
        assert_eq!(location.mode, None);
        assert_eq!(location.types, None);
        assert_eq!(location.type_codes, None);
        assert_eq!(location.type_systems, None);
        assert_eq!(location.type_displays, None);
        assert_eq!(location.contact_purpose, None);
        assert_eq!(location.contact_purpose_codes, None);
        assert_eq!(location.contact_purpose_systems, None);
        assert_eq!(location.contact_purpose_displays, None);
        assert_eq!(location.contact_name, None);
        assert_eq!(location.contact_telecom_system, None);
        assert_eq!(location.contact_telecom_value, None);
        assert_eq!(location.contact_telecom_use, None);
        assert_eq!(location.contact_telecom_rank, None);
        assert_eq!(location.contact_address_use, None);
        assert_eq!(location.contact_address_type, None);
        assert_eq!(location.contact_address_text, None);
        assert_eq!(location.contact_address_line, None);
        assert_eq!(location.contact_address_city, None);
        assert_eq!(location.contact_address_district, None);
        assert_eq!(location.contact_address_state, None);
        assert_eq!(location.contact_address_postal_code, None);
        assert_eq!(location.contact_address_country, None);
        assert_eq!(location.contact_organization_id, None);
        assert_eq!(location.contact_organization_type, None);
        assert_eq!(location.contact_period_start, None);
        assert_eq!(location.contact_period_end, None);
        assert_eq!(location.address_use, None);
        assert_eq!(location.address_type, None);
        assert_eq!(location.address_text, None);
        assert_eq!(location.address_line, None);
        assert_eq!(location.address_city, None);
        assert_eq!(location.address_district, None);
        assert_eq!(location.address_state, None);
        assert_eq!(location.address_postal_code, None);
        assert_eq!(location.address_country, None);
        assert_eq!(location.address_period_start, None);
        assert_eq!(location.address_period_end, None);
        assert_eq!(location.form, None);
        assert_eq!(location.form_code, None);
        assert_eq!(location.form_system, None);
        assert_eq!(location.form_display, None);
        assert_eq!(location.longitude, None);
        assert_eq!(location.latitude, None);
        assert_eq!(location.altitude, None);
        assert_eq!(location.managing_organization_id, None);
        assert_eq!(location.managing_organization_type, None);
        assert_eq!(location.part_of_id, None);
        assert_eq!(location.part_of_type, None);
        assert_eq!(location.characteristics, None);
        assert_eq!(location.characteristic_codes, None);
        assert_eq!(location.characteristic_systems, None);
        assert_eq!(location.characteristic_displays, None);
        assert_eq!(location.hours_of_operation_days_of_week, None);
        assert_eq!(location.hours_of_operation_all_day, None);
        assert_eq!(location.hours_of_operation_opening_time, None);
        assert_eq!(location.hours_of_operation_closing_time, None);
        assert_eq!(location.virtual_service_channel_type, None);
        assert_eq!(location.virtual_service_channel_type_codes, None);
        assert_eq!(location.virtual_service_channel_type_systems, None);
        assert_eq!(location.virtual_service_channel_type_displays, None);
        assert_eq!(location.virtual_service_address_url, None);
        assert_eq!(location.virtual_service_address_extension, None);
        assert_eq!(location.virtual_service_extension, None);
        assert_eq!(location.virtual_service_extension_url, None);
        assert_eq!(location.virtual_service_extension_value, None);
        assert_eq!(location.endpoint_ids, None);
        assert_eq!(location.endpoint_types, None);
    }

    #[test]
    fn test_domain_location_missing_required_field() {
        let json = r#"{
            "status": "active"
        }"#;

        // This should fail because location_id is required
        let result: Result<DomainLocation, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
