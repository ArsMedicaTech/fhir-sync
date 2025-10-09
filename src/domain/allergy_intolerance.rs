use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainAllergyIntolerance {
    pub allergy_id: String,
    pub patient_demographic_no: String,
    
    // Basic allergy information
    pub clinical_status: Option<String>, // "active" | "inactive" | "resolved"
    pub verification_status: Option<String>, // "unconfirmed" | "presumed" | "confirmed" | "refuted" | "entered-in-error"
    pub allergy_type: Option<String>, // "allergy" | "intolerance"
    pub category: Option<String>, // "food" | "medication" | "environment" | "biologic"
    pub criticality: Option<String>, // "low" | "high" | "unable-to-assess"
    
    // Substance information
    pub substance_code: Option<String>, // SNOMED or other code for the substance
    pub substance_display: Option<String>, // Human-readable name of the substance
    pub substance_system: Option<String>, // Terminology system (e.g., "http://snomed.info/sct")
    
    // Temporal information
    pub onset_date: Option<String>, // ISO datetime string
    pub onset_age: Option<String>, // Age at onset (e.g., "5 years", "2 months")
    pub onset_description: Option<String>, // Text description of onset
    pub recorded_date: Option<String>, // ISO datetime string when first recorded
    pub last_occurrence_date: Option<String>, // ISO datetime string of last known occurrence
    
    // Participants
    pub recorder_id: Option<String>, // Who recorded the allergy
    pub recorder_type: Option<String>, // "Practitioner" | "Patient" | "RelatedPerson" | "Device" | "Organization" | "CareTeam"
    pub encounter_id: Option<String>, // Encounter when allergy was asserted
    
    // Reaction information
    pub reaction_substances: Option<Vec<String>>, // Specific substances that caused reactions
    pub reaction_manifestations: Option<Vec<String>>, // Clinical symptoms/signs
    pub reaction_descriptions: Option<Vec<String>>, // Description of reactions
    pub reaction_onset_dates: Option<Vec<String>>, // When manifestations showed
    pub reaction_severities: Option<Vec<String>>, // "mild" | "moderate" | "severe"
    pub reaction_exposure_routes: Option<Vec<String>>, // How the subject was exposed
    pub reaction_notes: Option<Vec<String>>, // Additional notes about reactions
    
    // Additional information
    pub notes: Option<String>, // Additional text not captured in other fields
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_allergy_intolerance_deserialization() {
        let json = r#"{
            "allergy_id": "allergy_12345",
            "patient_demographic_no": "12345",
            "clinical_status": "active",
            "verification_status": "confirmed",
            "allergy_type": "allergy",
            "category": "medication",
            "criticality": "high",
            "substance_code": "7980",
            "substance_display": "Penicillin",
            "substance_system": "http://snomed.info/sct",
            "onset_date": "2020-03-15T00:00:00Z",
            "onset_age": "25 years",
            "onset_description": "Patient developed rash after taking penicillin",
            "recorded_date": "2020-03-20T10:30:00Z",
            "last_occurrence_date": "2022-01-10T14:00:00Z",
            "recorder_id": "prac_001",
            "recorder_type": "Practitioner",
            "encounter_id": "enc_001",
            "reaction_substances": ["Penicillin", "Amoxicillin"],
            "reaction_manifestations": ["Rash", "Hives", "Difficulty breathing"],
            "reaction_descriptions": ["Severe allergic reaction with respiratory distress"],
            "reaction_onset_dates": ["2020-03-15T00:00:00Z", "2022-01-10T14:00:00Z"],
            "reaction_severities": ["severe", "moderate"],
            "reaction_exposure_routes": ["oral", "injection"],
            "reaction_notes": ["Required emergency treatment", "Patient carries epinephrine"],
            "notes": "Patient has severe penicillin allergy. Avoid all beta-lactam antibiotics."
        }"#;

        let allergy: DomainAllergyIntolerance = serde_json::from_str(json).unwrap();
        
        assert_eq!(allergy.allergy_id, "allergy_12345");
        assert_eq!(allergy.patient_demographic_no, "12345");
        assert_eq!(allergy.clinical_status, Some("active".to_string()));
        assert_eq!(allergy.verification_status, Some("confirmed".to_string()));
        assert_eq!(allergy.allergy_type, Some("allergy".to_string()));
        assert_eq!(allergy.category, Some("medication".to_string()));
        assert_eq!(allergy.criticality, Some("high".to_string()));
        assert_eq!(allergy.substance_code, Some("7980".to_string()));
        assert_eq!(allergy.substance_display, Some("Penicillin".to_string()));
        assert_eq!(allergy.substance_system, Some("http://snomed.info/sct".to_string()));
        assert_eq!(allergy.onset_date, Some("2020-03-15T00:00:00Z".to_string()));
        assert_eq!(allergy.onset_age, Some("25 years".to_string()));
        assert_eq!(allergy.onset_description, Some("Patient developed rash after taking penicillin".to_string()));
        assert_eq!(allergy.recorded_date, Some("2020-03-20T10:30:00Z".to_string()));
        assert_eq!(allergy.last_occurrence_date, Some("2022-01-10T14:00:00Z".to_string()));
        assert_eq!(allergy.recorder_id, Some("prac_001".to_string()));
        assert_eq!(allergy.recorder_type, Some("Practitioner".to_string()));
        assert_eq!(allergy.encounter_id, Some("enc_001".to_string()));
        assert_eq!(allergy.reaction_substances, Some(vec!["Penicillin".to_string(), "Amoxicillin".to_string()]));
        assert_eq!(allergy.reaction_manifestations, Some(vec!["Rash".to_string(), "Hives".to_string(), "Difficulty breathing".to_string()]));
        assert_eq!(allergy.reaction_descriptions, Some(vec!["Severe allergic reaction with respiratory distress".to_string()]));
        assert_eq!(allergy.reaction_onset_dates, Some(vec!["2020-03-15T00:00:00Z".to_string(), "2022-01-10T14:00:00Z".to_string()]));
        assert_eq!(allergy.reaction_severities, Some(vec!["severe".to_string(), "moderate".to_string()]));
        assert_eq!(allergy.reaction_exposure_routes, Some(vec!["oral".to_string(), "injection".to_string()]));
        assert_eq!(allergy.reaction_notes, Some(vec!["Required emergency treatment".to_string(), "Patient carries epinephrine".to_string()]));
        assert_eq!(allergy.notes, Some("Patient has severe penicillin allergy. Avoid all beta-lactam antibiotics.".to_string()));
    }

    #[test]
    fn test_domain_allergy_intolerance_minimal_deserialization() {
        let json = r#"{
            "allergy_id": "allergy_67890",
            "patient_demographic_no": "67890"
        }"#;

        let allergy: DomainAllergyIntolerance = serde_json::from_str(json).unwrap();
        
        assert_eq!(allergy.allergy_id, "allergy_67890");
        assert_eq!(allergy.patient_demographic_no, "67890");
        assert_eq!(allergy.clinical_status, None);
        assert_eq!(allergy.verification_status, None);
        assert_eq!(allergy.allergy_type, None);
        assert_eq!(allergy.category, None);
        assert_eq!(allergy.criticality, None);
        assert_eq!(allergy.substance_code, None);
        assert_eq!(allergy.substance_display, None);
        assert_eq!(allergy.substance_system, None);
        assert_eq!(allergy.onset_date, None);
        assert_eq!(allergy.onset_age, None);
        assert_eq!(allergy.onset_description, None);
        assert_eq!(allergy.recorded_date, None);
        assert_eq!(allergy.last_occurrence_date, None);
        assert_eq!(allergy.recorder_id, None);
        assert_eq!(allergy.recorder_type, None);
        assert_eq!(allergy.encounter_id, None);
        assert_eq!(allergy.reaction_substances, None);
        assert_eq!(allergy.reaction_manifestations, None);
        assert_eq!(allergy.reaction_descriptions, None);
        assert_eq!(allergy.reaction_onset_dates, None);
        assert_eq!(allergy.reaction_severities, None);
        assert_eq!(allergy.reaction_exposure_routes, None);
        assert_eq!(allergy.reaction_notes, None);
        assert_eq!(allergy.notes, None);
    }

    #[test]
    fn test_domain_allergy_intolerance_food_allergy() {
        let json = r#"{
            "allergy_id": "allergy_food_001",
            "patient_demographic_no": "12345",
            "clinical_status": "active",
            "verification_status": "confirmed",
            "allergy_type": "allergy",
            "category": "food",
            "criticality": "high",
            "substance_code": "762952008",
            "substance_display": "Peanut",
            "substance_system": "http://snomed.info/sct",
            "onset_age": "2 years",
            "onset_description": "Child developed hives and vomiting after eating peanut butter",
            "recorded_date": "2020-05-10T09:00:00Z",
            "last_occurrence_date": "2021-12-15T16:30:00Z",
            "recorder_id": "prac_002",
            "recorder_type": "Practitioner",
            "reaction_substances": ["Peanut", "Peanut oil"],
            "reaction_manifestations": ["Hives", "Vomiting", "Swelling of face"],
            "reaction_descriptions": ["Severe allergic reaction requiring epinephrine"],
            "reaction_onset_dates": ["2020-05-10T09:00:00Z", "2021-12-15T16:30:00Z"],
            "reaction_severities": ["severe", "severe"],
            "reaction_exposure_routes": ["ingestion", "ingestion"],
            "reaction_notes": ["Required emergency treatment", "Patient carries epinephrine auto-injector"],
            "notes": "Severe peanut allergy. Patient must avoid all peanut products and carry epinephrine."
        }"#;

        let allergy: DomainAllergyIntolerance = serde_json::from_str(json).unwrap();
        
        assert_eq!(allergy.allergy_id, "allergy_food_001");
        assert_eq!(allergy.patient_demographic_no, "12345");
        assert_eq!(allergy.clinical_status, Some("active".to_string()));
        assert_eq!(allergy.verification_status, Some("confirmed".to_string()));
        assert_eq!(allergy.allergy_type, Some("allergy".to_string()));
        assert_eq!(allergy.category, Some("food".to_string()));
        assert_eq!(allergy.criticality, Some("high".to_string()));
        assert_eq!(allergy.substance_code, Some("762952008".to_string()));
        assert_eq!(allergy.substance_display, Some("Peanut".to_string()));
        assert_eq!(allergy.substance_system, Some("http://snomed.info/sct".to_string()));
        assert_eq!(allergy.onset_age, Some("2 years".to_string()));
        assert_eq!(allergy.onset_description, Some("Child developed hives and vomiting after eating peanut butter".to_string()));
        assert_eq!(allergy.recorded_date, Some("2020-05-10T09:00:00Z".to_string()));
        assert_eq!(allergy.last_occurrence_date, Some("2021-12-15T16:30:00Z".to_string()));
        assert_eq!(allergy.recorder_id, Some("prac_002".to_string()));
        assert_eq!(allergy.recorder_type, Some("Practitioner".to_string()));
        assert_eq!(allergy.reaction_substances, Some(vec!["Peanut".to_string(), "Peanut oil".to_string()]));
        assert_eq!(allergy.reaction_manifestations, Some(vec!["Hives".to_string(), "Vomiting".to_string(), "Swelling of face".to_string()]));
        assert_eq!(allergy.reaction_descriptions, Some(vec!["Severe allergic reaction requiring epinephrine".to_string()]));
        assert_eq!(allergy.reaction_onset_dates, Some(vec!["2020-05-10T09:00:00Z".to_string(), "2021-12-15T16:30:00Z".to_string()]));
        assert_eq!(allergy.reaction_severities, Some(vec!["severe".to_string(), "severe".to_string()]));
        assert_eq!(allergy.reaction_exposure_routes, Some(vec!["ingestion".to_string(), "ingestion".to_string()]));
        assert_eq!(allergy.reaction_notes, Some(vec!["Required emergency treatment".to_string(), "Patient carries epinephrine auto-injector".to_string()]));
        assert_eq!(allergy.notes, Some("Severe peanut allergy. Patient must avoid all peanut products and carry epinephrine.".to_string()));
    }

    #[test]
    fn test_domain_allergy_intolerance_resolved_allergy() {
        let json = r#"{
            "allergy_id": "allergy_resolved_001",
            "patient_demographic_no": "12345",
            "clinical_status": "resolved",
            "verification_status": "confirmed",
            "allergy_type": "intolerance",
            "category": "medication",
            "criticality": "low",
            "substance_code": "372665000",
            "substance_display": "Aspirin",
            "substance_system": "http://snomed.info/sct",
            "onset_date": "2018-06-01T00:00:00Z",
            "onset_description": "Patient experienced mild stomach upset",
            "recorded_date": "2018-06-05T14:20:00Z",
            "recorder_id": "prac_003",
            "recorder_type": "Practitioner",
            "reaction_substances": ["Aspirin"],
            "reaction_manifestations": ["Stomach upset", "Nausea"],
            "reaction_descriptions": ["Mild gastrointestinal intolerance"],
            "reaction_onset_dates": ["2018-06-01T00:00:00Z"],
            "reaction_severities": ["mild"],
            "reaction_exposure_routes": ["oral"],
            "reaction_notes": ["Patient can tolerate other NSAIDs"],
            "notes": "Patient outgrew aspirin intolerance. No longer relevant."
        }"#;

        let allergy: DomainAllergyIntolerance = serde_json::from_str(json).unwrap();
        
        assert_eq!(allergy.allergy_id, "allergy_resolved_001");
        assert_eq!(allergy.patient_demographic_no, "12345");
        assert_eq!(allergy.clinical_status, Some("resolved".to_string()));
        assert_eq!(allergy.verification_status, Some("confirmed".to_string()));
        assert_eq!(allergy.allergy_type, Some("intolerance".to_string()));
        assert_eq!(allergy.category, Some("medication".to_string()));
        assert_eq!(allergy.criticality, Some("low".to_string()));
        assert_eq!(allergy.substance_code, Some("372665000".to_string()));
        assert_eq!(allergy.substance_display, Some("Aspirin".to_string()));
        assert_eq!(allergy.substance_system, Some("http://snomed.info/sct".to_string()));
        assert_eq!(allergy.onset_date, Some("2018-06-01T00:00:00Z".to_string()));
        assert_eq!(allergy.onset_description, Some("Patient experienced mild stomach upset".to_string()));
        assert_eq!(allergy.recorded_date, Some("2018-06-05T14:20:00Z".to_string()));
        assert_eq!(allergy.recorder_id, Some("prac_003".to_string()));
        assert_eq!(allergy.recorder_type, Some("Practitioner".to_string()));
        assert_eq!(allergy.reaction_substances, Some(vec!["Aspirin".to_string()]));
        assert_eq!(allergy.reaction_manifestations, Some(vec!["Stomach upset".to_string(), "Nausea".to_string()]));
        assert_eq!(allergy.reaction_descriptions, Some(vec!["Mild gastrointestinal intolerance".to_string()]));
        assert_eq!(allergy.reaction_onset_dates, Some(vec!["2018-06-01T00:00:00Z".to_string()]));
        assert_eq!(allergy.reaction_severities, Some(vec!["mild".to_string()]));
        assert_eq!(allergy.reaction_exposure_routes, Some(vec!["oral".to_string()]));
        assert_eq!(allergy.reaction_notes, Some(vec!["Patient can tolerate other NSAIDs".to_string()]));
        assert_eq!(allergy.notes, Some("Patient outgrew aspirin intolerance. No longer relevant.".to_string()));
    }

    #[test]
    fn test_domain_allergy_intolerance_missing_required_field() {
        let json = r#"{
            "recorder_id": "prac_001"
        }"#;

        // This should fail because allergy_id and patient_demographic_no are required
        let result: Result<DomainAllergyIntolerance, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
