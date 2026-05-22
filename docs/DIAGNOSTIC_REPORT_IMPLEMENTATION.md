# DiagnosticReport Entity Implementation

This document describes the implementation of the `DiagnosticReport` entity for the gRPC FHIR synchronization project.

## Files Created/Modified

### 1. Domain Model
- **File**: `src/domain/diagnostic_report.rs`
- **Purpose**: Defines the `DomainDiagnosticReport` struct that represents diagnostic report data in our domain model
- **Key Fields**:
  - `diagnostic_report_id`: String (required) - Unique identifier for the diagnostic report
  - `patient_demographic_no`: String (required) - Reference to the patient
  - `status`: Option<String> - Status (registered, partial, preliminary, modified, final, amended, corrected, appended, cancelled, entered-in-error, unknown)
  - `category`: Option<String> - Service category
  - `category_code`: Option<String> - Code for category
  - `category_system`: Option<String> - Terminology system for category
  - `category_display`: Option<String> - Display name for category
  - `code`: Option<String> - Name/Code for this diagnostic report
  - `code_code`: Option<String> - Code for the diagnostic report
  - `code_system`: Option<String> - Terminology system for code
  - `code_display`: Option<String> - Display name for code
  - `effective_date`: Option<String> - ISO datetime string for report time
  - `effective_period_start`: Option<String> - ISO datetime string for report period start
  - `effective_period_end`: Option<String> - ISO datetime string for report period end
  - `issued_date`: Option<String> - ISO datetime string when report was issued
  - `encounter_id`: Option<String> - Health care event when test ordered
  - `performer_ids`: Option<Vec<String>> - Responsible Diagnostic Service
  - `performer_types`: Option<Vec<String>> - Types of performers
  - `results_interpreter_ids`: Option<Vec<String>> - Primary result interpreter
  - `results_interpreter_types`: Option<Vec<String>> - Types of interpreters
  - `based_on_ids`: Option<Vec<String>> - What was requested
  - `based_on_types`: Option<Vec<String>> - Types of based on references
  - `specimen_ids`: Option<Vec<String>> - Specimens this report is based on
  - `result_observation_ids`: Option<Vec<String>> - Observations
  - `study_ids`: Option<Vec<String>> - Reference to full details of an analysis
  - `study_types`: Option<Vec<String>> - Types of studies (GenomicStudy, ImagingStudy)
  - `supporting_info_types`: Option<Vec<String>> - Supporting information role codes
  - `supporting_info_type_codes`: Option<Vec<String>> - Codes for supporting info types
  - `supporting_info_type_systems`: Option<Vec<String>> - Terminology systems for types
  - `supporting_info_type_displays`: Option<Vec<String>> - Display names for types
  - `supporting_info_reference_ids`: Option<Vec<String>> - Supporting information references
  - `supporting_info_reference_types`: Option<Vec<String>> - Types of supporting info references
  - `media_comments`: Option<Vec<String>> - Comments about the image or data
  - `media_link_ids`: Option<Vec<String>> - Reference to the image or data source
  - `presented_form_ids`: Option<Vec<String>> - Entire report as issued
  - `conclusion`: Option<String> - Clinical conclusion (interpretation) of test results
  - `conclusion_codes`: Option<Vec<String>> - Codes for the clinical conclusion
  - `conclusion_code_codes`: Option<Vec<String>> - Codes for conclusion codes
  - `conclusion_code_systems`: Option<Vec<String>> - Terminology systems for conclusion codes
  - `conclusion_code_displays`: Option<Vec<String>> - Display names for conclusion codes
  - `composition_id`: Option<String> - Reference to a Composition resource
  - `notes`: Option<String> - Comments about the diagnostic report

### 2. FHIR Adapter
- **File**: `src/adapters/entities/diagnostic_report.rs`
- **Purpose**: Implements the conversion from `DomainDiagnosticReport` to FHIR `DiagnosticReport` proto message
- **Key Features**:
  - Maps domain fields to FHIR DiagnosticReport structure
  - Handles status conversion with proper codes
  - Converts datetime strings to FHIR DateTime and Instant types
  - Creates proper FHIR references for patient, practitioner, encounter, and other resources
  - Handles supporting information with proper terminology systems
  - Maps media and attachments for images and documents
  - Creates proper FHIR conclusion codes and clinical interpretations

### 3. Module Updates
- **File**: `src/domain/mod.rs` - Added `pub mod diagnostic_report;`
- **File**: `src/adapters/entities/mod.rs` - Added `pub mod diagnostic_report;`

## FHIR Mapping Details

### Status
The adapter maps string status values to FHIR status codes:
- "registered" → 1
- "partial" → 2
- "preliminary" → 3
- "modified" → 4
- "final" → 5
- "amended" → 6
- "corrected" → 7
- "appended" → 8
- "cancelled" → 9
- "entered-in-error" → 10
- "unknown" → 11

### Category
The adapter maps string category values to FHIR CodeableConcept:
- System: Defaults to `http://terminology.hl7.org/CodeSystem/v2-0074` or uses provided system
- Code: The provided category code
- Display: The human-readable category name
- Text: The provided category string

### Code
The adapter maps string code values to FHIR CodeableConcept:
- System: Defaults to `http://loinc.org` or uses provided system
- Code: The provided code
- Display: The human-readable code name
- Text: The provided code string

### References
- Patient reference: `Patient/{demographic_no}`
- Encounter reference: `Encounter/{encounter_id}`
- Performer references: `{performer_type}/{performer_id}`
- Results interpreter references: `{interpreter_type}/{interpreter_id}`
- Based on references: `{based_on_type}/{based_on_id}`
- Specimen references: `Specimen/{specimen_id}`
- Result observation references: `Observation/{observation_id}`
- Study references: `{study_type}/{study_id}`
- Supporting info references: `{reference_type}/{reference_id}`
- Media link references: `DocumentReference/{link_id}`
- Composition reference: `Composition/{composition_id}`

### Identifiers
- System: `urn:arsmedicatech:diagnostic_report_id`
- Value: The diagnostic report ID

### Effective Time
The adapter handles temporal information in two ways:
1. **DateTime**: Single point in time for report
2. **Period**: Start and end times for report period

### Issued Time
The adapter converts issued date to FHIR Instant:
- ISO datetime strings are converted to FHIR Instant

### Supporting Information
The adapter handles supporting information:
- Type: Supporting information role code with proper terminology
- Reference: Reference to supporting resources (Procedure, Observation, DiagnosticReport, Citation)

### Media
The adapter handles media information:
- Comment: Comments about the image or data
- Link: Reference to DocumentReference containing the media

### Conclusion
The adapter handles clinical conclusions:
- Conclusion: Markdown text of the clinical interpretation
- Conclusion Codes: CodeableConcepts with proper terminology systems

## Testing

The implementation includes comprehensive unit tests covering:
- Full deserialization with all fields
- Minimal deserialization with only required fields
- Laboratory report scenario
- Radiology report scenario
- Pathology report scenario
- Error handling for missing required fields

## Usage Example

```rust
use crate::domain::diagnostic_report::DomainDiagnosticReport;
use crate::adapters::entities::diagnostic_report::*;

// Create a domain diagnostic report
let domain_report = DomainDiagnosticReport {
    diagnostic_report_id: "dr_12345".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("final".to_string()),
    category: Some("laboratory".to_string()),
    category_code: Some("LAB".to_string()),
    category_system: Some("http://terminology.hl7.org/CodeSystem/v2-0074".to_string()),
    category_display: Some("Laboratory".to_string()),
    code: Some("complete-blood-count".to_string()),
    code_code: Some("CBC".to_string()),
    code_system: Some("http://loinc.org".to_string()),
    code_display: Some("Complete Blood Count".to_string()),
    effective_date: Some("2024-01-15T10:30:00Z".to_string()),
    issued_date: Some("2024-01-15T11:00:00Z".to_string()),
    encounter_id: Some("enc_001".to_string()),
    performer_ids: Some(vec!["prac_001".to_string(), "lab_001".to_string()]),
    performer_types: Some(vec!["Practitioner".to_string(), "Organization".to_string()]),
    results_interpreter_ids: Some(vec!["prac_002".to_string()]),
    results_interpreter_types: Some(vec!["Practitioner".to_string()]),
    based_on_ids: Some(vec!["sr_001".to_string()]),
    based_on_types: Some(vec!["ServiceRequest".to_string()]),
    specimen_ids: Some(vec!["spec_001".to_string(), "spec_002".to_string()]),
    result_observation_ids: Some(vec!["obs_001".to_string(), "obs_002".to_string(), "obs_003".to_string()]),
    study_ids: Some(vec!["study_001".to_string()]),
    study_types: Some(vec!["ImagingStudy".to_string()]),
    supporting_info_types: Some(vec!["procedure".to_string(), "observation".to_string()]),
    supporting_info_type_codes: Some(vec!["procedure".to_string(), "observation".to_string()]),
    supporting_info_type_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v2-0074".to_string(), "http://terminology.hl7.org/CodeSystem/v2-0074".to_string()]),
    supporting_info_type_displays: Some(vec!["Procedure".to_string(), "Observation".to_string()]),
    supporting_info_reference_ids: Some(vec!["proc_001".to_string(), "obs_004".to_string()]),
    supporting_info_reference_types: Some(vec!["Procedure".to_string(), "Observation".to_string()]),
    media_comments: Some(vec!["Blood smear image".to_string(), "Cell morphology".to_string()]),
    media_link_ids: Some(vec!["doc_001".to_string(), "doc_002".to_string()]),
    presented_form_ids: Some(vec!["att_001".to_string()]),
    conclusion: Some("Complete blood count shows mild anemia with low hemoglobin and hematocrit. White blood cell count and platelet count are within normal limits.".to_string()),
    conclusion_codes: Some(vec!["anemia".to_string(), "low-hemoglobin".to_string()]),
    conclusion_code_codes: Some(vec!["D64.9".to_string(), "R71".to_string()]),
    conclusion_code_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    conclusion_code_displays: Some(vec!["Anemia, unspecified".to_string(), "Abnormality of red blood cells".to_string()]),
    composition_id: Some("comp_001".to_string()),
    notes: Some("Patient should follow up with primary care physician for anemia workup.".to_string()),
};

// Convert to FHIR DiagnosticReport
let fhir_report: DiagnosticReport = domain_report.into();
```

## Clinical Use Cases

### 1. Laboratory Report
```rust
let lab_report = DomainDiagnosticReport {
    diagnostic_report_id: "dr_lab_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("final".to_string()),
    category: Some("laboratory".to_string()),
    category_code: Some("LAB".to_string()),
    category_system: Some("http://terminology.hl7.org/CodeSystem/v2-0074".to_string()),
    category_display: Some("Laboratory".to_string()),
    code: Some("complete-blood-count".to_string()),
    code_code: Some("CBC".to_string()),
    code_system: Some("http://loinc.org".to_string()),
    code_display: Some("Complete Blood Count".to_string()),
    effective_date: Some("2024-01-15T10:30:00Z".to_string()),
    issued_date: Some("2024-01-15T11:00:00Z".to_string()),
    encounter_id: Some("enc_001".to_string()),
    performer_ids: Some(vec!["prac_001".to_string(), "lab_001".to_string()]),
    performer_types: Some(vec!["Practitioner".to_string(), "Organization".to_string()]),
    results_interpreter_ids: Some(vec!["prac_002".to_string()]),
    results_interpreter_types: Some(vec!["Practitioner".to_string()]),
    based_on_ids: Some(vec!["sr_001".to_string()]),
    based_on_types: Some(vec!["ServiceRequest".to_string()]),
    specimen_ids: Some(vec!["spec_001".to_string(), "spec_002".to_string()]),
    result_observation_ids: Some(vec!["obs_001".to_string(), "obs_002".to_string(), "obs_003".to_string()]),
    study_ids: Some(vec!["study_001".to_string()]),
    study_types: Some(vec!["ImagingStudy".to_string()]),
    supporting_info_types: Some(vec!["procedure".to_string(), "observation".to_string()]),
    supporting_info_type_codes: Some(vec!["procedure".to_string(), "observation".to_string()]),
    supporting_info_type_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v2-0074".to_string(), "http://terminology.hl7.org/CodeSystem/v2-0074".to_string()]),
    supporting_info_type_displays: Some(vec!["Procedure".to_string(), "Observation".to_string()]),
    supporting_info_reference_ids: Some(vec!["proc_001".to_string(), "obs_004".to_string()]),
    supporting_info_reference_types: Some(vec!["Procedure".to_string(), "Observation".to_string()]),
    media_comments: Some(vec!["Blood smear image".to_string(), "Cell morphology".to_string()]),
    media_link_ids: Some(vec!["doc_001".to_string(), "doc_002".to_string()]),
    presented_form_ids: Some(vec!["att_001".to_string()]),
    conclusion: Some("Complete blood count shows mild anemia with low hemoglobin and hematocrit. White blood cell count and platelet count are within normal limits.".to_string()),
    conclusion_codes: Some(vec!["anemia".to_string(), "low-hemoglobin".to_string()]),
    conclusion_code_codes: Some(vec!["D64.9".to_string(), "R71".to_string()]),
    conclusion_code_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    conclusion_code_displays: Some(vec!["Anemia, unspecified".to_string(), "Abnormality of red blood cells".to_string()]),
    composition_id: Some("comp_001".to_string()),
    notes: Some("Patient should follow up with primary care physician for anemia workup.".to_string()),
    ..Default::default()
};
```

### 2. Radiology Report
```rust
let radiology_report = DomainDiagnosticReport {
    diagnostic_report_id: "dr_rad_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("final".to_string()),
    category: Some("radiology".to_string()),
    category_code: Some("RAD".to_string()),
    category_system: Some("http://terminology.hl7.org/CodeSystem/v2-0074".to_string()),
    category_display: Some("Radiology".to_string()),
    code: Some("chest-x-ray".to_string()),
    code_code: Some("CXR".to_string()),
    code_system: Some("http://loinc.org".to_string()),
    code_display: Some("Chest X-ray".to_string()),
    effective_date: Some("2024-02-01T14:00:00Z".to_string()),
    issued_date: Some("2024-02-01T15:30:00Z".to_string()),
    encounter_id: Some("enc_002".to_string()),
    performer_ids: Some(vec!["prac_rad_001".to_string(), "org_rad_001".to_string()]),
    performer_types: Some(vec!["Practitioner".to_string(), "Organization".to_string()]),
    results_interpreter_ids: Some(vec!["prac_rad_002".to_string()]),
    results_interpreter_types: Some(vec!["Practitioner".to_string()]),
    based_on_ids: Some(vec!["sr_002".to_string()]),
    based_on_types: Some(vec!["ServiceRequest".to_string()]),
    study_ids: Some(vec!["study_rad_001".to_string()]),
    study_types: Some(vec!["ImagingStudy".to_string()]),
    supporting_info_types: Some(vec!["procedure".to_string()]),
    supporting_info_type_codes: Some(vec!["procedure".to_string()]),
    supporting_info_type_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v2-0074".to_string()]),
    supporting_info_type_displays: Some(vec!["Procedure".to_string()]),
    supporting_info_reference_ids: Some(vec!["proc_rad_001".to_string()]),
    supporting_info_reference_types: Some(vec!["Procedure".to_string()]),
    media_comments: Some(vec!["PA chest X-ray".to_string(), "Lateral chest X-ray".to_string()]),
    media_link_ids: Some(vec!["doc_rad_001".to_string(), "doc_rad_002".to_string()]),
    presented_form_ids: Some(vec!["att_rad_001".to_string()]),
    conclusion: Some("Chest X-ray shows clear lung fields bilaterally. No acute cardiopulmonary process. Heart size normal.".to_string()),
    conclusion_codes: Some(vec!["normal-chest-xray".to_string()]),
    conclusion_code_codes: Some(vec!["Z01.89".to_string()]),
    conclusion_code_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    conclusion_code_displays: Some(vec!["Encounter for other specified special examination".to_string()]),
    composition_id: Some("comp_rad_001".to_string()),
    notes: Some("Routine chest X-ray for pre-operative evaluation.".to_string()),
    ..Default::default()
};
```

### 3. Pathology Report
```rust
let pathology_report = DomainDiagnosticReport {
    diagnostic_report_id: "dr_path_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("final".to_string()),
    category: Some("pathology".to_string()),
    category_code: Some("PATH".to_string()),
    category_system: Some("http://terminology.hl7.org/CodeSystem/v2-0074".to_string()),
    category_display: Some("Pathology".to_string()),
    code: Some("biopsy-report".to_string()),
    code_code: Some("BIOPSY".to_string()),
    code_system: Some("http://loinc.org".to_string()),
    code_display: Some("Biopsy Report".to_string()),
    effective_date: Some("2024-03-01T09:00:00Z".to_string()),
    issued_date: Some("2024-03-01T16:00:00Z".to_string()),
    encounter_id: Some("enc_003".to_string()),
    performer_ids: Some(vec!["prac_path_001".to_string()]),
    performer_types: Some(vec!["Practitioner".to_string()]),
    results_interpreter_ids: Some(vec!["prac_path_002".to_string()]),
    results_interpreter_types: Some(vec!["Practitioner".to_string()]),
    based_on_ids: Some(vec!["sr_003".to_string()]),
    based_on_types: Some(vec!["ServiceRequest".to_string()]),
    specimen_ids: Some(vec!["spec_path_001".to_string()]),
    result_observation_ids: Some(vec!["obs_path_001".to_string(), "obs_path_002".to_string()]),
    study_ids: Some(vec!["study_path_001".to_string()]),
    study_types: Some(vec!["GenomicStudy".to_string()]),
    supporting_info_types: Some(vec!["procedure".to_string(), "observation".to_string()]),
    supporting_info_type_codes: Some(vec!["procedure".to_string(), "observation".to_string()]),
    supporting_info_type_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v2-0074".to_string(), "http://terminology.hl7.org/CodeSystem/v2-0074".to_string()]),
    supporting_info_type_displays: Some(vec!["Procedure".to_string(), "Observation".to_string()]),
    supporting_info_reference_ids: Some(vec!["proc_path_001".to_string(), "obs_path_003".to_string()]),
    supporting_info_reference_types: Some(vec!["Procedure".to_string(), "Observation".to_string()]),
    media_comments: Some(vec!["H&E stain".to_string(), "Immunohistochemistry".to_string()]),
    media_link_ids: Some(vec!["doc_path_001".to_string(), "doc_path_002".to_string()]),
    presented_form_ids: Some(vec!["att_path_001".to_string()]),
    conclusion: Some("Adenocarcinoma of the colon, moderately differentiated. Tumor invades into the muscularis propria. No lymphovascular invasion identified.".to_string()),
    conclusion_codes: Some(vec!["adenocarcinoma".to_string(), "colon-cancer".to_string()]),
    conclusion_code_codes: Some(vec!["C18.9".to_string(), "M8140/3".to_string()]),
    conclusion_code_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    conclusion_code_displays: Some(vec!["Malignant neoplasm of colon, unspecified".to_string(), "Adenocarcinoma, NOS".to_string()]),
    composition_id: Some("comp_path_001".to_string()),
    notes: Some("Patient should be referred to oncology for further management.".to_string()),
    ..Default::default()
};
```

### 4. Microbiology Report
```rust
let microbiology_report = DomainDiagnosticReport {
    diagnostic_report_id: "dr_micro_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("final".to_string()),
    category: Some("laboratory".to_string()),
    category_code: Some("LAB".to_string()),
    category_system: Some("http://terminology.hl7.org/CodeSystem/v2-0074".to_string()),
    category_display: Some("Laboratory".to_string()),
    code: Some("urine-culture".to_string()),
    code_code: Some("UC".to_string()),
    code_system: Some("http://loinc.org".to_string()),
    code_display: Some("Urine Culture".to_string()),
    effective_date: Some("2024-04-01T08:00:00Z".to_string()),
    issued_date: Some("2024-04-01T14:00:00Z".to_string()),
    encounter_id: Some("enc_004".to_string()),
    performer_ids: Some(vec!["prac_micro_001".to_string()]),
    performer_types: Some(vec!["Practitioner".to_string()]),
    results_interpreter_ids: Some(vec!["prac_micro_002".to_string()]),
    results_interpreter_types: Some(vec!["Practitioner".to_string()]),
    based_on_ids: Some(vec!["sr_004".to_string()]),
    based_on_types: Some(vec!["ServiceRequest".to_string()]),
    specimen_ids: Some(vec!["spec_micro_001".to_string()]),
    result_observation_ids: Some(vec!["obs_micro_001".to_string(), "obs_micro_002".to_string()]),
    supporting_info_types: Some(vec!["procedure".to_string()]),
    supporting_info_type_codes: Some(vec!["procedure".to_string()]),
    supporting_info_type_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v2-0074".to_string()]),
    supporting_info_type_displays: Some(vec!["Procedure".to_string()]),
    supporting_info_reference_ids: Some(vec!["proc_micro_001".to_string()]),
    supporting_info_reference_types: Some(vec!["Procedure".to_string()]),
    media_comments: Some(vec!["Gram stain", "Culture plate"]),
    media_link_ids: Some(vec!["doc_micro_001".to_string(), "doc_micro_002".to_string()]),
    presented_form_ids: Some(vec!["att_micro_001".to_string()]),
    conclusion: Some("Escherichia coli isolated from urine culture. Susceptible to ciprofloxacin and nitrofurantoin. Resistant to ampicillin.".to_string()),
    conclusion_codes: Some(vec!["uti", "e-coli"]),
    conclusion_code_codes: Some(vec!["N39.0".to_string(), "B96.20".to_string()]),
    conclusion_code_systems: Some(vec!["http://hl7.org/fhir/sid/icd-10-cm".to_string(), "http://hl7.org/fhir/sid/icd-10-cm".to_string()]),
    conclusion_code_displays: Some(vec!["Urinary tract infection, site not specified".to_string(), "Escherichia coli as the cause of diseases classified elsewhere".to_string()]),
    composition_id: Some("comp_micro_001".to_string()),
    notes: Some("Patient should be treated with appropriate antibiotic based on susceptibility results.".to_string()),
    ..Default::default()
};
```

## Next Steps

To complete the diagnostic report implementation, you may want to:

1. **Add to Service Layer**: Integrate the diagnostic report adapter into your gRPC service
2. **Add Database Adapters**: Create database adapters for storing/retrieving diagnostic reports
3. **Add Validation**: Implement validation rules for diagnostic report data
4. **Add Error Handling**: Enhance error handling for conversion failures
5. **Add Logging**: Add appropriate logging for diagnostic report operations
6. **Add Report Templates**: Create reusable report templates
7. **Add Report Workflows**: Integrate with laboratory and radiology systems
8. **Add Report Analytics**: Analyze report patterns and trends
9. **Add Report Notifications**: Implement report delivery and notification systems
10. **Add Report Archiving**: Implement report retention and archiving policies

## Related Entities

This implementation follows the same pattern as the existing entities and integrates with:
- **Patient**: The subject of the diagnostic report
- **Encounter**: When the test was ordered
- **Practitioner**: Who performed and interpreted the test
- **Organization**: The diagnostic service provider
- **Observation**: The test results
- **Specimen**: The specimens used for testing
- **ImagingStudy**: For radiology reports
- **GenomicStudy**: For genetic testing reports
- **Composition**: For structured report formatting

## Report Categories

The implementation supports various report categories:
- **Laboratory**: Blood tests, urine tests, cultures
- **Radiology**: X-rays, CT scans, MRI scans, ultrasounds
- **Pathology**: Biopsies, surgical specimens, autopsies
- **Microbiology**: Cultures, sensitivity testing
- **Cardiology**: EKGs, echocardiograms, stress tests
- **Pulmonology**: Spirometry, arterial blood gases
- **Endocrinology**: Hormone levels, glucose tolerance tests
- **Hematology**: Blood cell counts, coagulation studies
- **Immunology**: Allergy testing, immune function tests
- **Genetics**: Genetic testing, chromosomal analysis

## Report Status Lifecycle

The implementation supports the following report statuses:
- **Registered**: Report has been registered
- **Partial**: Partial results available
- **Preliminary**: Preliminary results available
- **Modified**: Report has been modified
- **Final**: Final report completed
- **Amended**: Report has been amended
- **Corrected**: Report has been corrected
- **Appended**: Additional information appended
- **Cancelled**: Report has been cancelled
- **Entered-in-Error**: Report was created by mistake
- **Unknown**: Status is not known

## Report Types

The implementation supports various report types:
- **Complete Blood Count**: Blood cell analysis
- **Basic Metabolic Panel**: Electrolyte and kidney function
- **Comprehensive Metabolic Panel**: Extended metabolic testing
- **Lipid Panel**: Cholesterol and triglyceride levels
- **Thyroid Function Tests**: Thyroid hormone levels
- **Chest X-ray**: Chest imaging
- **CT Scan**: Computed tomography
- **MRI**: Magnetic resonance imaging
- **Ultrasound**: Sonographic imaging
- **Biopsy Report**: Tissue analysis
- **Culture and Sensitivity**: Microbiological testing
- **Genetic Testing**: DNA analysis

## Media and Attachments

The implementation supports various media types:
- **Images**: X-rays, CT scans, MRI scans, photographs
- **Documents**: PDF reports, Word documents
- **Videos**: Ultrasound videos, procedure recordings
- **Audio**: Dictated reports, voice notes
- **Data Files**: Raw data, spreadsheets

## Conclusion Codes

The implementation supports various conclusion code systems:
- **ICD-10-CM**: International Classification of Diseases
- **SNOMED CT**: Systematized Nomenclature of Medicine
- **LOINC**: Logical Observation Identifiers Names and Codes
- **CPT**: Current Procedural Terminology
- **RADLEX**: Radiology Lexicon

This comprehensive diagnostic report implementation provides a solid foundation for managing diagnostic reports in your FHIR synchronization system, enabling accurate clinical documentation, result interpretation, and care coordination.
