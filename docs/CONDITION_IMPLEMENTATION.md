# Condition Entity Implementation

This document describes the implementation of the `Condition` entity for the gRPC FHIR synchronization project.

## Files Created/Modified

### 1. Domain Model
- **File**: `src/domain/condition.rs`
- **Purpose**: Defines the `DomainCondition` struct that represents condition data in our domain model
- **Key Fields**:
  - `condition_id`: String (required) - Unique identifier for the condition
  - `patient_demographic_no`: String (required) - Reference to the patient
  - `encounter_id`: Option<String> - Reference to the encounter
  - `practitioner_id`: Option<String> - Reference to the practitioner
  - `clinical_status`: Option<String> - Clinical status (active, resolved, etc.)
  - `verification_status`: Option<String> - Verification status (confirmed, provisional, etc.)
  - `category`: Option<String> - Category (problem-list-item, encounter-diagnosis)
  - `severity`: Option<String> - Severity level
  - `code`: Option<String> - ICD-10 or SNOMED code
  - `code_display`: Option<String> - Human-readable description of the code
  - `body_site`: Option<String> - Anatomical location
  - `onset_date`: Option<String> - ISO datetime string for onset
  - `onset_age`: Option<String> - Age at onset
  - `onset_description`: Option<String> - Text description of onset
  - `abatement_date`: Option<String> - ISO datetime string when resolved
  - `abatement_age`: Option<String> - Age at abatement
  - `abatement_description`: Option<String> - Text description of abatement
  - `recorded_date`: Option<String> - ISO datetime string when first recorded
  - `stage_summary`: Option<String> - Stage summary
  - `stage_type`: Option<String> - Type of staging
  - `stage_assessment_ids`: Option<Vec<String>> - References to assessment records
  - `notes`: Option<String> - Additional notes
  - `evidence_codes`: Option<Vec<String>> - Supporting evidence codes
  - `evidence_descriptions`: Option<Vec<String>> - Supporting evidence descriptions

### 2. FHIR Adapter
- **File**: `src/adapters/entities/condition.rs`
- **Purpose**: Implements the conversion from `DomainCondition` to FHIR `Condition` proto message
- **Key Features**:
  - Maps domain fields to FHIR Condition structure
  - Handles clinical and verification status conversion
  - Converts datetime strings to FHIR DateTime types
  - Creates proper FHIR references for patient, encounter, and practitioner
  - Handles onset and abatement information (date, age, or description)
  - Maps condition codes with proper terminology systems
  - Handles staging information and evidence
  - Creates proper FHIR participants

### 3. Module Updates
- **File**: `src/domain/mod.rs` - Added `pub mod condition;`
- **File**: `src/adapters/entities/mod.rs` - Added `pub mod condition;`

## FHIR Mapping Details

### Clinical Status
The adapter maps string status values to FHIR clinical status codes:
- "active" → `http://terminology.hl7.org/CodeSystem/condition-clinical` with display "Active"
- "recurrence" → "Recurrence"
- "relapse" → "Relapse"
- "inactive" → "Inactive"
- "remission" → "Remission"
- "resolved" → "Resolved"
- "unknown" → "Unknown"

### Verification Status
The adapter maps string status values to FHIR verification status codes:
- "unconfirmed" → `http://terminology.hl7.org/CodeSystem/condition-ver-status` with display "Unconfirmed"
- "provisional" → "Provisional"
- "differential" → "Differential"
- "confirmed" → "Confirmed"
- "refuted" → "Refuted"
- "entered-in-error" → "Entered in Error"

### Category
The adapter maps string category values to FHIR category codes:
- "problem-list-item" → `http://terminology.hl7.org/CodeSystem/condition-category` with display "Problem List Item"
- "encounter-diagnosis" → "Encounter Diagnosis"

### Condition Code
- System: `http://hl7.org/fhir/sid/icd-10-cm`
- Code: The provided ICD-10 code
- Display: The human-readable description

### Onset and Abatement
The adapter handles three types of onset/abatement information:
1. **DateTime**: ISO datetime strings are converted to FHIR DateTime
2. **Age**: Age strings (e.g., "65 years") are stored as String values
3. **Description**: Text descriptions are stored as String values

### References
- Patient reference: `Patient/{demographic_no}`
- Encounter reference: `Encounter/{encounter_id}`
- Practitioner reference: `Practitioner/{practitioner_id}`
- Assessment references: `Observation/{assessment_id}`

### Identifiers
- System: `urn:arsmedicatech:condition_id`
- Value: The condition ID

### Staging
- Stage summary and type are mapped to CodeableConcept
- Assessment IDs are converted to Observation references

### Evidence
- Evidence codes are mapped to ICD-10 coding system
- Evidence descriptions are used as display values

## Testing

The implementation includes comprehensive unit tests covering:
- Full deserialization with all fields
- Minimal deserialization with only required fields
- Resolved condition scenario
- Staging information scenario
- Error handling for missing required fields

## Usage Example

```rust
use crate::domain::condition::DomainCondition;
use crate::adapters::entities::condition::*;

// Create a domain condition
let domain_condition = DomainCondition {
    condition_id: "cond_12345".to_string(),
    patient_demographic_no: "12345".to_string(),
    encounter_id: Some("enc_001".to_string()),
    practitioner_id: Some("prac_001".to_string()),
    clinical_status: Some("active".to_string()),
    verification_status: Some("confirmed".to_string()),
    category: Some("problem-list-item".to_string()),
    severity: Some("moderate".to_string()),
    code: Some("I25.9".to_string()),
    code_display: Some("Chronic ischemic heart disease, unspecified".to_string()),
    body_site: Some("heart".to_string()),
    onset_date: Some("2023-01-15T00:00:00Z".to_string()),
    onset_age: Some("65 years".to_string()),
    onset_description: Some("Patient reported chest pain".to_string()),
    abatement_date: None,
    abatement_age: None,
    abatement_description: None,
    recorded_date: Some("2023-01-20T10:30:00Z".to_string()),
    stage_summary: Some("Stage 2".to_string()),
    stage_type: Some("TNM".to_string()),
    stage_assessment_ids: Some(vec!["assess_001".to_string(), "assess_002".to_string()]),
    notes: Some("Patient has family history of heart disease".to_string()),
    evidence_codes: Some(vec!["E11.9".to_string(), "Z87.891".to_string()]),
    evidence_descriptions: Some(vec!["Type 2 diabetes mellitus".to_string(), "Personal history of tobacco use".to_string()]),
};

// Convert to FHIR Condition
let fhir_condition: Condition = domain_condition.into();
```

## Clinical Use Cases

### 1. Active Chronic Condition
```rust
let chronic_condition = DomainCondition {
    condition_id: "cond_chronic_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    clinical_status: Some("active".to_string()),
    verification_status: Some("confirmed".to_string()),
    category: Some("problem-list-item".to_string()),
    code: Some("E11.9".to_string()),
    code_display: Some("Type 2 diabetes mellitus without complications".to_string()),
    onset_date: Some("2020-03-15T00:00:00Z".to_string()),
    recorded_date: Some("2020-03-20T14:30:00Z".to_string()),
    notes: Some("Patient on metformin 500mg BID".to_string()),
    ..Default::default()
};
```

### 2. Resolved Acute Condition
```rust
let acute_condition = DomainCondition {
    condition_id: "cond_acute_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    clinical_status: Some("resolved".to_string()),
    verification_status: Some("confirmed".to_string()),
    category: Some("encounter-diagnosis".to_string()),
    code: Some("J06.9".to_string()),
    code_display: Some("Acute upper respiratory infection, unspecified".to_string()),
    onset_date: Some("2023-12-01T00:00:00Z".to_string()),
    abatement_date: Some("2023-12-10T00:00:00Z".to_string()),
    abatement_description: Some("Symptoms resolved after treatment".to_string()),
    recorded_date: Some("2023-12-01T14:30:00Z".to_string()),
    notes: Some("Patient recovered fully with antibiotics".to_string()),
    ..Default::default()
};
```

### 3. Staged Cancer Condition
```rust
let cancer_condition = DomainCondition {
    condition_id: "cond_cancer_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    clinical_status: Some("active".to_string()),
    verification_status: Some("confirmed".to_string()),
    category: Some("problem-list-item".to_string()),
    code: Some("C78.00".to_string()),
    code_display: Some("Secondary malignant neoplasm of unspecified lung".to_string()),
    body_site: Some("lung".to_string()),
    stage_summary: Some("T2N1M0".to_string()),
    stage_type: Some("TNM".to_string()),
    stage_assessment_ids: Some(vec!["path_001".to_string(), "imaging_001".to_string()]),
    recorded_date: Some("2023-11-15T09:00:00Z".to_string()),
    notes: Some("Metastatic disease from primary breast cancer".to_string()),
    ..Default::default()
};
```

## Next Steps

To complete the condition implementation, you may want to:

1. **Add to Service Layer**: Integrate the condition adapter into your gRPC service
2. **Add Database Adapters**: Create database adapters for storing/retrieving conditions
3. **Add Validation**: Implement validation rules for condition data
4. **Add Error Handling**: Enhance error handling for conversion failures
5. **Add Logging**: Add appropriate logging for condition operations
6. **Add ICD-10 Validation**: Validate ICD-10 codes against official terminology
7. **Add SNOMED Support**: Add support for SNOMED CT codes
8. **Add Condition History**: Track condition status changes over time

## Related Entities

This implementation follows the same pattern as the existing `Patient` and `Appointment` entities and can be extended for other FHIR entities like:
- `Encounter`
- `Observation`
- `Medication`
- `Procedure`
- `DiagnosticReport`
- etc.

## Terminology Systems

The implementation uses the following FHIR terminology systems:
- **Clinical Status**: `http://terminology.hl7.org/CodeSystem/condition-clinical`
- **Verification Status**: `http://terminology.hl7.org/CodeSystem/condition-ver-status`
- **Category**: `http://terminology.hl7.org/CodeSystem/condition-category`
- **Condition Codes**: `http://hl7.org/fhir/sid/icd-10-cm`
- **Evidence Codes**: `http://hl7.org/fhir/sid/icd-10-cm`
