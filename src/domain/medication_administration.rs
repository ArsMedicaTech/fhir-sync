use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainMedicationAdministration {
    pub medication_administration_id: String,
    
    // Basic information
    pub status: String, // "in-progress" | "not-done" | "on-hold" | "completed" | "entered-in-error" | "stopped" | "unknown"
    pub status_reason: Option<Vec<String>>, // Reason administration not performed
    pub status_reason_codes: Option<Vec<String>>, // Codes for status reasons
    pub status_reason_systems: Option<Vec<String>>, // Terminology systems for status reasons
    pub status_reason_displays: Option<Vec<String>>, // Display names for status reasons
    
    // Category and medication
    pub category: Option<Vec<String>>, // Type of medication administration
    pub category_codes: Option<Vec<String>>, // Codes for categories
    pub category_systems: Option<Vec<String>>, // Terminology systems for categories
    pub category_displays: Option<Vec<String>>, // Display names for categories
    pub medication_code: Option<String>, // What was administered (code)
    pub medication_code_system: Option<String>, // Terminology system for medication code
    pub medication_code_display: Option<String>, // Display name for medication code
    pub medication_reference_id: Option<String>, // Reference to Medication
    pub medication_reference_type: Option<String>, // Type of medication reference
    
    // Subject and context
    pub subject_id: String, // Who received medication (Patient or Group)
    pub subject_type: String, // "Patient" or "Group"
    pub encounter_id: Option<String>, // Encounter administered as part of
    pub encounter_type: Option<String>, // Type of encounter reference
    
    // Timing
    pub occurrence_date_time: Option<String>, // Specific date/time of administration (ISO datetime)
    pub occurrence_period_start: Option<String>, // Start of administration period (ISO datetime)
    pub occurrence_period_end: Option<String>, // End of administration period (ISO datetime)
    pub recorded: Option<String>, // When first captured in subject's record (ISO datetime)
    
    // Administration details
    pub is_sub_potent: Option<bool>, // Full dose was not administered
    pub sub_potent_reason: Option<Vec<String>>, // Reason full dose was not administered
    pub sub_potent_reason_codes: Option<Vec<String>>, // Codes for sub potent reasons
    pub sub_potent_reason_systems: Option<Vec<String>>, // Terminology systems for sub potent reasons
    pub sub_potent_reason_displays: Option<Vec<String>>, // Display names for sub potent reasons
    
    // Performer
    pub performer_function: Option<Vec<String>>, // Type of performance
    pub performer_function_codes: Option<Vec<String>>, // Codes for performer functions
    pub performer_function_systems: Option<Vec<String>>, // Terminology systems for performer functions
    pub performer_function_displays: Option<Vec<String>>, // Display names for performer functions
    pub performer_actor_id: Option<Vec<String>>, // Who or what performed the administration
    pub performer_actor_type: Option<Vec<String>>, // Type of performer actor
    pub performer_actor_code: Option<Vec<String>>, // Code for performer actor
    pub performer_actor_system: Option<Vec<String>>, // System for performer actor code
    pub performer_actor_display: Option<Vec<String>>, // Display for performer actor code
    
    // Reason and request
    pub reason_code: Option<Vec<String>>, // Concept, condition or observation that supports why administered
    pub reason_code_system: Option<Vec<String>>, // Terminology systems for reason codes
    pub reason_code_display: Option<Vec<String>>, // Display names for reason codes
    pub reason_reference_id: Option<Vec<String>>, // Reference IDs for reasons
    pub reason_reference_type: Option<Vec<String>>, // Types of reason references
    pub request_id: Option<String>, // Request administration performed against
    pub request_type: Option<String>, // Type of request reference
    
    // Device
    pub device_code: Option<Vec<String>>, // Device used to administer
    pub device_code_system: Option<Vec<String>>, // Terminology systems for device codes
    pub device_code_display: Option<Vec<String>>, // Display names for device codes
    pub device_reference_id: Option<Vec<String>>, // Reference IDs for devices
    pub device_reference_type: Option<Vec<String>>, // Types of device references
    
    // Dosage
    pub dosage_text: Option<String>, // Free text dosage instructions
    pub dosage_site: Option<String>, // Body site administered to
    pub dosage_site_code: Option<String>, // Code for dosage site
    pub dosage_site_system: Option<String>, // System for dosage site code
    pub dosage_site_display: Option<String>, // Display for dosage site code
    pub dosage_route: Option<String>, // Path of substance into body
    pub dosage_route_code: Option<String>, // Code for dosage route
    pub dosage_route_system: Option<String>, // System for dosage route code
    pub dosage_route_display: Option<String>, // Display for dosage route code
    pub dosage_method: Option<String>, // How drug was administered
    pub dosage_method_code: Option<String>, // Code for dosage method
    pub dosage_method_system: Option<String>, // System for dosage method code
    pub dosage_method_display: Option<String>, // Display for dosage method code
    pub dosage_dose_value: Option<f64>, // Amount of medication per dose
    pub dosage_dose_unit: Option<String>, // Unit for dosage dose
    pub dosage_dose_system: Option<String>, // System for dosage dose unit
    pub dosage_dose_code: Option<String>, // Code for dosage dose unit
    pub dosage_rate_ratio_numerator_value: Option<f64>, // Rate ratio numerator value
    pub dosage_rate_ratio_numerator_unit: Option<String>, // Rate ratio numerator unit
    pub dosage_rate_ratio_numerator_system: Option<String>, // Rate ratio numerator system
    pub dosage_rate_ratio_numerator_code: Option<String>, // Rate ratio numerator code
    pub dosage_rate_ratio_denominator_value: Option<f64>, // Rate ratio denominator value
    pub dosage_rate_ratio_denominator_unit: Option<String>, // Rate ratio denominator unit
    pub dosage_rate_ratio_denominator_system: Option<String>, // Rate ratio denominator system
    pub dosage_rate_ratio_denominator_code: Option<String>, // Rate ratio denominator code
    pub dosage_rate_quantity_value: Option<f64>, // Rate quantity value
    pub dosage_rate_quantity_unit: Option<String>, // Rate quantity unit
    pub dosage_rate_quantity_system: Option<String>, // Rate quantity system
    pub dosage_rate_quantity_code: Option<String>, // Rate quantity code
    
    // Notes
    pub note: Option<Vec<String>>, // Information about the administration
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_medication_administration_deserialization() {
        let json = r#"{
            "medication_administration_id": "med_admin_12345",
            "status": "completed",
            "status_reason": ["patient-refused"],
            "status_reason_codes": ["REFUSED"],
            "status_reason_systems": ["http://terminology.hl7.org/CodeSystem/medication-admin-status-reason"],
            "status_reason_displays": ["Patient Refused"],
            "category": ["inpatient"],
            "category_codes": ["INPATIENT"],
            "category_systems": ["http://terminology.hl7.org/CodeSystem/medication-admin-category"],
            "category_displays": ["Inpatient"],
            "medication_code": "acetaminophen",
            "medication_code_system": "http://www.nlm.nih.gov/research/umls/rxnorm",
            "medication_code_display": "Acetaminophen",
            "medication_reference_id": "med_12345",
            "medication_reference_type": "Medication",
            "subject_id": "patient_001",
            "subject_type": "Patient",
            "encounter_id": "encounter_001",
            "encounter_type": "Encounter",
            "occurrence_date_time": "2024-01-15T10:30:00Z",
            "recorded": "2024-01-15T10:35:00Z",
            "is_sub_potent": false,
            "sub_potent_reason": [],
            "sub_potent_reason_codes": [],
            "sub_potent_reason_systems": [],
            "sub_potent_reason_displays": [],
            "performer_function": ["performer"],
            "performer_function_codes": ["PERF"],
            "performer_function_systems": ["http://terminology.hl7.org/CodeSystem/medication-admin-performer-function"],
            "performer_function_displays": ["Performer"],
            "performer_actor_id": ["practitioner_001"],
            "performer_actor_type": ["Practitioner"],
            "performer_actor_code": ["RN"],
            "performer_actor_system": ["http://terminology.hl7.org/CodeSystem/v2-0443"],
            "performer_actor_display": ["Registered Nurse"],
            "reason_code": ["pain-management"],
            "reason_code_system": ["http://terminology.hl7.org/CodeSystem/condition-code"],
            "reason_code_display": ["Pain Management"],
            "reason_reference_id": ["condition_001"],
            "reason_reference_type": ["Condition"],
            "request_id": "med_request_001",
            "request_type": "MedicationRequest",
            "device_code": ["syringe"],
            "device_code_system": ["http://terminology.hl7.org/CodeSystem/device-type"],
            "device_code_display": ["Syringe"],
            "device_reference_id": ["device_001"],
            "device_reference_type": ["Device"],
            "dosage_text": "Take 500mg by mouth every 6 hours as needed for pain",
            "dosage_site": "oral",
            "dosage_site_code": "ORAL",
            "dosage_site_system": "http://terminology.hl7.org/CodeSystem/body-site",
            "dosage_site_display": "Oral",
            "dosage_route": "oral",
            "dosage_route_code": "PO",
            "dosage_route_system": "http://terminology.hl7.org/CodeSystem/route-codes",
            "dosage_route_display": "Oral",
            "dosage_method": "swallow",
            "dosage_method_code": "SWALLOW",
            "dosage_method_system": "http://terminology.hl7.org/CodeSystem/medication-admin-method",
            "dosage_method_display": "Swallow",
            "dosage_dose_value": 500.0,
            "dosage_dose_unit": "mg",
            "dosage_dose_system": "http://unitsofmeasure.org",
            "dosage_dose_code": "mg",
            "dosage_rate_ratio_numerator_value": 500.0,
            "dosage_rate_ratio_numerator_unit": "mg",
            "dosage_rate_ratio_numerator_system": "http://unitsofmeasure.org",
            "dosage_rate_ratio_numerator_code": "mg",
            "dosage_rate_ratio_denominator_value": 6.0,
            "dosage_rate_ratio_denominator_unit": "h",
            "dosage_rate_ratio_denominator_system": "http://unitsofmeasure.org",
            "dosage_rate_ratio_denominator_code": "h",
            "dosage_rate_quantity_value": 500.0,
            "dosage_rate_quantity_unit": "mg",
            "dosage_rate_quantity_system": "http://unitsofmeasure.org",
            "dosage_rate_quantity_code": "mg",
            "note": ["Patient tolerated medication well", "No adverse reactions observed"]
        }"#;

        let medication_administration: DomainMedicationAdministration = serde_json::from_str(json).unwrap();
        
        assert_eq!(medication_administration.medication_administration_id, "med_admin_12345");
        assert_eq!(medication_administration.status, "completed");
        assert_eq!(medication_administration.status_reason, Some(vec!["patient-refused".to_string()]));
        assert_eq!(medication_administration.status_reason_codes, Some(vec!["REFUSED".to_string()]));
        assert_eq!(medication_administration.status_reason_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/medication-admin-status-reason".to_string()]));
        assert_eq!(medication_administration.status_reason_displays, Some(vec!["Patient Refused".to_string()]));
        assert_eq!(medication_administration.category, Some(vec!["inpatient".to_string()]));
        assert_eq!(medication_administration.category_codes, Some(vec!["INPATIENT".to_string()]));
        assert_eq!(medication_administration.category_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/medication-admin-category".to_string()]));
        assert_eq!(medication_administration.category_displays, Some(vec!["Inpatient".to_string()]));
        assert_eq!(medication_administration.medication_code, Some("acetaminophen".to_string()));
        assert_eq!(medication_administration.medication_code_system, Some("http://www.nlm.nih.gov/research/umls/rxnorm".to_string()));
        assert_eq!(medication_administration.medication_code_display, Some("Acetaminophen".to_string()));
        assert_eq!(medication_administration.medication_reference_id, Some("med_12345".to_string()));
        assert_eq!(medication_administration.medication_reference_type, Some("Medication".to_string()));
        assert_eq!(medication_administration.subject_id, "patient_001");
        assert_eq!(medication_administration.subject_type, "Patient");
        assert_eq!(medication_administration.encounter_id, Some("encounter_001".to_string()));
        assert_eq!(medication_administration.encounter_type, Some("Encounter".to_string()));
        assert_eq!(medication_administration.occurrence_date_time, Some("2024-01-15T10:30:00Z".to_string()));
        assert_eq!(medication_administration.recorded, Some("2024-01-15T10:35:00Z".to_string()));
        assert_eq!(medication_administration.is_sub_potent, Some(false));
        assert_eq!(medication_administration.sub_potent_reason, Some(vec![]));
        assert_eq!(medication_administration.sub_potent_reason_codes, Some(vec![]));
        assert_eq!(medication_administration.sub_potent_reason_systems, Some(vec![]));
        assert_eq!(medication_administration.sub_potent_reason_displays, Some(vec![]));
        assert_eq!(medication_administration.performer_function, Some(vec!["performer".to_string()]));
        assert_eq!(medication_administration.performer_function_codes, Some(vec!["PERF".to_string()]));
        assert_eq!(medication_administration.performer_function_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/medication-admin-performer-function".to_string()]));
        assert_eq!(medication_administration.performer_function_displays, Some(vec!["Performer".to_string()]));
        assert_eq!(medication_administration.performer_actor_id, Some(vec!["practitioner_001".to_string()]));
        assert_eq!(medication_administration.performer_actor_type, Some(vec!["Practitioner".to_string()]));
        assert_eq!(medication_administration.performer_actor_code, Some(vec!["RN".to_string()]));
        assert_eq!(medication_administration.performer_actor_system, Some(vec!["http://terminology.hl7.org/CodeSystem/v2-0443".to_string()]));
        assert_eq!(medication_administration.performer_actor_display, Some(vec!["Registered Nurse".to_string()]));
        assert_eq!(medication_administration.reason_code, Some(vec!["pain-management".to_string()]));
        assert_eq!(medication_administration.reason_code_system, Some(vec!["http://terminology.hl7.org/CodeSystem/condition-code".to_string()]));
        assert_eq!(medication_administration.reason_code_display, Some(vec!["Pain Management".to_string()]));
        assert_eq!(medication_administration.reason_reference_id, Some(vec!["condition_001".to_string()]));
        assert_eq!(medication_administration.reason_reference_type, Some(vec!["Condition".to_string()]));
        assert_eq!(medication_administration.request_id, Some("med_request_001".to_string()));
        assert_eq!(medication_administration.request_type, Some("MedicationRequest".to_string()));
        assert_eq!(medication_administration.device_code, Some(vec!["syringe".to_string()]));
        assert_eq!(medication_administration.device_code_system, Some(vec!["http://terminology.hl7.org/CodeSystem/device-type".to_string()]));
        assert_eq!(medication_administration.device_code_display, Some(vec!["Syringe".to_string()]));
        assert_eq!(medication_administration.device_reference_id, Some(vec!["device_001".to_string()]));
        assert_eq!(medication_administration.device_reference_type, Some(vec!["Device".to_string()]));
        assert_eq!(medication_administration.dosage_text, Some("Take 500mg by mouth every 6 hours as needed for pain".to_string()));
        assert_eq!(medication_administration.dosage_site, Some("oral".to_string()));
        assert_eq!(medication_administration.dosage_site_code, Some("ORAL".to_string()));
        assert_eq!(medication_administration.dosage_site_system, Some("http://terminology.hl7.org/CodeSystem/body-site".to_string()));
        assert_eq!(medication_administration.dosage_site_display, Some("Oral".to_string()));
        assert_eq!(medication_administration.dosage_route, Some("oral".to_string()));
        assert_eq!(medication_administration.dosage_route_code, Some("PO".to_string()));
        assert_eq!(medication_administration.dosage_route_system, Some("http://terminology.hl7.org/CodeSystem/route-codes".to_string()));
        assert_eq!(medication_administration.dosage_route_display, Some("Oral".to_string()));
        assert_eq!(medication_administration.dosage_method, Some("swallow".to_string()));
        assert_eq!(medication_administration.dosage_method_code, Some("SWALLOW".to_string()));
        assert_eq!(medication_administration.dosage_method_system, Some("http://terminology.hl7.org/CodeSystem/medication-admin-method".to_string()));
        assert_eq!(medication_administration.dosage_method_display, Some("Swallow".to_string()));
        assert_eq!(medication_administration.dosage_dose_value, Some(500.0));
        assert_eq!(medication_administration.dosage_dose_unit, Some("mg".to_string()));
        assert_eq!(medication_administration.dosage_dose_system, Some("http://unitsofmeasure.org".to_string()));
        assert_eq!(medication_administration.dosage_dose_code, Some("mg".to_string()));
        assert_eq!(medication_administration.dosage_rate_ratio_numerator_value, Some(500.0));
        assert_eq!(medication_administration.dosage_rate_ratio_numerator_unit, Some("mg".to_string()));
        assert_eq!(medication_administration.dosage_rate_ratio_numerator_system, Some("http://unitsofmeasure.org".to_string()));
        assert_eq!(medication_administration.dosage_rate_ratio_numerator_code, Some("mg".to_string()));
        assert_eq!(medication_administration.dosage_rate_ratio_denominator_value, Some(6.0));
        assert_eq!(medication_administration.dosage_rate_ratio_denominator_unit, Some("h".to_string()));
        assert_eq!(medication_administration.dosage_rate_ratio_denominator_system, Some("http://unitsofmeasure.org".to_string()));
        assert_eq!(medication_administration.dosage_rate_ratio_denominator_code, Some("h".to_string()));
        assert_eq!(medication_administration.dosage_rate_quantity_value, Some(500.0));
        assert_eq!(medication_administration.dosage_rate_quantity_unit, Some("mg".to_string()));
        assert_eq!(medication_administration.dosage_rate_quantity_system, Some("http://unitsofmeasure.org".to_string()));
        assert_eq!(medication_administration.dosage_rate_quantity_code, Some("mg".to_string()));
        assert_eq!(medication_administration.note, Some(vec!["Patient tolerated medication well".to_string(), "No adverse reactions observed".to_string()]));
    }

    #[test]
    fn test_domain_medication_administration_minimal_deserialization() {
        let json = r#"{
            "medication_administration_id": "med_admin_67890",
            "status": "completed",
            "subject_id": "patient_002",
            "subject_type": "Patient"
        }"#;

        let medication_administration: DomainMedicationAdministration = serde_json::from_str(json).unwrap();
        
        assert_eq!(medication_administration.medication_administration_id, "med_admin_67890");
        assert_eq!(medication_administration.status, "completed");
        assert_eq!(medication_administration.subject_id, "patient_002");
        assert_eq!(medication_administration.subject_type, "Patient");
        assert_eq!(medication_administration.status_reason, None);
        assert_eq!(medication_administration.status_reason_codes, None);
        assert_eq!(medication_administration.status_reason_systems, None);
        assert_eq!(medication_administration.status_reason_displays, None);
        assert_eq!(medication_administration.category, None);
        assert_eq!(medication_administration.category_codes, None);
        assert_eq!(medication_administration.category_systems, None);
        assert_eq!(medication_administration.category_displays, None);
        assert_eq!(medication_administration.medication_code, None);
        assert_eq!(medication_administration.medication_code_system, None);
        assert_eq!(medication_administration.medication_code_display, None);
        assert_eq!(medication_administration.medication_reference_id, None);
        assert_eq!(medication_administration.medication_reference_type, None);
        assert_eq!(medication_administration.encounter_id, None);
        assert_eq!(medication_administration.encounter_type, None);
        assert_eq!(medication_administration.occurrence_date_time, None);
        assert_eq!(medication_administration.occurrence_period_start, None);
        assert_eq!(medication_administration.occurrence_period_end, None);
        assert_eq!(medication_administration.recorded, None);
        assert_eq!(medication_administration.is_sub_potent, None);
        assert_eq!(medication_administration.sub_potent_reason, None);
        assert_eq!(medication_administration.sub_potent_reason_codes, None);
        assert_eq!(medication_administration.sub_potent_reason_systems, None);
        assert_eq!(medication_administration.sub_potent_reason_displays, None);
        assert_eq!(medication_administration.performer_function, None);
        assert_eq!(medication_administration.performer_function_codes, None);
        assert_eq!(medication_administration.performer_function_systems, None);
        assert_eq!(medication_administration.performer_function_displays, None);
        assert_eq!(medication_administration.performer_actor_id, None);
        assert_eq!(medication_administration.performer_actor_type, None);
        assert_eq!(medication_administration.performer_actor_code, None);
        assert_eq!(medication_administration.performer_actor_system, None);
        assert_eq!(medication_administration.performer_actor_display, None);
        assert_eq!(medication_administration.reason_code, None);
        assert_eq!(medication_administration.reason_code_system, None);
        assert_eq!(medication_administration.reason_code_display, None);
        assert_eq!(medication_administration.reason_reference_id, None);
        assert_eq!(medication_administration.reason_reference_type, None);
        assert_eq!(medication_administration.request_id, None);
        assert_eq!(medication_administration.request_type, None);
        assert_eq!(medication_administration.device_code, None);
        assert_eq!(medication_administration.device_code_system, None);
        assert_eq!(medication_administration.device_code_display, None);
        assert_eq!(medication_administration.device_reference_id, None);
        assert_eq!(medication_administration.device_reference_type, None);
        assert_eq!(medication_administration.dosage_text, None);
        assert_eq!(medication_administration.dosage_site, None);
        assert_eq!(medication_administration.dosage_site_code, None);
        assert_eq!(medication_administration.dosage_site_system, None);
        assert_eq!(medication_administration.dosage_site_display, None);
        assert_eq!(medication_administration.dosage_route, None);
        assert_eq!(medication_administration.dosage_route_code, None);
        assert_eq!(medication_administration.dosage_route_system, None);
        assert_eq!(medication_administration.dosage_route_display, None);
        assert_eq!(medication_administration.dosage_method, None);
        assert_eq!(medication_administration.dosage_method_code, None);
        assert_eq!(medication_administration.dosage_method_system, None);
        assert_eq!(medication_administration.dosage_method_display, None);
        assert_eq!(medication_administration.dosage_dose_value, None);
        assert_eq!(medication_administration.dosage_dose_unit, None);
        assert_eq!(medication_administration.dosage_dose_system, None);
        assert_eq!(medication_administration.dosage_dose_code, None);
        assert_eq!(medication_administration.dosage_rate_ratio_numerator_value, None);
        assert_eq!(medication_administration.dosage_rate_ratio_numerator_unit, None);
        assert_eq!(medication_administration.dosage_rate_ratio_numerator_system, None);
        assert_eq!(medication_administration.dosage_rate_ratio_numerator_code, None);
        assert_eq!(medication_administration.dosage_rate_ratio_denominator_value, None);
        assert_eq!(medication_administration.dosage_rate_ratio_denominator_unit, None);
        assert_eq!(medication_administration.dosage_rate_ratio_denominator_system, None);
        assert_eq!(medication_administration.dosage_rate_ratio_denominator_code, None);
        assert_eq!(medication_administration.dosage_rate_quantity_value, None);
        assert_eq!(medication_administration.dosage_rate_quantity_unit, None);
        assert_eq!(medication_administration.dosage_rate_quantity_system, None);
        assert_eq!(medication_administration.dosage_rate_quantity_code, None);
        assert_eq!(medication_administration.note, None);
    }

    #[test]
    fn test_domain_medication_administration_missing_required_field() {
        let json = r#"{
            "status": "completed"
        }"#;

        // This should fail because medication_administration_id is required
        let result: Result<DomainMedicationAdministration, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
