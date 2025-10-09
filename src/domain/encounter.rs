use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainEncounter {
    pub encounter_id: String,
    pub patient_demographic_no: String,
    
    // Basic encounter information
    pub status: Option<String>, // "planned" | "in-progress" | "on-hold" | "discharged" | "completed" | "cancelled" | "discontinued" | "entered-in-error" | "unknown"
    pub class_code: Option<String>, // "inpatient" | "outpatient" | "emergency" | "ambulatory" | "wellness" | "urgentcare" | "virtual"
    pub priority: Option<String>, // Priority level
    pub encounter_type: Option<String>, // Specific type of encounter
    pub service_type: Option<String>, // Specific type of service
    
    // Temporal information
    pub planned_start_date: Option<String>, // ISO datetime string
    pub planned_end_date: Option<String>, // ISO datetime string
    pub actual_start_date: Option<String>, // ISO datetime string
    pub actual_end_date: Option<String>, // ISO datetime string
    pub length_minutes: Option<u32>, // Duration in minutes
    
    // Participants and references
    pub practitioner_id: Option<String>, // Primary practitioner
    pub location_id: Option<String>, // Primary location
    pub service_provider_id: Option<String>, // Organization/facility
    pub appointment_id: Option<String>, // Related appointment
    pub part_of_encounter_id: Option<String>, // Parent encounter
    pub episode_of_care_id: Option<String>, // Episode of care
    
    // Clinical information
    pub reason_codes: Option<Vec<String>>, // Reason codes for the encounter
    pub reason_descriptions: Option<Vec<String>>, // Reason descriptions
    pub diagnosis_codes: Option<Vec<String>>, // Diagnosis codes
    pub diagnosis_descriptions: Option<Vec<String>>, // Diagnosis descriptions
    pub diagnosis_ranks: Option<Vec<u32>>, // Diagnosis ranking (1 = primary, 2 = secondary, etc.)
    
    // Patient status and special needs
    pub subject_status: Option<String>, // Patient status during encounter
    pub diet_preferences: Option<Vec<String>>, // Diet preferences
    pub special_arrangements: Option<Vec<String>>, // Special arrangements (wheelchair, translator, etc.)
    pub special_courtesies: Option<Vec<String>>, // Special courtesies (VIP, board member, etc.)
    
    // Admission information (for inpatient encounters)
    pub admission_source: Option<String>, // Source of admission
    pub admission_diagnosis: Option<String>, // Admission diagnosis
    pub discharge_disposition: Option<String>, // Discharge disposition
    pub discharge_diagnosis: Option<String>, // Discharge diagnosis
    
    // Additional information
    pub notes: Option<String>, // Additional notes
    pub care_team_ids: Option<Vec<String>>, // Care team member IDs
    pub account_ids: Option<Vec<String>>, // Billing account IDs
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_encounter_deserialization() {
        let json = r#"{
            "encounter_id": "enc_12345",
            "patient_demographic_no": "12345",
            "status": "completed",
            "class_code": "outpatient",
            "priority": "routine",
            "encounter_type": "consultation",
            "service_type": "cardiology",
            "planned_start_date": "2024-01-15T10:00:00Z",
            "planned_end_date": "2024-01-15T10:30:00Z",
            "actual_start_date": "2024-01-15T10:05:00Z",
            "actual_end_date": "2024-01-15T10:35:00Z",
            "length_minutes": 30,
            "practitioner_id": "prac_001",
            "location_id": "loc_001",
            "service_provider_id": "org_001",
            "appointment_id": "apt_001",
            "part_of_encounter_id": null,
            "episode_of_care_id": "episode_001",
            "reason_codes": ["Z00.00", "I25.9"],
            "reason_descriptions": ["Encounter for general adult medical examination", "Chronic ischemic heart disease"],
            "diagnosis_codes": ["I25.9", "E11.9"],
            "diagnosis_descriptions": ["Chronic ischemic heart disease, unspecified", "Type 2 diabetes mellitus without complications"],
            "diagnosis_ranks": [1, 2],
            "subject_status": "active",
            "diet_preferences": ["diabetic"],
            "special_arrangements": ["wheelchair"],
            "special_courtesies": [],
            "admission_source": null,
            "admission_diagnosis": null,
            "discharge_disposition": null,
            "discharge_diagnosis": null,
            "notes": "Patient stable, follow-up in 3 months",
            "care_team_ids": ["nurse_001", "tech_001"],
            "account_ids": ["acct_001"]
        }"#;

        let encounter: DomainEncounter = serde_json::from_str(json).unwrap();
        
        assert_eq!(encounter.encounter_id, "enc_12345");
        assert_eq!(encounter.patient_demographic_no, "12345");
        assert_eq!(encounter.status, Some("completed".to_string()));
        assert_eq!(encounter.class_code, Some("outpatient".to_string()));
        assert_eq!(encounter.priority, Some("routine".to_string()));
        assert_eq!(encounter.encounter_type, Some("consultation".to_string()));
        assert_eq!(encounter.service_type, Some("cardiology".to_string()));
        assert_eq!(encounter.planned_start_date, Some("2024-01-15T10:00:00Z".to_string()));
        assert_eq!(encounter.planned_end_date, Some("2024-01-15T10:30:00Z".to_string()));
        assert_eq!(encounter.actual_start_date, Some("2024-01-15T10:05:00Z".to_string()));
        assert_eq!(encounter.actual_end_date, Some("2024-01-15T10:35:00Z".to_string()));
        assert_eq!(encounter.length_minutes, Some(30));
        assert_eq!(encounter.practitioner_id, Some("prac_001".to_string()));
        assert_eq!(encounter.location_id, Some("loc_001".to_string()));
        assert_eq!(encounter.service_provider_id, Some("org_001".to_string()));
        assert_eq!(encounter.appointment_id, Some("apt_001".to_string()));
        assert_eq!(encounter.part_of_encounter_id, None);
        assert_eq!(encounter.episode_of_care_id, Some("episode_001".to_string()));
        assert_eq!(encounter.reason_codes, Some(vec!["Z00.00".to_string(), "I25.9".to_string()]));
        assert_eq!(encounter.reason_descriptions, Some(vec!["Encounter for general adult medical examination".to_string(), "Chronic ischemic heart disease".to_string()]));
        assert_eq!(encounter.diagnosis_codes, Some(vec!["I25.9".to_string(), "E11.9".to_string()]));
        assert_eq!(encounter.diagnosis_descriptions, Some(vec!["Chronic ischemic heart disease, unspecified".to_string(), "Type 2 diabetes mellitus without complications".to_string()]));
        assert_eq!(encounter.diagnosis_ranks, Some(vec![1, 2]));
        assert_eq!(encounter.subject_status, Some("active".to_string()));
        assert_eq!(encounter.diet_preferences, Some(vec!["diabetic".to_string()]));
        assert_eq!(encounter.special_arrangements, Some(vec!["wheelchair".to_string()]));
        assert_eq!(encounter.special_courtesies, Some(vec![]));
        assert_eq!(encounter.admission_source, None);
        assert_eq!(encounter.admission_diagnosis, None);
        assert_eq!(encounter.discharge_disposition, None);
        assert_eq!(encounter.discharge_diagnosis, None);
        assert_eq!(encounter.notes, Some("Patient stable, follow-up in 3 months".to_string()));
        assert_eq!(encounter.care_team_ids, Some(vec!["nurse_001".to_string(), "tech_001".to_string()]));
        assert_eq!(encounter.account_ids, Some(vec!["acct_001".to_string()]));
    }

    #[test]
    fn test_domain_encounter_minimal_deserialization() {
        let json = r#"{
            "encounter_id": "enc_67890",
            "patient_demographic_no": "67890"
        }"#;

        let encounter: DomainEncounter = serde_json::from_str(json).unwrap();
        
        assert_eq!(encounter.encounter_id, "enc_67890");
        assert_eq!(encounter.patient_demographic_no, "67890");
        assert_eq!(encounter.status, None);
        assert_eq!(encounter.class_code, None);
        assert_eq!(encounter.priority, None);
        assert_eq!(encounter.encounter_type, None);
        assert_eq!(encounter.service_type, None);
        assert_eq!(encounter.planned_start_date, None);
        assert_eq!(encounter.planned_end_date, None);
        assert_eq!(encounter.actual_start_date, None);
        assert_eq!(encounter.actual_end_date, None);
        assert_eq!(encounter.length_minutes, None);
        assert_eq!(encounter.practitioner_id, None);
        assert_eq!(encounter.location_id, None);
        assert_eq!(encounter.service_provider_id, None);
        assert_eq!(encounter.appointment_id, None);
        assert_eq!(encounter.part_of_encounter_id, None);
        assert_eq!(encounter.episode_of_care_id, None);
        assert_eq!(encounter.reason_codes, None);
        assert_eq!(encounter.reason_descriptions, None);
        assert_eq!(encounter.diagnosis_codes, None);
        assert_eq!(encounter.diagnosis_descriptions, None);
        assert_eq!(encounter.diagnosis_ranks, None);
        assert_eq!(encounter.subject_status, None);
        assert_eq!(encounter.diet_preferences, None);
        assert_eq!(encounter.special_arrangements, None);
        assert_eq!(encounter.special_courtesies, None);
        assert_eq!(encounter.admission_source, None);
        assert_eq!(encounter.admission_diagnosis, None);
        assert_eq!(encounter.discharge_disposition, None);
        assert_eq!(encounter.discharge_diagnosis, None);
        assert_eq!(encounter.notes, None);
        assert_eq!(encounter.care_team_ids, None);
        assert_eq!(encounter.account_ids, None);
    }

    #[test]
    fn test_domain_encounter_inpatient_scenario() {
        let json = r#"{
            "encounter_id": "enc_inpatient_001",
            "patient_demographic_no": "12345",
            "status": "discharged",
            "class_code": "inpatient",
            "priority": "urgent",
            "encounter_type": "emergency",
            "service_type": "cardiology",
            "planned_start_date": "2024-01-10T08:00:00Z",
            "actual_start_date": "2024-01-10T08:15:00Z",
            "actual_end_date": "2024-01-12T14:30:00Z",
            "length_minutes": 3240,
            "practitioner_id": "prac_002",
            "location_id": "loc_002",
            "service_provider_id": "org_001",
            "admission_source": "emergency",
            "admission_diagnosis": "Acute myocardial infarction",
            "discharge_disposition": "home",
            "discharge_diagnosis": "Acute ST elevation myocardial infarction, anterior wall",
            "diagnosis_codes": ["I21.01"],
            "diagnosis_descriptions": ["ST elevation myocardial infarction involving left anterior descending artery"],
            "diagnosis_ranks": [1],
            "notes": "Patient underwent PCI, stable for discharge"
        }"#;

        let encounter: DomainEncounter = serde_json::from_str(json).unwrap();
        
        assert_eq!(encounter.encounter_id, "enc_inpatient_001");
        assert_eq!(encounter.patient_demographic_no, "12345");
        assert_eq!(encounter.status, Some("discharged".to_string()));
        assert_eq!(encounter.class_code, Some("inpatient".to_string()));
        assert_eq!(encounter.priority, Some("urgent".to_string()));
        assert_eq!(encounter.encounter_type, Some("emergency".to_string()));
        assert_eq!(encounter.actual_start_date, Some("2024-01-10T08:15:00Z".to_string()));
        assert_eq!(encounter.actual_end_date, Some("2024-01-12T14:30:00Z".to_string()));
        assert_eq!(encounter.length_minutes, Some(3240));
        assert_eq!(encounter.admission_source, Some("emergency".to_string()));
        assert_eq!(encounter.admission_diagnosis, Some("Acute myocardial infarction".to_string()));
        assert_eq!(encounter.discharge_disposition, Some("home".to_string()));
        assert_eq!(encounter.discharge_diagnosis, Some("Acute ST elevation myocardial infarction, anterior wall".to_string()));
        assert_eq!(encounter.diagnosis_codes, Some(vec!["I21.01".to_string()]));
        assert_eq!(encounter.diagnosis_descriptions, Some(vec!["ST elevation myocardial infarction involving left anterior descending artery".to_string()]));
        assert_eq!(encounter.diagnosis_ranks, Some(vec![1]));
        assert_eq!(encounter.notes, Some("Patient underwent PCI, stable for discharge".to_string()));
    }

    #[test]
    fn test_domain_encounter_virtual_consultation() {
        let json = r#"{
            "encounter_id": "enc_virtual_001",
            "patient_demographic_no": "12345",
            "status": "completed",
            "class_code": "virtual",
            "encounter_type": "telemedicine",
            "service_type": "psychiatry",
            "planned_start_date": "2024-01-20T14:00:00Z",
            "actual_start_date": "2024-01-20T14:02:00Z",
            "actual_end_date": "2024-01-20T14:47:00Z",
            "length_minutes": 45,
            "practitioner_id": "prac_003",
            "service_provider_id": "org_002",
            "appointment_id": "apt_virtual_001",
            "reason_codes": ["Z71.1"],
            "reason_descriptions": ["Person with feared health complaint in whom no diagnosis is made"],
            "subject_status": "active",
            "notes": "Virtual consultation via secure video platform"
        }"#;

        let encounter: DomainEncounter = serde_json::from_str(json).unwrap();
        
        assert_eq!(encounter.encounter_id, "enc_virtual_001");
        assert_eq!(encounter.patient_demographic_no, "12345");
        assert_eq!(encounter.status, Some("completed".to_string()));
        assert_eq!(encounter.class_code, Some("virtual".to_string()));
        assert_eq!(encounter.encounter_type, Some("telemedicine".to_string()));
        assert_eq!(encounter.service_type, Some("psychiatry".to_string()));
        assert_eq!(encounter.planned_start_date, Some("2024-01-20T14:00:00Z".to_string()));
        assert_eq!(encounter.actual_start_date, Some("2024-01-20T14:02:00Z".to_string()));
        assert_eq!(encounter.actual_end_date, Some("2024-01-20T14:47:00Z".to_string()));
        assert_eq!(encounter.length_minutes, Some(45));
        assert_eq!(encounter.practitioner_id, Some("prac_003".to_string()));
        assert_eq!(encounter.service_provider_id, Some("org_002".to_string()));
        assert_eq!(encounter.appointment_id, Some("apt_virtual_001".to_string()));
        assert_eq!(encounter.reason_codes, Some(vec!["Z71.1".to_string()]));
        assert_eq!(encounter.reason_descriptions, Some(vec!["Person with feared health complaint in whom no diagnosis is made".to_string()]));
        assert_eq!(encounter.subject_status, Some("active".to_string()));
        assert_eq!(encounter.notes, Some("Virtual consultation via secure video platform".to_string()));
    }

    #[test]
    fn test_domain_encounter_missing_required_field() {
        let json = r#"{
            "practitioner_id": "prac_001"
        }"#;

        // This should fail because encounter_id and patient_demographic_no are required
        let result: Result<DomainEncounter, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
