use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainDiagnosticReport {
    pub diagnostic_report_id: String,
    pub patient_demographic_no: String,
    
    // Basic diagnostic report information
    pub status: Option<String>, // "registered" | "partial" | "preliminary" | "modified" | "final" | "amended" | "corrected" | "appended" | "cancelled" | "entered-in-error" | "unknown"
    pub category: Option<String>, // Service category
    pub category_code: Option<String>, // Code for category
    pub category_system: Option<String>, // Terminology system for category
    pub category_display: Option<String>, // Display name for category
    pub code: Option<String>, // Name/Code for this diagnostic report
    pub code_code: Option<String>, // Code for the diagnostic report
    pub code_system: Option<String>, // Terminology system for code
    pub code_display: Option<String>, // Display name for code
    
    // Temporal information
    pub effective_date: Option<String>, // ISO datetime string for report time
    pub effective_period_start: Option<String>, // ISO datetime string for report period start
    pub effective_period_end: Option<String>, // ISO datetime string for report period end
    pub issued_date: Option<String>, // ISO datetime string when report was issued
    
    // References and relationships
    pub encounter_id: Option<String>, // Health care event when test ordered
    pub performer_ids: Option<Vec<String>>, // Responsible Diagnostic Service
    pub performer_types: Option<Vec<String>>, // Types of performers
    pub results_interpreter_ids: Option<Vec<String>>, // Primary result interpreter
    pub results_interpreter_types: Option<Vec<String>>, // Types of interpreters
    pub based_on_ids: Option<Vec<String>>, // What was requested
    pub based_on_types: Option<Vec<String>>, // Types of based on references
    
    // Specimens and results
    pub specimen_ids: Option<Vec<String>>, // Specimens this report is based on
    pub result_observation_ids: Option<Vec<String>>, // Observations
    pub study_ids: Option<Vec<String>>, // Reference to full details of an analysis
    pub study_types: Option<Vec<String>>, // Types of studies (GenomicStudy, ImagingStudy)
    
    // Supporting information
    pub supporting_info_types: Option<Vec<String>>, // Supporting information role codes
    pub supporting_info_type_codes: Option<Vec<String>>, // Codes for supporting info types
    pub supporting_info_type_systems: Option<Vec<String>>, // Terminology systems for types
    pub supporting_info_type_displays: Option<Vec<String>>, // Display names for types
    pub supporting_info_reference_ids: Option<Vec<String>>, // Supporting information references
    pub supporting_info_reference_types: Option<Vec<String>>, // Types of supporting info references
    
    // Media and attachments
    pub media_comments: Option<Vec<String>>, // Comments about the image or data
    pub media_link_ids: Option<Vec<String>>, // Reference to the image or data source
    pub presented_form_ids: Option<Vec<String>>, // Entire report as issued
    
    // Clinical information
    pub conclusion: Option<String>, // Clinical conclusion (interpretation) of test results
    pub conclusion_codes: Option<Vec<String>>, // Codes for the clinical conclusion
    pub conclusion_code_codes: Option<Vec<String>>, // Codes for conclusion codes
    pub conclusion_code_systems: Option<Vec<String>>, // Terminology systems for conclusion codes
    pub conclusion_code_displays: Option<Vec<String>>, // Display names for conclusion codes
    pub composition_id: Option<String>, // Reference to a Composition resource
    
    // Additional information
    pub notes: Option<String>, // Comments about the diagnostic report
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_diagnostic_report_deserialization() {
        let json = r#"{
            "diagnostic_report_id": "dr_12345",
            "patient_demographic_no": "12345",
            "status": "final",
            "category": "laboratory",
            "category_code": "LAB",
            "category_system": "http://terminology.hl7.org/CodeSystem/v2-0074",
            "category_display": "Laboratory",
            "code": "complete-blood-count",
            "code_code": "CBC",
            "code_system": "http://loinc.org",
            "code_display": "Complete Blood Count",
            "effective_date": "2024-01-15T10:30:00Z",
            "issued_date": "2024-01-15T11:00:00Z",
            "encounter_id": "enc_001",
            "performer_ids": ["prac_001", "lab_001"],
            "performer_types": ["Practitioner", "Organization"],
            "results_interpreter_ids": ["prac_002"],
            "results_interpreter_types": ["Practitioner"],
            "based_on_ids": ["sr_001"],
            "based_on_types": ["ServiceRequest"],
            "specimen_ids": ["spec_001", "spec_002"],
            "result_observation_ids": ["obs_001", "obs_002", "obs_003"],
            "study_ids": ["study_001"],
            "study_types": ["ImagingStudy"],
            "supporting_info_types": ["procedure", "observation"],
            "supporting_info_type_codes": ["procedure", "observation"],
            "supporting_info_type_systems": ["http://terminology.hl7.org/CodeSystem/v2-0074", "http://terminology.hl7.org/CodeSystem/v2-0074"],
            "supporting_info_type_displays": ["Procedure", "Observation"],
            "supporting_info_reference_ids": ["proc_001", "obs_004"],
            "supporting_info_reference_types": ["Procedure", "Observation"],
            "media_comments": ["Blood smear image", "Cell morphology"],
            "media_link_ids": ["doc_001", "doc_002"],
            "presented_form_ids": ["att_001"],
            "conclusion": "Complete blood count shows mild anemia with low hemoglobin and hematocrit. White blood cell count and platelet count are within normal limits.",
            "conclusion_codes": ["anemia", "low-hemoglobin"],
            "conclusion_code_codes": ["D64.9", "R71"],
            "conclusion_code_systems": ["http://hl7.org/fhir/sid/icd-10-cm", "http://hl7.org/fhir/sid/icd-10-cm"],
            "conclusion_code_displays": ["Anemia, unspecified", "Abnormality of red blood cells"],
            "composition_id": "comp_001",
            "notes": "Patient should follow up with primary care physician for anemia workup."
        }"#;

        let diagnostic_report: DomainDiagnosticReport = serde_json::from_str(json).unwrap();
        
        assert_eq!(diagnostic_report.diagnostic_report_id, "dr_12345");
        assert_eq!(diagnostic_report.patient_demographic_no, "12345");
        assert_eq!(diagnostic_report.status, Some("final".to_string()));
        assert_eq!(diagnostic_report.category, Some("laboratory".to_string()));
        assert_eq!(diagnostic_report.category_code, Some("LAB".to_string()));
        assert_eq!(diagnostic_report.category_system, Some("http://terminology.hl7.org/CodeSystem/v2-0074".to_string()));
        assert_eq!(diagnostic_report.category_display, Some("Laboratory".to_string()));
        assert_eq!(diagnostic_report.code, Some("complete-blood-count".to_string()));
        assert_eq!(diagnostic_report.code_code, Some("CBC".to_string()));
        assert_eq!(diagnostic_report.code_system, Some("http://loinc.org".to_string()));
        assert_eq!(diagnostic_report.code_display, Some("Complete Blood Count".to_string()));
        assert_eq!(diagnostic_report.effective_date, Some("2024-01-15T10:30:00Z".to_string()));
        assert_eq!(diagnostic_report.issued_date, Some("2024-01-15T11:00:00Z".to_string()));
        assert_eq!(diagnostic_report.encounter_id, Some("enc_001".to_string()));
        assert_eq!(diagnostic_report.performer_ids, Some(vec!["prac_001".to_string(), "lab_001".to_string()]));
        assert_eq!(diagnostic_report.performer_types, Some(vec!["Practitioner".to_string(), "Organization".to_string()]));
        assert_eq!(diagnostic_report.results_interpreter_ids, Some(vec!["prac_002".to_string()]));
        assert_eq!(diagnostic_report.results_interpreter_types, Some(vec!["Practitioner".to_string()]));
        assert_eq!(diagnostic_report.based_on_ids, Some(vec!["sr_001".to_string()]));
        assert_eq!(diagnostic_report.based_on_types, Some(vec!["ServiceRequest".to_string()]));
        assert_eq!(diagnostic_report.specimen_ids, Some(vec!["spec_001".to_string(), "spec_002".to_string()]));
        assert_eq!(diagnostic_report.result_observation_ids, Some(vec!["obs_001".to_string(), "obs_002".to_string(), "obs_003".to_string()]));
        assert_eq!(diagnostic_report.study_ids, Some(vec!["study_001".to_string()]));
        assert_eq!(diagnostic_report.study_types, Some(vec!["ImagingStudy".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_types, Some(vec!["procedure".to_string(), "observation".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_type_codes, Some(vec!["procedure".to_string(), "observation".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_type_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/v2-0074".to_string(), "http://terminology.hl7.org/CodeSystem/v2-0074".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_type_displays, Some(vec!["Procedure".to_string(), "Observation".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_reference_ids, Some(vec!["proc_001".to_string(), "obs_004".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_reference_types, Some(vec!["Procedure".to_string(), "Observation".to_string()]));
        assert_eq!(diagnostic_report.media_comments, Some(vec!["Blood smear image".to_string(), "Cell morphology".to_string()]));
        assert_eq!(diagnostic_report.media_link_ids, Some(vec!["doc_001".to_string(), "doc_002".to_string()]));
        assert_eq!(diagnostic_report.presented_form_ids, Some(vec!["att_001".to_string()]));
        assert_eq!(diagnostic_report.conclusion, Some("Complete blood count shows mild anemia with low hemoglobin and hematocrit. White blood cell count and platelet count are within normal limits.".to_string()));
        assert_eq!(diagnostic_report.conclusion_codes, Some(vec!["anemia".to_string(), "low-hemoglobin".to_string()]));
        assert_eq!(diagnostic_report.conclusion_code_codes, Some(vec!["D64.9".to_string(), "R71".to_string()]));
        assert_eq!(diagnostic_report.conclusion_code_systems, Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]));
        assert_eq!(diagnostic_report.conclusion_code_displays, Some(vec!["Anemia, unspecified".to_string(), "Abnormality of red blood cells".to_string()]));
        assert_eq!(diagnostic_report.composition_id, Some("comp_001".to_string()));
        assert_eq!(diagnostic_report.notes, Some("Patient should follow up with primary care physician for anemia workup.".to_string()));
    }

    #[test]
    fn test_domain_diagnostic_report_minimal_deserialization() {
        let json = r#"{
            "diagnostic_report_id": "dr_67890",
            "patient_demographic_no": "67890"
        }"#;

        let diagnostic_report: DomainDiagnosticReport = serde_json::from_str(json).unwrap();
        
        assert_eq!(diagnostic_report.diagnostic_report_id, "dr_67890");
        assert_eq!(diagnostic_report.patient_demographic_no, "67890");
        assert_eq!(diagnostic_report.status, None);
        assert_eq!(diagnostic_report.category, None);
        assert_eq!(diagnostic_report.category_code, None);
        assert_eq!(diagnostic_report.category_system, None);
        assert_eq!(diagnostic_report.category_display, None);
        assert_eq!(diagnostic_report.code, None);
        assert_eq!(diagnostic_report.code_code, None);
        assert_eq!(diagnostic_report.code_system, None);
        assert_eq!(diagnostic_report.code_display, None);
        assert_eq!(diagnostic_report.effective_date, None);
        assert_eq!(diagnostic_report.effective_period_start, None);
        assert_eq!(diagnostic_report.effective_period_end, None);
        assert_eq!(diagnostic_report.issued_date, None);
        assert_eq!(diagnostic_report.encounter_id, None);
        assert_eq!(diagnostic_report.performer_ids, None);
        assert_eq!(diagnostic_report.performer_types, None);
        assert_eq!(diagnostic_report.results_interpreter_ids, None);
        assert_eq!(diagnostic_report.results_interpreter_types, None);
        assert_eq!(diagnostic_report.based_on_ids, None);
        assert_eq!(diagnostic_report.based_on_types, None);
        assert_eq!(diagnostic_report.specimen_ids, None);
        assert_eq!(diagnostic_report.result_observation_ids, None);
        assert_eq!(diagnostic_report.study_ids, None);
        assert_eq!(diagnostic_report.study_types, None);
        assert_eq!(diagnostic_report.supporting_info_types, None);
        assert_eq!(diagnostic_report.supporting_info_type_codes, None);
        assert_eq!(diagnostic_report.supporting_info_type_systems, None);
        assert_eq!(diagnostic_report.supporting_info_type_displays, None);
        assert_eq!(diagnostic_report.supporting_info_reference_ids, None);
        assert_eq!(diagnostic_report.supporting_info_reference_types, None);
        assert_eq!(diagnostic_report.media_comments, None);
        assert_eq!(diagnostic_report.media_link_ids, None);
        assert_eq!(diagnostic_report.presented_form_ids, None);
        assert_eq!(diagnostic_report.conclusion, None);
        assert_eq!(diagnostic_report.conclusion_codes, None);
        assert_eq!(diagnostic_report.conclusion_code_codes, None);
        assert_eq!(diagnostic_report.conclusion_code_systems, None);
        assert_eq!(diagnostic_report.conclusion_code_displays, None);
        assert_eq!(diagnostic_report.composition_id, None);
        assert_eq!(diagnostic_report.notes, None);
    }

    #[test]
    fn test_domain_diagnostic_report_radiology() {
        let json = r#"{
            "diagnostic_report_id": "dr_rad_001",
            "patient_demographic_no": "12345",
            "status": "final",
            "category": "radiology",
            "category_code": "RAD",
            "category_system": "http://terminology.hl7.org/CodeSystem/v2-0074",
            "category_display": "Radiology",
            "code": "chest-x-ray",
            "code_code": "CXR",
            "code_system": "http://loinc.org",
            "code_display": "Chest X-ray",
            "effective_date": "2024-02-01T14:00:00Z",
            "issued_date": "2024-02-01T15:30:00Z",
            "encounter_id": "enc_002",
            "performer_ids": ["prac_rad_001", "org_rad_001"],
            "performer_types": ["Practitioner", "Organization"],
            "results_interpreter_ids": ["prac_rad_002"],
            "results_interpreter_types": ["Practitioner"],
            "based_on_ids": ["sr_002"],
            "based_on_types": ["ServiceRequest"],
            "study_ids": ["study_rad_001"],
            "study_types": ["ImagingStudy"],
            "supporting_info_types": ["procedure"],
            "supporting_info_type_codes": ["procedure"],
            "supporting_info_type_systems": ["http://terminology.hl7.org/CodeSystem/v2-0074"],
            "supporting_info_type_displays": ["Procedure"],
            "supporting_info_reference_ids": ["proc_rad_001"],
            "supporting_info_reference_types": ["Procedure"],
            "media_comments": ["PA chest X-ray", "Lateral chest X-ray"],
            "media_link_ids": ["doc_rad_001", "doc_rad_002"],
            "presented_form_ids": ["att_rad_001"],
            "conclusion": "Chest X-ray shows clear lung fields bilaterally. No acute cardiopulmonary process. Heart size normal.",
            "conclusion_codes": ["normal-chest-xray"],
            "conclusion_code_codes": ["Z01.89"],
            "conclusion_code_systems": ["http://hl7.org/fhir/sid/icd-10-cm"],
            "conclusion_code_displays": ["Encounter for other specified special examination"],
            "composition_id": "comp_rad_001",
            "notes": "Routine chest X-ray for pre-operative evaluation."
        }"#;

        let diagnostic_report: DomainDiagnosticReport = serde_json::from_str(json).unwrap();
        
        assert_eq!(diagnostic_report.diagnostic_report_id, "dr_rad_001");
        assert_eq!(diagnostic_report.patient_demographic_no, "12345");
        assert_eq!(diagnostic_report.status, Some("final".to_string()));
        assert_eq!(diagnostic_report.category, Some("radiology".to_string()));
        assert_eq!(diagnostic_report.category_code, Some("RAD".to_string()));
        assert_eq!(diagnostic_report.category_system, Some("http://terminology.hl7.org/CodeSystem/v2-0074".to_string()));
        assert_eq!(diagnostic_report.category_display, Some("Radiology".to_string()));
        assert_eq!(diagnostic_report.code, Some("chest-x-ray".to_string()));
        assert_eq!(diagnostic_report.code_code, Some("CXR".to_string()));
        assert_eq!(diagnostic_report.code_system, Some("http://loinc.org".to_string()));
        assert_eq!(diagnostic_report.code_display, Some("Chest X-ray".to_string()));
        assert_eq!(diagnostic_report.effective_date, Some("2024-02-01T14:00:00Z".to_string()));
        assert_eq!(diagnostic_report.issued_date, Some("2024-02-01T15:30:00Z".to_string()));
        assert_eq!(diagnostic_report.encounter_id, Some("enc_002".to_string()));
        assert_eq!(diagnostic_report.performer_ids, Some(vec!["prac_rad_001".to_string(), "org_rad_001".to_string()]));
        assert_eq!(diagnostic_report.performer_types, Some(vec!["Practitioner".to_string(), "Organization".to_string()]));
        assert_eq!(diagnostic_report.results_interpreter_ids, Some(vec!["prac_rad_002".to_string()]));
        assert_eq!(diagnostic_report.results_interpreter_types, Some(vec!["Practitioner".to_string()]));
        assert_eq!(diagnostic_report.based_on_ids, Some(vec!["sr_002".to_string()]));
        assert_eq!(diagnostic_report.based_on_types, Some(vec!["ServiceRequest".to_string()]));
        assert_eq!(diagnostic_report.study_ids, Some(vec!["study_rad_001".to_string()]));
        assert_eq!(diagnostic_report.study_types, Some(vec!["ImagingStudy".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_types, Some(vec!["procedure".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_type_codes, Some(vec!["procedure".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_type_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/v2-0074".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_type_displays, Some(vec!["Procedure".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_reference_ids, Some(vec!["proc_rad_001".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_reference_types, Some(vec!["Procedure".to_string()]));
        assert_eq!(diagnostic_report.media_comments, Some(vec!["PA chest X-ray".to_string(), "Lateral chest X-ray".to_string()]));
        assert_eq!(diagnostic_report.media_link_ids, Some(vec!["doc_rad_001".to_string(), "doc_rad_002".to_string()]));
        assert_eq!(diagnostic_report.presented_form_ids, Some(vec!["att_rad_001".to_string()]));
        assert_eq!(diagnostic_report.conclusion, Some("Chest X-ray shows clear lung fields bilaterally. No acute cardiopulmonary process. Heart size normal.".to_string()));
        assert_eq!(diagnostic_report.conclusion_codes, Some(vec!["normal-chest-xray".to_string()]));
        assert_eq!(diagnostic_report.conclusion_code_codes, Some(vec!["Z01.89".to_string()]));
        assert_eq!(diagnostic_report.conclusion_code_systems, Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string()]));
        assert_eq!(diagnostic_report.conclusion_code_displays, Some(vec!["Encounter for other specified special examination".to_string()]));
        assert_eq!(diagnostic_report.composition_id, Some("comp_rad_001".to_string()));
        assert_eq!(diagnostic_report.notes, Some("Routine chest X-ray for pre-operative evaluation.".to_string()));
    }

    #[test]
    fn test_domain_diagnostic_report_pathology() {
        let json = r#"{
            "diagnostic_report_id": "dr_path_001",
            "patient_demographic_no": "12345",
            "status": "final",
            "category": "pathology",
            "category_code": "PATH",
            "category_system": "http://terminology.hl7.org/CodeSystem/v2-0074",
            "category_display": "Pathology",
            "code": "biopsy-report",
            "code_code": "BIOPSY",
            "code_system": "http://loinc.org",
            "code_display": "Biopsy Report",
            "effective_date": "2024-03-01T09:00:00Z",
            "issued_date": "2024-03-01T16:00:00Z",
            "encounter_id": "enc_003",
            "performer_ids": ["prac_path_001"],
            "performer_types": ["Practitioner"],
            "results_interpreter_ids": ["prac_path_002"],
            "results_interpreter_types": ["Practitioner"],
            "based_on_ids": ["sr_003"],
            "based_on_types": ["ServiceRequest"],
            "specimen_ids": ["spec_path_001"],
            "result_observation_ids": ["obs_path_001", "obs_path_002"],
            "study_ids": ["study_path_001"],
            "study_types": ["GenomicStudy"],
            "supporting_info_types": ["procedure", "observation"],
            "supporting_info_type_codes": ["procedure", "observation"],
            "supporting_info_type_systems": ["http://terminology.hl7.org/CodeSystem/v2-0074", "http://terminology.hl7.org/CodeSystem/v2-0074"],
            "supporting_info_type_displays": ["Procedure", "Observation"],
            "supporting_info_reference_ids": ["proc_path_001", "obs_path_003"],
            "supporting_info_reference_types": ["Procedure", "Observation"],
            "media_comments": ["H&E stain", "Immunohistochemistry"],
            "media_link_ids": ["doc_path_001", "doc_path_002"],
            "presented_form_ids": ["att_path_001"],
            "conclusion": "Adenocarcinoma of the colon, moderately differentiated. Tumor invades into the muscularis propria. No lymphovascular invasion identified.",
            "conclusion_codes": ["adenocarcinoma", "colon-cancer"],
            "conclusion_code_codes": ["C18.9", "M8140/3"],
            "conclusion_code_systems": ["http://hl7.org/fhir/sid/icd-10-cm", "http://hl7.org/fhir/sid/icd-10-cm"],
            "conclusion_code_displays": ["Malignant neoplasm of colon, unspecified", "Adenocarcinoma, NOS"],
            "composition_id": "comp_path_001",
            "notes": "Patient should be referred to oncology for further management."
        }"#;

        let diagnostic_report: DomainDiagnosticReport = serde_json::from_str(json).unwrap();
        
        assert_eq!(diagnostic_report.diagnostic_report_id, "dr_path_001");
        assert_eq!(diagnostic_report.patient_demographic_no, "12345");
        assert_eq!(diagnostic_report.status, Some("final".to_string()));
        assert_eq!(diagnostic_report.category, Some("pathology".to_string()));
        assert_eq!(diagnostic_report.category_code, Some("PATH".to_string()));
        assert_eq!(diagnostic_report.category_system, Some("http://terminology.hl7.org/CodeSystem/v2-0074".to_string()));
        assert_eq!(diagnostic_report.category_display, Some("Pathology".to_string()));
        assert_eq!(diagnostic_report.code, Some("biopsy-report".to_string()));
        assert_eq!(diagnostic_report.code_code, Some("BIOPSY".to_string()));
        assert_eq!(diagnostic_report.code_system, Some("http://loinc.org".to_string()));
        assert_eq!(diagnostic_report.code_display, Some("Biopsy Report".to_string()));
        assert_eq!(diagnostic_report.effective_date, Some("2024-03-01T09:00:00Z".to_string()));
        assert_eq!(diagnostic_report.issued_date, Some("2024-03-01T16:00:00Z".to_string()));
        assert_eq!(diagnostic_report.encounter_id, Some("enc_003".to_string()));
        assert_eq!(diagnostic_report.performer_ids, Some(vec!["prac_path_001".to_string()]));
        assert_eq!(diagnostic_report.performer_types, Some(vec!["Practitioner".to_string()]));
        assert_eq!(diagnostic_report.results_interpreter_ids, Some(vec!["prac_path_002".to_string()]));
        assert_eq!(diagnostic_report.results_interpreter_types, Some(vec!["Practitioner".to_string()]));
        assert_eq!(diagnostic_report.based_on_ids, Some(vec!["sr_003".to_string()]));
        assert_eq!(diagnostic_report.based_on_types, Some(vec!["ServiceRequest".to_string()]));
        assert_eq!(diagnostic_report.specimen_ids, Some(vec!["spec_path_001".to_string()]));
        assert_eq!(diagnostic_report.result_observation_ids, Some(vec!["obs_path_001".to_string(), "obs_path_002".to_string()]));
        assert_eq!(diagnostic_report.study_ids, Some(vec!["study_path_001".to_string()]));
        assert_eq!(diagnostic_report.study_types, Some(vec!["GenomicStudy".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_types, Some(vec!["procedure".to_string(), "observation".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_type_codes, Some(vec!["procedure".to_string(), "observation".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_type_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/v2-0074".to_string(), "http://terminology.hl7.org/CodeSystem/v2-0074".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_type_displays, Some(vec!["Procedure".to_string(), "Observation".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_reference_ids, Some(vec!["proc_path_001".to_string(), "obs_path_003".to_string()]));
        assert_eq!(diagnostic_report.supporting_info_reference_types, Some(vec!["Procedure".to_string(), "Observation".to_string()]));
        assert_eq!(diagnostic_report.media_comments, Some(vec!["H&E stain".to_string(), "Immunohistochemistry".to_string()]));
        assert_eq!(diagnostic_report.media_link_ids, Some(vec!["doc_path_001".to_string(), "doc_path_002".to_string()]));
        assert_eq!(diagnostic_report.presented_form_ids, Some(vec!["att_path_001".to_string()]));
        assert_eq!(diagnostic_report.conclusion, Some("Adenocarcinoma of the colon, moderately differentiated. Tumor invades into the muscularis propria. No lymphovascular invasion identified.".to_string()));
        assert_eq!(diagnostic_report.conclusion_codes, Some(vec!["adenocarcinoma".to_string(), "colon-cancer".to_string()]));
        assert_eq!(diagnostic_report.conclusion_code_codes, Some(vec!["C18.9".to_string(), "M8140/3".to_string()]));
        assert_eq!(diagnostic_report.conclusion_code_systems, Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]));
        assert_eq!(diagnostic_report.conclusion_code_displays, Some(vec!["Malignant neoplasm of colon, unspecified".to_string(), "Adenocarcinoma, NOS".to_string()]));
        assert_eq!(diagnostic_report.composition_id, Some("comp_path_001".to_string()));
        assert_eq!(diagnostic_report.notes, Some("Patient should be referred to oncology for further management.".to_string()));
    }

    #[test]
    fn test_domain_diagnostic_report_missing_required_field() {
        let json = r#"{
            "performer_ids": ["prac_001"]
        }"#;

        // This should fail because diagnostic_report_id and patient_demographic_no are required
        let result: Result<DomainDiagnosticReport, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
