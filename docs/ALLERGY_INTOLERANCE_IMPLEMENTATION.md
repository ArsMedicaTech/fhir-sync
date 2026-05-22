# AllergyIntolerance Entity Implementation

This document describes the implementation of the `AllergyIntolerance` entity for the gRPC FHIR synchronization project.

## Files Created/Modified

### 1. Domain Model
- **File**: `src/domain/allergy_intolerance.rs`
- **Purpose**: Defines the `DomainAllergyIntolerance` struct that represents allergy/intolerance data in our domain model
- **Key Fields**:
  - `allergy_id`: String (required) - Unique identifier for the allergy/intolerance
  - `patient_demographic_no`: String (required) - Reference to the patient
  - `clinical_status`: Option<String> - Clinical status (active, inactive, resolved)
  - `verification_status`: Option<String> - Verification status (unconfirmed, presumed, confirmed, refuted, entered-in-error)
  - `allergy_type`: Option<String> - Type (allergy, intolerance)
  - `category`: Option<String> - Category (food, medication, environment, biologic)
  - `criticality`: Option<String> - Criticality level (low, high, unable-to-assess)
  - `substance_code`: Option<String> - SNOMED or other code for the substance
  - `substance_display`: Option<String> - Human-readable name of the substance
  - `substance_system`: Option<String> - Terminology system
  - `onset_date`: Option<String> - ISO datetime string for onset
  - `onset_age`: Option<String> - Age at onset
  - `onset_description`: Option<String> - Text description of onset
  - `recorded_date`: Option<String> - ISO datetime string when first recorded
  - `last_occurrence_date`: Option<String> - ISO datetime string of last known occurrence
  - `recorder_id`: Option<String> - Who recorded the allergy
  - `recorder_type`: Option<String> - Type of recorder (Practitioner, Patient, etc.)
  - `encounter_id`: Option<String> - Encounter when allergy was asserted
  - `reaction_substances`: Option<Vec<String>> - Specific substances that caused reactions
  - `reaction_manifestations`: Option<Vec<String>> - Clinical symptoms/signs
  - `reaction_descriptions`: Option<Vec<String>> - Description of reactions
  - `reaction_onset_dates`: Option<Vec<String>> - When manifestations showed
  - `reaction_severities`: Option<Vec<String>> - Severity levels (mild, moderate, severe)
  - `reaction_exposure_routes`: Option<Vec<String>> - How the subject was exposed
  - `reaction_notes`: Option<Vec<String>> - Additional notes about reactions
  - `notes`: Option<String> - Additional text not captured in other fields

### 2. FHIR Adapter
- **File**: `src/adapters/entities/allergy_intolerance.rs`
- **Purpose**: Implements the conversion from `DomainAllergyIntolerance` to FHIR `AllergyIntolerance` proto message
- **Key Features**:
  - Maps domain fields to FHIR AllergyIntolerance structure
  - Handles clinical and verification status conversion
  - Converts datetime strings to FHIR DateTime types
  - Creates proper FHIR references for patient, practitioner, and encounter
  - Handles onset information (date, age, or description)
  - Maps substance codes with proper terminology systems
  - Handles reaction information with manifestations and severity
  - Creates proper FHIR participants and annotations

### 3. Module Updates
- **File**: `src/domain/mod.rs` - Added `pub mod allergy_intolerance;`
- **File**: `src/adapters/entities/mod.rs` - Added `pub mod allergy_intolerance;`

## FHIR Mapping Details

### Clinical Status
The adapter maps string status values to FHIR clinical status codes:
- "active" → `http://terminology.hl7.org/CodeSystem/allergyintolerance-clinical` with display "Active"
- "inactive" → "Inactive"
- "resolved" → "Resolved"

### Verification Status
The adapter maps string status values to FHIR verification status codes:
- "unconfirmed" → `http://terminology.hl7.org/CodeSystem/allergyintolerance-verification` with display "Unconfirmed"
- "presumed" → "Presumed"
- "confirmed" → "Confirmed"
- "refuted" → "Refuted"
- "entered-in-error" → "Entered in Error"

### Allergy Type
The adapter maps string type values to FHIR allergy intolerance type codes:
- "allergy" → `http://hl7.org/fhir/allergy-intolerance-type` with display "Allergy"
- "intolerance" → "Intolerance"

### Category
The adapter maps string category values to FHIR category codes:
- "food" → `http://hl7.org/fhir/allergy-intolerance-category` with display "Food"
- "medication" → "Medication"
- "environment" → "Environment"
- "biologic" → "Biologic"

### Criticality
The adapter maps string criticality values to FHIR criticality codes:
- "low" → `http://hl7.org/fhir/allergy-intolerance-criticality` with display "Low"
- "high" → "High"
- "unable-to-assess" → "Unable to Assess"

### Substance Code
- System: Defaults to `http://snomed.info/sct` or uses provided system
- Code: The provided substance code
- Display: The human-readable substance name

### Onset Information
The adapter handles three types of onset information:
1. **DateTime**: ISO datetime strings are converted to FHIR DateTime
2. **Age**: Age strings (e.g., "5 years") are stored as String values
3. **Description**: Text descriptions are stored as String values

### References
- Patient reference: `Patient/{demographic_no}`
- Encounter reference: `Encounter/{encounter_id}`
- Recorder reference: `{recorder_type}/{recorder_id}`

### Identifiers
- System: `urn:arsmedicatech:allergy_id`
- Value: The allergy ID

### Reaction Information
- **Substances**: Mapped to reaction.substance
- **Manifestations**: Mapped to reaction.manifestation as CodeableReference
- **Severity**: Mapped to reaction.severity with proper terminology
- **Exposure Routes**: Mapped to reaction.exposure_route
- **Onset Dates**: Converted to FHIR DateTime
- **Notes**: Mapped to reaction.note as Annotation

## Testing

The implementation includes comprehensive unit tests covering:
- Full deserialization with all fields
- Minimal deserialization with only required fields
- Food allergy scenario
- Resolved allergy scenario
- Error handling for missing required fields

## Usage Example

```rust
use crate::domain::allergy_intolerance::DomainAllergyIntolerance;
use crate::adapters::entities::allergy_intolerance::*;

// Create a domain allergy intolerance
let domain_allergy = DomainAllergyIntolerance {
    allergy_id: "allergy_12345".to_string(),
    patient_demographic_no: "12345".to_string(),
    clinical_status: Some("active".to_string()),
    verification_status: Some("confirmed".to_string()),
    allergy_type: Some("allergy".to_string()),
    category: Some("medication".to_string()),
    criticality: Some("high".to_string()),
    substance_code: Some("7980".to_string()),
    substance_display: Some("Penicillin".to_string()),
    substance_system: Some("http://snomed.info/sct".to_string()),
    onset_date: Some("2020-03-15T00:00:00Z".to_string()),
    onset_age: Some("25 years".to_string()),
    onset_description: Some("Patient developed rash after taking penicillin".to_string()),
    recorded_date: Some("2020-03-20T10:30:00Z".to_string()),
    last_occurrence_date: Some("2022-01-10T14:00:00Z".to_string()),
    recorder_id: Some("prac_001".to_string()),
    recorder_type: Some("Practitioner".to_string()),
    encounter_id: Some("enc_001".to_string()),
    reaction_substances: Some(vec!["Penicillin".to_string(), "Amoxicillin".to_string()]),
    reaction_manifestations: Some(vec!["Rash".to_string(), "Hives".to_string(), "Difficulty breathing".to_string()]),
    reaction_descriptions: Some(vec!["Severe allergic reaction with respiratory distress".to_string()]),
    reaction_onset_dates: Some(vec!["2020-03-15T00:00:00Z".to_string(), "2022-01-10T14:00:00Z".to_string()]),
    reaction_severities: Some(vec!["severe".to_string(), "moderate".to_string()]),
    reaction_exposure_routes: Some(vec!["oral".to_string(), "injection".to_string()]),
    reaction_notes: Some(vec!["Required emergency treatment".to_string(), "Patient carries epinephrine".to_string()]),
    notes: Some("Patient has severe penicillin allergy. Avoid all beta-lactam antibiotics.".to_string()),
};

// Convert to FHIR AllergyIntolerance
let fhir_allergy: AllergyIntolerance = domain_allergy.into();
```

## Clinical Use Cases

### 1. Medication Allergy
```rust
let medication_allergy = DomainAllergyIntolerance {
    allergy_id: "allergy_med_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    clinical_status: Some("active".to_string()),
    verification_status: Some("confirmed".to_string()),
    allergy_type: Some("allergy".to_string()),
    category: Some("medication".to_string()),
    criticality: Some("high".to_string()),
    substance_code: Some("7980".to_string()),
    substance_display: Some("Penicillin".to_string()),
    substance_system: Some("http://snomed.info/sct".to_string()),
    onset_date: Some("2020-03-15T00:00:00Z".to_string()),
    recorded_date: Some("2020-03-20T10:30:00Z".to_string()),
    recorder_id: Some("prac_001".to_string()),
    recorder_type: Some("Practitioner".to_string()),
    reaction_substances: Some(vec!["Penicillin".to_string()]),
    reaction_manifestations: Some(vec!["Rash".to_string(), "Hives".to_string()]),
    reaction_severities: Some(vec!["severe".to_string()]),
    reaction_exposure_routes: Some(vec!["oral".to_string()]),
    notes: Some("Patient has severe penicillin allergy. Avoid all beta-lactam antibiotics.".to_string()),
    ..Default::default()
};
```

### 2. Food Allergy
```rust
let food_allergy = DomainAllergyIntolerance {
    allergy_id: "allergy_food_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    clinical_status: Some("active".to_string()),
    verification_status: Some("confirmed".to_string()),
    allergy_type: Some("allergy".to_string()),
    category: Some("food".to_string()),
    criticality: Some("high".to_string()),
    substance_code: Some("762952008".to_string()),
    substance_display: Some("Peanut".to_string()),
    substance_system: Some("http://snomed.info/sct".to_string()),
    onset_age: Some("2 years".to_string()),
    onset_description: Some("Child developed hives and vomiting after eating peanut butter".to_string()),
    recorded_date: Some("2020-05-10T09:00:00Z".to_string()),
    recorder_id: Some("prac_002".to_string()),
    recorder_type: Some("Practitioner".to_string()),
    reaction_substances: Some(vec!["Peanut".to_string(), "Peanut oil".to_string()]),
    reaction_manifestations: Some(vec!["Hives".to_string(), "Vomiting".to_string(), "Swelling of face".to_string()]),
    reaction_severities: Some(vec!["severe".to_string()]),
    reaction_exposure_routes: Some(vec!["ingestion".to_string()]),
    notes: Some("Severe peanut allergy. Patient must avoid all peanut products and carry epinephrine.".to_string()),
    ..Default::default()
};
```

### 3. Resolved Intolerance
```rust
let resolved_intolerance = DomainAllergyIntolerance {
    allergy_id: "allergy_resolved_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    clinical_status: Some("resolved".to_string()),
    verification_status: Some("confirmed".to_string()),
    allergy_type: Some("intolerance".to_string()),
    category: Some("medication".to_string()),
    criticality: Some("low".to_string()),
    substance_code: Some("372665000".to_string()),
    substance_display: Some("Aspirin".to_string()),
    substance_system: Some("http://snomed.info/sct".to_string()),
    onset_date: Some("2018-06-01T00:00:00Z".to_string()),
    onset_description: Some("Patient experienced mild stomach upset".to_string()),
    recorded_date: Some("2018-06-05T14:20:00Z".to_string()),
    recorder_id: Some("prac_003".to_string()),
    recorder_type: Some("Practitioner".to_string()),
    reaction_substances: Some(vec!["Aspirin".to_string()]),
    reaction_manifestations: Some(vec!["Stomach upset".to_string(), "Nausea".to_string()]),
    reaction_severities: Some(vec!["mild".to_string()]),
    reaction_exposure_routes: Some(vec!["oral".to_string()]),
    notes: Some("Patient outgrew aspirin intolerance. No longer relevant.".to_string()),
    ..Default::default()
};
```

### 4. Environmental Allergy
```rust
let environmental_allergy = DomainAllergyIntolerance {
    allergy_id: "allergy_env_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    clinical_status: Some("active".to_string()),
    verification_status: Some("confirmed".to_string()),
    allergy_type: Some("allergy".to_string()),
    category: Some("environment".to_string()),
    criticality: Some("moderate".to_string()),
    substance_code: Some("256350006".to_string()),
    substance_display: Some("Pollen".to_string()),
    substance_system: Some("http://snomed.info/sct".to_string()),
    onset_age: Some("15 years".to_string()),
    onset_description: Some("Seasonal allergies developed in teenage years".to_string()),
    recorded_date: Some("2019-04-15T08:00:00Z".to_string()),
    recorder_id: Some("prac_004".to_string()),
    recorder_type: Some("Practitioner".to_string()),
    reaction_substances: Some(vec!["Pollen".to_string(), "Grass pollen".to_string()]),
    reaction_manifestations: Some(vec!["Sneezing".to_string(), "Runny nose".to_string(), "Itchy eyes".to_string()]),
    reaction_severities: Some(vec!["moderate".to_string()]),
    reaction_exposure_routes: Some(vec!["inhalation".to_string()]),
    notes: Some("Seasonal allergies. Patient takes antihistamines during pollen season.".to_string()),
    ..Default::default()
};
```

## Next Steps

To complete the allergy intolerance implementation, you may want to:

1. **Add to Service Layer**: Integrate the allergy intolerance adapter into your gRPC service
2. **Add Database Adapters**: Create database adapters for storing/retrieving allergy intolerances
3. **Add Validation**: Implement validation rules for allergy intolerance data
4. **Add Error Handling**: Enhance error handling for conversion failures
5. **Add Logging**: Add appropriate logging for allergy intolerance operations
6. **Add Allergy History**: Track allergy status changes over time
7. **Add Cross-Reference Validation**: Validate against medication lists and food databases
8. **Add Alert System**: Integrate with clinical decision support systems

## Related Entities

This implementation follows the same pattern as the existing entities and integrates with:
- **Patient**: The subject of the allergy/intolerance
- **Encounter**: When the allergy/intolerance was asserted
- **Practitioner**: Who recorded the allergy/intolerance
- **Condition**: Related conditions or reactions

## Terminology Systems

The implementation uses the following FHIR terminology systems:
- **Clinical Status**: `http://terminology.hl7.org/CodeSystem/allergyintolerance-clinical`
- **Verification Status**: `http://terminology.hl7.org/CodeSystem/allergyintolerance-verification`
- **Allergy Type**: `http://hl7.org/fhir/allergy-intolerance-type`
- **Category**: `http://hl7.org/fhir/allergy-intolerance-category`
- **Criticality**: `http://hl7.org/fhir/allergy-intolerance-criticality`
- **Reaction Severity**: `http://hl7.org/fhir/reaction-event-severity`
- **Substance Codes**: `http://snomed.info/sct` (default)

## Allergy vs Intolerance

The implementation distinguishes between:
- **Allergy**: An immune system response to a substance
- **Intolerance**: A non-immune system response (e.g., lactose intolerance)

## Criticality Levels

The implementation supports three criticality levels:
- **Low**: Minor reactions that are easily managed
- **High**: Severe reactions that require immediate attention
- **Unable to Assess**: Insufficient information to determine criticality

## Reaction Severity

The implementation supports three reaction severity levels:
- **Mild**: Minor symptoms that don't require treatment
- **Moderate**: Symptoms that require treatment but aren't life-threatening
- **Severe**: Life-threatening symptoms that require immediate treatment

This comprehensive allergy intolerance implementation provides a solid foundation for managing patient allergies and intolerances in your FHIR synchronization system.
