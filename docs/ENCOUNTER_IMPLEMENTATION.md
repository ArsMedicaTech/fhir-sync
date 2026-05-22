# Encounter Entity Implementation

This document describes the implementation of the `Encounter` entity for the gRPC FHIR synchronization project.

## Files Created/Modified

### 1. Domain Model
- **File**: `src/domain/encounter.rs`
- **Purpose**: Defines the `DomainEncounter` struct that represents encounter data in our domain model
- **Key Fields**:
  - `encounter_id`: String (required) - Unique identifier for the encounter
  - `patient_demographic_no`: String (required) - Reference to the patient
  - `status`: Option<String> - Encounter status (planned, in-progress, completed, etc.)
  - `class_code`: Option<String> - Encounter class (inpatient, outpatient, emergency, etc.)
  - `priority`: Option<String> - Priority level
  - `encounter_type`: Option<String> - Specific type of encounter
  - `service_type`: Option<String> - Specific type of service
  - `planned_start_date`: Option<String> - ISO datetime string for planned start
  - `planned_end_date`: Option<String> - ISO datetime string for planned end
  - `actual_start_date`: Option<String> - ISO datetime string for actual start
  - `actual_end_date`: Option<String> - ISO datetime string for actual end
  - `length_minutes`: Option<u32> - Duration in minutes
  - `practitioner_id`: Option<String> - Primary practitioner
  - `location_id`: Option<String> - Primary location
  - `service_provider_id`: Option<String> - Organization/facility
  - `appointment_id`: Option<String> - Related appointment
  - `part_of_encounter_id`: Option<String> - Parent encounter
  - `episode_of_care_id`: Option<String> - Episode of care
  - `reason_codes`: Option<Vec<String>> - Reason codes for the encounter
  - `reason_descriptions`: Option<Vec<String>> - Reason descriptions
  - `diagnosis_codes`: Option<Vec<String>> - Diagnosis codes
  - `diagnosis_descriptions`: Option<Vec<String>> - Diagnosis descriptions
  - `diagnosis_ranks`: Option<Vec<u32>> - Diagnosis ranking
  - `subject_status`: Option<String> - Patient status during encounter
  - `diet_preferences`: Option<Vec<String>> - Diet preferences
  - `special_arrangements`: Option<Vec<String>> - Special arrangements
  - `special_courtesies`: Option<Vec<String>> - Special courtesies
  - `admission_source`: Option<String> - Source of admission
  - `admission_diagnosis`: Option<String> - Admission diagnosis
  - `discharge_disposition`: Option<String> - Discharge disposition
  - `discharge_diagnosis`: Option<String> - Discharge diagnosis
  - `notes`: Option<String> - Additional notes
  - `care_team_ids`: Option<Vec<String>> - Care team member IDs
  - `account_ids`: Option<Vec<String>> - Billing account IDs

### 2. FHIR Adapter
- **File**: `src/adapters/entities/encounter.rs`
- **Purpose**: Implements the conversion from `DomainEncounter` to FHIR `Encounter` proto message
- **Key Features**:
  - Maps domain fields to FHIR Encounter structure
  - Handles encounter status conversion
  - Converts datetime strings to FHIR DateTime types
  - Creates proper FHIR references for patient, practitioner, location, and organization
  - Handles planned and actual periods
  - Maps encounter class with proper terminology systems
  - Handles participants, reasons, and diagnoses
  - Creates proper FHIR duration from minutes
  - Handles admission and discharge information

### 3. Module Updates
- **File**: `src/domain/mod.rs` - Added `pub mod encounter;`
- **File**: `src/adapters/entities/mod.rs` - Added `pub mod encounter;`

## FHIR Mapping Details

### Encounter Status
The adapter maps string status values to FHIR encounter status codes:
- "planned" → `EncounterStatusCode::Planned`
- "in-progress" → `EncounterStatusCode::InProgress`
- "on-hold" → `EncounterStatusCode::OnHold`
- "discharged" → `EncounterStatusCode::Discharged`
- "completed" → `EncounterStatusCode::Completed`
- "cancelled" → `EncounterStatusCode::Cancelled`
- "discontinued" → `EncounterStatusCode::Discontinued`
- "entered-in-error" → `EncounterStatusCode::EnteredInError`
- "unknown" → `EncounterStatusCode::Unknown`

### Encounter Class
The adapter maps string class values to FHIR encounter class codes:
- "inpatient" → `http://terminology.hl7.org/CodeSystem/v3-ActCode` with display "Inpatient"
- "outpatient" → "Outpatient"
- "emergency" → "Emergency"
- "ambulatory" → "Ambulatory"
- "wellness" → "Wellness"
- "urgentcare" → "Urgent Care"
- "virtual" → "Virtual"

### References
- Patient reference: `Patient/{demographic_no}`
- Practitioner reference: `Practitioner/{practitioner_id}`
- Location reference: `Location/{location_id}`
- Organization reference: `Organization/{service_provider_id}`
- Appointment reference: `Appointment/{appointment_id}`
- Encounter reference: `Encounter/{part_of_encounter_id}`
- Episode of Care reference: `EpisodeOfCare/{episode_of_care_id}`
- Care Team references: `CareTeam/{care_team_id}`
- Account references: `Account/{account_id}`

### Identifiers
- System: `urn:arsmedicatech:encounter_id`
- Value: The encounter ID

### Temporal Information
- **Planned Dates**: Converted to FHIR DateTime
- **Actual Period**: Created as FHIR Period with start and end dates
- **Duration**: Converted from minutes to FHIR Duration with unit "min"

### Participants
- **Primary Practitioner**: Mapped as participant with type "Primary Practitioner"
- **Location**: Mapped as participant with type "Location"

### Reasons and Diagnoses
- **Reason Codes**: Mapped to ICD-10 coding system
- **Diagnosis Codes**: Mapped to Condition references with ranking
- **Diagnosis Ranking**: 1 = primary, 2 = secondary, etc.

### Admission Information
- **Admission Source**: Mapped to admission.source
- **Admission Diagnosis**: Mapped to admission.diagnosis as Condition reference
- **Discharge Disposition**: Mapped to admission.destination as Location reference

## Testing

The implementation includes comprehensive unit tests covering:
- Full deserialization with all fields
- Minimal deserialization with only required fields
- Inpatient encounter scenario
- Virtual consultation scenario
- Error handling for missing required fields

## Usage Example

```rust
use crate::domain::encounter::DomainEncounter;
use crate::adapters::entities::encounter::*;

// Create a domain encounter
let domain_encounter = DomainEncounter {
    encounter_id: "enc_12345".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("completed".to_string()),
    class_code: Some("outpatient".to_string()),
    priority: Some("routine".to_string()),
    encounter_type: Some("consultation".to_string()),
    service_type: Some("cardiology".to_string()),
    planned_start_date: Some("2024-01-15T10:00:00Z".to_string()),
    planned_end_date: Some("2024-01-15T10:30:00Z".to_string()),
    actual_start_date: Some("2024-01-15T10:05:00Z".to_string()),
    actual_end_date: Some("2024-01-15T10:35:00Z".to_string()),
    length_minutes: Some(30),
    practitioner_id: Some("prac_001".to_string()),
    location_id: Some("loc_001".to_string()),
    service_provider_id: Some("org_001".to_string()),
    appointment_id: Some("apt_001".to_string()),
    reason_codes: Some(vec!["Z00.00".to_string(), "I25.9".to_string()]),
    reason_descriptions: Some(vec!["Encounter for general adult medical examination".to_string(), "Chronic ischemic heart disease".to_string()]),
    diagnosis_codes: Some(vec!["I25.9".to_string(), "E11.9".to_string()]),
    diagnosis_descriptions: Some(vec!["Chronic ischemic heart disease, unspecified".to_string(), "Type 2 diabetes mellitus without complications".to_string()]),
    diagnosis_ranks: Some(vec![1, 2]),
    subject_status: Some("active".to_string()),
    diet_preferences: Some(vec!["diabetic".to_string()]),
    special_arrangements: Some(vec!["wheelchair".to_string()]),
    notes: Some("Patient stable, follow-up in 3 months".to_string()),
    care_team_ids: Some(vec!["nurse_001".to_string(), "tech_001".to_string()]),
    account_ids: Some(vec!["acct_001".to_string()]),
    ..Default::default()
};

// Convert to FHIR Encounter
let fhir_encounter: Encounter = domain_encounter.into();
```

## Clinical Use Cases

### 1. Outpatient Consultation
```rust
let outpatient_encounter = DomainEncounter {
    encounter_id: "enc_outpatient_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("completed".to_string()),
    class_code: Some("outpatient".to_string()),
    encounter_type: Some("consultation".to_string()),
    service_type: Some("cardiology".to_string()),
    planned_start_date: Some("2024-01-15T10:00:00Z".to_string()),
    actual_start_date: Some("2024-01-15T10:05:00Z".to_string()),
    actual_end_date: Some("2024-01-15T10:35:00Z".to_string()),
    length_minutes: Some(30),
    practitioner_id: Some("prac_001".to_string()),
    location_id: Some("loc_001".to_string()),
    service_provider_id: Some("org_001".to_string()),
    appointment_id: Some("apt_001".to_string()),
    reason_codes: Some(vec!["Z00.00".to_string()]),
    reason_descriptions: Some(vec!["Encounter for general adult medical examination".to_string()]),
    diagnosis_codes: Some(vec!["I25.9".to_string()]),
    diagnosis_descriptions: Some(vec!["Chronic ischemic heart disease, unspecified".to_string()]),
    diagnosis_ranks: Some(vec![1]),
    notes: Some("Patient stable, follow-up in 3 months".to_string()),
    ..Default::default()
};
```

### 2. Inpatient Admission
```rust
let inpatient_encounter = DomainEncounter {
    encounter_id: "enc_inpatient_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("discharged".to_string()),
    class_code: Some("inpatient".to_string()),
    encounter_type: Some("emergency".to_string()),
    service_type: Some("cardiology".to_string()),
    planned_start_date: Some("2024-01-10T08:00:00Z".to_string()),
    actual_start_date: Some("2024-01-10T08:15:00Z".to_string()),
    actual_end_date: Some("2024-01-12T14:30:00Z".to_string()),
    length_minutes: Some(3240),
    practitioner_id: Some("prac_002".to_string()),
    location_id: Some("loc_002".to_string()),
    service_provider_id: Some("org_001".to_string()),
    admission_source: Some("emergency".to_string()),
    admission_diagnosis: Some("Acute myocardial infarction".to_string()),
    discharge_disposition: Some("home".to_string()),
    discharge_diagnosis: Some("Acute ST elevation myocardial infarction, anterior wall".to_string()),
    diagnosis_codes: Some(vec!["I21.01".to_string()]),
    diagnosis_descriptions: Some(vec!["ST elevation myocardial infarction involving left anterior descending artery".to_string()]),
    diagnosis_ranks: Some(vec![1]),
    notes: Some("Patient underwent PCI, stable for discharge".to_string()),
    ..Default::default()
};
```

### 3. Virtual Consultation
```rust
let virtual_encounter = DomainEncounter {
    encounter_id: "enc_virtual_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("completed".to_string()),
    class_code: Some("virtual".to_string()),
    encounter_type: Some("telemedicine".to_string()),
    service_type: Some("psychiatry".to_string()),
    planned_start_date: Some("2024-01-20T14:00:00Z".to_string()),
    actual_start_date: Some("2024-01-20T14:02:00Z".to_string()),
    actual_end_date: Some("2024-01-20T14:47:00Z".to_string()),
    length_minutes: Some(45),
    practitioner_id: Some("prac_003".to_string()),
    service_provider_id: Some("org_002".to_string()),
    appointment_id: Some("apt_virtual_001".to_string()),
    reason_codes: Some(vec!["Z71.1".to_string()]),
    reason_descriptions: Some(vec!["Person with feared health complaint in whom no diagnosis is made".to_string()]),
    subject_status: Some("active".to_string()),
    notes: Some("Virtual consultation via secure video platform".to_string()),
    ..Default::default()
};
```

### 4. Emergency Department Visit
```rust
let emergency_encounter = DomainEncounter {
    encounter_id: "enc_emergency_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("completed".to_string()),
    class_code: Some("emergency".to_string()),
    encounter_type: Some("emergency".to_string()),
    service_type: Some("emergency_medicine".to_string()),
    priority: Some("urgent".to_string()),
    actual_start_date: Some("2024-01-25T22:30:00Z".to_string()),
    actual_end_date: Some("2024-01-26T02:15:00Z".to_string()),
    length_minutes: Some(225),
    practitioner_id: Some("prac_004".to_string()),
    location_id: Some("loc_emergency".to_string()),
    service_provider_id: Some("org_001".to_string()),
    reason_codes: Some(vec!["R50.9".to_string()]),
    reason_descriptions: Some(vec!["Fever, unspecified".to_string()]),
    diagnosis_codes: Some(vec!["J06.9".to_string()]),
    diagnosis_descriptions: Some(vec!["Acute upper respiratory infection, unspecified".to_string()]),
    diagnosis_ranks: Some(vec![1]),
    subject_status: Some("active".to_string()),
    notes: Some("Patient treated for fever, discharged with antibiotics".to_string()),
    ..Default::default()
};
```

## Next Steps

To complete the encounter implementation, you may want to:

1. **Add to Service Layer**: Integrate the encounter adapter into your gRPC service
2. **Add Database Adapters**: Create database adapters for storing/retrieving encounters
3. **Add Validation**: Implement validation rules for encounter data
4. **Add Error Handling**: Enhance error handling for conversion failures
5. **Add Logging**: Add appropriate logging for encounter operations
6. **Add Encounter History**: Track encounter status changes over time
7. **Add Location Management**: Integrate with location management system
8. **Add Care Team Management**: Integrate with care team management system

## Related Entities

This implementation follows the same pattern as the existing entities and integrates with:
- **Patient**: The subject of the encounter
- **Appointment**: The scheduled appointment that led to the encounter
- **Condition**: Diagnoses made during the encounter
- **Practitioner**: Healthcare providers involved in the encounter
- **Location**: Where the encounter took place
- **Organization**: The service provider organization

## Terminology Systems

The implementation uses the following FHIR terminology systems:
- **Encounter Class**: `http://terminology.hl7.org/CodeSystem/v3-ActCode`
- **Reason Codes**: `http://hl7.org/fhir/sid/icd-10-cm`
- **Diagnosis Codes**: `http://hl7.org/fhir/sid/icd-10-cm`

## Encounter Lifecycle

The implementation supports the complete encounter lifecycle:
1. **Planned**: Encounter is scheduled
2. **In-Progress**: Encounter is currently happening
3. **On-Hold**: Encounter is temporarily paused
4. **Completed**: Encounter has finished successfully
5. **Discharged**: Patient has been discharged
6. **Cancelled**: Encounter was cancelled
7. **Discontinued**: Encounter was stopped early
8. **Entered-in-Error**: Encounter was created by mistake
9. **Unknown**: Status is not known

This comprehensive encounter implementation provides a solid foundation for managing healthcare encounters in your FHIR synchronization system.
