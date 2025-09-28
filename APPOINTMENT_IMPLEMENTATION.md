# Appointment Entity Implementation

This document describes the implementation of the `Appointment` entity for the gRPC FHIR synchronization project.

## Files Created/Modified

### 1. Domain Model
- **File**: `src/domain/appointment.rs`
- **Purpose**: Defines the `DomainAppointment` struct that represents appointment data in our domain model
- **Key Fields**:
  - `appointment_id`: String (required) - Unique identifier for the appointment
  - `patient_demographic_no`: String (required) - Reference to the patient
  - `practitioner_id`: Option<String> - Reference to the practitioner
  - `location_id`: Option<String> - Reference to the location
  - `status`: Option<String> - Appointment status (proposed, pending, booked, etc.)
  - `service_type`: Option<String> - Type of service for the appointment
  - `description`: Option<String> - Description of the appointment
  - `start_time`: Option<String> - ISO datetime string for start time
  - `end_time`: Option<String> - ISO datetime string for end time
  - `duration_minutes`: Option<u32> - Duration in minutes
  - `reason`: Option<String> - Reason for the appointment
  - `priority`: Option<String> - Priority level
  - `comments`: Option<String> - Additional comments
  - `created_date`: Option<String> - ISO datetime string for creation date
  - `cancellation_reason`: Option<String> - Reason for cancellation
  - `cancellation_date`: Option<String> - ISO datetime string for cancellation date

### 2. FHIR Adapter
- **File**: `src/adapters/entities/appointment.rs`
- **Purpose**: Implements the conversion from `DomainAppointment` to FHIR `Appointment` proto message
- **Key Features**:
  - Maps domain fields to FHIR Appointment structure
  - Handles status code conversion (string to FHIR enum)
  - Converts datetime strings to FHIR Instant/DateTime types
  - Creates proper FHIR references for patient, practitioner, and location
  - Handles participants (patient, practitioner, location)
  - Maps service types, reasons, and other coded concepts

### 3. Module Updates
- **File**: `src/domain/mod.rs` - Added `pub mod appointment;`
- **File**: `src/adapters/entities/mod.rs` - Added `pub mod appointment;`

## FHIR Mapping Details

### Status Codes
The adapter maps string status values to FHIR appointment status codes:
- "proposed" → `AppointmentStatusCode::Proposed`
- "pending" → `AppointmentStatusCode::Pending`
- "booked" → `AppointmentStatusCode::Booked`
- "arrived" → `AppointmentStatusCode::Arrived`
- "fulfilled" → `AppointmentStatusCode::Fulfilled`
- "cancelled" → `AppointmentStatusCode::Cancelled`
- "noshow" → `AppointmentStatusCode::Noshow`
- "entered-in-error" → `AppointmentStatusCode::EnteredInError`
- "checked-in" → `AppointmentStatusCode::CheckedIn`
- "waitlist" → `AppointmentStatusCode::Waitlist`

### Participants
The adapter creates FHIR participants for:
1. **Patient** - Always included as a required participant
2. **Practitioner** - Included if `practitioner_id` is provided
3. **Location** - Included if `location_id` is provided

### References
- Patient reference: `Patient/{demographic_no}`
- Practitioner reference: `Practitioner/{practitioner_id}`
- Location reference: `Location/{location_id}`

### Identifiers
- System: `urn:arsmedicatech:appointment_id`
- Value: The appointment ID

## Testing

The implementation includes comprehensive unit tests covering:
- Full deserialization with all fields
- Minimal deserialization with only required fields
- Cancelled appointment scenario
- Error handling for missing required fields

## Usage Example

```rust
use crate::domain::appointment::DomainAppointment;
use crate::adapters::entities::appointment::*;

// Create a domain appointment
let domain_appointment = DomainAppointment {
    appointment_id: "apt_12345".to_string(),
    patient_demographic_no: "12345".to_string(),
    practitioner_id: Some("prac_001".to_string()),
    location_id: Some("loc_001".to_string()),
    status: Some("booked".to_string()),
    service_type: Some("consultation".to_string()),
    description: Some("Follow-up appointment".to_string()),
    start_time: Some("2024-01-15T10:00:00Z".to_string()),
    end_time: Some("2024-01-15T10:30:00Z".to_string()),
    duration_minutes: Some(30),
    reason: Some("Follow-up care".to_string()),
    priority: Some("routine".to_string()),
    comments: Some("Patient prefers morning appointments".to_string()),
    created_date: Some("2024-01-01T09:00:00Z".to_string()),
    cancellation_reason: None,
    cancellation_date: None,
};

// Convert to FHIR Appointment
let fhir_appointment: Appointment = domain_appointment.into();
```

## Next Steps

To complete the appointment implementation, you may want to:

1. **Add to Service Layer**: Integrate the appointment adapter into your gRPC service
2. **Add Database Adapters**: Create database adapters for storing/retrieving appointments
3. **Add Validation**: Implement validation rules for appointment data
4. **Add Error Handling**: Enhance error handling for conversion failures
5. **Add Logging**: Add appropriate logging for appointment operations

## Related Entities

This implementation follows the same pattern as the existing `Patient` entity and can be extended for other FHIR entities like:
- `Condition`
- `Encounter`
- `Observation`
- `Medication`
- etc.
