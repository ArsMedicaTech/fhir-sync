use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainObservation {
    pub id: String,
    pub identifier: Vec<String>, // Business Identifier for observation
    pub instantiates_canonical: Option<String>, // Instantiates FHIR ObservationDefinition (Canonical)
    pub instantiates_reference_id: Option<String>, // Instantiates FHIR ObservationDefinition (Reference)
    pub based_on_ids: Vec<String>, // Fulfills plan, proposal or order (CarePlan, DeviceRequest, ImmunizationRecommendation, MedicationRequest, NutritionOrder, ServiceRequest)
    pub based_on_types: Vec<String>,
    pub triggered_by: Vec<DomainTriggeredBy>, // Triggering observation(s)
    pub part_of_ids: Vec<String>, // Part of referenced event (MedicationAdministration, MedicationDispense, MedicationStatement, Procedure, Immunization, ImagingStudy, GenomicStudy)
    pub part_of_types: Vec<String>,
    pub status: String, // registered | preliminary | final | amended +
    pub category: Vec<String>, // Classification of type of observation
    pub category_system: Vec<String>,
    pub category_code: Vec<String>,
    pub category_display: Vec<String>,
    pub code: String, // Type of observation (code / type)
    pub code_system: Option<String>,
    pub code_display: Option<String>,
    pub subject_id: String, // Who and/or what the observation is about (Patient, Group, Device, Location, Organization, Procedure, Practitioner, Medication, Substance, BiologicallyDerivedProduct, NutritionProduct)
    pub subject_type: String,
    pub focus_ids: Vec<String>, // What the observation is about, when it is not about the subject of record (Resource)
    pub focus_types: Vec<String>,
    pub encounter_id: Option<String>, // Healthcare event during which this observation is made (Encounter)
    pub effective_date_time: Option<String>, // Clinically relevant time/time-period for observation (DateTime)
    pub effective_period_start: Option<String>, // Clinically relevant time/time-period for observation (Period start)
    pub effective_period_end: Option<String>, // Clinically relevant time/time-period for observation (Period end)
    pub effective_timing_code: Option<String>, // Clinically relevant time/time-period for observation (Timing code)
    pub effective_timing_system: Option<String>,
    pub effective_timing_display: Option<String>,
    pub effective_instant: Option<String>, // Clinically relevant time/time-period for observation (Instant)
    pub issued: Option<String>, // Date/Time this version was made available (ISO "YYYY-MM-DDTHH:MM:SSZ")
    pub performer_ids: Vec<String>, // Who is responsible for the observation (Practitioner, PractitionerRole, Organization, CareTeam, Patient, RelatedPerson)
    pub performer_types: Vec<String>,
    pub value_quantity_value: Option<f64>, // Actual result (Quantity value)
    pub value_quantity_unit: Option<String>,
    pub value_quantity_system: Option<String>,
    pub value_quantity_code: Option<String>,
    pub value_codeable_concept_code: Option<String>, // Actual result (CodeableConcept code)
    pub value_codeable_concept_system: Option<String>,
    pub value_codeable_concept_display: Option<String>,
    pub value_string: Option<String>, // Actual result (String)
    pub value_boolean: Option<bool>, // Actual result (Boolean)
    pub value_integer: Option<i32>, // Actual result (Integer)
    pub value_range_low_value: Option<f64>, // Actual result (Range low)
    pub value_range_low_unit: Option<String>,
    pub value_range_high_value: Option<f64>, // Actual result (Range high)
    pub value_range_high_unit: Option<String>,
    pub value_ratio_numerator_value: Option<f64>, // Actual result (Ratio numerator)
    pub value_ratio_numerator_unit: Option<String>,
    pub value_ratio_denominator_value: Option<f64>, // Actual result (Ratio denominator)
    pub value_ratio_denominator_unit: Option<String>,
    pub value_sampled_data: Option<String>, // Actual result (SampledData - stored as JSON string)
    pub value_time: Option<String>, // Actual result (Time - HH:MM:SS format)
    pub value_date_time: Option<String>, // Actual result (DateTime)
    pub value_period_start: Option<String>, // Actual result (Period start)
    pub value_period_end: Option<String>, // Actual result (Period end)
    pub value_attachment: Option<String>, // Actual result (Attachment - stored as JSON string)
    pub value_reference_id: Option<String>, // Actual result (Reference to MolecularSequence)
    pub value_reference_type: Option<String>,
    pub data_absent_reason: Option<String>, // Why the result is missing
    pub data_absent_reason_system: Option<String>,
    pub data_absent_reason_code: Option<String>,
    pub data_absent_reason_display: Option<String>,
    pub interpretation: Vec<String>, // High, low, normal, etc
    pub interpretation_system: Vec<String>,
    pub interpretation_code: Vec<String>,
    pub interpretation_display: Vec<String>,
    pub note: Vec<String>, // Comments about the observation
    pub body_site: Option<String>, // Observed body part
    pub body_site_system: Option<String>,
    pub body_site_code: Option<String>,
    pub body_site_display: Option<String>,
    pub body_structure_id: Option<String>, // Observed body structure (BodyStructure)
    pub method: Option<String>, // How it was done
    pub method_system: Option<String>,
    pub method_code: Option<String>,
    pub method_display: Option<String>,
    pub specimen_id: Option<String>, // Specimen used for this observation (Specimen, Group)
    pub specimen_type: Option<String>,
    pub device_id: Option<String>, // A reference to the device that generates the measurements (Device, DeviceMetric)
    pub device_type: Option<String>,
    pub reference_range: Vec<DomainReferenceRange>, // Provides guide for interpretation
    pub has_member_ids: Vec<String>, // Related resource that belongs to the Observation group (Observation, QuestionnaireResponse, MolecularSequence)
    pub has_member_types: Vec<String>,
    pub derived_from_ids: Vec<String>, // Related resource from which the observation is made (DocumentReference, ImagingStudy, ImagingSelection, QuestionnaireResponse, Observation, MolecularSequence, GenomicStudy)
    pub derived_from_types: Vec<String>,
    pub component: Vec<DomainObservationComponent>, // Component results
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainTriggeredBy {
    pub observation_id: String, // Triggering observation (Observation)
    pub r#type: String, // reflex | repeat | re-run
    pub type_system: Option<String>,
    pub type_code: Option<String>,
    pub type_display: Option<String>,
    pub reason: Option<String>, // Reason that the observation was triggered
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainReferenceRange {
    pub low_value: Option<f64>, // Low Range, if relevant
    pub low_unit: Option<String>,
    pub low_system: Option<String>,
    pub low_code: Option<String>,
    pub high_value: Option<f64>, // High Range, if relevant
    pub high_unit: Option<String>,
    pub high_system: Option<String>,
    pub high_code: Option<String>,
    pub normal_value: Option<String>, // Normal value, if relevant
    pub normal_value_system: Option<String>,
    pub normal_value_code: Option<String>,
    pub normal_value_display: Option<String>,
    pub r#type: Option<String>, // Reference range qualifier
    pub type_system: Option<String>,
    pub type_code: Option<String>,
    pub type_display: Option<String>,
    pub applies_to: Vec<String>, // Reference range population
    pub applies_to_system: Vec<String>,
    pub applies_to_code: Vec<String>,
    pub applies_to_display: Vec<String>,
    pub age_low_value: Option<f64>, // Applicable age range, if relevant (Range low)
    pub age_low_unit: Option<String>,
    pub age_high_value: Option<f64>, // Applicable age range, if relevant (Range high)
    pub age_high_unit: Option<String>,
    pub text: Option<String>, // Text based reference range in an observation
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainObservationComponent {
    pub code: String, // Type of component observation (code / type)
    pub code_system: Option<String>,
    pub code_display: Option<String>,
    pub value_quantity_value: Option<f64>, // Actual component result (Quantity value)
    pub value_quantity_unit: Option<String>,
    pub value_quantity_system: Option<String>,
    pub value_quantity_code: Option<String>,
    pub value_codeable_concept_code: Option<String>, // Actual component result (CodeableConcept code)
    pub value_codeable_concept_system: Option<String>,
    pub value_codeable_concept_display: Option<String>,
    pub value_string: Option<String>, // Actual component result (String)
    pub value_boolean: Option<bool>, // Actual component result (Boolean)
    pub value_integer: Option<i32>, // Actual component result (Integer)
    pub value_range_low_value: Option<f64>, // Actual component result (Range low)
    pub value_range_low_unit: Option<String>,
    pub value_range_high_value: Option<f64>, // Actual component result (Range high)
    pub value_range_high_unit: Option<String>,
    pub value_ratio_numerator_value: Option<f64>, // Actual component result (Ratio numerator)
    pub value_ratio_numerator_unit: Option<String>,
    pub value_ratio_denominator_value: Option<f64>, // Actual component result (Ratio denominator)
    pub value_ratio_denominator_unit: Option<String>,
    pub value_sampled_data: Option<String>, // Actual component result (SampledData - stored as JSON string)
    pub value_time: Option<String>, // Actual component result (Time - HH:MM:SS format)
    pub value_date_time: Option<String>, // Actual component result (DateTime)
    pub value_period_start: Option<String>, // Actual component result (Period start)
    pub value_period_end: Option<String>, // Actual component result (Period end)
    pub value_attachment: Option<String>, // Actual component result (Attachment - stored as JSON string)
    pub value_reference_id: Option<String>, // Actual component result (Reference to MolecularSequence)
    pub value_reference_type: Option<String>,
    pub data_absent_reason: Option<String>, // Why the component result is missing
    pub data_absent_reason_system: Option<String>,
    pub data_absent_reason_code: Option<String>,
    pub data_absent_reason_display: Option<String>,
    pub interpretation: Vec<String>, // High, low, normal, etc
    pub interpretation_system: Vec<String>,
    pub interpretation_code: Vec<String>,
    pub interpretation_display: Vec<String>,
    pub reference_range: Vec<DomainReferenceRange>, // Provides guide for interpretation of component result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_observation_serialization() {
        let observation = DomainObservation {
            id: "obs-123".to_string(),
            identifier: vec!["OBS-001".to_string()],
            instantiates_canonical: None,
            instantiates_reference_id: None,
            based_on_ids: vec![],
            based_on_types: vec![],
            triggered_by: vec![],
            part_of_ids: vec![],
            part_of_types: vec![],
            status: "final".to_string(),
            category: vec!["vital-signs".to_string()],
            category_system: vec!["http://terminology.hl7.org/CodeSystem/observation-category".to_string()],
            category_code: vec!["vital-signs".to_string()],
            category_display: vec!["Vital Signs".to_string()],
            code: "blood-pressure".to_string(),
            code_system: Some("http://loinc.org".to_string()),
            code_display: Some("Blood pressure".to_string()),
            subject_id: "patient-456".to_string(),
            subject_type: "Patient".to_string(),
            focus_ids: vec![],
            focus_types: vec![],
            encounter_id: Some("encounter-789".to_string()),
            effective_date_time: Some("2024-01-15T10:30:00Z".to_string()),
            effective_period_start: None,
            effective_period_end: None,
            effective_timing_code: None,
            effective_timing_system: None,
            effective_timing_display: None,
            effective_instant: None,
            issued: Some("2024-01-15T10:35:00Z".to_string()),
            performer_ids: vec!["practitioner-123".to_string()],
            performer_types: vec!["Practitioner".to_string()],
            value_quantity_value: Some(120.0),
            value_quantity_unit: Some("mmHg".to_string()),
            value_quantity_system: Some("http://unitsofmeasure.org".to_string()),
            value_quantity_code: Some("mm[Hg]".to_string()),
            value_codeable_concept_code: None,
            value_codeable_concept_system: None,
            value_codeable_concept_display: None,
            value_string: None,
            value_boolean: None,
            value_integer: None,
            value_range_low_value: None,
            value_range_low_unit: None,
            value_range_high_value: None,
            value_range_high_unit: None,
            value_ratio_numerator_value: None,
            value_ratio_numerator_unit: None,
            value_ratio_denominator_value: None,
            value_ratio_denominator_unit: None,
            value_sampled_data: None,
            value_time: None,
            value_date_time: None,
            value_period_start: None,
            value_period_end: None,
            value_attachment: None,
            value_reference_id: None,
            value_reference_type: None,
            data_absent_reason: None,
            data_absent_reason_system: None,
            data_absent_reason_code: None,
            data_absent_reason_display: None,
            interpretation: vec!["normal".to_string()],
            interpretation_system: vec!["http://terminology.hl7.org/CodeSystem/v3-ObservationInterpretation".to_string()],
            interpretation_code: vec!["N".to_string()],
            interpretation_display: vec!["Normal".to_string()],
            note: vec!["Patient resting comfortably".to_string()],
            body_site: Some("arm".to_string()),
            body_site_system: Some("http://snomed.info/sct".to_string()),
            body_site_code: Some("40983000".to_string()),
            body_site_display: Some("Upper arm".to_string()),
            body_structure_id: None,
            method: Some("automated".to_string()),
            method_system: Some("http://terminology.hl7.org/CodeSystem/observation-method".to_string()),
            method_code: Some("automated".to_string()),
            method_display: Some("Automated".to_string()),
            specimen_id: None,
            specimen_type: None,
            device_id: Some("device-123".to_string()),
            device_type: Some("Device".to_string()),
            reference_range: vec![DomainReferenceRange {
                low_value: Some(90.0),
                low_unit: Some("mmHg".to_string()),
                low_system: Some("http://unitsofmeasure.org".to_string()),
                low_code: Some("mm[Hg]".to_string()),
                high_value: Some(140.0),
                high_unit: Some("mmHg".to_string()),
                high_system: Some("http://unitsofmeasure.org".to_string()),
                high_code: Some("mm[Hg]".to_string()),
                normal_value: None,
                normal_value_system: None,
                normal_value_code: None,
                normal_value_display: None,
                r#type: Some("normal".to_string()),
                type_system: Some("http://terminology.hl7.org/CodeSystem/referencerange-meaning".to_string()),
                type_code: Some("normal".to_string()),
                type_display: Some("Normal Range".to_string()),
                applies_to: vec![],
                applies_to_system: vec![],
                applies_to_code: vec![],
                applies_to_display: vec![],
                age_low_value: None,
                age_low_unit: None,
                age_high_value: None,
                age_high_unit: None,
                text: Some("Normal blood pressure range".to_string()),
            }],
            has_member_ids: vec![],
            has_member_types: vec![],
            derived_from_ids: vec![],
            derived_from_types: vec![],
            component: vec![],
        };

        let json = serde_json::to_string(&observation).unwrap();
        let deserialized: DomainObservation = serde_json::from_str(&json).unwrap();
        assert_eq!(observation.id, deserialized.id);
        assert_eq!(observation.status, deserialized.status);
        assert_eq!(observation.subject_id, deserialized.subject_id);
    }

    #[test]
    fn test_domain_triggered_by_serialization() {
        let triggered_by = DomainTriggeredBy {
            observation_id: "obs-456".to_string(),
            r#type: "repeat".to_string(),
            type_system: Some("http://terminology.hl7.org/CodeSystem/observation-triggeredbytype".to_string()),
            type_code: Some("repeat".to_string()),
            type_display: Some("Repeat".to_string()),
            reason: Some("Quality control check".to_string()),
        };

        let json = serde_json::to_string(&triggered_by).unwrap();
        let deserialized: DomainTriggeredBy = serde_json::from_str(&json).unwrap();
        assert_eq!(triggered_by.observation_id, deserialized.observation_id);
        assert_eq!(triggered_by.r#type, deserialized.r#type);
    }

    #[test]
    fn test_domain_reference_range_serialization() {
        let reference_range = DomainReferenceRange {
            low_value: Some(90.0),
            low_unit: Some("mmHg".to_string()),
            low_system: Some("http://unitsofmeasure.org".to_string()),
            low_code: Some("mm[Hg]".to_string()),
            high_value: Some(140.0),
            high_unit: Some("mmHg".to_string()),
            high_system: Some("http://unitsofmeasure.org".to_string()),
            high_code: Some("mm[Hg]".to_string()),
            normal_value: None,
            normal_value_system: None,
            normal_value_code: None,
            normal_value_display: None,
            r#type: Some("normal".to_string()),
            type_system: Some("http://terminology.hl7.org/CodeSystem/referencerange-meaning".to_string()),
            type_code: Some("normal".to_string()),
            type_display: Some("Normal Range".to_string()),
            applies_to: vec![],
            applies_to_system: vec![],
            applies_to_code: vec![],
            applies_to_display: vec![],
            age_low_value: None,
            age_low_unit: None,
            age_high_value: None,
            age_high_unit: None,
            text: Some("Normal blood pressure range".to_string()),
        };

        let json = serde_json::to_string(&reference_range).unwrap();
        let deserialized: DomainReferenceRange = serde_json::from_str(&json).unwrap();
        assert_eq!(reference_range.low_value, deserialized.low_value);
        assert_eq!(reference_range.high_value, deserialized.high_value);
    }

    #[test]
    fn test_domain_observation_component_serialization() {
        let component = DomainObservationComponent {
            code: "systolic".to_string(),
            code_system: Some("http://loinc.org".to_string()),
            code_display: Some("Systolic blood pressure".to_string()),
            value_quantity_value: Some(120.0),
            value_quantity_unit: Some("mmHg".to_string()),
            value_quantity_system: Some("http://unitsofmeasure.org".to_string()),
            value_quantity_code: Some("mm[Hg]".to_string()),
            value_codeable_concept_code: None,
            value_codeable_concept_system: None,
            value_codeable_concept_display: None,
            value_string: None,
            value_boolean: None,
            value_integer: None,
            value_range_low_value: None,
            value_range_low_unit: None,
            value_range_high_value: None,
            value_range_high_unit: None,
            value_ratio_numerator_value: None,
            value_ratio_numerator_unit: None,
            value_ratio_denominator_value: None,
            value_ratio_denominator_unit: None,
            value_sampled_data: None,
            value_time: None,
            value_date_time: None,
            value_period_start: None,
            value_period_end: None,
            value_attachment: None,
            value_reference_id: None,
            value_reference_type: None,
            data_absent_reason: None,
            data_absent_reason_system: None,
            data_absent_reason_code: None,
            data_absent_reason_display: None,
            interpretation: vec!["normal".to_string()],
            interpretation_system: vec!["http://terminology.hl7.org/CodeSystem/v3-ObservationInterpretation".to_string()],
            interpretation_code: vec!["N".to_string()],
            interpretation_display: vec!["Normal".to_string()],
            reference_range: vec![],
        };

        let json = serde_json::to_string(&component).unwrap();
        let deserialized: DomainObservationComponent = serde_json::from_str(&json).unwrap();
        assert_eq!(component.code, deserialized.code);
        assert_eq!(component.value_quantity_value, deserialized.value_quantity_value);
    }
}
