use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainClinicalImpression {
    pub clinical_impression_id: String,
    pub patient_demographic_no: String,
    
    // Basic clinical impression information
    pub status: Option<String>, // "preparation" | "in-progress" | "not-done" | "on-hold" | "stopped" | "completed" | "entered-in-error" | "unknown"
    pub status_reason: Option<String>, // Reason for current status
    pub description: Option<String>, // Why/how the assessment was performed
    
    // Temporal information
    pub effective_date: Option<String>, // ISO datetime string for assessment time
    pub effective_period_start: Option<String>, // ISO datetime string for assessment period start
    pub effective_period_end: Option<String>, // ISO datetime string for assessment period end
    pub documented_date: Option<String>, // ISO datetime string when assessment was documented
    
    // References and relationships
    pub encounter_id: Option<String>, // Encounter during which this ClinicalImpression was created
    pub performer_id: Option<String>, // The clinician performing the assessment
    pub performer_type: Option<String>, // Type of performer (Practitioner, PractitionerRole)
    pub previous_impression_id: Option<String>, // Reference to last assessment
    
    // Clinical information
    pub problem_condition_ids: Option<Vec<String>>, // Relevant impressions of patient state (conditions)
    pub problem_allergy_ids: Option<Vec<String>>, // Relevant impressions of patient state (allergies)
    pub change_pattern: Option<String>, // Change in status/pattern since previously assessed
    pub protocol_uris: Option<Vec<String>>, // Clinical Protocol followed
    pub summary: Option<String>, // Summary of the assessment
    
    // Findings
    pub finding_items: Option<Vec<String>>, // What was found (descriptions)
    pub finding_codes: Option<Vec<String>>, // What was found (codes)
    pub finding_systems: Option<Vec<String>>, // Terminology systems for findings
    pub finding_descriptions: Option<Vec<String>>, // Human-readable descriptions of findings
    pub finding_basis: Option<Vec<String>>, // Which investigations support finding
    
    // Prognosis
    pub prognosis_codes: Option<Vec<String>>, // Estimate of likely outcome (codes)
    pub prognosis_descriptions: Option<Vec<String>>, // Estimate of likely outcome (descriptions)
    pub prognosis_systems: Option<Vec<String>>, // Terminology systems for prognosis
    pub prognosis_reference_ids: Option<Vec<String>>, // RiskAssessment expressing likely outcome
    
    // Supporting information
    pub supporting_info_ids: Option<Vec<String>>, // Information supporting the clinical impression
    pub notes: Option<String>, // Comments made about the ClinicalImpression
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_clinical_impression_deserialization() {
        let json = r#"{
            "clinical_impression_id": "ci_12345",
            "patient_demographic_no": "12345",
            "status": "completed",
            "status_reason": "Assessment completed successfully",
            "description": "Comprehensive clinical assessment for chest pain evaluation",
            "effective_date": "2024-01-15T10:30:00Z",
            "documented_date": "2024-01-15T11:00:00Z",
            "encounter_id": "enc_001",
            "performer_id": "prac_001",
            "performer_type": "Practitioner",
            "previous_impression_id": "ci_00001",
            "problem_condition_ids": ["cond_001", "cond_002"],
            "problem_allergy_ids": ["allergy_001"],
            "change_pattern": "improving",
            "protocol_uris": ["http://example.org/protocols/chest-pain-assessment"],
            "summary": "Patient presents with chest pain. EKG shows no acute changes. Troponins negative. Likely musculoskeletal origin.",
            "finding_items": ["Chest pain", "EKG normal", "Troponins negative", "Muscle tenderness"],
            "finding_codes": ["R06.02", "Z01.810", "Z01.811", "M79.3"],
            "finding_systems": ["http://hl7.org/fhir/sid/icd-10-cm", "http://hl7.org/fhir/sid/icd-10-cm", "http://hl7.org/fhir/sid/icd-10-cm", "http://hl7.org/fhir/sid/icd-10-cm"],
            "finding_descriptions": ["Chest pain, unspecified", "Encounter for preprocedural cardiovascular examination", "Encounter for preprocedural laboratory examination", "Panniculitis, unspecified"],
            "finding_basis": ["Patient report", "EKG interpretation", "Laboratory results", "Physical examination"],
            "prognosis_codes": ["Z51.11"],
            "prognosis_descriptions": ["Encounter for antineoplastic chemotherapy"],
            "prognosis_systems": ["http://hl7.org/fhir/sid/icd-10-cm"],
            "prognosis_reference_ids": ["risk_001"],
            "supporting_info_ids": ["obs_001", "obs_002", "proc_001"],
            "notes": "Patient stable. No acute cardiac event. Follow-up in 1 week if symptoms persist."
        }"#;

        let clinical_impression: DomainClinicalImpression = serde_json::from_str(json).unwrap();
        
        assert_eq!(clinical_impression.clinical_impression_id, "ci_12345");
        assert_eq!(clinical_impression.patient_demographic_no, "12345");
        assert_eq!(clinical_impression.status, Some("completed".to_string()));
        assert_eq!(clinical_impression.status_reason, Some("Assessment completed successfully".to_string()));
        assert_eq!(clinical_impression.description, Some("Comprehensive clinical assessment for chest pain evaluation".to_string()));
        assert_eq!(clinical_impression.effective_date, Some("2024-01-15T10:30:00Z".to_string()));
        assert_eq!(clinical_impression.documented_date, Some("2024-01-15T11:00:00Z".to_string()));
        assert_eq!(clinical_impression.encounter_id, Some("enc_001".to_string()));
        assert_eq!(clinical_impression.performer_id, Some("prac_001".to_string()));
        assert_eq!(clinical_impression.performer_type, Some("Practitioner".to_string()));
        assert_eq!(clinical_impression.previous_impression_id, Some("ci_00001".to_string()));
        assert_eq!(clinical_impression.problem_condition_ids, Some(vec!["cond_001".to_string(), "cond_002".to_string()]));
        assert_eq!(clinical_impression.problem_allergy_ids, Some(vec!["allergy_001".to_string()]));
        assert_eq!(clinical_impression.change_pattern, Some("improving".to_string()));
        assert_eq!(clinical_impression.protocol_uris, Some(vec!["http://example.org/protocols/chest-pain-assessment".to_string()]));
        assert_eq!(clinical_impression.summary, Some("Patient presents with chest pain. EKG shows no acute changes. Troponins negative. Likely musculoskeletal origin.".to_string()));
        assert_eq!(clinical_impression.finding_items, Some(vec!["Chest pain".to_string(), "EKG normal".to_string(), "Troponins negative".to_string(), "Muscle tenderness".to_string()]));
        assert_eq!(clinical_impression.finding_codes, Some(vec!["R06.02".to_string(), "Z01.810".to_string(), "Z01.811".to_string(), "M79.3".to_string()]));
        assert_eq!(clinical_impression.finding_systems, Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]));
        assert_eq!(clinical_impression.finding_descriptions, Some(vec!["Chest pain, unspecified".to_string(), "Encounter for preprocedural cardiovascular examination".to_string(), "Encounter for preprocedural laboratory examination".to_string(), "Panniculitis, unspecified".to_string()]));
        assert_eq!(clinical_impression.finding_basis, Some(vec!["Patient report".to_string(), "EKG interpretation".to_string(), "Laboratory results".to_string(), "Physical examination".to_string()]));
        assert_eq!(clinical_impression.prognosis_codes, Some(vec!["Z51.11".to_string()]));
        assert_eq!(clinical_impression.prognosis_descriptions, Some(vec!["Encounter for antineoplastic chemotherapy".to_string()]));
        assert_eq!(clinical_impression.prognosis_systems, Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string()]));
        assert_eq!(clinical_impression.prognosis_reference_ids, Some(vec!["risk_001".to_string()]));
        assert_eq!(clinical_impression.supporting_info_ids, Some(vec!["obs_001".to_string(), "obs_002".to_string(), "proc_001".to_string()]));
        assert_eq!(clinical_impression.notes, Some("Patient stable. No acute cardiac event. Follow-up in 1 week if symptoms persist.".to_string()));
    }

    #[test]
    fn test_domain_clinical_impression_minimal_deserialization() {
        let json = r#"{
            "clinical_impression_id": "ci_67890",
            "patient_demographic_no": "67890"
        }"#;

        let clinical_impression: DomainClinicalImpression = serde_json::from_str(json).unwrap();
        
        assert_eq!(clinical_impression.clinical_impression_id, "ci_67890");
        assert_eq!(clinical_impression.patient_demographic_no, "67890");
        assert_eq!(clinical_impression.status, None);
        assert_eq!(clinical_impression.status_reason, None);
        assert_eq!(clinical_impression.description, None);
        assert_eq!(clinical_impression.effective_date, None);
        assert_eq!(clinical_impression.effective_period_start, None);
        assert_eq!(clinical_impression.effective_period_end, None);
        assert_eq!(clinical_impression.documented_date, None);
        assert_eq!(clinical_impression.encounter_id, None);
        assert_eq!(clinical_impression.performer_id, None);
        assert_eq!(clinical_impression.performer_type, None);
        assert_eq!(clinical_impression.previous_impression_id, None);
        assert_eq!(clinical_impression.problem_condition_ids, None);
        assert_eq!(clinical_impression.problem_allergy_ids, None);
        assert_eq!(clinical_impression.change_pattern, None);
        assert_eq!(clinical_impression.protocol_uris, None);
        assert_eq!(clinical_impression.summary, None);
        assert_eq!(clinical_impression.finding_items, None);
        assert_eq!(clinical_impression.finding_codes, None);
        assert_eq!(clinical_impression.finding_systems, None);
        assert_eq!(clinical_impression.finding_descriptions, None);
        assert_eq!(clinical_impression.finding_basis, None);
        assert_eq!(clinical_impression.prognosis_codes, None);
        assert_eq!(clinical_impression.prognosis_descriptions, None);
        assert_eq!(clinical_impression.prognosis_systems, None);
        assert_eq!(clinical_impression.prognosis_reference_ids, None);
        assert_eq!(clinical_impression.supporting_info_ids, None);
        assert_eq!(clinical_impression.notes, None);
    }

    #[test]
    fn test_domain_clinical_impression_psychiatric_assessment() {
        let json = r#"{
            "clinical_impression_id": "ci_psych_001",
            "patient_demographic_no": "12345",
            "status": "completed",
            "status_reason": "Assessment completed",
            "description": "Comprehensive psychiatric assessment for depression evaluation",
            "effective_date": "2024-02-01T14:00:00Z",
            "documented_date": "2024-02-01T15:30:00Z",
            "encounter_id": "enc_psych_001",
            "performer_id": "prac_psych_001",
            "performer_type": "Practitioner",
            "problem_condition_ids": ["cond_depression_001"],
            "change_pattern": "worsening",
            "protocol_uris": ["http://example.org/protocols/depression-assessment"],
            "summary": "Patient presents with major depressive episode. PHQ-9 score 18. Suicidal ideation present but no immediate risk.",
            "finding_items": ["Depressed mood", "Anhedonia", "Sleep disturbance", "PHQ-9 score 18"],
            "finding_codes": ["F32.9", "F32.9", "G47.00", "Z13.89"],
            "finding_systems": ["http://hl7.org/fhir/sid/icd-10-cm", "http://hl7.org/fhir/sid/icd-10-cm", "http://hl7.org/fhir/sid/icd-10-cm", "http://hl7.org/fhir/sid/icd-10-cm"],
            "finding_descriptions": ["Major depressive disorder, single episode, unspecified", "Major depressive disorder, single episode, unspecified", "Insomnia, unspecified", "Encounter for screening for other specified diseases and disorders"],
            "finding_basis": ["Clinical interview", "Clinical interview", "Patient report", "PHQ-9 questionnaire"],
            "prognosis_codes": ["Z51.11"],
            "prognosis_descriptions": ["Encounter for antineoplastic chemotherapy"],
            "prognosis_systems": ["http://hl7.org/fhir/sid/icd-10-cm"],
            "prognosis_reference_ids": ["risk_depression_001"],
            "supporting_info_ids": ["obs_phq9_001", "obs_sleep_001"],
            "notes": "Patient engaged in treatment. Safety plan established. Follow-up in 1 week."
        }"#;

        let clinical_impression: DomainClinicalImpression = serde_json::from_str(json).unwrap();
        
        assert_eq!(clinical_impression.clinical_impression_id, "ci_psych_001");
        assert_eq!(clinical_impression.patient_demographic_no, "12345");
        assert_eq!(clinical_impression.status, Some("completed".to_string()));
        assert_eq!(clinical_impression.status_reason, Some("Assessment completed".to_string()));
        assert_eq!(clinical_impression.description, Some("Comprehensive psychiatric assessment for depression evaluation".to_string()));
        assert_eq!(clinical_impression.effective_date, Some("2024-02-01T14:00:00Z".to_string()));
        assert_eq!(clinical_impression.documented_date, Some("2024-02-01T15:30:00Z".to_string()));
        assert_eq!(clinical_impression.encounter_id, Some("enc_psych_001".to_string()));
        assert_eq!(clinical_impression.performer_id, Some("prac_psych_001".to_string()));
        assert_eq!(clinical_impression.performer_type, Some("Practitioner".to_string()));
        assert_eq!(clinical_impression.problem_condition_ids, Some(vec!["cond_depression_001".to_string()]));
        assert_eq!(clinical_impression.change_pattern, Some("worsening".to_string()));
        assert_eq!(clinical_impression.protocol_uris, Some(vec!["http://example.org/protocols/depression-assessment".to_string()]));
        assert_eq!(clinical_impression.summary, Some("Patient presents with major depressive episode. PHQ-9 score 18. Suicidal ideation present but no immediate risk.".to_string()));
        assert_eq!(clinical_impression.finding_items, Some(vec!["Depressed mood".to_string(), "Anhedonia".to_string(), "Sleep disturbance".to_string(), "PHQ-9 score 18".to_string()]));
        assert_eq!(clinical_impression.finding_codes, Some(vec!["F32.9".to_string(), "F32.9".to_string(), "G47.00".to_string(), "Z13.89".to_string()]));
        assert_eq!(clinical_impression.finding_systems, Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]));
        assert_eq!(clinical_impression.finding_descriptions, Some(vec!["Major depressive disorder, single episode, unspecified".to_string(), "Major depressive disorder, single episode, unspecified".to_string(), "Insomnia, unspecified".to_string(), "Encounter for screening for other specified diseases and disorders".to_string()]));
        assert_eq!(clinical_impression.finding_basis, Some(vec!["Clinical interview".to_string(), "Clinical interview".to_string(), "Patient report".to_string(), "PHQ-9 questionnaire".to_string()]));
        assert_eq!(clinical_impression.prognosis_codes, Some(vec!["Z51.11".to_string()]));
        assert_eq!(clinical_impression.prognosis_descriptions, Some(vec!["Encounter for antineoplastic chemotherapy".to_string()]));
        assert_eq!(clinical_impression.prognosis_systems, Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string()]));
        assert_eq!(clinical_impression.prognosis_reference_ids, Some(vec!["risk_depression_001".to_string()]));
        assert_eq!(clinical_impression.supporting_info_ids, Some(vec!["obs_phq9_001".to_string(), "obs_sleep_001".to_string()]));
        assert_eq!(clinical_impression.notes, Some("Patient engaged in treatment. Safety plan established. Follow-up in 1 week.".to_string()));
    }

    #[test]
    fn test_domain_clinical_impression_pediatric_assessment() {
        let json = r#"{
            "clinical_impression_id": "ci_peds_001",
            "patient_demographic_no": "12345",
            "status": "completed",
            "status_reason": "Assessment completed",
            "description": "Developmental assessment for 2-year-old child",
            "effective_period_start": "2024-03-01T09:00:00Z",
            "effective_period_end": "2024-03-01T10:30:00Z",
            "documented_date": "2024-03-01T11:00:00Z",
            "encounter_id": "enc_peds_001",
            "performer_id": "prac_peds_001",
            "performer_type": "Practitioner",
            "problem_condition_ids": ["cond_developmental_001"],
            "change_pattern": "no-change",
            "protocol_uris": ["http://example.org/protocols/developmental-assessment"],
            "summary": "Child shows age-appropriate development. No concerns identified. Continue routine monitoring.",
            "finding_items": ["Gross motor skills normal", "Fine motor skills normal", "Language development normal", "Social interaction normal"],
            "finding_codes": ["Z00.121", "Z00.121", "Z00.121", "Z00.121"],
            "finding_systems": ["http://hl7.org/fhir/sid/icd-10-cm", "http://hl7.org/fhir/sid/icd-10-cm", "http://hl7.org/fhir/sid/icd-10-cm", "http://hl7.org/fhir/sid/icd-10-cm"],
            "finding_descriptions": ["Encounter for routine child health examination with abnormal findings", "Encounter for routine child health examination with abnormal findings", "Encounter for routine child health examination with abnormal findings", "Encounter for routine child health examination with abnormal findings"],
            "finding_basis": ["Developmental assessment", "Developmental assessment", "Developmental assessment", "Developmental assessment"],
            "prognosis_codes": ["Z00.121"],
            "prognosis_descriptions": ["Encounter for routine child health examination with abnormal findings"],
            "prognosis_systems": ["http://hl7.org/fhir/sid/icd-10-cm"],
            "supporting_info_ids": ["obs_developmental_001", "obs_growth_001"],
            "notes": "Child meeting all developmental milestones. Parent education provided. Next assessment in 6 months."
        }"#;

        let clinical_impression: DomainClinicalImpression = serde_json::from_str(json).unwrap();
        
        assert_eq!(clinical_impression.clinical_impression_id, "ci_peds_001");
        assert_eq!(clinical_impression.patient_demographic_no, "12345");
        assert_eq!(clinical_impression.status, Some("completed".to_string()));
        assert_eq!(clinical_impression.status_reason, Some("Assessment completed".to_string()));
        assert_eq!(clinical_impression.description, Some("Developmental assessment for 2-year-old child".to_string()));
        assert_eq!(clinical_impression.effective_period_start, Some("2024-03-01T09:00:00Z".to_string()));
        assert_eq!(clinical_impression.effective_period_end, Some("2024-03-01T10:30:00Z".to_string()));
        assert_eq!(clinical_impression.documented_date, Some("2024-03-01T11:00:00Z".to_string()));
        assert_eq!(clinical_impression.encounter_id, Some("enc_peds_001".to_string()));
        assert_eq!(clinical_impression.performer_id, Some("prac_peds_001".to_string()));
        assert_eq!(clinical_impression.performer_type, Some("Practitioner".to_string()));
        assert_eq!(clinical_impression.problem_condition_ids, Some(vec!["cond_developmental_001".to_string()]));
        assert_eq!(clinical_impression.change_pattern, Some("no-change".to_string()));
        assert_eq!(clinical_impression.protocol_uris, Some(vec!["http://example.org/protocols/developmental-assessment".to_string()]));
        assert_eq!(clinical_impression.summary, Some("Child shows age-appropriate development. No concerns identified. Continue routine monitoring.".to_string()));
        assert_eq!(clinical_impression.finding_items, Some(vec!["Gross motor skills normal".to_string(), "Fine motor skills normal".to_string(), "Language development normal".to_string(), "Social interaction normal".to_string()]));
        assert_eq!(clinical_impression.finding_codes, Some(vec!["Z00.121".to_string(), "Z00.121".to_string(), "Z00.121".to_string(), "Z00.121".to_string()]));
        assert_eq!(clinical_impression.finding_systems, Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]));
        assert_eq!(clinical_impression.finding_descriptions, Some(vec!["Encounter for routine child health examination with abnormal findings".to_string(), "Encounter for routine child health examination with abnormal findings".to_string(), "Encounter for routine child health examination with abnormal findings".to_string(), "Encounter for routine child health examination with abnormal findings".to_string()]));
        assert_eq!(clinical_impression.finding_basis, Some(vec!["Developmental assessment".to_string(), "Developmental assessment".to_string(), "Developmental assessment".to_string(), "Developmental assessment".to_string()]));
        assert_eq!(clinical_impression.prognosis_codes, Some(vec!["Z00.121".to_string()]));
        assert_eq!(clinical_impression.prognosis_descriptions, Some(vec!["Encounter for routine child health examination with abnormal findings".to_string()]));
        assert_eq!(clinical_impression.prognosis_systems, Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string()]));
        assert_eq!(clinical_impression.supporting_info_ids, Some(vec!["obs_developmental_001".to_string(), "obs_growth_001".to_string()]));
        assert_eq!(clinical_impression.notes, Some("Child meeting all developmental milestones. Parent education provided. Next assessment in 6 months.".to_string()));
    }

    #[test]
    fn test_domain_clinical_impression_missing_required_field() {
        let json = r#"{
            "performer_id": "prac_001"
        }"#;

        // This should fail because clinical_impression_id and patient_demographic_no are required
        let result: Result<DomainClinicalImpression, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
