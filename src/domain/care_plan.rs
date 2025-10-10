use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainCarePlan {
    pub care_plan_id: String,
    pub patient_demographic_no: String,
    
    // Basic care plan information
    pub status: Option<String>, // "draft" | "active" | "on-hold" | "revoked" | "completed" | "entered-in-error" | "unknown"
    pub intent: Option<String>, // "proposal" | "plan" | "order" | "option" | "directive"
    pub category: Option<String>, // Type of plan (e.g., "diabetes-management", "post-surgical-care")
    pub title: Option<String>, // Human-friendly name for the care plan
    pub description: Option<String>, // Summary of nature of plan
    
    // Temporal information
    pub period_start: Option<String>, // ISO datetime string for plan start
    pub period_end: Option<String>, // ISO datetime string for plan end
    pub created_date: Option<String>, // ISO datetime string when first recorded
    
    // References and relationships
    pub encounter_id: Option<String>, // Encounter during which this CarePlan was created
    pub custodian_id: Option<String>, // Who is the designated responsible party
    pub custodian_type: Option<String>, // Type of custodian (Patient, Practitioner, etc.)
    pub contributor_ids: Option<Vec<String>>, // Who provided the content of the care plan
    pub contributor_types: Option<Vec<String>>, // Types of contributors
    pub care_team_ids: Option<Vec<String>>, // Care team members involved
    pub based_on_ids: Option<Vec<String>>, // References to other care plans or requests
    pub based_on_types: Option<Vec<String>>, // Types of based-on references
    pub replaces_ids: Option<Vec<String>>, // Care plans replaced by this one
    pub part_of_ids: Option<Vec<String>>, // Care plans this is part of
    
    // Clinical information
    pub addresses_codes: Option<Vec<String>>, // Health issues this plan addresses (codes)
    pub addresses_descriptions: Option<Vec<String>>, // Health issues descriptions
    pub supporting_info_ids: Option<Vec<String>>, // Information considered as part of plan
    pub goal_ids: Option<Vec<String>>, // Desired outcomes of plan
    
    // Activities
    pub activity_descriptions: Option<Vec<String>>, // Descriptions of activities
    pub activity_codes: Option<Vec<String>>, // Codes for activities
    pub activity_references: Option<Vec<String>>, // References to planned activities
    pub activity_reference_types: Option<Vec<String>>, // Types of activity references
    pub activity_progress_notes: Option<Vec<String>>, // Progress notes for activities
    
    // Additional information
    pub notes: Option<String>, // Comments about the plan
    pub instantiates_canonical: Option<Vec<String>>, // FHIR protocol or definition references
    pub instantiates_uri: Option<Vec<String>>, // External protocol or definition references
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_care_plan_deserialization() {
        let json = r#"{
            "care_plan_id": "cp_12345",
            "patient_demographic_no": "12345",
            "status": "active",
            "intent": "plan",
            "category": "diabetes-management",
            "title": "Diabetes Management Plan",
            "description": "Comprehensive care plan for type 2 diabetes management",
            "period_start": "2024-01-01T00:00:00Z",
            "period_end": "2024-12-31T23:59:59Z",
            "created_date": "2024-01-01T10:00:00Z",
            "encounter_id": "enc_001",
            "custodian_id": "prac_001",
            "custodian_type": "Practitioner",
            "contributor_ids": ["prac_002", "nurse_001"],
            "contributor_types": ["Practitioner", "Practitioner"],
            "care_team_ids": ["team_001", "team_002"],
            "based_on_ids": ["cp_00001"],
            "based_on_types": ["CarePlan"],
            "replaces_ids": [],
            "part_of_ids": [],
            "addresses_codes": ["E11.9", "I10"],
            "addresses_descriptions": ["Type 2 diabetes mellitus without complications", "Essential hypertension"],
            "supporting_info_ids": ["obs_001", "obs_002"],
            "goal_ids": ["goal_001", "goal_002"],
            "activity_descriptions": ["Blood glucose monitoring", "Medication adherence", "Dietary counseling"],
            "activity_codes": ["glucose-monitoring", "medication-adherence", "dietary-counseling"],
            "activity_references": ["apt_001", "med_001", "task_001"],
            "activity_reference_types": ["Appointment", "MedicationRequest", "Task"],
            "activity_progress_notes": ["Patient monitoring glucose daily", "Medications taken as prescribed", "Following dietary guidelines"],
            "notes": "Patient is motivated and engaged in self-care. Regular follow-up scheduled.",
            "instantiates_canonical": ["http://example.org/fhir/PlanDefinition/diabetes-management"],
            "instantiates_uri": ["https://example.org/protocols/diabetes-care"]
        }"#;

        let care_plan: DomainCarePlan = serde_json::from_str(json).unwrap();
        
        assert_eq!(care_plan.care_plan_id, "cp_12345");
        assert_eq!(care_plan.patient_demographic_no, "12345");
        assert_eq!(care_plan.status, Some("active".to_string()));
        assert_eq!(care_plan.intent, Some("plan".to_string()));
        assert_eq!(care_plan.category, Some("diabetes-management".to_string()));
        assert_eq!(care_plan.title, Some("Diabetes Management Plan".to_string()));
        assert_eq!(care_plan.description, Some("Comprehensive care plan for type 2 diabetes management".to_string()));
        assert_eq!(care_plan.period_start, Some("2024-01-01T00:00:00Z".to_string()));
        assert_eq!(care_plan.period_end, Some("2024-12-31T23:59:59Z".to_string()));
        assert_eq!(care_plan.created_date, Some("2024-01-01T10:00:00Z".to_string()));
        assert_eq!(care_plan.encounter_id, Some("enc_001".to_string()));
        assert_eq!(care_plan.custodian_id, Some("prac_001".to_string()));
        assert_eq!(care_plan.custodian_type, Some("Practitioner".to_string()));
        assert_eq!(care_plan.contributor_ids, Some(vec!["prac_002".to_string(), "nurse_001".to_string()]));
        assert_eq!(care_plan.contributor_types, Some(vec!["Practitioner".to_string(), "Practitioner".to_string()]));
        assert_eq!(care_plan.care_team_ids, Some(vec!["team_001".to_string(), "team_002".to_string()]));
        assert_eq!(care_plan.based_on_ids, Some(vec!["cp_00001".to_string()]));
        assert_eq!(care_plan.based_on_types, Some(vec!["CarePlan".to_string()]));
        assert_eq!(care_plan.replaces_ids, Some(vec![]));
        assert_eq!(care_plan.part_of_ids, Some(vec![]));
        assert_eq!(care_plan.addresses_codes, Some(vec!["E11.9".to_string(), "I10".to_string()]));
        assert_eq!(care_plan.addresses_descriptions, Some(vec!["Type 2 diabetes mellitus without complications".to_string(), "Essential hypertension".to_string()]));
        assert_eq!(care_plan.supporting_info_ids, Some(vec!["obs_001".to_string(), "obs_002".to_string()]));
        assert_eq!(care_plan.goal_ids, Some(vec!["goal_001".to_string(), "goal_002".to_string()]));
        assert_eq!(care_plan.activity_descriptions, Some(vec!["Blood glucose monitoring".to_string(), "Medication adherence".to_string(), "Dietary counseling".to_string()]));
        assert_eq!(care_plan.activity_codes, Some(vec!["glucose-monitoring".to_string(), "medication-adherence".to_string(), "dietary-counseling".to_string()]));
        assert_eq!(care_plan.activity_references, Some(vec!["apt_001".to_string(), "med_001".to_string(), "task_001".to_string()]));
        assert_eq!(care_plan.activity_reference_types, Some(vec!["Appointment".to_string(), "MedicationRequest".to_string(), "Task".to_string()]));
        assert_eq!(care_plan.activity_progress_notes, Some(vec!["Patient monitoring glucose daily".to_string(), "Medications taken as prescribed".to_string(), "Following dietary guidelines".to_string()]));
        assert_eq!(care_plan.notes, Some("Patient is motivated and engaged in self-care. Regular follow-up scheduled.".to_string()));
        assert_eq!(care_plan.instantiates_canonical, Some(vec!["http://example.org/fhir/PlanDefinition/diabetes-management".to_string()]));
        assert_eq!(care_plan.instantiates_uri, Some(vec!["https://example.org/protocols/diabetes-care".to_string()]));
    }

    #[test]
    fn test_domain_care_plan_minimal_deserialization() {
        let json = r#"{
            "care_plan_id": "cp_67890",
            "patient_demographic_no": "67890"
        }"#;

        let care_plan: DomainCarePlan = serde_json::from_str(json).unwrap();
        
        assert_eq!(care_plan.care_plan_id, "cp_67890");
        assert_eq!(care_plan.patient_demographic_no, "67890");
        assert_eq!(care_plan.status, None);
        assert_eq!(care_plan.intent, None);
        assert_eq!(care_plan.category, None);
        assert_eq!(care_plan.title, None);
        assert_eq!(care_plan.description, None);
        assert_eq!(care_plan.period_start, None);
        assert_eq!(care_plan.period_end, None);
        assert_eq!(care_plan.created_date, None);
        assert_eq!(care_plan.encounter_id, None);
        assert_eq!(care_plan.custodian_id, None);
        assert_eq!(care_plan.custodian_type, None);
        assert_eq!(care_plan.contributor_ids, None);
        assert_eq!(care_plan.contributor_types, None);
        assert_eq!(care_plan.care_team_ids, None);
        assert_eq!(care_plan.based_on_ids, None);
        assert_eq!(care_plan.based_on_types, None);
        assert_eq!(care_plan.replaces_ids, None);
        assert_eq!(care_plan.part_of_ids, None);
        assert_eq!(care_plan.addresses_codes, None);
        assert_eq!(care_plan.addresses_descriptions, None);
        assert_eq!(care_plan.supporting_info_ids, None);
        assert_eq!(care_plan.goal_ids, None);
        assert_eq!(care_plan.activity_descriptions, None);
        assert_eq!(care_plan.activity_codes, None);
        assert_eq!(care_plan.activity_references, None);
        assert_eq!(care_plan.activity_reference_types, None);
        assert_eq!(care_plan.activity_progress_notes, None);
        assert_eq!(care_plan.notes, None);
        assert_eq!(care_plan.instantiates_canonical, None);
        assert_eq!(care_plan.instantiates_uri, None);
    }

    #[test]
    fn test_domain_care_plan_post_surgical_care() {
        let json = r#"{
            "care_plan_id": "cp_surgical_001",
            "patient_demographic_no": "12345",
            "status": "active",
            "intent": "order",
            "category": "post-surgical-care",
            "title": "Post-Surgical Recovery Plan",
            "description": "Comprehensive recovery plan following appendectomy",
            "period_start": "2024-02-15T00:00:00Z",
            "period_end": "2024-03-15T23:59:59Z",
            "created_date": "2024-02-15T14:30:00Z",
            "encounter_id": "enc_surgical_001",
            "custodian_id": "prac_surgeon_001",
            "custodian_type": "Practitioner",
            "contributor_ids": ["nurse_001", "pt_001"],
            "contributor_types": ["Practitioner", "Practitioner"],
            "care_team_ids": ["team_surgical_001"],
            "addresses_codes": ["K35.9"],
            "addresses_descriptions": ["Acute appendicitis, unspecified"],
            "supporting_info_ids": ["proc_001", "obs_surgical_001"],
            "goal_ids": ["goal_recovery_001", "goal_mobility_001"],
            "activity_descriptions": ["Pain management", "Wound care", "Physical therapy", "Follow-up appointments"],
            "activity_codes": ["pain-management", "wound-care", "physical-therapy", "follow-up"],
            "activity_references": ["med_pain_001", "task_wound_001", "apt_pt_001", "apt_followup_001"],
            "activity_reference_types": ["MedicationRequest", "Task", "Appointment", "Appointment"],
            "activity_progress_notes": ["Pain well controlled", "Wound healing normally", "PT sessions going well", "Follow-up scheduled"],
            "notes": "Patient recovering well. No complications noted. Continue current plan.",
            "instantiates_canonical": ["http://example.org/fhir/PlanDefinition/post-surgical-care"]
        }"#;

        let care_plan: DomainCarePlan = serde_json::from_str(json).unwrap();
        
        assert_eq!(care_plan.care_plan_id, "cp_surgical_001");
        assert_eq!(care_plan.patient_demographic_no, "12345");
        assert_eq!(care_plan.status, Some("active".to_string()));
        assert_eq!(care_plan.intent, Some("order".to_string()));
        assert_eq!(care_plan.category, Some("post-surgical-care".to_string()));
        assert_eq!(care_plan.title, Some("Post-Surgical Recovery Plan".to_string()));
        assert_eq!(care_plan.description, Some("Comprehensive recovery plan following appendectomy".to_string()));
        assert_eq!(care_plan.period_start, Some("2024-02-15T00:00:00Z".to_string()));
        assert_eq!(care_plan.period_end, Some("2024-03-15T23:59:59Z".to_string()));
        assert_eq!(care_plan.created_date, Some("2024-02-15T14:30:00Z".to_string()));
        assert_eq!(care_plan.encounter_id, Some("enc_surgical_001".to_string()));
        assert_eq!(care_plan.custodian_id, Some("prac_surgeon_001".to_string()));
        assert_eq!(care_plan.custodian_type, Some("Practitioner".to_string()));
        assert_eq!(care_plan.contributor_ids, Some(vec!["nurse_001".to_string(), "pt_001".to_string()]));
        assert_eq!(care_plan.contributor_types, Some(vec!["Practitioner".to_string(), "Practitioner".to_string()]));
        assert_eq!(care_plan.care_team_ids, Some(vec!["team_surgical_001".to_string()]));
        assert_eq!(care_plan.addresses_codes, Some(vec!["K35.9".to_string()]));
        assert_eq!(care_plan.addresses_descriptions, Some(vec!["Acute appendicitis, unspecified".to_string()]));
        assert_eq!(care_plan.supporting_info_ids, Some(vec!["proc_001".to_string(), "obs_surgical_001".to_string()]));
        assert_eq!(care_plan.goal_ids, Some(vec!["goal_recovery_001".to_string(), "goal_mobility_001".to_string()]));
        assert_eq!(care_plan.activity_descriptions, Some(vec!["Pain management".to_string(), "Wound care".to_string(), "Physical therapy".to_string(), "Follow-up appointments".to_string()]));
        assert_eq!(care_plan.activity_codes, Some(vec!["pain-management".to_string(), "wound-care".to_string(), "physical-therapy".to_string(), "follow-up".to_string()]));
        assert_eq!(care_plan.activity_references, Some(vec!["med_pain_001".to_string(), "task_wound_001".to_string(), "apt_pt_001".to_string(), "apt_followup_001".to_string()]));
        assert_eq!(care_plan.activity_reference_types, Some(vec!["MedicationRequest".to_string(), "Task".to_string(), "Appointment".to_string(), "Appointment".to_string()]));
        assert_eq!(care_plan.activity_progress_notes, Some(vec!["Pain well controlled".to_string(), "Wound healing normally".to_string(), "PT sessions going well".to_string(), "Follow-up scheduled".to_string()]));
        assert_eq!(care_plan.notes, Some("Patient recovering well. No complications noted. Continue current plan.".to_string()));
        assert_eq!(care_plan.instantiates_canonical, Some(vec!["http://example.org/fhir/PlanDefinition/post-surgical-care".to_string()]));
    }

    #[test]
    fn test_domain_care_plan_palliative_care() {
        let json = r#"{
            "care_plan_id": "cp_palliative_001",
            "patient_demographic_no": "12345",
            "status": "active",
            "intent": "plan",
            "category": "palliative-care",
            "title": "Palliative Care Plan",
            "description": "Comprehensive palliative care plan for end-of-life comfort",
            "period_start": "2024-03-01T00:00:00Z",
            "period_end": null,
            "created_date": "2024-03-01T09:00:00Z",
            "custodian_id": "prac_palliative_001",
            "custodian_type": "Practitioner",
            "contributor_ids": ["nurse_palliative_001", "social_worker_001"],
            "contributor_types": ["Practitioner", "Practitioner"],
            "care_team_ids": ["team_palliative_001"],
            "addresses_codes": ["C78.00"],
            "addresses_descriptions": ["Secondary malignant neoplasm of unspecified lung"],
            "supporting_info_ids": ["obs_pain_001", "obs_quality_001"],
            "goal_ids": ["goal_comfort_001", "goal_quality_001"],
            "activity_descriptions": ["Pain management", "Symptom control", "Family support", "Spiritual care"],
            "activity_codes": ["pain-management", "symptom-control", "family-support", "spiritual-care"],
            "activity_references": ["med_pain_002", "task_symptoms_001", "task_family_001", "task_spiritual_001"],
            "activity_reference_types": ["MedicationRequest", "Task", "Task", "Task"],
            "activity_progress_notes": ["Pain well managed", "Symptoms controlled", "Family meetings scheduled", "Spiritual needs addressed"],
            "notes": "Patient and family comfortable with care plan. Focus on comfort and quality of life.",
            "instantiates_canonical": ["http://example.org/fhir/PlanDefinition/palliative-care"]
        }"#;

        let care_plan: DomainCarePlan = serde_json::from_str(json).unwrap();
        
        assert_eq!(care_plan.care_plan_id, "cp_palliative_001");
        assert_eq!(care_plan.patient_demographic_no, "12345");
        assert_eq!(care_plan.status, Some("active".to_string()));
        assert_eq!(care_plan.intent, Some("plan".to_string()));
        assert_eq!(care_plan.category, Some("palliative-care".to_string()));
        assert_eq!(care_plan.title, Some("Palliative Care Plan".to_string()));
        assert_eq!(care_plan.description, Some("Comprehensive palliative care plan for end-of-life comfort".to_string()));
        assert_eq!(care_plan.period_start, Some("2024-03-01T00:00:00Z".to_string()));
        assert_eq!(care_plan.period_end, None);
        assert_eq!(care_plan.created_date, Some("2024-03-01T09:00:00Z".to_string()));
        assert_eq!(care_plan.custodian_id, Some("prac_palliative_001".to_string()));
        assert_eq!(care_plan.custodian_type, Some("Practitioner".to_string()));
        assert_eq!(care_plan.contributor_ids, Some(vec!["nurse_palliative_001".to_string(), "social_worker_001".to_string()]));
        assert_eq!(care_plan.contributor_types, Some(vec!["Practitioner".to_string(), "Practitioner".to_string()]));
        assert_eq!(care_plan.care_team_ids, Some(vec!["team_palliative_001".to_string()]));
        assert_eq!(care_plan.addresses_codes, Some(vec!["C78.00".to_string()]));
        assert_eq!(care_plan.addresses_descriptions, Some(vec!["Secondary malignant neoplasm of unspecified lung".to_string()]));
        assert_eq!(care_plan.supporting_info_ids, Some(vec!["obs_pain_001".to_string(), "obs_quality_001".to_string()]));
        assert_eq!(care_plan.goal_ids, Some(vec!["goal_comfort_001".to_string(), "goal_quality_001".to_string()]));
        assert_eq!(care_plan.activity_descriptions, Some(vec!["Pain management".to_string(), "Symptom control".to_string(), "Family support".to_string(), "Spiritual care".to_string()]));
        assert_eq!(care_plan.activity_codes, Some(vec!["pain-management".to_string(), "symptom-control".to_string(), "family-support".to_string(), "spiritual-care".to_string()]));
        assert_eq!(care_plan.activity_references, Some(vec!["med_pain_002".to_string(), "task_symptoms_001".to_string(), "task_family_001".to_string(), "task_spiritual_001".to_string()]));
        assert_eq!(care_plan.activity_reference_types, Some(vec!["MedicationRequest".to_string(), "Task".to_string(), "Task".to_string(), "Task".to_string()]));
        assert_eq!(care_plan.activity_progress_notes, Some(vec!["Pain well managed".to_string(), "Symptoms controlled".to_string(), "Family meetings scheduled".to_string(), "Spiritual needs addressed".to_string()]));
        assert_eq!(care_plan.notes, Some("Patient and family comfortable with care plan. Focus on comfort and quality of life.".to_string()));
        assert_eq!(care_plan.instantiates_canonical, Some(vec!["http://example.org/fhir/PlanDefinition/palliative-care".to_string()]));
    }

    #[test]
    fn test_domain_care_plan_missing_required_field() {
        let json = r#"{
            "custodian_id": "prac_001"
        }"#;

        // This should fail because care_plan_id and patient_demographic_no are required
        let result: Result<DomainCarePlan, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
