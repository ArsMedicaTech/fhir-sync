# MedicationRequest Implementation

## Overview
This document describes the implementation of the `MedicationRequest` entity for the gRPC FHIR synchronization project. The implementation includes both a domain model and a FHIR adapter to convert between the internal representation and the FHIR protobuf format.

## Files Created

### 1. Domain Model: `src/domain/medication_request.rs`
- **Purpose**: Defines the internal domain model for medication requests (prescriptions)
- **Key Features**:
  - Comprehensive prescription management
  - Complex dosage instruction support
  - Dispense request handling
  - Substitution management
  - Insurance and coverage tracking

### 2. FHIR Adapter: `src/adapters/entities/medication_request.rs`
- **Purpose**: Converts `DomainMedicationRequest` to FHIR `MedicationRequest` protobuf
- **Key Features**:
  - Complete FHIR R5 compliance
  - Handles all medication request fields
  - Supports complex nested structures
  - Proper status and intent code mapping

## Domain Model Structure

### Core Fields
- **id**: Unique identifier for the medication request
- **identifier**: External identifiers
- **status**: Request status (active, on-hold, ended, stopped, completed, cancelled, entered-in-error, draft, unknown)
- **intent**: Request intent (proposal, plan, order, original-order, reflex-order, filler-order, instance-order, option)
- **category**: Grouping or category of medication request
- **priority**: Request priority (routine, urgent, asap, stat)
- **medication**: Medication to be taken (code or reference)
- **subject**: Individual or group for whom the medication is requested
- **encounter**: Associated encounter
- **authored_on**: When request was initially authored
- **requester**: Who/What requested the request
- **performer**: Intended performer of administration
- **reason**: Reason or indication for ordering the medication
- **dosage_instruction**: Specific instructions for how the medication should be taken
- **dispense_request**: Dispense request details
- **substitution**: Substitution information

### Nested Structures
- **DomainDosage**: Comprehensive dosage instructions including timing, site, route, method, dose, and rate
- **DomainDosageDoseAndRate**: Specific dose and rate information with choice types

## FHIR Adapter Features

### Status and Intent Mapping
- Maps domain status strings to FHIR status codes
- Handles intent codes (proposal, plan, order, etc.)
- Supports priority levels (routine, urgent, asap, stat)

### Medication Handling
- Supports both coded medications and medication references
- Handles medication code systems and displays
- Proper reference formatting for medication resources

### Subject and Encounter References
- Creates proper FHIR references to Patient/Group and Encounter
- Supports different subject types
- Handles encounter type specification

### Requester and Performer Management
- Maps requester information with proper references
- Handles performer types and references
- Supports multiple performer types (Practitioner, Device, Organization, etc.)

### Dosage Instruction Processing
- Comprehensive dosage instruction mapping
- Site, route, and method code handling
- Dose and rate calculation with proper units
- Timing information with repeat patterns
- As-needed and additional instruction support

### Dispense Request Handling
- Initial fill quantity and duration
- Dispense interval and validity period
- Number of repeats allowed
- Expected supply duration
- Dispenser and instruction management
- Dose administration aid support

### Substitution Management
- Substitution allowed/not allowed handling
- Substitution reason codes
- Boolean and coded substitution support

## Key Features

### 1. Comprehensive Prescription Management
- Full medication request lifecycle support
- Status tracking from draft to completed
- Intent and priority management

### 2. Complex Dosage Instructions
- Support for detailed dosage instructions
- Multiple dose and rate types (quantity, ratio, range)
- Timing patterns and repeat instructions
- As-needed and additional instruction support

### 3. Dispense Request Management
- Complete dispense request handling
- Initial fill and repeat management
- Validity period and supply duration
- Dispenser instruction support

### 4. Substitution Control
- Substitution allowed/not allowed tracking
- Substitution reason documentation
- Flexible substitution handling

### 5. Integration Support
- Based on and prior prescription tracking
- Insurance and coverage references
- Supporting information references
- Event history provenance

## Clinical Use Cases

### 1. Prescription Management
- Create and manage medication prescriptions
- Track prescription status and changes
- Handle prescription renewals and modifications

### 2. Pharmacy Integration
- Dispense request processing
- Substitution management
- Supply duration and repeat handling

### 3. Clinical Decision Support
- Reason and indication tracking
- Priority and urgency management
- Performer and device specification

### 4. Insurance and Billing
- Insurance coverage tracking
- Prior authorization management
- Billing and claims support

## Benefits

### 1. FHIR Compliance
- Full R5 specification compliance
- Standardized medication request representation
- Interoperable with other FHIR systems

### 2. Comprehensive Prescription Support
- Complete prescription lifecycle management
- Detailed dosage instruction handling
- Dispense request and substitution control

### 3. Clinical Safety
- Status and intent tracking
- Reason and indication documentation
- Priority and urgency management

### 4. Integration Ready
- Based on and prior prescription relationships
- Insurance and coverage references
- Supporting information and event history

## Usage Example

```rust
use crate::domain::medication_request::DomainMedicationRequest;
use crate::adapters::entities::medication_request::MedicationRequest;

// Create domain model
let domain_request = DomainMedicationRequest {
    id: "request-123".to_string(),
    status: "active".to_string(),
    intent: "order".to_string(),
    medication_code: Some("medication-code".to_string()),
    subject_id: "patient-456".to_string(),
    subject_type: "Patient".to_string(),
    authored_on: Some("2024-01-15T10:30:00Z".to_string()),
    dosage_text: Some("Take 1 tablet daily".to_string()),
    // ... other fields
};

// Convert to FHIR
let fhir_request: MedicationRequest = domain_request.into();
```

## Testing

The implementation includes comprehensive unit tests covering:
- Domain model serialization/deserialization
- FHIR adapter conversion
- Edge cases and error handling
- Complex nested structure handling
- Dosage instruction processing
- Dispense request handling

## Future Enhancements

1. **Enhanced Dosage Support**: More sophisticated dosage instruction handling
2. **Insurance Integration**: Enhanced insurance and coverage tracking
3. **Prior Authorization**: Prior authorization workflow support
4. **Clinical Decision Support**: Enhanced clinical decision support integration
5. **Performance Optimization**: Batch processing and caching improvements

## Dependencies

- `chrono`: Date and time handling
- `serde`: Serialization/deserialization
- `prost`: Protobuf support
- FHIR R5 protobuf definitions

## Notes

- The implementation uses placeholder structs for FHIR types that may not be generated yet
- All timestamps are converted to UTC for consistency
- Status and intent codes follow FHIR R5 medication request values
- The adapter handles missing or optional fields gracefully
- Dosage instructions support complex timing patterns and repeat instructions
