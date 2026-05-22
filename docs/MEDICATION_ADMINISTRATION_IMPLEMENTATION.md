# MedicationAdministration Implementation

## Overview
This document describes the implementation of the `MedicationAdministration` entity for the gRPC FHIR synchronization project. The implementation includes both a domain model and a FHIR adapter to convert between the internal representation and the FHIR protobuf format.

## Files Created

### 1. Domain Model: `src/domain/medication_administration.rs`
- **Purpose**: Defines the internal domain model for medication administration events
- **Key Features**:
  - Comprehensive medication administration tracking
  - Support for complex dosage information
  - Performer and device tracking
  - Status and reason management
  - Reaction and protocol tracking

### 2. FHIR Adapter: `src/adapters/entities/medication_administration.rs`
- **Purpose**: Converts `DomainMedicationAdministration` to FHIR `MedicationAdministration` protobuf
- **Key Features**:
  - Complete FHIR R5 compliance
  - Handles all medication administration fields
  - Supports complex nested structures
  - Proper status and reason code mapping

## Domain Model Structure

### Core Fields
- **id**: Unique identifier for the medication administration
- **identifier**: External identifiers
- **status**: Administration status (in-progress, not-done, on-hold, completed, entered-in-error, stopped, unknown)
- **status_reason**: Reason for current status
- **category**: Type of medication administration
- **medication**: What was administered (code or reference)
- **subject**: Who received the medication
- **encounter**: Associated encounter
- **occurrence**: When the administration occurred (datetime, period, or timing)
- **recorded**: When the administration was first captured
- **performer**: Who or what performed the administration
- **reason**: Why the medication was administered
- **device**: Device used for administration
- **dosage**: Administration details (site, route, method, dose, rate)

### Nested Structures
- **DomainMedicationAdministrationPerformer**: Performer details with function and actor information
- **DomainDosage**: Comprehensive dosage instructions including timing, site, route, method, dose, and rate
- **DomainDosageDoseAndRate**: Specific dose and rate information with choice types

## FHIR Adapter Features

### Status Mapping
- Maps domain status strings to FHIR status codes
- Handles status reason codes and displays
- Supports complex status reason structures

### Medication Handling
- Supports both coded medications and medication references
- Handles medication code systems and displays
- Proper reference formatting for medication resources

### Subject and Encounter References
- Creates proper FHIR references to Patient/Group and Encounter
- Supports different subject types
- Handles encounter type specification

### Occurrence Handling
- Supports DateTime, Period, and Timing occurrence types
- Proper timezone handling with UTC conversion
- Flexible occurrence specification

### Performer Management
- Maps performer function codes
- Handles both coded and referenced performers
- Supports multiple performer types (Practitioner, Device, Organization, etc.)

### Dosage Information
- Comprehensive dosage instruction mapping
- Site, route, and method code handling
- Dose and rate calculation with proper units
- Timing information with repeat patterns

### Device and Reason Tracking
- Device code and reference mapping
- Reason code and reference handling
- Support for multiple devices and reasons

## Key Features

### 1. Comprehensive Medication Tracking
- Full medication administration lifecycle support
- Status tracking from in-progress to completed
- Reason codes for status changes

### 2. Flexible Dosage Management
- Support for complex dosage instructions
- Multiple dose and rate types (quantity, ratio, range)
- Timing patterns and repeat instructions

### 3. Performer and Device Support
- Multiple performer types and functions
- Device tracking for administration
- Proper actor and function mapping

### 4. Clinical Documentation
- Note and annotation support
- Reaction tracking
- Protocol application information

### 5. Integration Support
- Based on and part of relationship tracking
- Supporting information references
- Event history provenance

## Clinical Use Cases

### 1. Inpatient Medication Administration
- Track medication given to patients in hospital
- Record administration time and performer
- Document dosage and route information

### 2. Outpatient Medication Tracking
- Monitor medication compliance
- Track administration events
- Record patient reactions

### 3. Clinical Research
- Document medication administration in trials
- Track protocol compliance
- Record adverse events and reactions

### 4. Quality Assurance
- Monitor medication administration practices
- Track performer performance
- Document device usage

## Benefits

### 1. FHIR Compliance
- Full R5 specification compliance
- Standardized medication administration representation
- Interoperable with other FHIR systems

### 2. Comprehensive Tracking
- Complete medication administration lifecycle
- Detailed dosage and timing information
- Performer and device accountability

### 3. Clinical Safety
- Status and reason tracking
- Reaction documentation
- Protocol compliance monitoring

### 4. Integration Ready
- Based on and part of relationships
- Supporting information references
- Event history tracking

## Usage Example

```rust
use crate::domain::medication_administration::DomainMedicationAdministration;
use crate::adapters::entities::medication_administration::MedicationAdministration;

// Create domain model
let domain_admin = DomainMedicationAdministration {
    id: "admin-123".to_string(),
    status: "completed".to_string(),
    medication_code: Some("medication-code".to_string()),
    subject_id: "patient-456".to_string(),
    subject_type: "Patient".to_string(),
    occurrence_date_time: Some("2024-01-15T10:30:00Z".to_string()),
    dosage_text: Some("Take 1 tablet daily".to_string()),
    // ... other fields
};

// Convert to FHIR
let fhir_admin: MedicationAdministration = domain_admin.into();
```

## Testing

The implementation includes comprehensive unit tests covering:
- Domain model serialization/deserialization
- FHIR adapter conversion
- Edge cases and error handling
- Complex nested structure handling

## Future Enhancements

1. **Enhanced Timing Support**: More sophisticated timing pattern handling
2. **Reaction Tracking**: Detailed adverse reaction documentation
3. **Protocol Integration**: Enhanced protocol compliance tracking
4. **Device Integration**: More detailed device usage tracking
5. **Performance Optimization**: Batch processing and caching improvements

## Dependencies

- `chrono`: Date and time handling
- `serde`: Serialization/deserialization
- `prost`: Protobuf support
- FHIR R5 protobuf definitions

## Notes

- The implementation uses placeholder structs for FHIR types that may not be generated yet
- All timestamps are converted to UTC for consistency
- Status codes follow FHIR R5 medication administration status values
- The adapter handles missing or optional fields gracefully
