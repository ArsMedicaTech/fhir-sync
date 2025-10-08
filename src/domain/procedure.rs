use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainProcedure {
    pub id: String,
    pub identifier: Vec<String>, // External Identifiers for this procedure
    pub instantiates_canonical: Vec<String>, // Instantiates FHIR protocol or definition
    pub instantiates_uri: Vec<String>, // Instantiates external protocol or definition
    pub based_on_ids: Vec<String>, // A request for this procedure (CarePlan, ServiceRequest)
    pub based_on_types: Vec<String>,
    pub part_of_ids: Vec<String>, // Part of referenced event (Procedure, Observation, MedicationAdministration)
    pub part_of_types: Vec<String>,
    pub status: String, // preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    pub status_reason: Option<String>, // Reason for current status
    pub status_reason_system: Option<String>,
    pub status_reason_code: Option<String>,
    pub status_reason_display: Option<String>,
    pub category: Vec<String>, // Classification of the procedure
    pub category_system: Vec<String>,
    pub category_code: Vec<String>,
    pub category_display: Vec<String>,
    pub code: Option<String>, // Identification of the procedure
    pub code_system: Option<String>,
    pub code_display: Option<String>,
    pub subject_id: String, // Individual or entity the procedure was performed on (Patient, Group, Device, Practitioner, Organization, Location)
    pub subject_type: String,
    pub focus_id: Option<String>, // Who is the target of the procedure when it is not the subject of record only
    pub focus_type: Option<String>,
    pub encounter_id: Option<String>, // The Encounter during which this Procedure was created
    pub occurrence_date_time: Option<String>, // When the procedure occurred or is occurring (DateTime)
    pub occurrence_period_start: Option<String>, // When the procedure occurred or is occurring (Period start)
    pub occurrence_period_end: Option<String>, // When the procedure occurred or is occurring (Period end)
    pub occurrence_string: Option<String>, // When the procedure occurred or is occurring (String)
    pub occurrence_age_value: Option<f64>, // When the procedure occurred or is occurring (Age value)
    pub occurrence_age_unit: Option<String>, // When the procedure occurred or is occurring (Age unit)
    pub occurrence_range_low_value: Option<f64>, // When the procedure occurred or is occurring (Range low)
    pub occurrence_range_low_unit: Option<String>,
    pub occurrence_range_high_value: Option<f64>, // When the procedure occurred or is occurring (Range high)
    pub occurrence_range_high_unit: Option<String>,
    pub occurrence_timing_code: Option<String>, // When the procedure occurred or is occurring (Timing code)
    pub occurrence_timing_system: Option<String>,
    pub occurrence_timing_display: Option<String>,
    pub recorded: Option<String>, // When the procedure was first captured in the subject's record (ISO "YYYY-MM-DDTHH:MM:SSZ")
    pub recorder_id: Option<String>, // Who recorded the procedure (Patient, RelatedPerson, Practitioner, PractitionerRole)
    pub recorder_type: Option<String>,
    pub reported_boolean: Option<bool>, // Reported rather than primary record (Boolean)
    pub reported_reference_id: Option<String>, // Reported rather than primary record (Reference)
    pub reported_reference_type: Option<String>,
    pub performer: Vec<DomainProcedurePerformer>, // Who performed the procedure and what they did
    pub location_id: Option<String>, // Where the procedure happened (Location)
    pub reason: Vec<String>, // The justification that the procedure was performed
    pub reason_system: Vec<String>,
    pub reason_code: Vec<String>,
    pub reason_display: Vec<String>,
    pub reason_reference_id: Vec<String>, // The justification that the procedure was performed (Reference)
    pub reason_reference_type: Vec<String>,
    pub body_site: Vec<String>, // Target body sites
    pub body_site_system: Vec<String>,
    pub body_site_code: Vec<String>,
    pub body_site_display: Vec<String>,
    pub outcome: Option<String>, // The result of procedure
    pub outcome_system: Option<String>,
    pub outcome_code: Option<String>,
    pub outcome_display: Option<String>,
    pub report_ids: Vec<String>, // Any report resulting from the procedure (DiagnosticReport, DocumentReference, Composition)
    pub report_types: Vec<String>,
    pub complication: Vec<String>, // Complication following the procedure
    pub complication_system: Vec<String>,
    pub complication_code: Vec<String>,
    pub complication_display: Vec<String>,
    pub complication_reference_id: Vec<String>, // Complication following the procedure (Reference)
    pub complication_reference_type: Vec<String>,
    pub follow_up: Vec<String>, // Instructions for follow up
    pub follow_up_system: Vec<String>,
    pub follow_up_code: Vec<String>,
    pub follow_up_display: Vec<String>,
    pub note: Vec<String>, // Additional information about the procedure
    pub focal_device: Vec<DomainFocalDevice>, // Manipulated, implanted, or removed device
    pub used: Vec<String>, // Items used during procedure
    pub used_system: Vec<String>,
    pub used_code: Vec<String>,
    pub used_display: Vec<String>,
    pub used_reference_id: Vec<String>, // Items used during procedure (Reference)
    pub used_reference_type: Vec<String>,
    pub supporting_info_ids: Vec<String>, // Extra information relevant to the procedure (Resource)
    pub supporting_info_types: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainProcedurePerformer {
    pub function: Option<String>, // Type of performance
    pub function_system: Option<String>,
    pub function_code: Option<String>,
    pub function_display: Option<String>,
    pub actor_id: String, // Who performed the procedure (Practitioner, PractitionerRole, Organization, Patient, RelatedPerson, Device, CareTeam, HealthcareService)
    pub actor_type: String,
    pub on_behalf_of_id: Option<String>, // Organization the device or practitioner was acting for (Organization)
    pub period_start: Option<String>, // When the performer performed the procedure start (ISO "YYYY-MM-DDTHH:MM:SSZ")
    pub period_end: Option<String>, // When the performer performed the procedure end (ISO "YYYY-MM-DDTHH:MM:SSZ")
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainFocalDevice {
    pub action: Option<String>, // Kind of change to device
    pub action_system: Option<String>,
    pub action_code: Option<String>,
    pub action_display: Option<String>,
    pub manipulated_id: String, // Device that was changed (Device)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_procedure_serialization() {
        let procedure = DomainProcedure {
            id: "proc-123".to_string(),
            identifier: vec!["PROC-001".to_string()],
            instantiates_canonical: vec![],
            instantiates_uri: vec![],
            based_on_ids: vec![],
            based_on_types: vec![],
            part_of_ids: vec![],
            part_of_types: vec![],
            status: "completed".to_string(),
            status_reason: Some("Procedure completed successfully".to_string()),
            status_reason_system: Some("http://terminology.hl7.org/CodeSystem/procedure-status-reason".to_string()),
            status_reason_code: Some("completed".to_string()),
            status_reason_display: Some("Completed".to_string()),
            category: vec!["surgery".to_string()],
            category_system: vec!["http://terminology.hl7.org/CodeSystem/procedure-category".to_string()],
            category_code: vec!["surgery".to_string()],
            category_display: vec!["Surgery".to_string()],
            code: Some("appendectomy".to_string()),
            code_system: Some("http://snomed.info/sct".to_string()),
            code_display: Some("Appendectomy".to_string()),
            subject_id: "patient-456".to_string(),
            subject_type: "Patient".to_string(),
            focus_id: None,
            focus_type: None,
            encounter_id: Some("encounter-789".to_string()),
            occurrence_date_time: Some("2024-01-15T10:30:00Z".to_string()),
            occurrence_period_start: None,
            occurrence_period_end: None,
            occurrence_string: None,
            occurrence_age_value: None,
            occurrence_age_unit: None,
            occurrence_range_low_value: None,
            occurrence_range_low_unit: None,
            occurrence_range_high_value: None,
            occurrence_range_high_unit: None,
            occurrence_timing_code: None,
            occurrence_timing_system: None,
            occurrence_timing_display: None,
            recorded: Some("2024-01-15T10:35:00Z".to_string()),
            recorder_id: Some("practitioner-123".to_string()),
            recorder_type: Some("Practitioner".to_string()),
            reported_boolean: Some(false),
            reported_reference_id: None,
            reported_reference_type: None,
            performer: vec![DomainProcedurePerformer {
                function: Some("surgeon".to_string()),
                function_system: Some("http://terminology.hl7.org/CodeSystem/procedure-performer-function".to_string()),
                function_code: Some("surgeon".to_string()),
                function_display: Some("Surgeon".to_string()),
                actor_id: "practitioner-123".to_string(),
                actor_type: "Practitioner".to_string(),
                on_behalf_of_id: Some("organization-456".to_string()),
                period_start: Some("2024-01-15T10:30:00Z".to_string()),
                period_end: Some("2024-01-15T12:00:00Z".to_string()),
            }],
            location_id: Some("location-789".to_string()),
            reason: vec!["acute appendicitis".to_string()],
            reason_system: vec!["http://snomed.info/sct".to_string()],
            reason_code: vec!["54089009".to_string()],
            reason_display: vec!["Acute appendicitis".to_string()],
            reason_reference_id: vec![],
            reason_reference_type: vec![],
            body_site: vec!["appendix".to_string()],
            body_site_system: vec!["http://snomed.info/sct".to_string()],
            body_site_code: vec!["181416001".to_string()],
            body_site_display: vec!["Appendix".to_string()],
            outcome: Some("successful".to_string()),
            outcome_system: Some("http://terminology.hl7.org/CodeSystem/procedure-outcome".to_string()),
            outcome_code: Some("successful".to_string()),
            outcome_display: Some("Successful".to_string()),
            report_ids: vec!["report-123".to_string()],
            report_types: vec!["DiagnosticReport".to_string()],
            complication: vec![],
            complication_system: vec![],
            complication_code: vec![],
            complication_display: vec![],
            complication_reference_id: vec![],
            complication_reference_type: vec![],
            follow_up: vec!["routine follow-up".to_string()],
            follow_up_system: vec!["http://terminology.hl7.org/CodeSystem/procedure-follow-up".to_string()],
            follow_up_code: vec!["routine".to_string()],
            follow_up_display: vec!["Routine follow-up".to_string()],
            note: vec!["Patient tolerated procedure well".to_string()],
            focal_device: vec![],
            used: vec!["surgical instruments".to_string()],
            used_system: vec!["http://snomed.info/sct".to_string()],
            used_code: vec!["4421005".to_string()],
            used_display: vec!["Surgical instruments".to_string()],
            used_reference_id: vec![],
            used_reference_type: vec![],
            supporting_info_ids: vec!["lab-result-123".to_string()],
            supporting_info_types: vec!["Observation".to_string()],
        };

        let json = serde_json::to_string(&procedure).unwrap();
        let deserialized: DomainProcedure = serde_json::from_str(&json).unwrap();
        assert_eq!(procedure.id, deserialized.id);
        assert_eq!(procedure.status, deserialized.status);
        assert_eq!(procedure.subject_id, deserialized.subject_id);
    }

    #[test]
    fn test_domain_procedure_performer_serialization() {
        let performer = DomainProcedurePerformer {
            function: Some("surgeon".to_string()),
            function_system: Some("http://terminology.hl7.org/CodeSystem/procedure-performer-function".to_string()),
            function_code: Some("surgeon".to_string()),
            function_display: Some("Surgeon".to_string()),
            actor_id: "practitioner-123".to_string(),
            actor_type: "Practitioner".to_string(),
            on_behalf_of_id: Some("organization-456".to_string()),
            period_start: Some("2024-01-15T10:30:00Z".to_string()),
            period_end: Some("2024-01-15T12:00:00Z".to_string()),
        };

        let json = serde_json::to_string(&performer).unwrap();
        let deserialized: DomainProcedurePerformer = serde_json::from_str(&json).unwrap();
        assert_eq!(performer.actor_id, deserialized.actor_id);
        assert_eq!(performer.function, deserialized.function);
    }

    #[test]
    fn test_domain_focal_device_serialization() {
        let focal_device = DomainFocalDevice {
            action: Some("implanted".to_string()),
            action_system: Some("http://terminology.hl7.org/CodeSystem/device-action".to_string()),
            action_code: Some("implanted".to_string()),
            action_display: Some("Implanted".to_string()),
            manipulated_id: "device-123".to_string(),
        };

        let json = serde_json::to_string(&focal_device).unwrap();
        let deserialized: DomainFocalDevice = serde_json::from_str(&json).unwrap();
        assert_eq!(focal_device.manipulated_id, deserialized.manipulated_id);
        assert_eq!(focal_device.action, deserialized.action);
    }
}
