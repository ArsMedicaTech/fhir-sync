use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainMedicationRequest {
    pub medication_request_id: String,
    
    // Basic information
    pub status: String, // "active" | "on-hold" | "ended" | "stopped" | "completed" | "cancelled" | "entered-in-error" | "draft" | "unknown"
    pub status_reason: Option<String>, // Reason for current status
    pub status_reason_code: Option<String>, // Code for status reason
    pub status_reason_system: Option<String>, // System for status reason code
    pub status_reason_display: Option<String>, // Display for status reason code
    pub status_changed: Option<String>, // When the status was changed (ISO datetime)
    pub intent: String, // "proposal" | "plan" | "order" | "original-order" | "reflex-order" | "filler-order" | "instance-order" | "option"
    
    // Category and priority
    pub category: Option<Vec<String>>, // Grouping or category of medication request
    pub category_codes: Option<Vec<String>>, // Codes for categories
    pub category_systems: Option<Vec<String>>, // Systems for category codes
    pub category_displays: Option<Vec<String>>, // Displays for category codes
    pub priority: Option<String>, // "routine" | "urgent" | "asap" | "stat"
    pub do_not_perform: Option<bool>, // True if patient is to stop taking or not to start taking the medication
    
    // Medication and subject
    pub medication_code: Option<String>, // Medication to be taken (code)
    pub medication_code_system: Option<String>, // System for medication code
    pub medication_code_display: Option<String>, // Display for medication code
    pub medication_reference_id: Option<String>, // Reference to Medication
    pub medication_reference_type: Option<String>, // Type of medication reference
    pub subject_id: String, // Individual or group for whom the medication has been requested
    pub subject_type: String, // "Patient" or "Group"
    
    // Context and timing
    pub encounter_id: Option<String>, // Encounter created as part of encounter/admission/stay
    pub encounter_type: Option<String>, // Type of encounter reference
    pub authored_on: Option<String>, // When request was initially authored (ISO datetime)
    pub reported: Option<bool>, // Reported rather than primary record
    
    // Requester and performer
    pub requester_id: Option<String>, // Who/What requested the Request
    pub requester_type: Option<String>, // Type of requester
    pub performer_type: Option<String>, // Desired kind of performer of the medication administration
    pub performer_type_code: Option<String>, // Code for performer type
    pub performer_type_system: Option<String>, // System for performer type code
    pub performer_type_display: Option<String>, // Display for performer type code
    pub performer_ids: Option<Vec<String>>, // Intended performer of administration
    pub performer_types: Option<Vec<String>>, // Types of performers
    
    // Device and recorder
    pub device_code: Option<Vec<String>>, // Intended type of device for the administration
    pub device_code_system: Option<Vec<String>>, // Systems for device codes
    pub device_code_display: Option<Vec<String>>, // Displays for device codes
    pub device_reference_id: Option<Vec<String>>, // Reference IDs for devices
    pub device_reference_type: Option<Vec<String>>, // Types of device references
    pub recorder_id: Option<String>, // Person who entered the request
    pub recorder_type: Option<String>, // Type of recorder
    
    // Reason and course
    pub reason_code: Option<Vec<String>>, // Reason or indication for ordering or not ordering the medication
    pub reason_code_system: Option<Vec<String>>, // Systems for reason codes
    pub reason_code_display: Option<Vec<String>>, // Displays for reason codes
    pub reason_reference_id: Option<Vec<String>>, // Reference IDs for reasons
    pub reason_reference_type: Option<Vec<String>>, // Types of reason references
    pub course_of_therapy_type: Option<String>, // Overall pattern of medication administration
    pub course_of_therapy_type_code: Option<String>, // Code for course of therapy type
    pub course_of_therapy_type_system: Option<String>, // System for course of therapy type code
    pub course_of_therapy_type_display: Option<String>, // Display for course of therapy type code
    
    // Insurance and notes
    pub insurance_ids: Option<Vec<String>>, // Associated insurance coverage
    pub insurance_types: Option<Vec<String>>, // Types of insurance references
    pub note: Option<Vec<String>>, // Information about the prescription
    pub rendered_dosage_instruction: Option<String>, // Full representation of the dosage instructions
    
    // Effective period
    pub effective_dose_period_start: Option<String>, // Start of period over which the medication is to be taken (ISO datetime)
    pub effective_dose_period_end: Option<String>, // End of period over which the medication is to be taken (ISO datetime)
    
    // Dosage instructions
    pub dosage_text: Option<Vec<String>>, // Free text dosage instructions
    pub dosage_site: Option<Vec<String>>, // Body site administered to
    pub dosage_site_code: Option<Vec<String>>, // Codes for dosage sites
    pub dosage_site_system: Option<Vec<String>>, // Systems for dosage site codes
    pub dosage_site_display: Option<Vec<String>>, // Displays for dosage site codes
    pub dosage_route: Option<Vec<String>>, // Path of substance into body
    pub dosage_route_code: Option<Vec<String>>, // Codes for dosage routes
    pub dosage_route_system: Option<Vec<String>>, // Systems for dosage route codes
    pub dosage_route_display: Option<Vec<String>>, // Displays for dosage route codes
    pub dosage_method: Option<Vec<String>>, // How drug was administered
    pub dosage_method_code: Option<Vec<String>>, // Codes for dosage methods
    pub dosage_method_system: Option<Vec<String>>, // Systems for dosage method codes
    pub dosage_method_display: Option<Vec<String>>, // Displays for dosage method codes
    pub dosage_dose_value: Option<Vec<f64>>, // Amount of medication per dose
    pub dosage_dose_unit: Option<Vec<String>>, // Units for dosage doses
    pub dosage_dose_system: Option<Vec<String>>, // Systems for dosage dose units
    pub dosage_dose_code: Option<Vec<String>>, // Codes for dosage dose units
    pub dosage_rate_ratio_numerator_value: Option<Vec<f64>>, // Rate ratio numerator values
    pub dosage_rate_ratio_numerator_unit: Option<Vec<String>>, // Rate ratio numerator units
    pub dosage_rate_ratio_numerator_system: Option<Vec<String>>, // Rate ratio numerator systems
    pub dosage_rate_ratio_numerator_code: Option<Vec<String>>, // Rate ratio numerator codes
    pub dosage_rate_ratio_denominator_value: Option<Vec<f64>>, // Rate ratio denominator values
    pub dosage_rate_ratio_denominator_unit: Option<Vec<String>>, // Rate ratio denominator units
    pub dosage_rate_ratio_denominator_system: Option<Vec<String>>, // Rate ratio denominator systems
    pub dosage_rate_ratio_denominator_code: Option<Vec<String>>, // Rate ratio denominator codes
    pub dosage_rate_quantity_value: Option<Vec<f64>>, // Rate quantity values
    pub dosage_rate_quantity_unit: Option<Vec<String>>, // Rate quantity units
    pub dosage_rate_quantity_system: Option<Vec<String>>, // Rate quantity systems
    pub dosage_rate_quantity_code: Option<Vec<String>>, // Rate quantity codes
    
    // Dispense request
    pub dispense_initial_fill_quantity_value: Option<f64>, // First fill quantity value
    pub dispense_initial_fill_quantity_unit: Option<String>, // First fill quantity unit
    pub dispense_initial_fill_quantity_system: Option<String>, // First fill quantity system
    pub dispense_initial_fill_quantity_code: Option<String>, // First fill quantity code
    pub dispense_initial_fill_duration_value: Option<f64>, // First fill duration value
    pub dispense_initial_fill_duration_unit: Option<String>, // First fill duration unit
    pub dispense_initial_fill_duration_system: Option<String>, // First fill duration system
    pub dispense_initial_fill_duration_code: Option<String>, // First fill duration code
    pub dispense_interval_value: Option<f64>, // Minimum period of time between dispenses value
    pub dispense_interval_unit: Option<String>, // Minimum period of time between dispenses unit
    pub dispense_interval_system: Option<String>, // Minimum period of time between dispenses system
    pub dispense_interval_code: Option<String>, // Minimum period of time between dispenses code
    pub dispense_validity_period_start: Option<String>, // Start of time period supply is authorized for (ISO datetime)
    pub dispense_validity_period_end: Option<String>, // End of time period supply is authorized for (ISO datetime)
    pub dispense_number_of_repeats_allowed: Option<u32>, // Number of refills authorized
    pub dispense_quantity_value: Option<f64>, // Amount of medication to supply per dispense value
    pub dispense_quantity_unit: Option<String>, // Amount of medication to supply per dispense unit
    pub dispense_quantity_system: Option<String>, // Amount of medication to supply per dispense system
    pub dispense_quantity_code: Option<String>, // Amount of medication to supply per dispense code
    pub dispense_expected_supply_duration_value: Option<f64>, // Number of days supply per dispense value
    pub dispense_expected_supply_duration_unit: Option<String>, // Number of days supply per dispense unit
    pub dispense_expected_supply_duration_system: Option<String>, // Number of days supply per dispense system
    pub dispense_expected_supply_duration_code: Option<String>, // Number of days supply per dispense code
    pub dispense_dispenser_id: Option<String>, // Intended performer of dispense
    pub dispense_dispenser_type: Option<String>, // Type of dispense performer
    pub dispense_dispenser_instruction: Option<Vec<String>>, // Additional information for the dispenser
    pub dispense_dose_administration_aid: Option<String>, // Type of adherence packaging to use for the dispense
    pub dispense_dose_administration_aid_code: Option<String>, // Code for dose administration aid
    pub dispense_dose_administration_aid_system: Option<String>, // System for dose administration aid code
    pub dispense_dose_administration_aid_display: Option<String>, // Display for dose administration aid code
    
    // Substitution
    pub substitution_allowed: Option<bool>, // Whether substitution is allowed or not
    pub substitution_reason: Option<String>, // Why should (not) substitution be made
    pub substitution_reason_code: Option<String>, // Code for substitution reason
    pub substitution_reason_system: Option<String>, // System for substitution reason code
    pub substitution_reason_display: Option<String>, // Display for substitution reason code
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_medication_request_deserialization() {
        let json = r#"{
            "medication_request_id": "med_request_12345",
            "status": "active",
            "status_reason": "patient-request",
            "status_reason_code": "PATREQ",
            "status_reason_system": "http://terminology.hl7.org/CodeSystem/medicationrequest-status-reason",
            "status_reason_display": "Patient Request",
            "status_changed": "2024-01-15T10:00:00Z",
            "intent": "order",
            "category": ["inpatient"],
            "category_codes": ["INPATIENT"],
            "category_systems": ["http://terminology.hl7.org/CodeSystem/medicationrequest-category"],
            "category_displays": ["Inpatient"],
            "priority": "routine",
            "do_not_perform": false,
            "medication_code": "acetaminophen",
            "medication_code_system": "http://www.nlm.nih.gov/research/umls/rxnorm",
            "medication_code_display": "Acetaminophen",
            "medication_reference_id": "med_12345",
            "medication_reference_type": "Medication",
            "subject_id": "patient_001",
            "subject_type": "Patient",
            "encounter_id": "encounter_001",
            "encounter_type": "Encounter",
            "authored_on": "2024-01-15T09:00:00Z",
            "reported": false,
            "requester_id": "practitioner_001",
            "requester_type": "Practitioner",
            "performer_type": "nurse",
            "performer_type_code": "NURSE",
            "performer_type_system": "http://terminology.hl7.org/CodeSystem/medicationrequest-performer-type",
            "performer_type_display": "Nurse",
            "performer_ids": ["practitioner_001", "practitioner_002"],
            "performer_types": ["Practitioner", "Practitioner"],
            "device_code": ["syringe"],
            "device_code_system": ["http://terminology.hl7.org/CodeSystem/device-type"],
            "device_code_display": ["Syringe"],
            "device_reference_id": ["device_001"],
            "device_reference_type": ["Device"],
            "recorder_id": "practitioner_001",
            "recorder_type": "Practitioner",
            "reason_code": ["pain-management"],
            "reason_code_system": ["http://terminology.hl7.org/CodeSystem/condition-code"],
            "reason_code_display": ["Pain Management"],
            "reason_reference_id": ["condition_001"],
            "reason_reference_type": ["Condition"],
            "course_of_therapy_type": "acute",
            "course_of_therapy_type_code": "ACUTE",
            "course_of_therapy_type_system": "http://terminology.hl7.org/CodeSystem/medicationrequest-course-of-therapy",
            "course_of_therapy_type_display": "Acute",
            "insurance_ids": ["coverage_001"],
            "insurance_types": ["Coverage"],
            "note": ["Patient requested pain medication", "No known allergies"],
            "rendered_dosage_instruction": "Take 500mg by mouth every 6 hours as needed for pain",
            "effective_dose_period_start": "2024-01-15T00:00:00Z",
            "effective_dose_period_end": "2024-01-22T23:59:59Z",
            "dosage_text": ["Take 500mg by mouth every 6 hours as needed for pain"],
            "dosage_site": ["oral"],
            "dosage_site_code": ["ORAL"],
            "dosage_site_system": ["http://terminology.hl7.org/CodeSystem/body-site"],
            "dosage_site_display": ["Oral"],
            "dosage_route": ["oral"],
            "dosage_route_code": ["PO"],
            "dosage_route_system": ["http://terminology.hl7.org/CodeSystem/route-codes"],
            "dosage_route_display": ["Oral"],
            "dosage_method": ["swallow"],
            "dosage_method_code": ["SWALLOW"],
            "dosage_method_system": ["http://terminology.hl7.org/CodeSystem/medication-admin-method"],
            "dosage_method_display": ["Swallow"],
            "dosage_dose_value": [500.0],
            "dosage_dose_unit": ["mg"],
            "dosage_dose_system": ["http://unitsofmeasure.org"],
            "dosage_dose_code": ["mg"],
            "dosage_rate_ratio_numerator_value": [500.0],
            "dosage_rate_ratio_numerator_unit": ["mg"],
            "dosage_rate_ratio_numerator_system": ["http://unitsofmeasure.org"],
            "dosage_rate_ratio_numerator_code": ["mg"],
            "dosage_rate_ratio_denominator_value": [6.0],
            "dosage_rate_ratio_denominator_unit": ["h"],
            "dosage_rate_ratio_denominator_system": ["http://unitsofmeasure.org"],
            "dosage_rate_ratio_denominator_code": ["h"],
            "dosage_rate_quantity_value": [500.0],
            "dosage_rate_quantity_unit": ["mg"],
            "dosage_rate_quantity_system": ["http://unitsofmeasure.org"],
            "dosage_rate_quantity_code": ["mg"],
            "dispense_initial_fill_quantity_value": 30.0,
            "dispense_initial_fill_quantity_unit": "tab",
            "dispense_initial_fill_quantity_system": "http://unitsofmeasure.org",
            "dispense_initial_fill_quantity_code": "tab",
            "dispense_initial_fill_duration_value": 7.0,
            "dispense_initial_fill_duration_unit": "d",
            "dispense_initial_fill_duration_system": "http://unitsofmeasure.org",
            "dispense_initial_fill_duration_code": "d",
            "dispense_interval_value": 1.0,
            "dispense_interval_unit": "d",
            "dispense_interval_system": "http://unitsofmeasure.org",
            "dispense_interval_code": "d",
            "dispense_validity_period_start": "2024-01-15T00:00:00Z",
            "dispense_validity_period_end": "2024-01-22T23:59:59Z",
            "dispense_number_of_repeats_allowed": 2,
            "dispense_quantity_value": 30.0,
            "dispense_quantity_unit": "tab",
            "dispense_quantity_system": "http://unitsofmeasure.org",
            "dispense_quantity_code": "tab",
            "dispense_expected_supply_duration_value": 7.0,
            "dispense_expected_supply_duration_unit": "d",
            "dispense_expected_supply_duration_system": "http://unitsofmeasure.org",
            "dispense_expected_supply_duration_code": "d",
            "dispense_dispenser_id": "pharmacy_001",
            "dispense_dispenser_type": "Organization",
            "dispense_dispenser_instruction": ["Dispense as written", "No substitutions"],
            "dispense_dose_administration_aid": "blister-pack",
            "dispense_dose_administration_aid_code": "BLISTER",
            "dispense_dose_administration_aid_system": "http://terminology.hl7.org/CodeSystem/medicationrequest-dose-administration-aid",
            "dispense_dose_administration_aid_display": "Blister Pack",
            "substitution_allowed": false,
            "substitution_reason": "patient-allergy",
            "substitution_reason_code": "PATALL",
            "substitution_reason_system": "http://terminology.hl7.org/CodeSystem/medicationrequest-substitution-reason",
            "substitution_reason_display": "Patient Allergy"
        }"#;

        let medication_request: DomainMedicationRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(medication_request.medication_request_id, "med_request_12345");
        assert_eq!(medication_request.status, "active");
        assert_eq!(medication_request.status_reason, Some("patient-request".to_string()));
        assert_eq!(medication_request.status_reason_code, Some("PATREQ".to_string()));
        assert_eq!(medication_request.status_reason_system, Some("http://terminology.hl7.org/CodeSystem/medicationrequest-status-reason".to_string()));
        assert_eq!(medication_request.status_reason_display, Some("Patient Request".to_string()));
        assert_eq!(medication_request.status_changed, Some("2024-01-15T10:00:00Z".to_string()));
        assert_eq!(medication_request.intent, "order");
        assert_eq!(medication_request.category, Some(vec!["inpatient".to_string()]));
        assert_eq!(medication_request.category_codes, Some(vec!["INPATIENT".to_string()]));
        assert_eq!(medication_request.category_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/medicationrequest-category".to_string()]));
        assert_eq!(medication_request.category_displays, Some(vec!["Inpatient".to_string()]));
        assert_eq!(medication_request.priority, Some("routine".to_string()));
        assert_eq!(medication_request.do_not_perform, Some(false));
        assert_eq!(medication_request.medication_code, Some("acetaminophen".to_string()));
        assert_eq!(medication_request.medication_code_system, Some("http://www.nlm.nih.gov/research/umls/rxnorm".to_string()));
        assert_eq!(medication_request.medication_code_display, Some("Acetaminophen".to_string()));
        assert_eq!(medication_request.medication_reference_id, Some("med_12345".to_string()));
        assert_eq!(medication_request.medication_reference_type, Some("Medication".to_string()));
        assert_eq!(medication_request.subject_id, "patient_001");
        assert_eq!(medication_request.subject_type, "Patient");
        assert_eq!(medication_request.encounter_id, Some("encounter_001".to_string()));
        assert_eq!(medication_request.encounter_type, Some("Encounter".to_string()));
        assert_eq!(medication_request.authored_on, Some("2024-01-15T09:00:00Z".to_string()));
        assert_eq!(medication_request.reported, Some(false));
        assert_eq!(medication_request.requester_id, Some("practitioner_001".to_string()));
        assert_eq!(medication_request.requester_type, Some("Practitioner".to_string()));
        assert_eq!(medication_request.performer_type, Some("nurse".to_string()));
        assert_eq!(medication_request.performer_type_code, Some("NURSE".to_string()));
        assert_eq!(medication_request.performer_type_system, Some("http://terminology.hl7.org/CodeSystem/medicationrequest-performer-type".to_string()));
        assert_eq!(medication_request.performer_type_display, Some("Nurse".to_string()));
        assert_eq!(medication_request.performer_ids, Some(vec!["practitioner_001".to_string(), "practitioner_002".to_string()]));
        assert_eq!(medication_request.performer_types, Some(vec!["Practitioner".to_string(), "Practitioner".to_string()]));
        assert_eq!(medication_request.device_code, Some(vec!["syringe".to_string()]));
        assert_eq!(medication_request.device_code_system, Some(vec!["http://terminology.hl7.org/CodeSystem/device-type".to_string()]));
        assert_eq!(medication_request.device_code_display, Some(vec!["Syringe".to_string()]));
        assert_eq!(medication_request.device_reference_id, Some(vec!["device_001".to_string()]));
        assert_eq!(medication_request.device_reference_type, Some(vec!["Device".to_string()]));
        assert_eq!(medication_request.recorder_id, Some("practitioner_001".to_string()));
        assert_eq!(medication_request.recorder_type, Some("Practitioner".to_string()));
        assert_eq!(medication_request.reason_code, Some(vec!["pain-management".to_string()]));
        assert_eq!(medication_request.reason_code_system, Some(vec!["http://terminology.hl7.org/CodeSystem/condition-code".to_string()]));
        assert_eq!(medication_request.reason_code_display, Some(vec!["Pain Management".to_string()]));
        assert_eq!(medication_request.reason_reference_id, Some(vec!["condition_001".to_string()]));
        assert_eq!(medication_request.reason_reference_type, Some(vec!["Condition".to_string()]));
        assert_eq!(medication_request.course_of_therapy_type, Some("acute".to_string()));
        assert_eq!(medication_request.course_of_therapy_type_code, Some("ACUTE".to_string()));
        assert_eq!(medication_request.course_of_therapy_type_system, Some("http://terminology.hl7.org/CodeSystem/medicationrequest-course-of-therapy".to_string()));
        assert_eq!(medication_request.course_of_therapy_type_display, Some("Acute".to_string()));
        assert_eq!(medication_request.insurance_ids, Some(vec!["coverage_001".to_string()]));
        assert_eq!(medication_request.insurance_types, Some(vec!["Coverage".to_string()]));
        assert_eq!(medication_request.note, Some(vec!["Patient requested pain medication".to_string(), "No known allergies".to_string()]));
        assert_eq!(medication_request.rendered_dosage_instruction, Some("Take 500mg by mouth every 6 hours as needed for pain".to_string()));
        assert_eq!(medication_request.effective_dose_period_start, Some("2024-01-15T00:00:00Z".to_string()));
        assert_eq!(medication_request.effective_dose_period_end, Some("2024-01-22T23:59:59Z".to_string()));
        assert_eq!(medication_request.dosage_text, Some(vec!["Take 500mg by mouth every 6 hours as needed for pain".to_string()]));
        assert_eq!(medication_request.dosage_site, Some(vec!["oral".to_string()]));
        assert_eq!(medication_request.dosage_site_code, Some(vec!["ORAL".to_string()]));
        assert_eq!(medication_request.dosage_site_system, Some(vec!["http://terminology.hl7.org/CodeSystem/body-site".to_string()]));
        assert_eq!(medication_request.dosage_site_display, Some(vec!["Oral".to_string()]));
        assert_eq!(medication_request.dosage_route, Some(vec!["oral".to_string()]));
        assert_eq!(medication_request.dosage_route_code, Some(vec!["PO".to_string()]));
        assert_eq!(medication_request.dosage_route_system, Some(vec!["http://terminology.hl7.org/CodeSystem/route-codes".to_string()]));
        assert_eq!(medication_request.dosage_route_display, Some(vec!["Oral".to_string()]));
        assert_eq!(medication_request.dosage_method, Some(vec!["swallow".to_string()]));
        assert_eq!(medication_request.dosage_method_code, Some(vec!["SWALLOW".to_string()]));
        assert_eq!(medication_request.dosage_method_system, Some(vec!["http://terminology.hl7.org/CodeSystem/medication-admin-method".to_string()]));
        assert_eq!(medication_request.dosage_method_display, Some(vec!["Swallow".to_string()]));
        assert_eq!(medication_request.dosage_dose_value, Some(vec![500.0]));
        assert_eq!(medication_request.dosage_dose_unit, Some(vec!["mg".to_string()]));
        assert_eq!(medication_request.dosage_dose_system, Some(vec!["http://unitsofmeasure.org".to_string()]));
        assert_eq!(medication_request.dosage_dose_code, Some(vec!["mg".to_string()]));
        assert_eq!(medication_request.dosage_rate_ratio_numerator_value, Some(vec![500.0]));
        assert_eq!(medication_request.dosage_rate_ratio_numerator_unit, Some(vec!["mg".to_string()]));
        assert_eq!(medication_request.dosage_rate_ratio_numerator_system, Some(vec!["http://unitsofmeasure.org".to_string()]));
        assert_eq!(medication_request.dosage_rate_ratio_numerator_code, Some(vec!["mg".to_string()]));
        assert_eq!(medication_request.dosage_rate_ratio_denominator_value, Some(vec![6.0]));
        assert_eq!(medication_request.dosage_rate_ratio_denominator_unit, Some(vec!["h".to_string()]));
        assert_eq!(medication_request.dosage_rate_ratio_denominator_system, Some(vec!["http://unitsofmeasure.org".to_string()]));
        assert_eq!(medication_request.dosage_rate_ratio_denominator_code, Some(vec!["h".to_string()]));
        assert_eq!(medication_request.dosage_rate_quantity_value, Some(vec![500.0]));
        assert_eq!(medication_request.dosage_rate_quantity_unit, Some(vec!["mg".to_string()]));
        assert_eq!(medication_request.dosage_rate_quantity_system, Some(vec!["http://unitsofmeasure.org".to_string()]));
        assert_eq!(medication_request.dosage_rate_quantity_code, Some(vec!["mg".to_string()]));
        assert_eq!(medication_request.dispense_initial_fill_quantity_value, Some(30.0));
        assert_eq!(medication_request.dispense_initial_fill_quantity_unit, Some("tab".to_string()));
        assert_eq!(medication_request.dispense_initial_fill_quantity_system, Some("http://unitsofmeasure.org".to_string()));
        assert_eq!(medication_request.dispense_initial_fill_quantity_code, Some("tab".to_string()));
        assert_eq!(medication_request.dispense_initial_fill_duration_value, Some(7.0));
        assert_eq!(medication_request.dispense_initial_fill_duration_unit, Some("d".to_string()));
        assert_eq!(medication_request.dispense_initial_fill_duration_system, Some("http://unitsofmeasure.org".to_string()));
        assert_eq!(medication_request.dispense_initial_fill_duration_code, Some("d".to_string()));
        assert_eq!(medication_request.dispense_interval_value, Some(1.0));
        assert_eq!(medication_request.dispense_interval_unit, Some("d".to_string()));
        assert_eq!(medication_request.dispense_interval_system, Some("http://unitsofmeasure.org".to_string()));
        assert_eq!(medication_request.dispense_interval_code, Some("d".to_string()));
        assert_eq!(medication_request.dispense_validity_period_start, Some("2024-01-15T00:00:00Z".to_string()));
        assert_eq!(medication_request.dispense_validity_period_end, Some("2024-01-22T23:59:59Z".to_string()));
        assert_eq!(medication_request.dispense_number_of_repeats_allowed, Some(2));
        assert_eq!(medication_request.dispense_quantity_value, Some(30.0));
        assert_eq!(medication_request.dispense_quantity_unit, Some("tab".to_string()));
        assert_eq!(medication_request.dispense_quantity_system, Some("http://unitsofmeasure.org".to_string()));
        assert_eq!(medication_request.dispense_quantity_code, Some("tab".to_string()));
        assert_eq!(medication_request.dispense_expected_supply_duration_value, Some(7.0));
        assert_eq!(medication_request.dispense_expected_supply_duration_unit, Some("d".to_string()));
        assert_eq!(medication_request.dispense_expected_supply_duration_system, Some("http://unitsofmeasure.org".to_string()));
        assert_eq!(medication_request.dispense_expected_supply_duration_code, Some("d".to_string()));
        assert_eq!(medication_request.dispense_dispenser_id, Some("pharmacy_001".to_string()));
        assert_eq!(medication_request.dispense_dispenser_type, Some("Organization".to_string()));
        assert_eq!(medication_request.dispense_dispenser_instruction, Some(vec!["Dispense as written".to_string(), "No substitutions".to_string()]));
        assert_eq!(medication_request.dispense_dose_administration_aid, Some("blister-pack".to_string()));
        assert_eq!(medication_request.dispense_dose_administration_aid_code, Some("BLISTER".to_string()));
        assert_eq!(medication_request.dispense_dose_administration_aid_system, Some("http://terminology.hl7.org/CodeSystem/medicationrequest-dose-administration-aid".to_string()));
        assert_eq!(medication_request.dispense_dose_administration_aid_display, Some("Blister Pack".to_string()));
        assert_eq!(medication_request.substitution_allowed, Some(false));
        assert_eq!(medication_request.substitution_reason, Some("patient-allergy".to_string()));
        assert_eq!(medication_request.substitution_reason_code, Some("PATALL".to_string()));
        assert_eq!(medication_request.substitution_reason_system, Some("http://terminology.hl7.org/CodeSystem/medicationrequest-substitution-reason".to_string()));
        assert_eq!(medication_request.substitution_reason_display, Some("Patient Allergy".to_string()));
    }

    #[test]
    fn test_domain_medication_request_minimal_deserialization() {
        let json = r#"{
            "medication_request_id": "med_request_67890",
            "status": "active",
            "intent": "order",
            "subject_id": "patient_002",
            "subject_type": "Patient"
        }"#;

        let medication_request: DomainMedicationRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(medication_request.medication_request_id, "med_request_67890");
        assert_eq!(medication_request.status, "active");
        assert_eq!(medication_request.intent, "order");
        assert_eq!(medication_request.subject_id, "patient_002");
        assert_eq!(medication_request.subject_type, "Patient");
        assert_eq!(medication_request.status_reason, None);
        assert_eq!(medication_request.status_reason_code, None);
        assert_eq!(medication_request.status_reason_system, None);
        assert_eq!(medication_request.status_reason_display, None);
        assert_eq!(medication_request.status_changed, None);
        assert_eq!(medication_request.category, None);
        assert_eq!(medication_request.category_codes, None);
        assert_eq!(medication_request.category_systems, None);
        assert_eq!(medication_request.category_displays, None);
        assert_eq!(medication_request.priority, None);
        assert_eq!(medication_request.do_not_perform, None);
        assert_eq!(medication_request.medication_code, None);
        assert_eq!(medication_request.medication_code_system, None);
        assert_eq!(medication_request.medication_code_display, None);
        assert_eq!(medication_request.medication_reference_id, None);
        assert_eq!(medication_request.medication_reference_type, None);
        assert_eq!(medication_request.encounter_id, None);
        assert_eq!(medication_request.encounter_type, None);
        assert_eq!(medication_request.authored_on, None);
        assert_eq!(medication_request.reported, None);
        assert_eq!(medication_request.requester_id, None);
        assert_eq!(medication_request.requester_type, None);
        assert_eq!(medication_request.performer_type, None);
        assert_eq!(medication_request.performer_type_code, None);
        assert_eq!(medication_request.performer_type_system, None);
        assert_eq!(medication_request.performer_type_display, None);
        assert_eq!(medication_request.performer_ids, None);
        assert_eq!(medication_request.performer_types, None);
        assert_eq!(medication_request.device_code, None);
        assert_eq!(medication_request.device_code_system, None);
        assert_eq!(medication_request.device_code_display, None);
        assert_eq!(medication_request.device_reference_id, None);
        assert_eq!(medication_request.device_reference_type, None);
        assert_eq!(medication_request.recorder_id, None);
        assert_eq!(medication_request.recorder_type, None);
        assert_eq!(medication_request.reason_code, None);
        assert_eq!(medication_request.reason_code_system, None);
        assert_eq!(medication_request.reason_code_display, None);
        assert_eq!(medication_request.reason_reference_id, None);
        assert_eq!(medication_request.reason_reference_type, None);
        assert_eq!(medication_request.course_of_therapy_type, None);
        assert_eq!(medication_request.course_of_therapy_type_code, None);
        assert_eq!(medication_request.course_of_therapy_type_system, None);
        assert_eq!(medication_request.course_of_therapy_type_display, None);
        assert_eq!(medication_request.insurance_ids, None);
        assert_eq!(medication_request.insurance_types, None);
        assert_eq!(medication_request.note, None);
        assert_eq!(medication_request.rendered_dosage_instruction, None);
        assert_eq!(medication_request.effective_dose_period_start, None);
        assert_eq!(medication_request.effective_dose_period_end, None);
        assert_eq!(medication_request.dosage_text, None);
        assert_eq!(medication_request.dosage_site, None);
        assert_eq!(medication_request.dosage_site_code, None);
        assert_eq!(medication_request.dosage_site_system, None);
        assert_eq!(medication_request.dosage_site_display, None);
        assert_eq!(medication_request.dosage_route, None);
        assert_eq!(medication_request.dosage_route_code, None);
        assert_eq!(medication_request.dosage_route_system, None);
        assert_eq!(medication_request.dosage_route_display, None);
        assert_eq!(medication_request.dosage_method, None);
        assert_eq!(medication_request.dosage_method_code, None);
        assert_eq!(medication_request.dosage_method_system, None);
        assert_eq!(medication_request.dosage_method_display, None);
        assert_eq!(medication_request.dosage_dose_value, None);
        assert_eq!(medication_request.dosage_dose_unit, None);
        assert_eq!(medication_request.dosage_dose_system, None);
        assert_eq!(medication_request.dosage_dose_code, None);
        assert_eq!(medication_request.dosage_rate_ratio_numerator_value, None);
        assert_eq!(medication_request.dosage_rate_ratio_numerator_unit, None);
        assert_eq!(medication_request.dosage_rate_ratio_numerator_system, None);
        assert_eq!(medication_request.dosage_rate_ratio_numerator_code, None);
        assert_eq!(medication_request.dosage_rate_ratio_denominator_value, None);
        assert_eq!(medication_request.dosage_rate_ratio_denominator_unit, None);
        assert_eq!(medication_request.dosage_rate_ratio_denominator_system, None);
        assert_eq!(medication_request.dosage_rate_ratio_denominator_code, None);
        assert_eq!(medication_request.dosage_rate_quantity_value, None);
        assert_eq!(medication_request.dosage_rate_quantity_unit, None);
        assert_eq!(medication_request.dosage_rate_quantity_system, None);
        assert_eq!(medication_request.dosage_rate_quantity_code, None);
        assert_eq!(medication_request.dispense_initial_fill_quantity_value, None);
        assert_eq!(medication_request.dispense_initial_fill_quantity_unit, None);
        assert_eq!(medication_request.dispense_initial_fill_quantity_system, None);
        assert_eq!(medication_request.dispense_initial_fill_quantity_code, None);
        assert_eq!(medication_request.dispense_initial_fill_duration_value, None);
        assert_eq!(medication_request.dispense_initial_fill_duration_unit, None);
        assert_eq!(medication_request.dispense_initial_fill_duration_system, None);
        assert_eq!(medication_request.dispense_initial_fill_duration_code, None);
        assert_eq!(medication_request.dispense_interval_value, None);
        assert_eq!(medication_request.dispense_interval_unit, None);
        assert_eq!(medication_request.dispense_interval_system, None);
        assert_eq!(medication_request.dispense_interval_code, None);
        assert_eq!(medication_request.dispense_validity_period_start, None);
        assert_eq!(medication_request.dispense_validity_period_end, None);
        assert_eq!(medication_request.dispense_number_of_repeats_allowed, None);
        assert_eq!(medication_request.dispense_quantity_value, None);
        assert_eq!(medication_request.dispense_quantity_unit, None);
        assert_eq!(medication_request.dispense_quantity_system, None);
        assert_eq!(medication_request.dispense_quantity_code, None);
        assert_eq!(medication_request.dispense_expected_supply_duration_value, None);
        assert_eq!(medication_request.dispense_expected_supply_duration_unit, None);
        assert_eq!(medication_request.dispense_expected_supply_duration_system, None);
        assert_eq!(medication_request.dispense_expected_supply_duration_code, None);
        assert_eq!(medication_request.dispense_dispenser_id, None);
        assert_eq!(medication_request.dispense_dispenser_type, None);
        assert_eq!(medication_request.dispense_dispenser_instruction, None);
        assert_eq!(medication_request.dispense_dose_administration_aid, None);
        assert_eq!(medication_request.dispense_dose_administration_aid_code, None);
        assert_eq!(medication_request.dispense_dose_administration_aid_system, None);
        assert_eq!(medication_request.dispense_dose_administration_aid_display, None);
        assert_eq!(medication_request.substitution_allowed, None);
        assert_eq!(medication_request.substitution_reason, None);
        assert_eq!(medication_request.substitution_reason_code, None);
        assert_eq!(medication_request.substitution_reason_system, None);
        assert_eq!(medication_request.substitution_reason_display, None);
    }

    #[test]
    fn test_domain_medication_request_missing_required_field() {
        let json = r#"{
            "status": "active"
        }"#;

        // This should fail because medication_request_id is required
        let result: Result<DomainMedicationRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
