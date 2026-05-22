# ClinicalImpression Entity Implementation

This document describes the implementation of the `ClinicalImpression` entity for the gRPC FHIR synchronization project.

## Files Created/Modified

### 1. Domain Model
- **File**: `src/domain/clinical_impression.rs`
- **Purpose**: Defines the `DomainClinicalImpression` struct that represents clinical assessment data in our domain model
- **Key Fields**:
  - `clinical_impression_id`: String (required) - Unique identifier for the clinical impression
  - `patient_demographic_no`: String (required) - Reference to the patient
  - `status`: Option<String> - Status (preparation, in-progress, not-done, on-hold, stopped, completed, entered-in-error, unknown)
  - `status_reason`: Option<String> - Reason for current status
  - `description`: Option<String> - Why/how the assessment was performed
  - `effective_date`: Option<String> - ISO datetime string for assessment time
  - `effective_period_start`: Option<String> - ISO datetime string for assessment period start
  - `effective_period_end`: Option<String> - ISO datetime string for assessment period end
  - `documented_date`: Option<String> - ISO datetime string when assessment was documented
  - `encounter_id`: Option<String> - Encounter during which this ClinicalImpression was created
  - `performer_id`: Option<String> - The clinician performing the assessment
  - `performer_type`: Option<String> - Type of performer (Practitioner, PractitionerRole)
  - `previous_impression_id`: Option<String> - Reference to last assessment
  - `problem_condition_ids`: Option<Vec<String>> - Relevant impressions of patient state (conditions)
  - `problem_allergy_ids`: Option<Vec<String>> - Relevant impressions of patient state (allergies)
  - `change_pattern`: Option<String> - Change in status/pattern since previously assessed
  - `protocol_uris`: Option<Vec<String>> - Clinical Protocol followed
  - `summary`: Option<String> - Summary of the assessment
  - `finding_items`: Option<Vec<String>> - What was found (descriptions)
  - `finding_codes`: Option<Vec<String>> - What was found (codes)
  - `finding_systems`: Option<Vec<String>> - Terminology systems for findings
  - `finding_descriptions`: Option<Vec<String>> - Human-readable descriptions of findings
  - `finding_basis`: Option<Vec<String>> - Which investigations support finding
  - `prognosis_codes`: Option<Vec<String>> - Estimate of likely outcome (codes)
  - `prognosis_descriptions`: Option<Vec<String>> - Estimate of likely outcome (descriptions)
  - `prognosis_systems`: Option<Vec<String>> - Terminology systems for prognosis
  - `prognosis_reference_ids`: Option<Vec<String>> - RiskAssessment expressing likely outcome
  - `supporting_info_ids`: Option<Vec<String>> - Information supporting the clinical impression
  - `notes`: Option<String> - Comments made about the ClinicalImpression

### 2. FHIR Adapter
- **File**: `src/adapters/entities/clinical_impression.rs`
- **Purpose**: Implements the conversion from `DomainClinicalImpression` to FHIR `ClinicalImpression` proto message
- **Key Features**:
  - Maps domain fields to FHIR ClinicalImpression structure
  - Handles status conversion with proper codes
  - Converts datetime strings to FHIR DateTime types
  - Creates proper FHIR references for patient, practitioner, encounter, and other resources
  - Handles findings with proper terminology systems
  - Maps prognosis information with codes and references
  - Creates proper FHIR problem references for conditions and allergies

### 3. Module Updates
- **File**: `src/domain/mod.rs` - Added `pub mod clinical_impression;`
- **File**: `src/adapters/entities/mod.rs` - Added `pub mod clinical_impression;`

## FHIR Mapping Details

### Status
The adapter maps string status values to FHIR status codes:
- "preparation" → 1
- "in-progress" → 2
- "not-done" → 3
- "on-hold" → 4
- "stopped" → 5
- "completed" → 6
- "entered-in-error" → 7
- "unknown" → 8

### Effective Time
The adapter handles temporal information in two ways:
1. **DateTime**: Single point in time for assessment
2. **Period**: Start and end times for assessment period

### References
- Patient reference: `Patient/{demographic_no}`
- Encounter reference: `Encounter/{encounter_id}`
- Performer reference: `{performer_type}/{performer_id}`
- Previous impression reference: `ClinicalImpression/{previous_impression_id}`
- Problem condition references: `Condition/{condition_id}`
- Problem allergy references: `AllergyIntolerance/{allergy_id}`
- Prognosis risk assessment references: `RiskAssessment/{prognosis_reference_id}`
- Supporting info references: `Resource/{supporting_info_id}`

### Identifiers
- System: `urn:arsmedicatech:clinical_impression_id`
- Value: The clinical impression ID

### Findings
- **Item**: What was found (descriptions and codes)
- **Basis**: Which investigations support the finding
- **System**: Terminology system for codes (defaults to ICD-10-CM)
- **Display**: Human-readable descriptions

### Prognosis
- **CodeableConcept**: Estimate of likely outcome with codes and descriptions
- **Reference**: RiskAssessment expressing likely outcome

### Change Pattern
- Text: The provided change pattern description
- System: Custom system for change patterns

## Testing

The implementation includes comprehensive unit tests covering:
- Full deserialization with all fields
- Minimal deserialization with only required fields
- Chest pain assessment scenario
- Psychiatric assessment scenario
- Pediatric developmental assessment scenario
- Error handling for missing required fields

## Usage Example

```rust
use crate::domain::clinical_impression::DomainClinicalImpression;
use crate::adapters::entities::clinical_impression::*;

// Create a domain clinical impression
let domain_impression = DomainClinicalImpression {
    clinical_impression_id: "ci_12345".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("completed".to_string()),
    status_reason: Some("Assessment completed successfully".to_string()),
    description: Some("Comprehensive clinical assessment for chest pain evaluation".to_string()),
    effective_date: Some("2024-01-15T10:30:00Z".to_string()),
    documented_date: Some("2024-01-15T11:00:00Z".to_string()),
    encounter_id: Some("enc_001".to_string()),
    performer_id: Some("prac_001".to_string()),
    performer_type: Some("Practitioner".to_string()),
    previous_impression_id: Some("ci_00001".to_string()),
    problem_condition_ids: Some(vec!["cond_001".to_string(), "cond_002".to_string()]),
    problem_allergy_ids: Some(vec!["allergy_001".to_string()]),
    change_pattern: Some("improving".to_string()),
    protocol_uris: Some(vec!["http://example.org/protocols/chest-pain-assessment".to_string()]),
    summary: Some("Patient presents with chest pain. EKG shows no acute changes. Troponins negative. Likely musculoskeletal origin.".to_string()),
    finding_items: Some(vec!["Chest pain".to_string(), "EKG normal".to_string(), "Troponins negative".to_string(), "Muscle tenderness".to_string()]),
    finding_codes: Some(vec!["R06.02".to_string(), "Z01.810".to_string(), "Z01.811".to_string(), "M79.3".to_string()]),
    finding_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    finding_descriptions: Some(vec!["Chest pain, unspecified".to_string(), "Encounter for preprocedural cardiovascular examination".to_string(), "Encounter for preprocedural laboratory examination".to_string(), "Panniculitis, unspecified".to_string()]),
    finding_basis: Some(vec!["Patient report".to_string(), "EKG interpretation".to_string(), "Laboratory results".to_string(), "Physical examination".to_string()]),
    prognosis_codes: Some(vec!["Z51.11".to_string()]),
    prognosis_descriptions: Some(vec!["Encounter for antineoplastic chemotherapy".to_string()]),
    prognosis_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    prognosis_reference_ids: Some(vec!["risk_001".to_string()]),
    supporting_info_ids: Some(vec!["obs_001".to_string(), "obs_002".to_string(), "proc_001".to_string()]),
    notes: Some("Patient stable. No acute cardiac event. Follow-up in 1 week if symptoms persist.".to_string()),
};

// Convert to FHIR ClinicalImpression
let fhir_impression: ClinicalImpression = domain_impression.into();
```

## Clinical Use Cases

### 1. Chest Pain Assessment
```rust
let chest_pain_assessment = DomainClinicalImpression {
    clinical_impression_id: "ci_chest_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("completed".to_string()),
    status_reason: Some("Assessment completed successfully".to_string()),
    description: Some("Comprehensive clinical assessment for chest pain evaluation".to_string()),
    effective_date: Some("2024-01-15T10:30:00Z".to_string()),
    documented_date: Some("2024-01-15T11:00:00Z".to_string()),
    encounter_id: Some("enc_001".to_string()),
    performer_id: Some("prac_001".to_string()),
    performer_type: Some("Practitioner".to_string()),
    previous_impression_id: Some("ci_00001".to_string()),
    problem_condition_ids: Some(vec!["cond_001".to_string(), "cond_002".to_string()]),
    problem_allergy_ids: Some(vec!["allergy_001".to_string()]),
    change_pattern: Some("improving".to_string()),
    protocol_uris: Some(vec!["http://example.org/protocols/chest-pain-assessment".to_string()]),
    summary: Some("Patient presents with chest pain. EKG shows no acute changes. Troponins negative. Likely musculoskeletal origin.".to_string()),
    finding_items: Some(vec!["Chest pain".to_string(), "EKG normal".to_string(), "Troponins negative".to_string(), "Muscle tenderness".to_string()]),
    finding_codes: Some(vec!["R06.02".to_string(), "Z01.810".to_string(), "Z01.811".to_string(), "M79.3".to_string()]),
    finding_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    finding_descriptions: Some(vec!["Chest pain, unspecified".to_string(), "Encounter for preprocedural cardiovascular examination".to_string(), "Encounter for preprocedural laboratory examination".to_string(), "Panniculitis, unspecified".to_string()]),
    finding_basis: Some(vec!["Patient report".to_string(), "EKG interpretation".to_string(), "Laboratory results".to_string(), "Physical examination".to_string()]),
    prognosis_codes: Some(vec!["Z51.11".to_string()]),
    prognosis_descriptions: Some(vec!["Encounter for antineoplastic chemotherapy".to_string()]),
    prognosis_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    prognosis_reference_ids: Some(vec!["risk_001".to_string()]),
    supporting_info_ids: Some(vec!["obs_001".to_string(), "obs_002".to_string(), "proc_001".to_string()]),
    notes: Some("Patient stable. No acute cardiac event. Follow-up in 1 week if symptoms persist.".to_string()),
    ..Default::default()
};
```

### 2. Psychiatric Assessment
```rust
let psychiatric_assessment = DomainClinicalImpression {
    clinical_impression_id: "ci_psych_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("completed".to_string()),
    status_reason: Some("Assessment completed".to_string()),
    description: Some("Comprehensive psychiatric assessment for depression evaluation".to_string()),
    effective_date: Some("2024-02-01T14:00:00Z".to_string()),
    documented_date: Some("2024-02-01T15:30:00Z".to_string()),
    encounter_id: Some("enc_psych_001".to_string()),
    performer_id: Some("prac_psych_001".to_string()),
    performer_type: Some("Practitioner".to_string()),
    problem_condition_ids: Some(vec!["cond_depression_001".to_string()]),
    change_pattern: Some("worsening".to_string()),
    protocol_uris: Some(vec!["http://example.org/protocols/depression-assessment".to_string()]),
    summary: Some("Patient presents with major depressive episode. PHQ-9 score 18. Suicidal ideation present but no immediate risk.".to_string()),
    finding_items: Some(vec!["Depressed mood".to_string(), "Anhedonia".to_string(), "Sleep disturbance".to_string(), "PHQ-9 score 18".to_string()]),
    finding_codes: Some(vec!["F32.9".to_string(), "F32.9".to_string(), "G47.00".to_string(), "Z13.89".to_string()]),
    finding_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    finding_descriptions: Some(vec!["Major depressive disorder, single episode, unspecified".to_string(), "Major depressive disorder, single episode, unspecified".to_string(), "Insomnia, unspecified".to_string(), "Encounter for screening for other specified diseases and disorders".to_string()]),
    finding_basis: Some(vec!["Clinical interview".to_string(), "Clinical interview".to_string(), "Patient report".to_string(), "PHQ-9 questionnaire".to_string()]),
    prognosis_codes: Some(vec!["Z51.11".to_string()]),
    prognosis_descriptions: Some(vec!["Encounter for antineoplastic chemotherapy".to_string()]),
    prognosis_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    prognosis_reference_ids: Some(vec!["risk_depression_001".to_string()]),
    supporting_info_ids: Some(vec!["obs_phq9_001".to_string(), "obs_sleep_001".to_string()]),
    notes: Some("Patient engaged in treatment. Safety plan established. Follow-up in 1 week.".to_string()),
    ..Default::default()
};
```

### 3. Pediatric Developmental Assessment
```rust
let pediatric_assessment = DomainClinicalImpression {
    clinical_impression_id: "ci_peds_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("completed".to_string()),
    status_reason: Some("Assessment completed".to_string()),
    description: Some("Developmental assessment for 2-year-old child".to_string()),
    effective_period_start: Some("2024-03-01T09:00:00Z".to_string()),
    effective_period_end: Some("2024-03-01T10:30:00Z".to_string()),
    documented_date: Some("2024-03-01T11:00:00Z".to_string()),
    encounter_id: Some("enc_peds_001".to_string()),
    performer_id: Some("prac_peds_001".to_string()),
    performer_type: Some("Practitioner".to_string()),
    problem_condition_ids: Some(vec!["cond_developmental_001".to_string()]),
    change_pattern: Some("no-change".to_string()),
    protocol_uris: Some(vec!["http://example.org/protocols/developmental-assessment".to_string()]),
    summary: Some("Child shows age-appropriate development. No concerns identified. Continue routine monitoring.".to_string()),
    finding_items: Some(vec!["Gross motor skills normal".to_string(), "Fine motor skills normal".to_string(), "Language development normal".to_string(), "Social interaction normal".to_string()]),
    finding_codes: Some(vec!["Z00.121".to_string(), "Z00.121".to_string(), "Z00.121".to_string(), "Z00.121".to_string()]),
    finding_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    finding_descriptions: Some(vec!["Encounter for routine child health examination with abnormal findings".to_string(), "Encounter for routine child health examination with abnormal findings".to_string(), "Encounter for routine child health examination with abnormal findings".to_string(), "Encounter for routine child health examination with abnormal findings".to_string()]),
    finding_basis: Some(vec!["Developmental assessment".to_string(), "Developmental assessment".to_string(), "Developmental assessment".to_string(), "Developmental assessment".to_string()]),
    prognosis_codes: Some(vec!["Z00.121".to_string()]),
    prognosis_descriptions: Some(vec!["Encounter for routine child health examination with abnormal findings".to_string()]),
    prognosis_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    supporting_info_ids: Some(vec!["obs_developmental_001".to_string(), "obs_growth_001".to_string()]),
    notes: Some("Child meeting all developmental milestones. Parent education provided. Next assessment in 6 months.".to_string()),
    ..Default::default()
};
```

### 4. Geriatric Assessment
```rust
let geriatric_assessment = DomainClinicalImpression {
    clinical_impression_id: "ci_geriatric_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("completed".to_string()),
    status_reason: Some("Assessment completed".to_string()),
    description: Some("Comprehensive geriatric assessment for frailty evaluation".to_string()),
    effective_date: Some("2024-04-01T09:00:00Z".to_string()),
    documented_date: Some("2024-04-01T10:30:00Z".to_string()),
    encounter_id: Some("enc_geriatric_001".to_string()),
    performer_id: Some("prac_geriatric_001".to_string()),
    performer_type: Some("Practitioner".to_string()),
    problem_condition_ids: Some(vec!["cond_frailty_001".to_string(), "cond_dementia_001".to_string()]),
    change_pattern: Some("worsening".to_string()),
    protocol_uris: Some(vec!["http://example.org/protocols/geriatric-assessment".to_string()]),
    summary: Some("Patient shows signs of frailty and mild cognitive impairment. Functional decline noted. Recommend comprehensive care plan.".to_string()),
    finding_items: Some(vec!["Frailty score 6/9".to_string(), "MMSE score 24/30".to_string(), "ADL limitations".to_string(), "Social isolation".to_string()]),
    finding_codes: Some(vec!["R54".to_string(), "F03.90".to_string(), "Z74.01".to_string(), "Z60.4".to_string()]),
    finding_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    finding_descriptions: Some(vec!["Senility without mention of psychosis".to_string(), "Unspecified dementia without behavioral disturbance".to_string(), "Bed confinement status".to_string(), "Social exclusion and rejection".to_string()]),
    finding_basis: Some(vec!["Frailty assessment".to_string(), "MMSE test".to_string(), "ADL assessment".to_string(), "Social assessment".to_string()]),
    prognosis_codes: Some(vec!["Z51.11".to_string()]),
    prognosis_descriptions: Some(vec!["Encounter for antineoplastic chemotherapy".to_string()]),
    prognosis_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    prognosis_reference_ids: Some(vec!["risk_frailty_001".to_string()]),
    supporting_info_ids: Some(vec!["obs_frailty_001".to_string(), "obs_cognitive_001".to_string()]),
    notes: Some("Patient requires comprehensive care coordination. Family involved in care planning. Regular monitoring recommended.".to_string()),
    ..Default::default()
};
```

## Next Steps

To complete the clinical impression implementation, you may want to:

1. **Add to Service Layer**: Integrate the clinical impression adapter into your gRPC service
2. **Add Database Adapters**: Create database adapters for storing/retrieving clinical impressions
3. **Add Validation**: Implement validation rules for clinical impression data
4. **Add Error Handling**: Enhance error handling for conversion failures
5. **Add Logging**: Add appropriate logging for clinical impression operations
6. **Add Assessment Templates**: Create reusable assessment templates
7. **Add Progress Tracking**: Track assessment progress and outcomes
8. **Add Assessment Analytics**: Analyze assessment patterns and outcomes
9. **Add Multi-Provider Assessments**: Enable collaborative assessments
10. **Add Assessment Workflows**: Integrate with clinical decision support systems

## Related Entities

This implementation follows the same pattern as the existing entities and integrates with:
- **Patient**: The subject of the clinical impression
- **Encounter**: When the clinical impression was created
- **Practitioner**: Who performed the assessment
- **Condition**: Health problems addressed in the assessment
- **AllergyIntolerance**: Allergies addressed in the assessment
- **RiskAssessment**: Prognosis and risk information
- **Observation**: Supporting information for the assessment

## Assessment Types

The implementation supports various assessment types:
- **Chest Pain Assessment**: Cardiac evaluation and risk stratification
- **Psychiatric Assessment**: Mental health evaluation and treatment planning
- **Pediatric Developmental Assessment**: Child development monitoring
- **Geriatric Assessment**: Frailty and functional status evaluation
- **Neurological Assessment**: Neurological function evaluation
- **Cardiovascular Assessment**: Heart and vascular system evaluation
- **Respiratory Assessment**: Lung and breathing function evaluation
- **Gastrointestinal Assessment**: Digestive system evaluation
- **Musculoskeletal Assessment**: Bone, joint, and muscle evaluation
- **Dermatological Assessment**: Skin condition evaluation

## Assessment Status Lifecycle

The implementation supports the following assessment statuses:
- **Preparation**: Assessment is being prepared
- **In-Progress**: Assessment is currently being performed
- **Not-Done**: Assessment was not performed
- **On-Hold**: Assessment is temporarily paused
- **Stopped**: Assessment was stopped
- **Completed**: Assessment has been finished
- **Entered-in-Error**: Assessment was created by mistake
- **Unknown**: Status is not known

## Change Patterns

The implementation supports various change patterns:
- **Improving**: Patient's condition is getting better
- **Worsening**: Patient's condition is getting worse
- **No-Change**: Patient's condition remains the same
- **Stable**: Patient's condition is stable
- **Fluctuating**: Patient's condition varies over time
- **Uncertain**: Change pattern is unclear

## Finding Types

The implementation supports various finding types:
- **Clinical Findings**: Physical examination findings
- **Laboratory Findings**: Test results and lab values
- **Imaging Findings**: Radiology and imaging results
- **Functional Findings**: Functional assessment results
- **Behavioral Findings**: Behavioral and psychological findings
- **Social Findings**: Social and environmental factors
- **Risk Findings**: Risk factors and risk assessments

## Prognosis Types

The implementation supports various prognosis types:
- **Recovery**: Expected recovery and healing
- **Chronic**: Long-term management needs
- **Progressive**: Expected deterioration
- **Stable**: Expected stability
- **Uncertain**: Prognosis is unclear
- **Palliative**: Comfort care focus

This comprehensive clinical impression implementation provides a solid foundation for managing clinical assessments in your FHIR synchronization system, enabling evidence-based clinical decision making and coordinated patient care.
