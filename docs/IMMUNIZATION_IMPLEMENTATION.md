# Immunization Entity Implementation

This document describes the implementation of the `Immunization` entity for the gRPC FHIR synchronization project.

## Files Created/Modified

### 1. Domain Model
- **File**: `src/domain/immunization.rs`
- **Purpose**: Defines the `DomainImmunization` struct that represents immunization data in our domain model
- **Key Fields**:
  - `immunization_id`: String (required) - Unique identifier for the immunization
  - `patient_demographic_no`: String (required) - Reference to the patient
  - `status`: Option<String> - Status (completed, entered-in-error, not-done)
  - `status_reason`: Option<String> - Reason for current status
  - `vaccine_code`: Option<String> - Vaccine administered
  - `administered_product`: Option<String> - Product that was administered
  - `manufacturer`: Option<String> - Vaccine manufacturer
  - `lot_number`: Option<String> - Vaccine lot number
  - `expiration_date`: Option<String> - ISO date string for vaccine expiration date
  - `occurrence_date`: Option<String> - ISO datetime string for vaccine administration date
  - `occurrence_string`: Option<String> - String description of occurrence
  - `primary_source`: Option<bool> - Indicates context the data was captured in
  - `information_source`: Option<String> - Indicates the source of a reported record
  - `location_id`: Option<String> - Where immunization occurred
  - `site`: Option<String> - Body site vaccine was administered
  - `route`: Option<String> - How vaccine entered body
  - `dose_quantity_value`: Option<f64> - Amount of vaccine administered
  - `dose_quantity_unit`: Option<String> - Unit for dose quantity
  - `encounter_id`: Option<String> - Encounter immunization was part of
  - `based_on_ids`: Option<Vec<String>> - Authority that the immunization event is based on
  - `based_on_types`: Option<Vec<String>> - Types of based on references
  - `supporting_information_ids`: Option<Vec<String>> - Additional information in support of the immunization
  - `performer_function_codes`: Option<Vec<String>> - What type of performance was done
  - `performer_actor_ids`: Option<Vec<String>> - Individual or organization who was performing
  - `performer_actor_types`: Option<Vec<String>> - Types of performers
  - `reason_codes`: Option<Vec<String>> - Why immunization occurred
  - `is_subpotent`: Option<bool> - Dose potency
  - `subpotent_reason_codes`: Option<Vec<String>> - Reason for being subpotent
  - `program_eligibility_programs`: Option<Vec<String>> - The program that eligibility is declared for
  - `program_eligibility_statuses`: Option<Vec<String>> - The patient's eligibility status for the program
  - `funding_source`: Option<String> - Funding source for the vaccine
  - `reaction_dates`: Option<Vec<String>> - When reaction started (ISO datetime strings)
  - `reaction_manifestations`: Option<Vec<String>> - Additional information on reaction
  - `reaction_reported`: Option<Vec<bool>> - Indicates self-reported reaction
  - `protocol_series`: Option<Vec<String>> - Name of vaccine series
  - `protocol_authority_ids`: Option<Vec<String>> - Who is responsible for publishing the recommendations
  - `protocol_target_diseases`: Option<Vec<String>> - Vaccine preventable disease being targeted
  - `protocol_dose_numbers`: Option<Vec<String>> - Dose number within series
  - `protocol_series_doses`: Option<Vec<String>> - Recommended number of doses for immunity
  - `notes`: Option<String> - Additional immunization notes

### 2. FHIR Adapter
- **File**: `src/adapters/entities/immunization.rs`
- **Purpose**: Implements the conversion from `DomainImmunization` to FHIR `Immunization` proto message
- **Key Features**:
  - Maps domain fields to FHIR Immunization structure
  - Handles status conversion with proper codes
  - Converts datetime strings to FHIR DateTime types
  - Creates proper FHIR references for patient and other resources
  - Handles vaccine information with codes and terminology
  - Maps administration details (site, route, dose)
  - Handles performers and reasons
  - Maps program eligibility and funding source
  - Handles reactions and protocol information

### 3. Module Updates
- **File**: `src/domain/mod.rs` - Added `pub mod immunization;`
- **File**: `src/adapters/entities/mod.rs` - Added `pub mod immunization;`

## FHIR Mapping Details

### Status
The adapter maps string status values to FHIR status codes:
- "completed" → 1
- "entered-in-error" → 2
- "not-done" → 3

### Vaccine Code
The adapter maps vaccine information to FHIR CodeableConcept:
- System: Defaults to `http://hl7.org/fhir/sid/cvx` or uses provided system
- Code: The provided vaccine code
- Display: The human-readable vaccine name
- Text: The provided vaccine string

### Administered Product
The adapter maps administered product information to FHIR CodeableReference:
- System: Defaults to `http://www.fda.gov/` or uses provided system
- Code: The provided administered product code
- Display: The human-readable administered product name
- Text: The provided administered product string

### Manufacturer
The adapter maps manufacturer information to FHIR CodeableReference:
- System: Defaults to `http://hl7.org/fhir/sid/mvx` or uses provided system
- Code: The provided manufacturer code
- Display: The human-readable manufacturer name
- Text: The provided manufacturer string

### References
- Patient reference: `Patient/{demographic_no}`
- Encounter reference: `Encounter/{encounter_id}`
- Location reference: `Location/{location_id}`
- Performer actor references: `{actor_type}/{actor_id}`
- Based on references: `{based_on_type}/{based_on_id}`
- Supporting information references: `Resource/{info_id}`

### Identifiers
- System: `urn:arsmedicatech:immunization_id`
- Value: The immunization ID

### Occurrence
The adapter handles occurrence information in two formats:
1. **DateTime**: Specific date/time when immunization occurred
2. **String**: String description of occurrence

### Site and Route
The adapter maps site and route information to FHIR CodeableConcept:
- Site system: Defaults to `http://terminology.hl7.org/CodeSystem/v3-ActSite`
- Route system: Defaults to `http://terminology.hl7.org/CodeSystem/v3-RouteOfAdministration`

### Dose Quantity
The adapter maps dose quantity to FHIR SimpleQuantity:
- Value: The dose quantity value
- Unit: The dose quantity unit

### Program Eligibility
The adapter maps program eligibility information:
- Program system: Defaults to `http://terminology.hl7.org/CodeSystem/vaccination-program`
- Status system: Defaults to `http://terminology.hl7.org/CodeSystem/vaccination-program-status`

### Funding Source
The adapter maps funding source to FHIR CodeableConcept:
- System: Defaults to `http://terminology.hl7.org/CodeSystem/vaccination-funding-source`

### Reactions
The adapter maps reaction information:
- Manifestation system: Defaults to `http://snomed.info/sct`
- Date: When reaction started
- Manifestation: Additional information on reaction
- Reported: Indicates self-reported reaction

### Protocol Applied
The adapter maps protocol information:
- Target disease system: Defaults to `http://snomed.info/sct`
- Authority reference: `Organization/{authority_id}`
- Series: Name of vaccine series
- Dose number: Dose number within series
- Series doses: Recommended number of doses for immunity

## Testing

The implementation includes comprehensive unit tests covering:
- Full deserialization with all fields
- Minimal deserialization with only required fields
- COVID-19 vaccination scenario
- Influenza vaccination scenario
- Error handling for missing required fields

## Usage Example

```rust
use crate::domain::immunization::DomainImmunization;
use crate::adapters::entities::immunization::*;

// Create a domain immunization
let domain_immunization = DomainImmunization {
    immunization_id: "imm_12345".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("completed".to_string()),
    status_reason: Some("routine".to_string()),
    status_reason_code: Some("routine".to_string()),
    status_reason_system: Some("http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string()),
    status_reason_display: Some("Routine".to_string()),
    vaccine_code: Some("covid-19-vaccine".to_string()),
    vaccine_code_code: Some("207".to_string()),
    vaccine_code_system: Some("http://hl7.org/fhir/sid/cvx".to_string()),
    vaccine_code_display: Some("COVID-19, mRNA, LNP-S, PF, 100 mcg/0.5mL dose".to_string()),
    administered_product: Some("pfizer-biontech-covid-19-vaccine".to_string()),
    administered_product_code: Some("EU/1/20/1528".to_string()),
    administered_product_system: Some("http://ema.europa.eu/".to_string()),
    administered_product_display: Some("Comirnaty".to_string()),
    manufacturer: Some("pfizer".to_string()),
    manufacturer_code: Some("PFR".to_string()),
    manufacturer_system: Some("http://hl7.org/fhir/sid/mvx".to_string()),
    manufacturer_display: Some("Pfizer, Inc".to_string()),
    lot_number: Some("EW0165".to_string()),
    expiration_date: Some("2024-12-31".to_string()),
    occurrence_date: Some("2024-01-15T10:30:00Z".to_string()),
    primary_source: Some(true),
    information_source: Some("patient".to_string()),
    information_source_code: Some("patient".to_string()),
    information_source_system: Some("http://terminology.hl7.org/CodeSystem/information-source".to_string()),
    information_source_display: Some("Patient".to_string()),
    location_id: Some("loc_001".to_string()),
    site: Some("left-deltoid".to_string()),
    site_code: Some("LA".to_string()),
    site_system: Some("http://terminology.hl7.org/CodeSystem/v3-ActSite".to_string()),
    site_display: Some("Left deltoid".to_string()),
    route: Some("intramuscular".to_string()),
    route_code: Some("IM".to_string()),
    route_system: Some("http://terminology.hl7.org/CodeSystem/v3-RouteOfAdministration".to_string()),
    route_display: Some("Intramuscular".to_string()),
    dose_quantity_value: Some(0.5),
    dose_quantity_unit: Some("mL".to_string()),
    encounter_id: Some("enc_001".to_string()),
    based_on_ids: Some(vec!["sr_001".to_string()]),
    based_on_types: Some(vec!["ServiceRequest".to_string()]),
    supporting_information_ids: Some(vec!["obs_001".to_string()]),
    performer_function_codes: Some(vec!["performer".to_string()]),
    performer_function_code_codes: Some(vec!["performer".to_string()]),
    performer_function_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ParticipationType".to_string()]),
    performer_function_code_displays: Some(vec!["Performer".to_string()]),
    performer_actor_ids: Some(vec!["prac_001".to_string()]),
    performer_actor_types: Some(vec!["Practitioner".to_string()]),
    reason_codes: Some(vec!["routine".to_string()]),
    reason_code_codes: Some(vec!["routine".to_string()]),
    reason_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string()]),
    reason_code_displays: Some(vec!["Routine".to_string()]),
    is_subpotent: Some(false),
    program_eligibility_programs: Some(vec!["covid-19-vaccination".to_string()]),
    program_eligibility_program_codes: Some(vec!["covid-19-vaccination".to_string()]),
    program_eligibility_program_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/vaccination-program".to_string()]),
    program_eligibility_program_displays: Some(vec!["COVID-19 Vaccination Program".to_string()]),
    program_eligibility_statuses: Some(vec!["eligible".to_string()]),
    program_eligibility_status_codes: Some(vec!["eligible".to_string()]),
    program_eligibility_status_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/vaccination-program-status".to_string()]),
    program_eligibility_status_displays: Some(vec!["Eligible".to_string()]),
    funding_source: Some("government".to_string()),
    funding_source_code: Some("government".to_string()),
    funding_source_system: Some("http://terminology.hl7.org/CodeSystem/vaccination-funding-source".to_string()),
    funding_source_display: Some("Government".to_string()),
    reaction_dates: Some(vec!["2024-01-15T14:00:00Z".to_string()]),
    reaction_manifestations: Some(vec!["fever".to_string()]),
    reaction_manifestation_codes: Some(vec!["386661006".to_string()]),
    reaction_manifestation_systems: Some(vec!["http://snomed.info/sct".to_string()]),
    reaction_manifestation_displays: Some(vec!["Fever".to_string()]),
    reaction_reported: Some(vec![true]),
    protocol_series: Some(vec!["covid-19-primary-series".to_string()]),
    protocol_authority_ids: Some(vec!["org_cdc".to_string()]),
    protocol_target_diseases: Some(vec!["covid-19".to_string()]),
    protocol_target_disease_codes: Some(vec!["840539006".to_string()]),
    protocol_target_disease_systems: Some(vec!["http://snomed.info/sct".to_string()]),
    protocol_target_disease_displays: Some(vec!["COVID-19".to_string()]),
    protocol_dose_numbers: Some(vec!["1".to_string()]),
    protocol_series_doses: Some(vec!["2".to_string()]),
    notes: Some("Patient tolerated vaccination well. No immediate adverse reactions observed.".to_string()),
};

// Convert to FHIR Immunization
let fhir_immunization: Immunization = domain_immunization.into();
```

## Clinical Use Cases

### 1. COVID-19 Vaccination
```rust
let covid_vaccination = DomainImmunization {
    immunization_id: "imm_covid_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("completed".to_string()),
    status_reason: Some("routine".to_string()),
    status_reason_code: Some("routine".to_string()),
    status_reason_system: Some("http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string()),
    status_reason_display: Some("Routine".to_string()),
    vaccine_code: Some("covid-19-vaccine".to_string()),
    vaccine_code_code: Some("207".to_string()),
    vaccine_code_system: Some("http://hl7.org/fhir/sid/cvx".to_string()),
    vaccine_code_display: Some("COVID-19, mRNA, LNP-S, PF, 100 mcg/0.5mL dose".to_string()),
    administered_product: Some("pfizer-biontech-covid-19-vaccine".to_string()),
    administered_product_code: Some("EU/1/20/1528".to_string()),
    administered_product_system: Some("http://ema.europa.eu/".to_string()),
    administered_product_display: Some("Comirnaty".to_string()),
    manufacturer: Some("pfizer".to_string()),
    manufacturer_code: Some("PFR".to_string()),
    manufacturer_system: Some("http://hl7.org/fhir/sid/mvx".to_string()),
    manufacturer_display: Some("Pfizer, Inc".to_string()),
    lot_number: Some("EW0165".to_string()),
    expiration_date: Some("2024-12-31".to_string()),
    occurrence_date: Some("2024-01-15T10:30:00Z".to_string()),
    primary_source: Some(true),
    information_source: Some("patient".to_string()),
    information_source_code: Some("patient".to_string()),
    information_source_system: Some("http://terminology.hl7.org/CodeSystem/information-source".to_string()),
    information_source_display: Some("Patient".to_string()),
    location_id: Some("loc_001".to_string()),
    site: Some("left-deltoid".to_string()),
    site_code: Some("LA".to_string()),
    site_system: Some("http://terminology.hl7.org/CodeSystem/v3-ActSite".to_string()),
    site_display: Some("Left deltoid".to_string()),
    route: Some("intramuscular".to_string()),
    route_code: Some("IM".to_string()),
    route_system: Some("http://terminology.hl7.org/CodeSystem/v3-RouteOfAdministration".to_string()),
    route_display: Some("Intramuscular".to_string()),
    dose_quantity_value: Some(0.5),
    dose_quantity_unit: Some("mL".to_string()),
    encounter_id: Some("enc_001".to_string()),
    based_on_ids: Some(vec!["sr_001".to_string()]),
    based_on_types: Some(vec!["ServiceRequest".to_string()]),
    supporting_information_ids: Some(vec!["obs_001".to_string()]),
    performer_function_codes: Some(vec!["performer".to_string()]),
    performer_function_code_codes: Some(vec!["performer".to_string()]),
    performer_function_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ParticipationType".to_string()]),
    performer_function_code_displays: Some(vec!["Performer".to_string()]),
    performer_actor_ids: Some(vec!["prac_001".to_string()]),
    performer_actor_types: Some(vec!["Practitioner".to_string()]),
    reason_codes: Some(vec!["routine".to_string()]),
    reason_code_codes: Some(vec!["routine".to_string()]),
    reason_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string()]),
    reason_code_displays: Some(vec!["Routine".to_string()]),
    is_subpotent: Some(false),
    program_eligibility_programs: Some(vec!["covid-19-vaccination".to_string()]),
    program_eligibility_program_codes: Some(vec!["covid-19-vaccination".to_string()]),
    program_eligibility_program_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/vaccination-program".to_string()]),
    program_eligibility_program_displays: Some(vec!["COVID-19 Vaccination Program".to_string()]),
    program_eligibility_statuses: Some(vec!["eligible".to_string()]),
    program_eligibility_status_codes: Some(vec!["eligible".to_string()]),
    program_eligibility_status_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/vaccination-program-status".to_string()]),
    program_eligibility_status_displays: Some(vec!["Eligible".to_string()]),
    funding_source: Some("government".to_string()),
    funding_source_code: Some("government".to_string()),
    funding_source_system: Some("http://terminology.hl7.org/CodeSystem/vaccination-funding-source".to_string()),
    funding_source_display: Some("Government".to_string()),
    reaction_dates: Some(vec!["2024-01-15T14:00:00Z".to_string()]),
    reaction_manifestations: Some(vec!["fever".to_string()]),
    reaction_manifestation_codes: Some(vec!["386661006".to_string()]),
    reaction_manifestation_systems: Some(vec!["http://snomed.info/sct".to_string()]),
    reaction_manifestation_displays: Some(vec!["Fever".to_string()]),
    reaction_reported: Some(vec![true]),
    protocol_series: Some(vec!["covid-19-primary-series".to_string()]),
    protocol_authority_ids: Some(vec!["org_cdc".to_string()]),
    protocol_target_diseases: Some(vec!["covid-19".to_string()]),
    protocol_target_disease_codes: Some(vec!["840539006".to_string()]),
    protocol_target_disease_systems: Some(vec!["http://snomed.info/sct".to_string()]),
    protocol_target_disease_displays: Some(vec!["COVID-19".to_string()]),
    protocol_dose_numbers: Some(vec!["1".to_string()]),
    protocol_series_doses: Some(vec!["2".to_string()]),
    notes: Some("Patient tolerated vaccination well. No immediate adverse reactions observed.".to_string()),
    ..Default::default()
};
```

### 2. Influenza Vaccination
```rust
let flu_vaccination = DomainImmunization {
    immunization_id: "imm_flu_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("completed".to_string()),
    status_reason: Some("routine".to_string()),
    status_reason_code: Some("routine".to_string()),
    status_reason_system: Some("http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string()),
    status_reason_display: Some("Routine".to_string()),
    vaccine_code: Some("influenza-vaccine".to_string()),
    vaccine_code_code: Some("140".to_string()),
    vaccine_code_system: Some("http://hl7.org/fhir/sid/cvx".to_string()),
    vaccine_code_display: Some("Influenza, seasonal, injectable".to_string()),
    administered_product: Some("fluzone-quadrivalent".to_string()),
    administered_product_code: Some("49281-0400-78".to_string()),
    administered_product_system: Some("http://www.fda.gov/".to_string()),
    administered_product_display: Some("Fluzone Quadrivalent".to_string()),
    manufacturer: Some("sanofi-pasteur".to_string()),
    manufacturer_code: Some("PMC".to_string()),
    manufacturer_system: Some("http://hl7.org/fhir/sid/mvx".to_string()),
    manufacturer_display: Some("Sanofi Pasteur".to_string()),
    lot_number: Some("FLU2024001".to_string()),
    expiration_date: Some("2025-06-30".to_string()),
    occurrence_date: Some("2024-10-15T09:00:00Z".to_string()),
    primary_source: Some(true),
    information_source: Some("practitioner".to_string()),
    information_source_code: Some("practitioner".to_string()),
    information_source_system: Some("http://terminology.hl7.org/CodeSystem/information-source".to_string()),
    information_source_display: Some("Practitioner".to_string()),
    location_id: Some("loc_002".to_string()),
    site: Some("left-deltoid".to_string()),
    site_code: Some("LA".to_string()),
    site_system: Some("http://terminology.hl7.org/CodeSystem/v3-ActSite".to_string()),
    site_display: Some("Left deltoid".to_string()),
    route: Some("intramuscular".to_string()),
    route_code: Some("IM".to_string()),
    route_system: Some("http://terminology.hl7.org/CodeSystem/v3-RouteOfAdministration".to_string()),
    route_display: Some("Intramuscular".to_string()),
    dose_quantity_value: Some(0.5),
    dose_quantity_unit: Some("mL".to_string()),
    encounter_id: Some("enc_002".to_string()),
    performer_function_codes: Some(vec!["performer".to_string()]),
    performer_function_code_codes: Some(vec!["performer".to_string()]),
    performer_function_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ParticipationType".to_string()]),
    performer_function_code_displays: Some(vec!["Performer".to_string()]),
    performer_actor_ids: Some(vec!["prac_002".to_string()]),
    performer_actor_types: Some(vec!["Practitioner".to_string()]),
    reason_codes: Some(vec!["routine".to_string()]),
    reason_code_codes: Some(vec!["routine".to_string()]),
    reason_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string()]),
    reason_code_displays: Some(vec!["Routine".to_string()]),
    is_subpotent: Some(false),
    program_eligibility_programs: Some(vec!["influenza-vaccination".to_string()]),
    program_eligibility_program_codes: Some(vec!["influenza-vaccination".to_string()]),
    program_eligibility_program_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/vaccination-program".to_string()]),
    program_eligibility_program_displays: Some(vec!["Influenza Vaccination Program".to_string()]),
    program_eligibility_statuses: Some(vec!["eligible".to_string()]),
    program_eligibility_status_codes: Some(vec!["eligible".to_string()]),
    program_eligibility_status_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/vaccination-program-status".to_string()]),
    program_eligibility_status_displays: Some(vec!["Eligible".to_string()]),
    funding_source: Some("insurance".to_string()),
    funding_source_code: Some("insurance".to_string()),
    funding_source_system: Some("http://terminology.hl7.org/CodeSystem/vaccination-funding-source".to_string()),
    funding_source_display: Some("Insurance".to_string()),
    protocol_series: Some(vec!["influenza-annual".to_string()]),
    protocol_authority_ids: Some(vec!["org_cdc".to_string()]),
    protocol_target_diseases: Some(vec!["influenza".to_string()]),
    protocol_target_disease_codes: Some(vec!["6142004".to_string()]),
    protocol_target_disease_systems: Some(vec!["http://snomed.info/sct".to_string()]),
    protocol_target_disease_displays: Some(vec!["Influenza".to_string()]),
    protocol_dose_numbers: Some(vec!["1".to_string()]),
    protocol_series_doses: Some(vec!["1".to_string()]),
    notes: Some("Annual influenza vaccination. Patient reported no adverse reactions.".to_string()),
    ..Default::default()
};
```

## Next Steps

To complete the immunization implementation, you may want to:

1. **Add to Service Layer**: Integrate the immunization adapter into your gRPC service
2. **Add Database Adapters**: Create database adapters for storing/retrieving immunizations
3. **Add Validation**: Implement validation rules for immunization data
4. **Add Error Handling**: Enhance error handling for conversion failures
5. **Add Logging**: Add appropriate logging for immunization operations
6. **Add Vaccine Inventory**: Integrate with vaccine inventory management
7. **Add Adverse Event Reporting**: Implement adverse event reporting
8. **Add Immunization Schedules**: Create immunization schedule management
9. **Add Vaccine Recommendations**: Implement vaccine recommendation algorithms
10. **Add Immunization Analytics**: Analyze immunization patterns and coverage

## Related Entities

This implementation follows the same pattern as the existing entities and integrates with:
- **Patient**: The subject of the immunization
- **Practitioner**: Who performed the immunization
- **Location**: Where the immunization occurred
- **Encounter**: The healthcare encounter during which immunization was given
- **Organization**: The manufacturer and authority organizations
- **Observation**: Supporting information and reactions

## Vaccine Types Supported

The implementation supports various vaccine types:
- **COVID-19**: mRNA vaccines, viral vector vaccines, protein subunit vaccines
- **Influenza**: Seasonal influenza vaccines, high-dose vaccines
- **Childhood Vaccines**: DTaP, MMR, Polio, Hepatitis B, etc.
- **Adult Vaccines**: Tdap, Shingles, Pneumococcal, etc.
- **Travel Vaccines**: Yellow fever, Typhoid, Hepatitis A, etc.
- **Special Population Vaccines**: Immunocompromised, pregnant women, etc.

## Administration Sites

The implementation supports various administration sites:
- **Deltoid**: Left deltoid, Right deltoid
- **Thigh**: Left thigh, Right thigh
- **Gluteal**: Left gluteal, Right gluteal
- **Oral**: Oral administration
- **Nasal**: Intranasal administration
- **Other**: Other specified sites

## Administration Routes

The implementation supports various administration routes:
- **Intramuscular**: IM injection
- **Subcutaneous**: SC injection
- **Intradermal**: ID injection
- **Oral**: Oral administration
- **Nasal**: Intranasal administration
- **Other**: Other specified routes

## Funding Sources

The implementation supports various funding sources:
- **Government**: Public health programs
- **Insurance**: Private insurance coverage
- **Self-Pay**: Patient pays directly
- **Charity**: Charitable organizations
- **Other**: Other funding sources

## Program Eligibility

The implementation supports various vaccination programs:
- **Routine**: Routine vaccination programs
- **Catch-up**: Catch-up vaccination programs
- **High-Risk**: High-risk population programs
- **Travel**: Travel vaccination programs
- **Outbreak**: Outbreak response programs
- **Other**: Other vaccination programs

## Reaction Types

The implementation supports various reaction types:
- **Local**: Injection site reactions
- **Systemic**: Fever, fatigue, headache
- **Allergic**: Anaphylaxis, urticaria
- **Severe**: Hospitalization, death
- **Other**: Other specified reactions

## Protocol Information

The implementation supports various protocol information:
- **Series**: Primary series, booster series
- **Dose Number**: First dose, second dose, etc.
- **Series Doses**: Total recommended doses
- **Authority**: CDC, WHO, local health departments
- **Target Diseases**: Diseases prevented by the vaccine

This comprehensive immunization implementation provides a solid foundation for managing vaccination records in your FHIR synchronization system, enabling public health tracking, vaccine inventory management, and immunization analytics.
