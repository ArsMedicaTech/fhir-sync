use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainFamilyMemberHistory {
    pub family_member_history_id: String,
    pub patient_demographic_no: String,
    
    // Basic family member history information
    pub status: Option<String>, // "partial" | "completed" | "entered-in-error" | "health-unknown"
    pub data_absent_reason: Option<String>, // "subject-unknown" | "withheld" | "unable-to-obtain" | "deferred"
    pub data_absent_reason_code: Option<String>, // Code for data absent reason
    pub data_absent_reason_system: Option<String>, // Terminology system for data absent reason
    pub data_absent_reason_display: Option<String>, // Display name for data absent reason
    pub date_recorded: Option<String>, // ISO datetime string when history was recorded or last updated
    
    // Family member information
    pub name: Option<String>, // The family member described
    pub relationship: Option<String>, // Relationship to the subject
    pub relationship_code: Option<String>, // Code for relationship
    pub relationship_system: Option<String>, // Terminology system for relationship
    pub relationship_display: Option<String>, // Display name for relationship
    pub sex: Option<String>, // "male" | "female" | "other" | "unknown"
    pub sex_code: Option<String>, // Code for sex
    pub sex_system: Option<String>, // Terminology system for sex
    pub sex_display: Option<String>, // Display name for sex
    
    // Birth and age information
    pub born_date: Option<String>, // ISO date string for birth date
    pub born_period_start: Option<String>, // ISO date string for birth period start
    pub born_period_end: Option<String>, // ISO date string for birth period end
    pub born_string: Option<String>, // String description of birth
    pub age_value: Option<f64>, // Age value
    pub age_unit: Option<String>, // Age unit (years, months, days)
    pub age_range_low: Option<f64>, // Age range low value
    pub age_range_high: Option<f64>, // Age range high value
    pub age_range_unit: Option<String>, // Age range unit
    pub age_string: Option<String>, // String description of age
    pub estimated_age: Option<bool>, // Age is estimated?
    
    // Death information
    pub deceased: Option<bool>, // Dead?
    pub deceased_age_value: Option<f64>, // Age at death
    pub deceased_age_unit: Option<String>, // Age at death unit
    pub deceased_age_range_low: Option<f64>, // Age at death range low
    pub deceased_age_range_high: Option<f64>, // Age at death range high
    pub deceased_age_range_unit: Option<String>, // Age at death range unit
    pub deceased_date: Option<String>, // ISO date string for death date
    pub deceased_string: Option<String>, // String description of death
    
    // Reason for family member history
    pub reason_codes: Option<Vec<String>>, // Why was family member history performed?
    pub reason_code_codes: Option<Vec<String>>, // Codes for reasons
    pub reason_code_systems: Option<Vec<String>>, // Terminology systems for reasons
    pub reason_code_displays: Option<Vec<String>>, // Display names for reasons
    pub reason_reference_ids: Option<Vec<String>>, // References for reasons
    pub reason_reference_types: Option<Vec<String>>, // Types of reason references
    
    // Participants
    pub participant_function_codes: Option<Vec<String>>, // Type of involvement
    pub participant_function_code_codes: Option<Vec<String>>, // Codes for participant functions
    pub participant_function_code_systems: Option<Vec<String>>, // Terminology systems for functions
    pub participant_function_code_displays: Option<Vec<String>>, // Display names for functions
    pub participant_actor_ids: Option<Vec<String>>, // Who or what participated
    pub participant_actor_types: Option<Vec<String>>, // Types of participants
    
    // Conditions
    pub condition_codes: Option<Vec<String>>, // Condition suffered by relation
    pub condition_code_codes: Option<Vec<String>>, // Codes for conditions
    pub condition_code_systems: Option<Vec<String>>, // Terminology systems for conditions
    pub condition_code_displays: Option<Vec<String>>, // Display names for conditions
    pub condition_outcomes: Option<Vec<String>>, // deceased | permanent disability | etc
    pub condition_outcome_codes: Option<Vec<String>>, // Codes for outcomes
    pub condition_outcome_systems: Option<Vec<String>>, // Terminology systems for outcomes
    pub condition_outcome_displays: Option<Vec<String>>, // Display names for outcomes
    pub condition_contributed_to_death: Option<Vec<bool>>, // Whether the condition contributed to the cause of death
    pub condition_onset_ages: Option<Vec<f64>>, // When condition first manifested (age)
    pub condition_onset_age_units: Option<Vec<String>>, // Age units for onset
    pub condition_onset_age_ranges_low: Option<Vec<f64>>, // Onset age range low
    pub condition_onset_age_ranges_high: Option<Vec<f64>>, // Onset age range high
    pub condition_onset_age_range_units: Option<Vec<String>>, // Onset age range units
    pub condition_onset_periods_start: Option<Vec<String>>, // Onset period start
    pub condition_onset_periods_end: Option<Vec<String>>, // Onset period end
    pub condition_onset_strings: Option<Vec<String>>, // String descriptions of onset
    pub condition_notes: Option<Vec<String>>, // Extra information about condition
    
    // Procedures
    pub procedure_codes: Option<Vec<String>>, // Procedures performed on the related person
    pub procedure_code_codes: Option<Vec<String>>, // Codes for procedures
    pub procedure_code_systems: Option<Vec<String>>, // Terminology systems for procedures
    pub procedure_code_displays: Option<Vec<String>>, // Display names for procedures
    pub procedure_outcomes: Option<Vec<String>>, // What happened following the procedure
    pub procedure_outcome_codes: Option<Vec<String>>, // Codes for procedure outcomes
    pub procedure_outcome_systems: Option<Vec<String>>, // Terminology systems for outcomes
    pub procedure_outcome_displays: Option<Vec<String>>, // Display names for outcomes
    pub procedure_contributed_to_death: Option<Vec<bool>>, // Whether the procedure contributed to the cause of death
    pub procedure_performed_ages: Option<Vec<f64>>, // When the procedure was performed (age)
    pub procedure_performed_age_units: Option<Vec<String>>, // Age units for performed
    pub procedure_performed_age_ranges_low: Option<Vec<f64>>, // Performed age range low
    pub procedure_performed_age_ranges_high: Option<Vec<f64>>, // Performed age range high
    pub procedure_performed_age_range_units: Option<Vec<String>>, // Performed age range units
    pub procedure_performed_periods_start: Option<Vec<String>>, // Performed period start
    pub procedure_performed_periods_end: Option<Vec<String>>, // Performed period end
    pub procedure_performed_dates: Option<Vec<String>>, // ISO datetime string for performed
    pub procedure_performed_strings: Option<Vec<String>>, // String descriptions of performed
    pub procedure_notes: Option<Vec<String>>, // Extra information about the procedure
    
    // Additional information
    pub notes: Option<String>, // General note about related person
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_family_member_history_deserialization() {
        let json = r#"{
            "family_member_history_id": "fmh_12345",
            "patient_demographic_no": "12345",
            "status": "completed",
            "data_absent_reason": "subject-unknown",
            "data_absent_reason_code": "subject-unknown",
            "data_absent_reason_system": "http://terminology.hl7.org/CodeSystem/data-absent-reason",
            "data_absent_reason_display": "Subject Unknown",
            "date_recorded": "2024-01-15T10:30:00Z",
            "name": "John Smith",
            "relationship": "father",
            "relationship_code": "father",
            "relationship_system": "http://terminology.hl7.org/CodeSystem/v3-RoleCode",
            "relationship_display": "Father",
            "sex": "male",
            "sex_code": "M",
            "sex_system": "http://hl7.org/fhir/administrative-gender",
            "sex_display": "Male",
            "born_date": "1950-05-15",
            "age_value": 73.5,
            "age_unit": "years",
            "estimated_age": false,
            "deceased": true,
            "deceased_age_value": 72.0,
            "deceased_age_unit": "years",
            "deceased_date": "2022-12-01",
            "reason_codes": ["genetic-counseling", "risk-assessment"],
            "reason_code_codes": ["genetic-counseling", "risk-assessment"],
            "reason_code_systems": ["http://terminology.hl7.org/CodeSystem/v3-ActReason", "http://terminology.hl7.org/CodeSystem/v3-ActReason"],
            "reason_code_displays": ["Genetic Counseling", "Risk Assessment"],
            "participant_function_codes": ["informant"],
            "participant_function_code_codes": ["informant"],
            "participant_function_code_systems": ["http://terminology.hl7.org/CodeSystem/v3-ParticipationType"],
            "participant_function_code_displays": ["Informant"],
            "participant_actor_ids": ["pat_12345"],
            "participant_actor_types": ["Patient"],
            "condition_codes": ["diabetes", "hypertension", "heart-disease"],
            "condition_code_codes": ["E11.9", "I10", "I25.9"],
            "condition_code_systems": ["http://hl7.org/fhir/sid/icd-10-cm", "http://hl7.org/fhir/sid/icd-10-cm", "http://hl7.org/fhir/sid/icd-10-cm"],
            "condition_code_displays": ["Type 2 diabetes mellitus without complications", "Essential hypertension", "Chronic ischemic heart disease, unspecified"],
            "condition_outcomes": ["deceased", "permanent-disability", "deceased"],
            "condition_outcome_codes": ["deceased", "permanent-disability", "deceased"],
            "condition_outcome_systems": ["http://terminology.hl7.org/CodeSystem/condition-outcome", "http://terminology.hl7.org/CodeSystem/condition-outcome", "http://terminology.hl7.org/CodeSystem/condition-outcome"],
            "condition_outcome_displays": ["Deceased", "Permanent Disability", "Deceased"],
            "condition_contributed_to_death": [true, false, true],
            "condition_onset_ages": [45.0, 50.0, 60.0],
            "condition_onset_age_units": ["years", "years", "years"],
            "condition_onset_strings": ["Mid-40s", "Early 50s", "Early 60s"],
            "condition_notes": ["Well-controlled with medication", "Required daily medication", "Led to heart attack"],
            "procedure_codes": ["coronary-bypass", "angioplasty"],
            "procedure_code_codes": ["02100Z0", "02703ZZ"],
            "procedure_code_systems": ["http://www.ama-assn.org/go/cpt", "http://www.ama-assn.org/go/cpt"],
            "procedure_code_displays": ["Bypass Coronary Artery, One Artery from Coronary Artery", "Dilation of Coronary Artery, Three Arteries, Percutaneous Approach"],
            "procedure_outcomes": ["successful", "successful"],
            "procedure_outcome_codes": ["successful", "successful"],
            "procedure_outcome_systems": ["http://terminology.hl7.org/CodeSystem/procedure-outcome", "http://terminology.hl7.org/CodeSystem/procedure-outcome"],
            "procedure_outcome_displays": ["Successful", "Successful"],
            "procedure_contributed_to_death": [false, false],
            "procedure_performed_ages": [65.0, 68.0],
            "procedure_performed_age_units": ["years", "years"],
            "procedure_performed_dates": ["2015-03-15T08:00:00Z", "2018-07-20T10:30:00Z"],
            "procedure_notes": ["Triple bypass surgery", "Stent placement"],
            "notes": "Father had significant cardiovascular history. Died at age 72 from complications of diabetes and heart disease."
        }"#;

        let family_member_history: DomainFamilyMemberHistory = serde_json::from_str(json).unwrap();
        
        assert_eq!(family_member_history.family_member_history_id, "fmh_12345");
        assert_eq!(family_member_history.patient_demographic_no, "12345");
        assert_eq!(family_member_history.status, Some("completed".to_string()));
        assert_eq!(family_member_history.data_absent_reason, Some("subject-unknown".to_string()));
        assert_eq!(family_member_history.data_absent_reason_code, Some("subject-unknown".to_string()));
        assert_eq!(family_member_history.data_absent_reason_system, Some("http://terminology.hl7.org/CodeSystem/data-absent-reason".to_string()));
        assert_eq!(family_member_history.data_absent_reason_display, Some("Subject Unknown".to_string()));
        assert_eq!(family_member_history.date_recorded, Some("2024-01-15T10:30:00Z".to_string()));
        assert_eq!(family_member_history.name, Some("John Smith".to_string()));
        assert_eq!(family_member_history.relationship, Some("father".to_string()));
        assert_eq!(family_member_history.relationship_code, Some("father".to_string()));
        assert_eq!(family_member_history.relationship_system, Some("http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string()));
        assert_eq!(family_member_history.relationship_display, Some("Father".to_string()));
        assert_eq!(family_member_history.sex, Some("male".to_string()));
        assert_eq!(family_member_history.sex_code, Some("M".to_string()));
        assert_eq!(family_member_history.sex_system, Some("http://hl7.org/fhir/administrative-gender".to_string()));
        assert_eq!(family_member_history.sex_display, Some("Male".to_string()));
        assert_eq!(family_member_history.born_date, Some("1950-05-15".to_string()));
        assert_eq!(family_member_history.age_value, Some(73.5));
        assert_eq!(family_member_history.age_unit, Some("years".to_string()));
        assert_eq!(family_member_history.estimated_age, Some(false));
        assert_eq!(family_member_history.deceased, Some(true));
        assert_eq!(family_member_history.deceased_age_value, Some(72.0));
        assert_eq!(family_member_history.deceased_age_unit, Some("years".to_string()));
        assert_eq!(family_member_history.deceased_date, Some("2022-12-01".to_string()));
        assert_eq!(family_member_history.reason_codes, Some(vec!["genetic-counseling".to_string(), "risk-assessment".to_string()]));
        assert_eq!(family_member_history.reason_code_codes, Some(vec!["genetic-counseling".to_string(), "risk-assessment".to_string()]));
        assert_eq!(family_member_history.reason_code_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string(), "http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string()]));
        assert_eq!(family_member_history.reason_code_displays, Some(vec!["Genetic Counseling".to_string(), "Risk Assessment".to_string()]));
        assert_eq!(family_member_history.participant_function_codes, Some(vec!["informant".to_string()]));
        assert_eq!(family_member_history.participant_function_code_codes, Some(vec!["informant".to_string()]));
        assert_eq!(family_member_history.participant_function_code_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ParticipationType".to_string()]));
        assert_eq!(family_member_history.participant_function_code_displays, Some(vec!["Informant".to_string()]));
        assert_eq!(family_member_history.participant_actor_ids, Some(vec!["pat_12345".to_string()]));
        assert_eq!(family_member_history.participant_actor_types, Some(vec!["Patient".to_string()]));
        assert_eq!(family_member_history.condition_codes, Some(vec!["diabetes".to_string(), "hypertension".to_string(), "heart-disease".to_string()]));
        assert_eq!(family_member_history.condition_code_codes, Some(vec!["E11.9".to_string(), "I10".to_string(), "I25.9".to_string()]));
        assert_eq!(family_member_history.condition_code_systems, Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]));
        assert_eq!(family_member_history.condition_code_displays, Some(vec!["Type 2 diabetes mellitus without complications".to_string(), "Essential hypertension".to_string(), "Chronic ischemic heart disease, unspecified".to_string()]));
        assert_eq!(family_member_history.condition_outcomes, Some(vec!["deceased".to_string(), "permanent-disability".to_string(), "deceased".to_string()]));
        assert_eq!(family_member_history.condition_outcome_codes, Some(vec!["deceased".to_string(), "permanent-disability".to_string(), "deceased".to_string()]));
        assert_eq!(family_member_history.condition_outcome_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/condition-outcome".to_string(), "http://terminology.hl7.org/CodeSystem/condition-outcome".to_string(), "http://terminology.hl7.org/CodeSystem/condition-outcome".to_string()]));
        assert_eq!(family_member_history.condition_outcome_displays, Some(vec!["Deceased".to_string(), "Permanent Disability".to_string(), "Deceased".to_string()]));
        assert_eq!(family_member_history.condition_contributed_to_death, Some(vec![true, false, true]));
        assert_eq!(family_member_history.condition_onset_ages, Some(vec![45.0, 50.0, 60.0]));
        assert_eq!(family_member_history.condition_onset_age_units, Some(vec!["years".to_string(), "years".to_string(), "years".to_string()]));
        assert_eq!(family_member_history.condition_onset_strings, Some(vec!["Mid-40s".to_string(), "Early 50s".to_string(), "Early 60s".to_string()]));
        assert_eq!(family_member_history.condition_notes, Some(vec!["Well-controlled with medication".to_string(), "Required daily medication".to_string(), "Led to heart attack".to_string()]));
        assert_eq!(family_member_history.procedure_codes, Some(vec!["coronary-bypass".to_string(), "angioplasty".to_string()]));
        assert_eq!(family_member_history.procedure_code_codes, Some(vec!["02100Z0".to_string(), "02703ZZ".to_string()]));
        assert_eq!(family_member_history.procedure_code_systems, Some(vec!["http://www.ama-assn.org/go/cpt".to_string(), "http://www.ama-assn.org/go/cpt".to_string()]));
        assert_eq!(family_member_history.procedure_code_displays, Some(vec!["Bypass Coronary Artery, One Artery from Coronary Artery".to_string(), "Dilation of Coronary Artery, Three Arteries, Percutaneous Approach".to_string()]));
        assert_eq!(family_member_history.procedure_outcomes, Some(vec!["successful".to_string(), "successful".to_string()]));
        assert_eq!(family_member_history.procedure_outcome_codes, Some(vec!["successful".to_string(), "successful".to_string()]));
        assert_eq!(family_member_history.procedure_outcome_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/procedure-outcome".to_string(), "http://terminology.hl7.org/CodeSystem/procedure-outcome".to_string()]));
        assert_eq!(family_member_history.procedure_outcome_displays, Some(vec!["Successful".to_string(), "Successful".to_string()]));
        assert_eq!(family_member_history.procedure_contributed_to_death, Some(vec![false, false]));
        assert_eq!(family_member_history.procedure_performed_ages, Some(vec![65.0, 68.0]));
        assert_eq!(family_member_history.procedure_performed_age_units, Some(vec!["years".to_string(), "years".to_string()]));
        assert_eq!(family_member_history.procedure_performed_dates, Some(vec!["2015-03-15T08:00:00Z".to_string(), "2018-07-20T10:30:00Z".to_string()]));
        assert_eq!(family_member_history.procedure_notes, Some(vec!["Triple bypass surgery".to_string(), "Stent placement".to_string()]));
        assert_eq!(family_member_history.notes, Some("Father had significant cardiovascular history. Died at age 72 from complications of diabetes and heart disease.".to_string()));
    }

    #[test]
    fn test_domain_family_member_history_minimal_deserialization() {
        let json = r#"{
            "family_member_history_id": "fmh_67890",
            "patient_demographic_no": "67890"
        }"#;

        let family_member_history: DomainFamilyMemberHistory = serde_json::from_str(json).unwrap();
        
        assert_eq!(family_member_history.family_member_history_id, "fmh_67890");
        assert_eq!(family_member_history.patient_demographic_no, "67890");
        assert_eq!(family_member_history.status, None);
        assert_eq!(family_member_history.data_absent_reason, None);
        assert_eq!(family_member_history.data_absent_reason_code, None);
        assert_eq!(family_member_history.data_absent_reason_system, None);
        assert_eq!(family_member_history.data_absent_reason_display, None);
        assert_eq!(family_member_history.date_recorded, None);
        assert_eq!(family_member_history.name, None);
        assert_eq!(family_member_history.relationship, None);
        assert_eq!(family_member_history.relationship_code, None);
        assert_eq!(family_member_history.relationship_system, None);
        assert_eq!(family_member_history.relationship_display, None);
        assert_eq!(family_member_history.sex, None);
        assert_eq!(family_member_history.sex_code, None);
        assert_eq!(family_member_history.sex_system, None);
        assert_eq!(family_member_history.sex_display, None);
        assert_eq!(family_member_history.born_date, None);
        assert_eq!(family_member_history.born_period_start, None);
        assert_eq!(family_member_history.born_period_end, None);
        assert_eq!(family_member_history.born_string, None);
        assert_eq!(family_member_history.age_value, None);
        assert_eq!(family_member_history.age_unit, None);
        assert_eq!(family_member_history.age_range_low, None);
        assert_eq!(family_member_history.age_range_high, None);
        assert_eq!(family_member_history.age_range_unit, None);
        assert_eq!(family_member_history.age_string, None);
        assert_eq!(family_member_history.estimated_age, None);
        assert_eq!(family_member_history.deceased, None);
        assert_eq!(family_member_history.deceased_age_value, None);
        assert_eq!(family_member_history.deceased_age_unit, None);
        assert_eq!(family_member_history.deceased_age_range_low, None);
        assert_eq!(family_member_history.deceased_age_range_high, None);
        assert_eq!(family_member_history.deceased_age_range_unit, None);
        assert_eq!(family_member_history.deceased_date, None);
        assert_eq!(family_member_history.deceased_string, None);
        assert_eq!(family_member_history.reason_codes, None);
        assert_eq!(family_member_history.reason_code_codes, None);
        assert_eq!(family_member_history.reason_code_systems, None);
        assert_eq!(family_member_history.reason_code_displays, None);
        assert_eq!(family_member_history.reason_reference_ids, None);
        assert_eq!(family_member_history.reason_reference_types, None);
        assert_eq!(family_member_history.participant_function_codes, None);
        assert_eq!(family_member_history.participant_function_code_codes, None);
        assert_eq!(family_member_history.participant_function_code_systems, None);
        assert_eq!(family_member_history.participant_function_code_displays, None);
        assert_eq!(family_member_history.participant_actor_ids, None);
        assert_eq!(family_member_history.participant_actor_types, None);
        assert_eq!(family_member_history.condition_codes, None);
        assert_eq!(family_member_history.condition_code_codes, None);
        assert_eq!(family_member_history.condition_code_systems, None);
        assert_eq!(family_member_history.condition_code_displays, None);
        assert_eq!(family_member_history.condition_outcomes, None);
        assert_eq!(family_member_history.condition_outcome_codes, None);
        assert_eq!(family_member_history.condition_outcome_systems, None);
        assert_eq!(family_member_history.condition_outcome_displays, None);
        assert_eq!(family_member_history.condition_contributed_to_death, None);
        assert_eq!(family_member_history.condition_onset_ages, None);
        assert_eq!(family_member_history.condition_onset_age_units, None);
        assert_eq!(family_member_history.condition_onset_age_ranges_low, None);
        assert_eq!(family_member_history.condition_onset_age_ranges_high, None);
        assert_eq!(family_member_history.condition_onset_age_range_units, None);
        assert_eq!(family_member_history.condition_onset_periods_start, None);
        assert_eq!(family_member_history.condition_onset_periods_end, None);
        assert_eq!(family_member_history.condition_onset_strings, None);
        assert_eq!(family_member_history.condition_notes, None);
        assert_eq!(family_member_history.procedure_codes, None);
        assert_eq!(family_member_history.procedure_code_codes, None);
        assert_eq!(family_member_history.procedure_code_systems, None);
        assert_eq!(family_member_history.procedure_code_displays, None);
        assert_eq!(family_member_history.procedure_outcomes, None);
        assert_eq!(family_member_history.procedure_outcome_codes, None);
        assert_eq!(family_member_history.procedure_outcome_systems, None);
        assert_eq!(family_member_history.procedure_outcome_displays, None);
        assert_eq!(family_member_history.procedure_contributed_to_death, None);
        assert_eq!(family_member_history.procedure_performed_ages, None);
        assert_eq!(family_member_history.procedure_performed_age_units, None);
        assert_eq!(family_member_history.procedure_performed_age_ranges_low, None);
        assert_eq!(family_member_history.procedure_performed_age_ranges_high, None);
        assert_eq!(family_member_history.procedure_performed_age_range_units, None);
        assert_eq!(family_member_history.procedure_performed_periods_start, None);
        assert_eq!(family_member_history.procedure_performed_periods_end, None);
        assert_eq!(family_member_history.procedure_performed_dates, None);
        assert_eq!(family_member_history.procedure_performed_strings, None);
        assert_eq!(family_member_history.procedure_notes, None);
        assert_eq!(family_member_history.notes, None);
    }

    #[test]
    fn test_domain_family_member_history_mother() {
        let json = r#"{
            "family_member_history_id": "fmh_mother_001",
            "patient_demographic_no": "12345",
            "status": "completed",
            "date_recorded": "2024-01-15T10:30:00Z",
            "name": "Mary Smith",
            "relationship": "mother",
            "relationship_code": "mother",
            "relationship_system": "http://terminology.hl7.org/CodeSystem/v3-RoleCode",
            "relationship_display": "Mother",
            "sex": "female",
            "sex_code": "F",
            "sex_system": "http://hl7.org/fhir/administrative-gender",
            "sex_display": "Female",
            "born_date": "1955-08-20",
            "age_value": 68.5,
            "age_unit": "years",
            "estimated_age": false,
            "deceased": false,
            "reason_codes": ["genetic-counseling"],
            "reason_code_codes": ["genetic-counseling"],
            "reason_code_systems": ["http://terminology.hl7.org/CodeSystem/v3-ActReason"],
            "reason_code_displays": ["Genetic Counseling"],
            "participant_function_codes": ["informant"],
            "participant_function_code_codes": ["informant"],
            "participant_function_code_systems": ["http://terminology.hl7.org/CodeSystem/v3-ParticipationType"],
            "participant_function_code_displays": ["Informant"],
            "participant_actor_ids": ["pat_12345"],
            "participant_actor_types": ["Patient"],
            "condition_codes": ["breast-cancer", "osteoporosis"],
            "condition_code_codes": ["C50.9", "M81.0"],
            "condition_code_systems": ["http://hl7.org/fhir/sid/icd-10-cm", "http://hl7.org/fhir/sid/icd-10-cm"],
            "condition_code_displays": ["Malignant neoplasm of breast, unspecified", "Age-related osteoporosis without current pathological fracture"],
            "condition_outcomes": ["recovered", "ongoing"],
            "condition_outcome_codes": ["recovered", "ongoing"],
            "condition_outcome_systems": ["http://terminology.hl7.org/CodeSystem/condition-outcome", "http://terminology.hl7.org/CodeSystem/condition-outcome"],
            "condition_outcome_displays": ["Recovered", "Ongoing"],
            "condition_contributed_to_death": [false, false],
            "condition_onset_ages": [55.0, 60.0],
            "condition_onset_age_units": ["years", "years"],
            "condition_onset_strings": ["Mid-50s", "Early 60s"],
            "condition_notes": ["Diagnosed at age 55, treated successfully", "Diagnosed at age 60, ongoing treatment"],
            "procedure_codes": ["mastectomy", "bone-density-scan"],
            "procedure_code_codes": ["19303", "77080"],
            "procedure_code_systems": ["http://www.ama-assn.org/go/cpt", "http://www.ama-assn.org/go/cpt"],
            "procedure_code_displays": ["Mastectomy, simple, complete", "Dual-energy X-ray absorptiometry (DXA), bone density study"],
            "procedure_outcomes": ["successful", "successful"],
            "procedure_outcome_codes": ["successful", "successful"],
            "procedure_outcome_systems": ["http://terminology.hl7.org/CodeSystem/procedure-outcome", "http://terminology.hl7.org/CodeSystem/procedure-outcome"],
            "procedure_outcome_displays": ["Successful", "Successful"],
            "procedure_contributed_to_death": [false, false],
            "procedure_performed_ages": [55.0, 60.0],
            "procedure_performed_age_units": ["years", "years"],
            "procedure_performed_dates": ["2010-06-15T08:00:00Z", "2015-09-10T10:00:00Z"],
            "procedure_notes": ["Left breast mastectomy", "Annual bone density scan"],
            "notes": "Mother is alive and well. Had breast cancer at age 55, treated successfully. Currently has osteoporosis, well-managed with medication."
        }"#;

        let family_member_history: DomainFamilyMemberHistory = serde_json::from_str(json).unwrap();
        
        assert_eq!(family_member_history.family_member_history_id, "fmh_mother_001");
        assert_eq!(family_member_history.patient_demographic_no, "12345");
        assert_eq!(family_member_history.status, Some("completed".to_string()));
        assert_eq!(family_member_history.date_recorded, Some("2024-01-15T10:30:00Z".to_string()));
        assert_eq!(family_member_history.name, Some("Mary Smith".to_string()));
        assert_eq!(family_member_history.relationship, Some("mother".to_string()));
        assert_eq!(family_member_history.relationship_code, Some("mother".to_string()));
        assert_eq!(family_member_history.relationship_system, Some("http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string()));
        assert_eq!(family_member_history.relationship_display, Some("Mother".to_string()));
        assert_eq!(family_member_history.sex, Some("female".to_string()));
        assert_eq!(family_member_history.sex_code, Some("F".to_string()));
        assert_eq!(family_member_history.sex_system, Some("http://hl7.org/fhir/administrative-gender".to_string()));
        assert_eq!(family_member_history.sex_display, Some("Female".to_string()));
        assert_eq!(family_member_history.born_date, Some("1955-08-20".to_string()));
        assert_eq!(family_member_history.age_value, Some(68.5));
        assert_eq!(family_member_history.age_unit, Some("years".to_string()));
        assert_eq!(family_member_history.estimated_age, Some(false));
        assert_eq!(family_member_history.deceased, Some(false));
        assert_eq!(family_member_history.reason_codes, Some(vec!["genetic-counseling".to_string()]));
        assert_eq!(family_member_history.reason_code_codes, Some(vec!["genetic-counseling".to_string()]));
        assert_eq!(family_member_history.reason_code_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string()]));
        assert_eq!(family_member_history.reason_code_displays, Some(vec!["Genetic Counseling".to_string()]));
        assert_eq!(family_member_history.participant_function_codes, Some(vec!["informant".to_string()]));
        assert_eq!(family_member_history.participant_function_code_codes, Some(vec!["informant".to_string()]));
        assert_eq!(family_member_history.participant_function_code_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ParticipationType".to_string()]));
        assert_eq!(family_member_history.participant_function_code_displays, Some(vec!["Informant".to_string()]));
        assert_eq!(family_member_history.participant_actor_ids, Some(vec!["pat_12345".to_string()]));
        assert_eq!(family_member_history.participant_actor_types, Some(vec!["Patient".to_string()]));
        assert_eq!(family_member_history.condition_codes, Some(vec!["breast-cancer".to_string(), "osteoporosis".to_string()]));
        assert_eq!(family_member_history.condition_code_codes, Some(vec!["C50.9".to_string(), "M81.0".to_string()]));
        assert_eq!(family_member_history.condition_code_systems, Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]));
        assert_eq!(family_member_history.condition_code_displays, Some(vec!["Malignant neoplasm of breast, unspecified".to_string(), "Age-related osteoporosis without current pathological fracture".to_string()]));
        assert_eq!(family_member_history.condition_outcomes, Some(vec!["recovered".to_string(), "ongoing".to_string()]));
        assert_eq!(family_member_history.condition_outcome_codes, Some(vec!["recovered".to_string(), "ongoing".to_string()]));
        assert_eq!(family_member_history.condition_outcome_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/condition-outcome".to_string(), "http://terminology.hl7.org/CodeSystem/condition-outcome".to_string()]));
        assert_eq!(family_member_history.condition_outcome_displays, Some(vec!["Recovered".to_string(), "Ongoing".to_string()]));
        assert_eq!(family_member_history.condition_contributed_to_death, Some(vec![false, false]));
        assert_eq!(family_member_history.condition_onset_ages, Some(vec![55.0, 60.0]));
        assert_eq!(family_member_history.condition_onset_age_units, Some(vec!["years".to_string(), "years".to_string()]));
        assert_eq!(family_member_history.condition_onset_strings, Some(vec!["Mid-50s".to_string(), "Early 60s".to_string()]));
        assert_eq!(family_member_history.condition_notes, Some(vec!["Diagnosed at age 55, treated successfully".to_string(), "Diagnosed at age 60, ongoing treatment".to_string()]));
        assert_eq!(family_member_history.procedure_codes, Some(vec!["mastectomy".to_string(), "bone-density-scan".to_string()]));
        assert_eq!(family_member_history.procedure_code_codes, Some(vec!["19303".to_string(), "77080".to_string()]));
        assert_eq!(family_member_history.procedure_code_systems, Some(vec!["http://www.ama-assn.org/go/cpt".to_string(), "http://www.ama-assn.org/go/cpt".to_string()]));
        assert_eq!(family_member_history.procedure_code_displays, Some(vec!["Mastectomy, simple, complete".to_string(), "Dual-energy X-ray absorptiometry (DXA), bone density study".to_string()]));
        assert_eq!(family_member_history.procedure_outcomes, Some(vec!["successful".to_string(), "successful".to_string()]));
        assert_eq!(family_member_history.procedure_outcome_codes, Some(vec!["successful".to_string(), "successful".to_string()]));
        assert_eq!(family_member_history.procedure_outcome_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/procedure-outcome".to_string(), "http://terminology.hl7.org/CodeSystem/procedure-outcome".to_string()]));
        assert_eq!(family_member_history.procedure_outcome_displays, Some(vec!["Successful".to_string(), "Successful".to_string()]));
        assert_eq!(family_member_history.procedure_contributed_to_death, Some(vec![false, false]));
        assert_eq!(family_member_history.procedure_performed_ages, Some(vec![55.0, 60.0]));
        assert_eq!(family_member_history.procedure_performed_age_units, Some(vec!["years".to_string(), "years".to_string()]));
        assert_eq!(family_member_history.procedure_performed_dates, Some(vec!["2010-06-15T08:00:00Z".to_string(), "2015-09-10T10:00:00Z".to_string()]));
        assert_eq!(family_member_history.procedure_notes, Some(vec!["Left breast mastectomy".to_string(), "Annual bone density scan".to_string()]));
        assert_eq!(family_member_history.notes, Some("Mother is alive and well. Had breast cancer at age 55, treated successfully. Currently has osteoporosis, well-managed with medication.".to_string()));
    }

    #[test]
    fn test_domain_family_member_history_missing_required_field() {
        let json = r#"{
            "participant_actor_ids": ["prac_001"]
        }"#;

        // This should fail because family_member_history_id and patient_demographic_no are required
        let result: Result<DomainFamilyMemberHistory, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
