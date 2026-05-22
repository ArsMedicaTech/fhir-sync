# FamilyMemberHistory Entity Implementation

This document describes the implementation of the `FamilyMemberHistory` entity for the gRPC FHIR synchronization project.

## Files Created/Modified

### 1. Domain Model
- **File**: `src/domain/family_member_history.rs`
- **Purpose**: Defines the `DomainFamilyMemberHistory` struct that represents family member history data in our domain model
- **Key Fields**:
  - `family_member_history_id`: String (required) - Unique identifier for the family member history
  - `patient_demographic_no`: String (required) - Reference to the patient
  - `status`: Option<String> - Status (partial, completed, entered-in-error, health-unknown)
  - `data_absent_reason`: Option<String> - Reason for missing data (subject-unknown, withheld, unable-to-obtain, deferred)
  - `data_absent_reason_code`: Option<String> - Code for data absent reason
  - `data_absent_reason_system`: Option<String> - Terminology system for data absent reason
  - `data_absent_reason_display`: Option<String> - Display name for data absent reason
  - `date_recorded`: Option<String> - ISO datetime string when history was recorded or last updated
  - `name`: Option<String> - The family member described
  - `relationship`: Option<String> - Relationship to the subject
  - `relationship_code`: Option<String> - Code for relationship
  - `relationship_system`: Option<String> - Terminology system for relationship
  - `relationship_display`: Option<String> - Display name for relationship
  - `sex`: Option<String> - "male" | "female" | "other" | "unknown"
  - `sex_code`: Option<String> - Code for sex
  - `sex_system`: Option<String> - Terminology system for sex
  - `sex_display`: Option<String> - Display name for sex
  - `born_date`: Option<String> - ISO date string for birth date
  - `born_period_start`: Option<String> - ISO date string for birth period start
  - `born_period_end`: Option<String> - ISO date string for birth period end
  - `born_string`: Option<String> - String description of birth
  - `age_value`: Option<f64> - Age value
  - `age_unit`: Option<String> - Age unit (years, months, days)
  - `age_range_low`: Option<f64> - Age range low value
  - `age_range_high`: Option<f64> - Age range high value
  - `age_range_unit`: Option<String> - Age range unit
  - `age_string`: Option<String> - String description of age
  - `estimated_age`: Option<bool> - Age is estimated?
  - `deceased`: Option<bool> - Dead?
  - `deceased_age_value`: Option<f64> - Age at death
  - `deceased_age_unit`: Option<String> - Age at death unit
  - `deceased_age_range_low`: Option<f64> - Age at death range low
  - `deceased_age_range_high`: Option<f64> - Age at death range high
  - `deceased_age_range_unit`: Option<String> - Age at death range unit
  - `deceased_date`: Option<String> - ISO date string for death date
  - `deceased_string`: Option<String> - String description of death
  - `reason_codes`: Option<Vec<String>> - Why was family member history performed?
  - `reason_code_codes`: Option<Vec<String>> - Codes for reasons
  - `reason_code_systems`: Option<Vec<String>> - Terminology systems for reasons
  - `reason_code_displays`: Option<Vec<String>> - Display names for reasons
  - `reason_reference_ids`: Option<Vec<String>> - References for reasons
  - `reason_reference_types`: Option<Vec<String>> - Types of reason references
  - `participant_function_codes`: Option<Vec<String>> - Type of involvement
  - `participant_function_code_codes`: Option<Vec<String>> - Codes for participant functions
  - `participant_function_code_systems`: Option<Vec<String>> - Terminology systems for functions
  - `participant_function_code_displays`: Option<Vec<String>> - Display names for functions
  - `participant_actor_ids`: Option<Vec<String>> - Who or what participated
  - `participant_actor_types`: Option<Vec<String>> - Types of participants
  - `condition_codes`: Option<Vec<String>> - Condition suffered by relation
  - `condition_code_codes`: Option<Vec<String>> - Codes for conditions
  - `condition_code_systems`: Option<Vec<String>> - Terminology systems for conditions
  - `condition_code_displays`: Option<Vec<String>> - Display names for conditions
  - `condition_outcomes`: Option<Vec<String>> - deceased | permanent disability | etc
  - `condition_outcome_codes`: Option<Vec<String>> - Codes for outcomes
  - `condition_outcome_systems`: Option<Vec<String>> - Terminology systems for outcomes
  - `condition_outcome_displays`: Option<Vec<String>> - Display names for outcomes
  - `condition_contributed_to_death`: Option<Vec<bool>> - Whether the condition contributed to the cause of death
  - `condition_onset_ages`: Option<Vec<f64>> - When condition first manifested (age)
  - `condition_onset_age_units`: Option<Vec<String>> - Age units for onset
  - `condition_onset_age_ranges_low`: Option<Vec<f64>> - Onset age range low
  - `condition_onset_age_ranges_high`: Option<Vec<f64>> - Onset age range high
  - `condition_onset_age_range_units`: Option<Vec<String>> - Onset age range units
  - `condition_onset_periods_start`: Option<Vec<String>> - Onset period start
  - `condition_onset_periods_end`: Option<Vec<String>> - Onset period end
  - `condition_onset_strings`: Option<Vec<String>> - String descriptions of onset
  - `condition_notes`: Option<Vec<String>> - Extra information about condition
  - `procedure_codes`: Option<Vec<String>> - Procedures performed on the related person
  - `procedure_code_codes`: Option<Vec<String>> - Codes for procedures
  - `procedure_code_systems`: Option<Vec<String>> - Terminology systems for procedures
  - `procedure_code_displays`: Option<Vec<String>> - Display names for procedures
  - `procedure_outcomes`: Option<Vec<String>> - What happened following the procedure
  - `procedure_outcome_codes`: Option<Vec<String>> - Codes for procedure outcomes
  - `procedure_outcome_systems`: Option<Vec<String>> - Terminology systems for outcomes
  - `procedure_outcome_displays`: Option<Vec<String>> - Display names for outcomes
  - `procedure_contributed_to_death`: Option<Vec<bool>> - Whether the procedure contributed to the cause of death
  - `procedure_performed_ages`: Option<Vec<f64>> - When the procedure was performed (age)
  - `procedure_performed_age_units`: Option<Vec<String>> - Age units for performed
  - `procedure_performed_age_ranges_low`: Option<Vec<f64>> - Performed age range low
  - `procedure_performed_age_ranges_high`: Option<Vec<f64>> - Performed age range high
  - `procedure_performed_age_range_units`: Option<Vec<String>> - Performed age range units
  - `procedure_performed_periods_start`: Option<Vec<String>> - Performed period start
  - `procedure_performed_periods_end`: Option<Vec<String>> - Performed period end
  - `procedure_performed_dates`: Option<Vec<String>> - ISO datetime string for performed
  - `procedure_performed_strings`: Option<Vec<String>> - String descriptions of performed
  - `procedure_notes`: Option<Vec<String>> - Extra information about the procedure
  - `notes`: Option<String> - General note about related person

### 2. FHIR Adapter
- **File**: `src/adapters/entities/family_member_history.rs`
- **Purpose**: Implements the conversion from `DomainFamilyMemberHistory` to FHIR `FamilyMemberHistory` proto message
- **Key Features**:
  - Maps domain fields to FHIR FamilyMemberHistory structure
  - Handles status conversion with proper codes
  - Converts datetime strings to FHIR DateTime types
  - Creates proper FHIR references for patient and other resources
  - Handles birth information with multiple formats (date, period, string)
  - Maps age information with multiple formats (age, range, string)
  - Handles death information with multiple formats (boolean, age, range, date, string)
  - Maps conditions with onset information and outcomes
  - Maps procedures with performed information and outcomes
  - Handles participants and reason information

### 3. Module Updates
- **File**: `src/domain/mod.rs` - Added `pub mod family_member_history;`
- **File**: `src/adapters/entities/mod.rs` - Added `pub mod family_member_history;`

## FHIR Mapping Details

### Status
The adapter maps string status values to FHIR status codes:
- "partial" → 1
- "completed" → 2
- "entered-in-error" → 3
- "health-unknown" → 4

### Data Absent Reason
The adapter maps string data absent reason values to FHIR CodeableConcept:
- System: Defaults to `http://terminology.hl7.org/CodeSystem/data-absent-reason` or uses provided system
- Code: The provided data absent reason code
- Display: The human-readable data absent reason name
- Text: The provided data absent reason string

### Relationship
The adapter maps string relationship values to FHIR CodeableConcept:
- System: Defaults to `http://terminology.hl7.org/CodeSystem/v3-RoleCode` or uses provided system
- Code: The provided relationship code
- Display: The human-readable relationship name
- Text: The provided relationship string

### Sex
The adapter maps string sex values to FHIR CodeableConcept:
- System: Defaults to `http://hl7.org/fhir/administrative-gender` or uses provided system
- Code: The provided sex code
- Display: The human-readable sex name
- Text: The provided sex string

### References
- Patient reference: `Patient/{demographic_no}`
- Participant actor references: `{actor_type}/{actor_id}`
- Reason reference references: `{reference_type}/{reference_id}`

### Identifiers
- System: `urn:arsmedicatech:family_member_history_id`
- Value: The family member history ID

### Born Information
The adapter handles birth information in three formats:
1. **Date**: Single birth date
2. **Period**: Birth period with start and end dates
3. **String**: String description of birth

### Age Information
The adapter handles age information in three formats:
1. **Age**: Single age value with unit
2. **Range**: Age range with low and high values
3. **String**: String description of age

### Deceased Information
The adapter handles death information in five formats:
1. **Boolean**: Simple deceased/not deceased
2. **Age**: Age at death
3. **Range**: Age range at death
4. **Date**: Date of death
5. **String**: String description of death

### Condition Onset
The adapter handles condition onset information in four formats:
1. **Age**: Age when condition first manifested
2. **Range**: Age range when condition first manifested
3. **Period**: Period when condition first manifested
4. **String**: String description of onset

### Procedure Performed
The adapter handles procedure performed information in five formats:
1. **Age**: Age when procedure was performed
2. **Range**: Age range when procedure was performed
3. **Period**: Period when procedure was performed
4. **DateTime**: Specific date/time when procedure was performed
5. **String**: String description of when procedure was performed

## Testing

The implementation includes comprehensive unit tests covering:
- Full deserialization with all fields
- Minimal deserialization with only required fields
- Father's medical history scenario
- Mother's medical history scenario
- Error handling for missing required fields

## Usage Example

```rust
use crate::domain::family_member_history::DomainFamilyMemberHistory;
use crate::adapters::entities::family_member_history::*;

// Create a domain family member history
let domain_family_member_history = DomainFamilyMemberHistory {
    family_member_history_id: "fmh_12345".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("completed".to_string()),
    data_absent_reason: Some("subject-unknown".to_string()),
    data_absent_reason_code: Some("subject-unknown".to_string()),
    data_absent_reason_system: Some("http://terminology.hl7.org/CodeSystem/data-absent-reason".to_string()),
    data_absent_reason_display: Some("Subject Unknown".to_string()),
    date_recorded: Some("2024-01-15T10:30:00Z".to_string()),
    name: Some("John Smith".to_string()),
    relationship: Some("father".to_string()),
    relationship_code: Some("father".to_string()),
    relationship_system: Some("http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string()),
    relationship_display: Some("Father".to_string()),
    sex: Some("male".to_string()),
    sex_code: Some("M".to_string()),
    sex_system: Some("http://hl7.org/fhir/administrative-gender".to_string()),
    sex_display: Some("Male".to_string()),
    born_date: Some("1950-05-15".to_string()),
    age_value: Some(73.5),
    age_unit: Some("years".to_string()),
    estimated_age: Some(false),
    deceased: Some(true),
    deceased_age_value: Some(72.0),
    deceased_age_unit: Some("years".to_string()),
    deceased_date: Some("2022-12-01".to_string()),
    reason_codes: Some(vec!["genetic-counseling".to_string(), "risk-assessment".to_string()]),
    reason_code_codes: Some(vec!["genetic-counseling".to_string(), "risk-assessment".to_string()]),
    reason_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string(), "http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string()]),
    reason_code_displays: Some(vec!["Genetic Counseling".to_string(), "Risk Assessment".to_string()]),
    participant_function_codes: Some(vec!["informant".to_string()]),
    participant_function_code_codes: Some(vec!["informant".to_string()]),
    participant_function_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ParticipationType".to_string()]),
    participant_function_code_displays: Some(vec!["Informant".to_string()]),
    participant_actor_ids: Some(vec!["pat_12345".to_string()]),
    participant_actor_types: Some(vec!["Patient".to_string()]),
    condition_codes: Some(vec!["diabetes".to_string(), "hypertension".to_string(), "heart-disease".to_string()]),
    condition_code_codes: Some(vec!["E11.9".to_string(), "I10".to_string(), "I25.9".to_string()]),
    condition_code_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    condition_code_displays: Some(vec!["Type 2 diabetes mellitus without complications".to_string(), "Essential hypertension".to_string(), "Chronic ischemic heart disease, unspecified".to_string()]),
    condition_outcomes: Some(vec!["deceased".to_string(), "permanent-disability".to_string(), "deceased".to_string()]),
    condition_outcome_codes: Some(vec!["deceased".to_string(), "permanent-disability".to_string(), "deceased".to_string()]),
    condition_outcome_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/condition-outcome".to_string(), "http://terminology.hl7.org/CodeSystem/condition-outcome".to_string(), "http://terminology.hl7.org/CodeSystem/condition-outcome".to_string()]),
    condition_outcome_displays: Some(vec!["Deceased".to_string(), "Permanent Disability".to_string(), "Deceased".to_string()]),
    condition_contributed_to_death: Some(vec![true, false, true]),
    condition_onset_ages: Some(vec![45.0, 50.0, 60.0]),
    condition_onset_age_units: Some(vec!["years".to_string(), "years".to_string(), "years".to_string()]),
    condition_onset_strings: Some(vec!["Mid-40s".to_string(), "Early 50s".to_string(), "Early 60s".to_string()]),
    condition_notes: Some(vec!["Well-controlled with medication".to_string(), "Required daily medication".to_string(), "Led to heart attack".to_string()]),
    procedure_codes: Some(vec!["coronary-bypass".to_string(), "angioplasty".to_string()]),
    procedure_code_codes: Some(vec!["02100Z0".to_string(), "02703ZZ".to_string()]),
    procedure_code_systems: Some(vec!["http://www.ama-assn.org/go/cpt".to_string(), "http://www.ama-assn.org/go/cpt".to_string()]),
    procedure_code_displays: Some(vec!["Bypass Coronary Artery, One Artery from Coronary Artery".to_string(), "Dilation of Coronary Artery, Three Arteries, Percutaneous Approach".to_string()]),
    procedure_outcomes: Some(vec!["successful".to_string(), "successful".to_string()]),
    procedure_outcome_codes: Some(vec!["successful".to_string(), "successful".to_string()]),
    procedure_outcome_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/procedure-outcome".to_string(), "http://terminology.hl7.org/CodeSystem/procedure-outcome".to_string()]),
    procedure_outcome_displays: Some(vec!["Successful".to_string(), "Successful".to_string()]),
    procedure_contributed_to_death: Some(vec![false, false]),
    procedure_performed_ages: Some(vec![65.0, 68.0]),
    procedure_performed_age_units: Some(vec!["years".to_string(), "years".to_string()]),
    procedure_performed_dates: Some(vec!["2015-03-15T08:00:00Z".to_string(), "2018-07-20T10:30:00Z".to_string()]),
    procedure_notes: Some(vec!["Triple bypass surgery".to_string(), "Stent placement".to_string()]),
    notes: Some("Father had significant cardiovascular history. Died at age 72 from complications of diabetes and heart disease.".to_string()),
};

// Convert to FHIR FamilyMemberHistory
let fhir_family_member_history: FamilyMemberHistory = domain_family_member_history.into();
```

## Clinical Use Cases

### 1. Father's Medical History
```rust
let father_history = DomainFamilyMemberHistory {
    family_member_history_id: "fmh_father_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("completed".to_string()),
    date_recorded: Some("2024-01-15T10:30:00Z".to_string()),
    name: Some("John Smith".to_string()),
    relationship: Some("father".to_string()),
    relationship_code: Some("father".to_string()),
    relationship_system: Some("http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string()),
    relationship_display: Some("Father".to_string()),
    sex: Some("male".to_string()),
    sex_code: Some("M".to_string()),
    sex_system: Some("http://hl7.org/fhir/administrative-gender".to_string()),
    sex_display: Some("Male".to_string()),
    born_date: Some("1950-05-15".to_string()),
    age_value: Some(73.5),
    age_unit: Some("years".to_string()),
    estimated_age: Some(false),
    deceased: Some(true),
    deceased_age_value: Some(72.0),
    deceased_age_unit: Some("years".to_string()),
    deceased_date: Some("2022-12-01".to_string()),
    reason_codes: Some(vec!["genetic-counseling".to_string(), "risk-assessment".to_string()]),
    reason_code_codes: Some(vec!["genetic-counseling".to_string(), "risk-assessment".to_string()]),
    reason_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string(), "http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string()]),
    reason_code_displays: Some(vec!["Genetic Counseling".to_string(), "Risk Assessment".to_string()]),
    participant_function_codes: Some(vec!["informant".to_string()]),
    participant_function_code_codes: Some(vec!["informant".to_string()]),
    participant_function_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ParticipationType".to_string()]),
    participant_function_code_displays: Some(vec!["Informant".to_string()]),
    participant_actor_ids: Some(vec!["pat_12345".to_string()]),
    participant_actor_types: Some(vec!["Patient".to_string()]),
    condition_codes: Some(vec!["diabetes".to_string(), "hypertension".to_string(), "heart-disease".to_string()]),
    condition_code_codes: Some(vec!["E11.9".to_string(), "I10".to_string(), "I25.9".to_string()]),
    condition_code_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    condition_code_displays: Some(vec!["Type 2 diabetes mellitus without complications".to_string(), "Essential hypertension".to_string(), "Chronic ischemic heart disease, unspecified".to_string()]),
    condition_outcomes: Some(vec!["deceased".to_string(), "permanent-disability".to_string(), "deceased".to_string()]),
    condition_outcome_codes: Some(vec!["deceased".to_string(), "permanent-disability".to_string(), "deceased".to_string()]),
    condition_outcome_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/condition-outcome".to_string(), "http://terminology.hl7.org/CodeSystem/condition-outcome".to_string(), "http://terminology.hl7.org/CodeSystem/condition-outcome".to_string()]),
    condition_outcome_displays: Some(vec!["Deceased".to_string(), "Permanent Disability".to_string(), "Deceased".to_string()]),
    condition_contributed_to_death: Some(vec![true, false, true]),
    condition_onset_ages: Some(vec![45.0, 50.0, 60.0]),
    condition_onset_age_units: Some(vec!["years".to_string(), "years".to_string(), "years".to_string()]),
    condition_onset_strings: Some(vec!["Mid-40s".to_string(), "Early 50s".to_string(), "Early 60s".to_string()]),
    condition_notes: Some(vec!["Well-controlled with medication".to_string(), "Required daily medication".to_string(), "Led to heart attack".to_string()]),
    procedure_codes: Some(vec!["coronary-bypass".to_string(), "angioplasty".to_string()]),
    procedure_code_codes: Some(vec!["02100Z0".to_string(), "02703ZZ".to_string()]),
    procedure_code_systems: Some(vec!["http://www.ama-assn.org/go/cpt".to_string(), "http://www.ama-assn.org/go/cpt".to_string()]),
    procedure_code_displays: Some(vec!["Bypass Coronary Artery, One Artery from Coronary Artery".to_string(), "Dilation of Coronary Artery, Three Arteries, Percutaneous Approach".to_string()]),
    procedure_outcomes: Some(vec!["successful".to_string(), "successful".to_string()]),
    procedure_outcome_codes: Some(vec!["successful".to_string(), "successful".to_string()]),
    procedure_outcome_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/procedure-outcome".to_string(), "http://terminology.hl7.org/CodeSystem/procedure-outcome".to_string()]),
    procedure_outcome_displays: Some(vec!["Successful".to_string(), "Successful".to_string()]),
    procedure_contributed_to_death: Some(vec![false, false]),
    procedure_performed_ages: Some(vec![65.0, 68.0]),
    procedure_performed_age_units: Some(vec!["years".to_string(), "years".to_string()]),
    procedure_performed_dates: Some(vec!["2015-03-15T08:00:00Z".to_string(), "2018-07-20T10:30:00Z".to_string()]),
    procedure_notes: Some(vec!["Triple bypass surgery".to_string(), "Stent placement".to_string()]),
    notes: Some("Father had significant cardiovascular history. Died at age 72 from complications of diabetes and heart disease.".to_string()),
    ..Default::default()
};
```

### 2. Mother's Medical History
```rust
let mother_history = DomainFamilyMemberHistory {
    family_member_history_id: "fmh_mother_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("completed".to_string()),
    date_recorded: Some("2024-01-15T10:30:00Z".to_string()),
    name: Some("Mary Smith".to_string()),
    relationship: Some("mother".to_string()),
    relationship_code: Some("mother".to_string()),
    relationship_system: Some("http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string()),
    relationship_display: Some("Mother".to_string()),
    sex: Some("female".to_string()),
    sex_code: Some("F".to_string()),
    sex_system: Some("http://hl7.org/fhir/administrative-gender".to_string()),
    sex_display: Some("Female".to_string()),
    born_date: Some("1955-08-20".to_string()),
    age_value: Some(68.5),
    age_unit: Some("years".to_string()),
    estimated_age: Some(false),
    deceased: Some(false),
    reason_codes: Some(vec!["genetic-counseling".to_string()]),
    reason_code_codes: Some(vec!["genetic-counseling".to_string()]),
    reason_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string()]),
    reason_code_displays: Some(vec!["Genetic Counseling".to_string()]),
    participant_function_codes: Some(vec!["informant".to_string()]),
    participant_function_code_codes: Some(vec!["informant".to_string()]),
    participant_function_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ParticipationType".to_string()]),
    participant_function_code_displays: Some(vec!["Informant".to_string()]),
    participant_actor_ids: Some(vec!["pat_12345".to_string()]),
    participant_actor_types: Some(vec!["Patient".to_string()]),
    condition_codes: Some(vec!["breast-cancer".to_string(), "osteoporosis".to_string()]),
    condition_code_codes: Some(vec!["C50.9".to_string(), "M81.0".to_string()]),
    condition_code_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    condition_code_displays: Some(vec!["Malignant neoplasm of breast, unspecified".to_string(), "Age-related osteoporosis without current pathological fracture".to_string()]),
    condition_outcomes: Some(vec!["recovered".to_string(), "ongoing".to_string()]),
    condition_outcome_codes: Some(vec!["recovered".to_string(), "ongoing".to_string()]),
    condition_outcome_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/condition-outcome".to_string(), "http://terminology.hl7.org/CodeSystem/condition-outcome".to_string()]),
    condition_outcome_displays: Some(vec!["Recovered".to_string(), "Ongoing".to_string()]),
    condition_contributed_to_death: Some(vec![false, false]),
    condition_onset_ages: Some(vec![55.0, 60.0]),
    condition_onset_age_units: Some(vec!["years".to_string(), "years".to_string()]),
    condition_onset_strings: Some(vec!["Mid-50s".to_string(), "Early 60s".to_string()]),
    condition_notes: Some(vec!["Diagnosed at age 55, treated successfully".to_string(), "Diagnosed at age 60, ongoing treatment".to_string()]),
    procedure_codes: Some(vec!["mastectomy".to_string(), "bone-density-scan".to_string()]),
    procedure_code_codes: Some(vec!["19303".to_string(), "77080".to_string()]),
    procedure_code_systems: Some(vec!["http://www.ama-assn.org/go/cpt".to_string(), "http://www.ama-assn.org/go/cpt".to_string()]),
    procedure_code_displays: Some(vec!["Mastectomy, simple, complete".to_string(), "Dual-energy X-ray absorptiometry (DXA), bone density study".to_string()]),
    procedure_outcomes: Some(vec!["successful".to_string(), "successful".to_string()]),
    procedure_outcome_codes: Some(vec!["successful".to_string(), "successful".to_string()]),
    procedure_outcome_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/procedure-outcome".to_string(), "http://terminology.hl7.org/CodeSystem/procedure-outcome".to_string()]),
    procedure_outcome_displays: Some(vec!["Successful".to_string(), "Successful".to_string()]),
    procedure_contributed_to_death: Some(vec![false, false]),
    procedure_performed_ages: Some(vec![55.0, 60.0]),
    procedure_performed_age_units: Some(vec!["years".to_string(), "years".to_string()]),
    procedure_performed_dates: Some(vec!["2010-06-15T08:00:00Z".to_string(), "2015-09-10T10:00:00Z".to_string()]),
    procedure_notes: Some(vec!["Left breast mastectomy".to_string(), "Annual bone density scan".to_string()]),
    notes: Some("Mother is alive and well. Had breast cancer at age 55, treated successfully. Currently has osteoporosis, well-managed with medication.".to_string()),
    ..Default::default()
};
```

### 3. Sibling's Medical History
```rust
let sibling_history = DomainFamilyMemberHistory {
    family_member_history_id: "fmh_sibling_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("completed".to_string()),
    date_recorded: Some("2024-01-15T10:30:00Z".to_string()),
    name: Some("Jane Smith".to_string()),
    relationship: Some("sister".to_string()),
    relationship_code: Some("sister".to_string()),
    relationship_system: Some("http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string()),
    relationship_display: Some("Sister".to_string()),
    sex: Some("female".to_string()),
    sex_code: Some("F".to_string()),
    sex_system: Some("http://hl7.org/fhir/administrative-gender".to_string()),
    sex_display: Some("Female".to_string()),
    born_date: Some("1980-03-10".to_string()),
    age_value: Some(43.5),
    age_unit: Some("years".to_string()),
    estimated_age: Some(false),
    deceased: Some(false),
    reason_codes: Some(vec!["genetic-counseling".to_string()]),
    reason_code_codes: Some(vec!["genetic-counseling".to_string()]),
    reason_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string()]),
    reason_code_displays: Some(vec!["Genetic Counseling".to_string()]),
    participant_function_codes: Some(vec!["informant".to_string()]),
    participant_function_code_codes: Some(vec!["informant".to_string()]),
    participant_function_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ParticipationType".to_string()]),
    participant_function_code_displays: Some(vec!["Informant".to_string()]),
    participant_actor_ids: Some(vec!["pat_12345".to_string()]),
    participant_actor_types: Some(vec!["Patient".to_string()]),
    condition_codes: Some(vec!["migraine".to_string(), "depression".to_string()]),
    condition_code_codes: Some(vec!["G43.9".to_string(), "F32.9".to_string()]),
    condition_code_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    condition_code_displays: Some(vec!["Migraine, unspecified".to_string(), "Major depressive disorder, single episode, unspecified".to_string()]),
    condition_outcomes: Some(vec!["ongoing".to_string(), "ongoing".to_string()]),
    condition_outcome_codes: Some(vec!["ongoing".to_string(), "ongoing".to_string()]),
    condition_outcome_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/condition-outcome".to_string(), "http://terminology.hl7.org/CodeSystem/condition-outcome".to_string()]),
    condition_outcome_displays: Some(vec!["Ongoing".to_string(), "Ongoing".to_string()]),
    condition_contributed_to_death: Some(vec![false, false]),
    condition_onset_ages: Some(vec![25.0, 30.0]),
    condition_onset_age_units: Some(vec!["years".to_string(), "years".to_string()]),
    condition_onset_strings: Some(vec!["Mid-20s".to_string(), "Early 30s".to_string()]),
    condition_notes: Some(vec!["Well-controlled with medication".to_string(), "Managed with therapy and medication".to_string()]),
    notes: Some("Sister is alive and well. Has migraine headaches and depression, both well-managed with treatment.".to_string()),
    ..Default::default()
};
```

### 4. Grandparent's Medical History
```rust
let grandparent_history = DomainFamilyMemberHistory {
    family_member_history_id: "fmh_grandparent_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("completed".to_string()),
    date_recorded: Some("2024-01-15T10:30:00Z".to_string()),
    name: Some("Robert Smith".to_string()),
    relationship: Some("grandfather".to_string()),
    relationship_code: Some("grandfather".to_string()),
    relationship_system: Some("http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string()),
    relationship_display: Some("Grandfather".to_string()),
    sex: Some("male".to_string()),
    sex_code: Some("M".to_string()),
    sex_system: Some("http://hl7.org/fhir/administrative-gender".to_string()),
    sex_display: Some("Male".to_string()),
    born_date: Some("1920-12-25".to_string()),
    age_value: Some(103.0),
    age_unit: Some("years".to_string()),
    estimated_age: Some(false),
    deceased: Some(true),
    deceased_age_value: Some(95.0),
    deceased_age_unit: Some("years".to_string()),
    deceased_date: Some("2015-12-25".to_string()),
    reason_codes: Some(vec!["genetic-counseling".to_string()]),
    reason_code_codes: Some(vec!["genetic-counseling".to_string()]),
    reason_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string()]),
    reason_code_displays: Some(vec!["Genetic Counseling".to_string()]),
    participant_function_codes: Some(vec!["informant".to_string()]),
    participant_function_code_codes: Some(vec!["informant".to_string()]),
    participant_function_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ParticipationType".to_string()]),
    participant_function_code_displays: Some(vec!["Informant".to_string()]),
    participant_actor_ids: Some(vec!["pat_12345".to_string()]),
    participant_actor_types: Some(vec!["Patient".to_string()]),
    condition_codes: Some(vec!["alzheimer-disease".to_string(), "stroke".to_string()]),
    condition_code_codes: Some(vec!["G30.9".to_string(), "I64".to_string()]),
    condition_code_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    condition_code_displays: Some(vec!["Alzheimer's disease, unspecified".to_string(), "Stroke, not specified as hemorrhagic or ischemic".to_string()]),
    condition_outcomes: Some(vec!["deceased".to_string(), "deceased".to_string()]),
    condition_outcome_codes: Some(vec!["deceased".to_string(), "deceased".to_string()]),
    condition_outcome_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/condition-outcome".to_string(), "http://terminology.hl7.org/CodeSystem/condition-outcome".to_string()]),
    condition_outcome_displays: Some(vec!["Deceased".to_string(), "Deceased".to_string()]),
    condition_contributed_to_death: Some(vec![true, true]),
    condition_onset_ages: Some(vec![85.0, 90.0]),
    condition_onset_age_units: Some(vec!["years".to_string(), "years".to_string()]),
    condition_onset_strings: Some(vec!["Mid-80s".to_string(), "Early 90s".to_string()]),
    condition_notes: Some(vec!["Progressive dementia".to_string(), "Led to death".to_string()]),
    notes: Some("Grandfather died at age 95. Had Alzheimer's disease and stroke in his later years.".to_string()),
    ..Default::default()
};
```

## Next Steps

To complete the family member history implementation, you may want to:

1. **Add to Service Layer**: Integrate the family member history adapter into your gRPC service
2. **Add Database Adapters**: Create database adapters for storing/retrieving family member history
3. **Add Validation**: Implement validation rules for family member history data
4. **Add Error Handling**: Enhance error handling for conversion failures
5. **Add Logging**: Add appropriate logging for family member history operations
6. **Add Genetic Counseling**: Integrate with genetic counseling systems
7. **Add Risk Assessment**: Implement risk assessment algorithms
8. **Add Family Tree Visualization**: Create family tree visualization tools
9. **Add Pedigree Analysis**: Implement pedigree analysis features
10. **Add Family History Analytics**: Analyze family history patterns and trends

## Related Entities

This implementation follows the same pattern as the existing entities and integrates with:
- **Patient**: The subject of the family member history
- **Practitioner**: Who recorded the family member history
- **RelatedPerson**: The family member being described
- **Condition**: Conditions suffered by family members
- **Procedure**: Procedures performed on family members
- **Observation**: Observations about family members

## Family Relationships

The implementation supports various family relationships:
- **Parent**: Father, Mother
- **Sibling**: Brother, Sister
- **Grandparent**: Grandfather, Grandmother
- **Uncle/Aunt**: Uncle, Aunt
- **Cousin**: Cousin
- **Child**: Son, Daughter
- **Grandchild**: Grandson, Granddaughter
- **Spouse**: Husband, Wife
- **Other**: Step-parent, Step-sibling, Half-sibling, Adopted

## Data Absent Reasons

The implementation supports various data absent reasons:
- **Subject Unknown**: Family member is unknown
- **Withheld**: Information is withheld for privacy
- **Unable to Obtain**: Information cannot be obtained
- **Deferred**: Information collection is deferred

## Condition Outcomes

The implementation supports various condition outcomes:
- **Deceased**: Family member died from the condition
- **Permanent Disability**: Condition caused permanent disability
- **Recovered**: Family member recovered from the condition
- **Ongoing**: Condition is ongoing
- **Unknown**: Outcome is unknown

## Procedure Outcomes

The implementation supports various procedure outcomes:
- **Successful**: Procedure was successful
- **Unsuccessful**: Procedure was unsuccessful
- **Complications**: Procedure had complications
- **Unknown**: Outcome is unknown

## Age Formats

The implementation supports various age formats:
- **Exact Age**: Single age value with unit
- **Age Range**: Range of ages with low and high values
- **String Description**: Text description of age (e.g., "Mid-40s", "Early 50s")

## Birth Formats

The implementation supports various birth formats:
- **Exact Date**: Single birth date
- **Birth Period**: Range of birth dates
- **String Description**: Text description of birth (e.g., "Early 1950s", "Mid-1960s")

## Death Formats

The implementation supports various death formats:
- **Boolean**: Simple deceased/not deceased
- **Age at Death**: Age when death occurred
- **Age Range at Death**: Range of ages when death occurred
- **Date of Death**: Specific date of death
- **String Description**: Text description of death

This comprehensive family member history implementation provides a solid foundation for managing family medical history in your FHIR synchronization system, enabling genetic counseling, risk assessment, and family health management.
