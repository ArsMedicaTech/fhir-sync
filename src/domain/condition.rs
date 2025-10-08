use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainCondition {
    pub condition_id: String,
    pub patient_demographic_no: String,
    pub encounter_id: Option<String>,
    pub practitioner_id: Option<String>,
    
    // Clinical information
    pub clinical_status: Option<String>, // "active" | "recurrence" | "relapse" | "inactive" | "remission" | "resolved" | "unknown"
    pub verification_status: Option<String>, // "unconfirmed" | "provisional" | "differential" | "confirmed" | "refuted" | "entered-in-error"
    pub category: Option<String>, // "problem-list-item" | "encounter-diagnosis"
    pub severity: Option<String>, // Severity level
    pub code: Option<String>, // ICD-10 or SNOMED code
    pub code_display: Option<String>, // Human-readable description of the code
    pub body_site: Option<String>, // Anatomical location
    
    // Temporal information
    pub onset_date: Option<String>, // ISO datetime string
    pub onset_age: Option<String>, // Age at onset (e.g., "45 years", "2 months")
    pub onset_description: Option<String>, // Text description of onset
    pub abatement_date: Option<String>, // ISO datetime string when condition resolved
    pub abatement_age: Option<String>, // Age at abatement
    pub abatement_description: Option<String>, // Text description of abatement
    pub recorded_date: Option<String>, // ISO datetime string when first recorded
    
    // Staging information
    pub stage_summary: Option<String>, // Stage summary
    pub stage_type: Option<String>, // Type of staging
    pub stage_assessment_ids: Option<Vec<String>>, // References to assessment records
    
    // Additional information
    pub notes: Option<String>, // Additional notes
    pub evidence_codes: Option<Vec<String>>, // Supporting evidence codes
    pub evidence_descriptions: Option<Vec<String>>, // Supporting evidence descriptions
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_condition_deserialization() {
        let json = r#"{
            "condition_id": "cond_12345",
            "patient_demographic_no": "12345",
            "encounter_id": "enc_001",
            "practitioner_id": "prac_001",
            "clinical_status": "active",
            "verification_status": "confirmed",
            "category": "problem-list-item",
            "severity": "moderate",
            "code": "I25.9",
            "code_display": "Chronic ischemic heart disease, unspecified",
            "body_site": "heart",
            "onset_date": "2023-01-15T00:00:00Z",
            "onset_age": "65 years",
            "onset_description": "Patient reported chest pain",
            "abatement_date": null,
            "abatement_age": null,
            "abatement_description": null,
            "recorded_date": "2023-01-20T10:30:00Z",
            "stage_summary": "Stage 2",
            "stage_type": "TNM",
            "stage_assessment_ids": ["assess_001", "assess_002"],
            "notes": "Patient has family history of heart disease",
            "evidence_codes": ["E11.9", "Z87.891"],
            "evidence_descriptions": ["Type 2 diabetes mellitus", "Personal history of tobacco use"]
        }"#;

        let condition: DomainCondition = serde_json::from_str(json).unwrap();
        
        assert_eq!(condition.condition_id, "cond_12345");
        assert_eq!(condition.patient_demographic_no, "12345");
        assert_eq!(condition.encounter_id, Some("enc_001".to_string()));
        assert_eq!(condition.practitioner_id, Some("prac_001".to_string()));
        assert_eq!(condition.clinical_status, Some("active".to_string()));
        assert_eq!(condition.verification_status, Some("confirmed".to_string()));
        assert_eq!(condition.category, Some("problem-list-item".to_string()));
        assert_eq!(condition.severity, Some("moderate".to_string()));
        assert_eq!(condition.code, Some("I25.9".to_string()));
        assert_eq!(condition.code_display, Some("Chronic ischemic heart disease, unspecified".to_string()));
        assert_eq!(condition.body_site, Some("heart".to_string()));
        assert_eq!(condition.onset_date, Some("2023-01-15T00:00:00Z".to_string()));
        assert_eq!(condition.onset_age, Some("65 years".to_string()));
        assert_eq!(condition.onset_description, Some("Patient reported chest pain".to_string()));
        assert_eq!(condition.abatement_date, None);
        assert_eq!(condition.abatement_age, None);
        assert_eq!(condition.abatement_description, None);
        assert_eq!(condition.recorded_date, Some("2023-01-20T10:30:00Z".to_string()));
        assert_eq!(condition.stage_summary, Some("Stage 2".to_string()));
        assert_eq!(condition.stage_type, Some("TNM".to_string()));
        assert_eq!(condition.stage_assessment_ids, Some(vec!["assess_001".to_string(), "assess_002".to_string()]));
        assert_eq!(condition.notes, Some("Patient has family history of heart disease".to_string()));
        assert_eq!(condition.evidence_codes, Some(vec!["E11.9".to_string(), "Z87.891".to_string()]));
        assert_eq!(condition.evidence_descriptions, Some(vec!["Type 2 diabetes mellitus".to_string(), "Personal history of tobacco use".to_string()]));
    }

    #[test]
    fn test_domain_condition_minimal_deserialization() {
        let json = r#"{
            "condition_id": "cond_67890",
            "patient_demographic_no": "67890"
        }"#;

        let condition: DomainCondition = serde_json::from_str(json).unwrap();
        
        assert_eq!(condition.condition_id, "cond_67890");
        assert_eq!(condition.patient_demographic_no, "67890");
        assert_eq!(condition.encounter_id, None);
        assert_eq!(condition.practitioner_id, None);
        assert_eq!(condition.clinical_status, None);
        assert_eq!(condition.verification_status, None);
        assert_eq!(condition.category, None);
        assert_eq!(condition.severity, None);
        assert_eq!(condition.code, None);
        assert_eq!(condition.code_display, None);
        assert_eq!(condition.body_site, None);
        assert_eq!(condition.onset_date, None);
        assert_eq!(condition.onset_age, None);
        assert_eq!(condition.onset_description, None);
        assert_eq!(condition.abatement_date, None);
        assert_eq!(condition.abatement_age, None);
        assert_eq!(condition.abatement_description, None);
        assert_eq!(condition.recorded_date, None);
        assert_eq!(condition.stage_summary, None);
        assert_eq!(condition.stage_type, None);
        assert_eq!(condition.stage_assessment_ids, None);
        assert_eq!(condition.notes, None);
        assert_eq!(condition.evidence_codes, None);
        assert_eq!(condition.evidence_descriptions, None);
    }

    #[test]
    fn test_domain_condition_resolved_condition() {
        let json = r#"{
            "condition_id": "cond_99999",
            "patient_demographic_no": "99999",
            "clinical_status": "resolved",
            "verification_status": "confirmed",
            "code": "J06.9",
            "code_display": "Acute upper respiratory infection, unspecified",
            "onset_date": "2023-12-01T00:00:00Z",
            "abatement_date": "2023-12-10T00:00:00Z",
            "abatement_description": "Symptoms resolved after treatment",
            "recorded_date": "2023-12-01T14:30:00Z",
            "notes": "Patient recovered fully with antibiotics"
        }"#;

        let condition: DomainCondition = serde_json::from_str(json).unwrap();
        
        assert_eq!(condition.condition_id, "cond_99999");
        assert_eq!(condition.patient_demographic_no, "99999");
        assert_eq!(condition.clinical_status, Some("resolved".to_string()));
        assert_eq!(condition.verification_status, Some("confirmed".to_string()));
        assert_eq!(condition.code, Some("J06.9".to_string()));
        assert_eq!(condition.code_display, Some("Acute upper respiratory infection, unspecified".to_string()));
        assert_eq!(condition.onset_date, Some("2023-12-01T00:00:00Z".to_string()));
        assert_eq!(condition.abatement_date, Some("2023-12-10T00:00:00Z".to_string()));
        assert_eq!(condition.abatement_description, Some("Symptoms resolved after treatment".to_string()));
        assert_eq!(condition.recorded_date, Some("2023-12-01T14:30:00Z".to_string()));
        assert_eq!(condition.notes, Some("Patient recovered fully with antibiotics".to_string()));
    }

    #[test]
    fn test_domain_condition_missing_required_field() {
        let json = r#"{
            "practitioner_id": "prac_001"
        }"#;

        // This should fail because condition_id and patient_demographic_no are required
        let result: Result<DomainCondition, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_domain_condition_staging_information() {
        let json = r#"{
            "condition_id": "cond_stage",
            "patient_demographic_no": "12345",
            "code": "C78.00",
            "code_display": "Secondary malignant neoplasm of unspecified lung",
            "stage_summary": "T2N1M0",
            "stage_type": "TNM",
            "stage_assessment_ids": ["path_001", "imaging_001"]
        }"#;

        let condition: DomainCondition = serde_json::from_str(json).unwrap();
        
        assert_eq!(condition.condition_id, "cond_stage");
        assert_eq!(condition.patient_demographic_no, "12345");
        assert_eq!(condition.code, Some("C78.00".to_string()));
        assert_eq!(condition.code_display, Some("Secondary malignant neoplasm of unspecified lung".to_string()));
        assert_eq!(condition.stage_summary, Some("T2N1M0".to_string()));
        assert_eq!(condition.stage_type, Some("TNM".to_string()));
        assert_eq!(condition.stage_assessment_ids, Some(vec!["path_001".to_string(), "imaging_001".to_string()]));
    }
}
