# Location Entity Implementation

This document describes the implementation of the `Location` entity for the gRPC FHIR synchronization project.

## Files Created/Modified

### 1. Domain Model
- **File**: `src/domain/location.rs`
- **Purpose**: Defines the `DomainLocation` struct that represents location data in our domain model
- **Key Fields**:
  - `location_id`: String (required) - Unique identifier for the location
  - `status`: Option<String> - "active" | "suspended" | "inactive"
  - `operational_status`: Option<String> - The operational status of the location
  - `operational_status_code`: Option<String> - Code for operational status
  - `operational_status_system`: Option<String> - Terminology system for operational status
  - `operational_status_display`: Option<String> - Display name for operational status
  - `name`: Option<String> - Name of the location as used by humans
  - `alias`: Option<Vec<String>> - A list of alternate names that the location is known as
  - `description`: Option<String> - Additional details about the location
  - `mode`: Option<String> - "instance" | "kind"
  - `types`: Option<Vec<String>> - Type of function performed
  - `type_codes`: Option<Vec<String>> - Codes for types
  - `type_systems`: Option<Vec<String>> - Terminology systems for types
  - `type_displays`: Option<Vec<String>> - Display names for types
  - `contact_purpose`: Option<Vec<String>> - The purpose of this contact
  - `contact_purpose_codes`: Option<Vec<String>> - Codes for contact purposes
  - `contact_purpose_systems`: Option<Vec<String>> - Terminology systems for contact purposes
  - `contact_purpose_displays`: Option<Vec<String>> - Display names for contact purposes
  - `contact_name`: Option<Vec<String>> - Name of an individual to contact
  - `contact_telecom_system`: Option<Vec<Vec<String>>> - Contact system (phone, fax, email, pager, url, sms, other)
  - `contact_telecom_value`: Option<Vec<Vec<String>>> - Contact value
  - `contact_telecom_use`: Option<Vec<Vec<String>>> - Contact use (work, temp, old, mobile)
  - `contact_telecom_rank`: Option<Vec<Vec<u32>>> - Specify preferred order of use (1 = highest)
  - `contact_address_use`: Option<Vec<String>> - Address use (work, temp, old, billing)
  - `contact_address_type`: Option<Vec<String>> - Address type (postal, physical, both)
  - `contact_address_text`: Option<Vec<String>> - Text representation of the address
  - `contact_address_line`: Option<Vec<Vec<String>>> - Street address lines
  - `contact_address_city`: Option<Vec<String>> - City name
  - `contact_address_district`: Option<Vec<String>> - District name (sublocality)
  - `contact_address_state`: Option<Vec<String>> - State name
  - `contact_address_postal_code`: Option<Vec<String>> - Postal code
  - `contact_address_country`: Option<Vec<String>> - Country name
  - `contact_organization_id`: Option<Vec<String>> - Organization associated with the contact
  - `contact_organization_type`: Option<Vec<String>> - Type of organization
  - `contact_period_start`: Option<Vec<String>> - Time period when the contact was/is in use (ISO datetime)
  - `contact_period_end`: Option<Vec<String>> - Time period when the contact was/is in use (ISO datetime)
  - `address_use`: Option<String> - Address use (work, temp, old, billing)
  - `address_type`: Option<String> - Address type (postal, physical, both)
  - `address_text`: Option<String> - Text representation of the address
  - `address_line`: Option<Vec<String>> - Street address lines
  - `address_city`: Option<String> - City name
  - `address_district`: Option<String> - District name (sublocality)
  - `address_state`: Option<String> - State name
  - `address_postal_code`: Option<String> - Postal code
  - `address_country`: Option<String> - Country name
  - `address_period_start`: Option<String> - Time period when address was/is in use (ISO datetime)
  - `address_period_end`: Option<String> - Time period when address was/is in use (ISO datetime)
  - `form`: Option<String> - Physical form of the location
  - `form_code`: Option<String> - Code for form
  - `form_system`: Option<String> - Terminology system for form
  - `form_display`: Option<String> - Display name for form
  - `longitude`: Option<f64> - Longitude with WGS84 datum
  - `latitude`: Option<f64> - Latitude with WGS84 datum
  - `altitude`: Option<f64> - Altitude with WGS84 datum
  - `managing_organization_id`: Option<String> - Organization responsible for provisioning and upkeep
  - `managing_organization_type`: Option<String> - Type of managing organization
  - `part_of_id`: Option<String> - Another Location this one is physically a part of
  - `part_of_type`: Option<String> - Type of part of location
  - `characteristics`: Option<Vec<String>> - Collection of characteristics (attributes)
  - `characteristic_codes`: Option<Vec<String>> - Codes for characteristics
  - `characteristic_systems`: Option<Vec<String>> - Terminology systems for characteristics
  - `characteristic_displays`: Option<Vec<String>> - Display names for characteristics
  - `hours_of_operation_days_of_week`: Option<Vec<Vec<String>>> - mon | tue | wed | thu | fri | sat | sun
  - `hours_of_operation_all_day`: Option<Vec<bool>> - The location is open all day
  - `hours_of_operation_opening_time`: Option<Vec<String>> - Time that the Location opens (ISO time)
  - `hours_of_operation_closing_time`: Option<Vec<String>> - Time that the Location closes (ISO time)
  - `virtual_service_channel_type`: Option<Vec<String>> - Channel type for virtual service
  - `virtual_service_channel_type_codes`: Option<Vec<String>> - Codes for channel types
  - `virtual_service_channel_type_systems`: Option<Vec<String>> - Terminology systems for channel types
  - `virtual_service_channel_type_displays`: Option<Vec<String>> - Display names for channel types
  - `virtual_service_address_url`: Option<Vec<String>> - Address for virtual service
  - `virtual_service_address_extension`: Option<Vec<String>> - Extension for virtual service address
  - `virtual_service_extension`: Option<Vec<String>> - Extension for virtual service
  - `virtual_service_extension_url`: Option<Vec<String>> - URL for virtual service extension
  - `virtual_service_extension_value`: Option<Vec<String>> - Value for virtual service extension
  - `endpoint_ids`: Option<Vec<String>> - Technical endpoints providing access to services
  - `endpoint_types`: Option<Vec<String>> - Types of endpoints

### 2. FHIR Adapter
- **File**: `src/adapters/entities/location.rs`
- **Purpose**: Implements the conversion from `DomainLocation` to FHIR `Location` proto message
- **Key Features**:
  - Maps domain fields to FHIR Location structure
  - Handles status and operational status
  - Maps name and alias information
  - Converts contact information with systems, uses, and periods
  - Handles address information with all components
  - Maps geographic position with longitude, latitude, and altitude
  - Handles organization and hierarchy relationships
  - Maps characteristics and attributes
  - Handles hours of operation
  - Maps virtual service details
  - Handles endpoint references

### 3. Module Updates
- **File**: `src/domain/mod.rs` - Added `pub mod location;`
- **File**: `src/adapters/entities/mod.rs` - Added `pub mod location;`

## FHIR Mapping Details

### Status
The adapter maps string status values to FHIR status codes:
- "active" → 1
- "suspended" → 2
- "inactive" → 3

### Mode
The adapter maps string mode values to FHIR mode codes:
- "instance" → 1
- "kind" → 2

### Type
The adapter maps type information to FHIR CodeableConcept:
- System: The provided terminology system
- Code: The provided type code
- Display: The provided type display
- Text: The type description

### Contact
The adapter maps contact information to FHIR ExtendedContactDetail:
- Purpose: The contact purpose with coding
- Name: The contact name
- Telecom: Array of contact points with systems, values, uses, and ranks
- Address: Contact address with all components
- Organization: Associated organization reference
- Period: Time period when contact was/is in use

### Address
The adapter maps address information to FHIR Address:
- Use: Address use (work, temp, old, billing)
- Type: Address type (postal, physical, both)
- Text: Text representation of the address
- Line: Array of street address lines
- City: City name
- District: District name (sublocality)
- State: State name
- Postal Code: Postal code
- Country: Country name
- Period: Time period when address was/is in use

### Form
The adapter maps form information to FHIR CodeableConcept:
- System: The provided terminology system
- Code: The provided form code
- Display: The provided form display
- Text: The form description

### Position
The adapter maps geographic position to FHIR Position:
- Longitude: Longitude with WGS84 datum
- Latitude: Latitude with WGS84 datum
- Altitude: Altitude with WGS84 datum

### Managing Organization
The adapter maps managing organization to FHIR Reference:
- Reference: `{organization_type}/{organization_id}`

### Part Of
The adapter maps part of relationship to FHIR Reference:
- Reference: `{location_type}/{location_id}`

### Characteristics
The adapter maps characteristics to FHIR CodeableConcept:
- System: The provided terminology system
- Code: The provided characteristic code
- Display: The provided characteristic display
- Text: The characteristic description

### Hours of Operation
The adapter maps hours of operation to FHIR Availability:
- Days of Week: Array of days (mon, tue, wed, thu, fri, sat, sun)
- All Day: Boolean indicating if location is open all day
- Opening Time: Time that the location opens
- Closing Time: Time that the location closes

### Virtual Service
The adapter maps virtual service to FHIR VirtualServiceDetail:
- Channel Type: The channel type with coding
- Address URL: The virtual service address

### Endpoints
The adapter maps endpoints to FHIR Reference:
- Reference: `{endpoint_type}/{endpoint_id}`

### References
- Managing organization references: `{organization_type}/{organization_id}`
- Part of location references: `{location_type}/{location_id}`
- Endpoint references: `{endpoint_type}/{endpoint_id}`

### Identifiers
- System: `urn:arsmedicatech:location_id`
- Value: The location ID

## Testing

The implementation includes comprehensive unit tests covering:
- Full deserialization with all fields
- Minimal deserialization with only required fields
- Error handling for missing required fields

## Usage Example

```rust
use crate::domain::location::DomainLocation;
use crate::adapters::entities::location::*;

// Create a domain location
let domain_location = DomainLocation {
    location_id: "loc_12345".to_string(),
    status: Some("active".to_string()),
    operational_status: Some("occupied".to_string()),
    operational_status_code: Some("O".to_string()),
    operational_status_system: Some("http://terminology.hl7.org/CodeSystem/v2-0116".to_string()),
    operational_status_display: Some("Occupied".to_string()),
    name: Some("Main Hospital - Emergency Department".to_string()),
    alias: Some(vec!["ED".to_string(), "Emergency Room".to_string(), "ER".to_string()]),
    description: Some("24/7 emergency department with 20 beds and trauma center".to_string()),
    mode: Some("instance".to_string()),
    types: Some(vec!["emergency-department".to_string(), "trauma-center".to_string()]),
    type_codes: Some(vec!["ED".to_string(), "TC".to_string()]),
    type_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string(), "http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string()]),
    type_displays: Some(vec!["Emergency Department".to_string(), "Trauma Center".to_string()]),
    contact_purpose: Some(vec!["general".to_string(), "emergency".to_string()]),
    contact_purpose_codes: Some(vec!["GENERAL".to_string(), "EMERGENCY".to_string()]),
    contact_purpose_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/contactentity-type".to_string(), "http://terminology.hl7.org/CodeSystem/contactentity-type".to_string()]),
    contact_purpose_displays: Some(vec!["General".to_string(), "Emergency".to_string()]),
    contact_name: Some(vec!["Emergency Department".to_string(), "Trauma Team".to_string()]),
    contact_telecom_system: Some(vec![vec!["phone".to_string(), "email".to_string()], vec!["phone".to_string(), "pager".to_string()]]),
    contact_telecom_value: Some(vec![vec!["+1-555-123-4567".to_string(), "ed@hospital.com".to_string()], vec!["+1-555-123-4568".to_string(), "trauma@hospital.com".to_string()]]),
    contact_telecom_use: Some(vec![vec!["work".to_string(), "work".to_string()], vec!["work".to_string(), "work".to_string()]]),
    contact_telecom_rank: Some(vec![vec![1, 2], vec![1, 2]]),
    contact_address_use: Some(vec!["work".to_string(), "work".to_string()]),
    contact_address_type: Some(vec!["physical".to_string(), "physical".to_string()]),
    contact_address_text: Some(vec!["123 Main St, Anytown, ST 12345, USA".to_string(), "123 Main St, Anytown, ST 12345, USA".to_string()]),
    contact_address_line: Some(vec![vec!["123 Main St".to_string()], vec!["123 Main St".to_string()]]),
    contact_address_city: Some(vec!["Anytown".to_string(), "Anytown".to_string()]),
    contact_address_state: Some(vec!["ST".to_string(), "ST".to_string()]),
    contact_address_postal_code: Some(vec!["12345".to_string(), "12345".to_string()]),
    contact_address_country: Some(vec!["USA".to_string(), "USA".to_string()]),
    contact_organization_id: Some(vec!["org_hospital".to_string(), "org_hospital".to_string()]),
    contact_organization_type: Some(vec!["Organization".to_string(), "Organization".to_string()]),
    contact_period_start: Some(vec!["2020-01-01T00:00:00Z".to_string(), "2020-01-01T00:00:00Z".to_string()]),
    contact_period_end: Some(vec!["2030-01-01T00:00:00Z".to_string(), "2030-01-01T00:00:00Z".to_string()]),
    address_use: Some("work".to_string()),
    address_type: Some("physical".to_string()),
    address_text: Some("123 Main St, Anytown, ST 12345, USA".to_string()),
    address_line: Some(vec!["123 Main St".to_string()]),
    address_city: Some("Anytown".to_string()),
    address_state: Some("ST".to_string()),
    address_postal_code: Some("12345".to_string()),
    address_country: Some("USA".to_string()),
    address_period_start: Some("2020-01-01T00:00:00Z".to_string()),
    address_period_end: Some("2030-01-01T00:00:00Z".to_string()),
    form: Some("building".to_string()),
    form_code: Some("bu".to_string()),
    form_system: Some("http://terminology.hl7.org/CodeSystem/location-physical-type".to_string()),
    form_display: Some("Building".to_string()),
    longitude: Some(-122.4194),
    latitude: Some(37.7749),
    altitude: Some(10.5),
    managing_organization_id: Some("org_hospital".to_string()),
    managing_organization_type: Some("Organization".to_string()),
    part_of_id: Some("loc_main_hospital".to_string()),
    part_of_type: Some("Location".to_string()),
    characteristics: Some(vec!["wheelchair-accessible".to_string(), "24-7-access".to_string()]),
    characteristic_codes: Some(vec!["WHEEL".to_string(), "24/7".to_string()]),
    characteristic_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/location-characteristic".to_string(), "http://terminology.hl7.org/CodeSystem/location-characteristic".to_string()]),
    characteristic_displays: Some(vec!["Wheelchair Accessible".to_string(), "24/7 Access".to_string()]),
    hours_of_operation_days_of_week: Some(vec![vec!["mon".to_string(), "tue".to_string(), "wed".to_string(), "thu".to_string(), "fri".to_string(), "sat".to_string(), "sun".to_string()]]),
    hours_of_operation_all_day: Some(vec![true]),
    hours_of_operation_opening_time: Some(vec!["00:00:00".to_string()]),
    hours_of_operation_closing_time: Some(vec!["23:59:59".to_string()]),
    virtual_service_channel_type: Some(vec!["video".to_string(), "audio".to_string()]),
    virtual_service_channel_type_codes: Some(vec!["video".to_string(), "audio".to_string()]),
    virtual_service_channel_type_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-EncounterChannel".to_string(), "http://terminology.hl7.org/CodeSystem/v3-EncounterChannel".to_string()]),
    virtual_service_channel_type_displays: Some(vec!["Video".to_string(), "Audio".to_string()]),
    virtual_service_address_url: Some(vec!["https://hospital.com/ed-video".to_string(), "https://hospital.com/ed-audio".to_string()]),
    endpoint_ids: Some(vec!["endpoint_ed_1".to_string(), "endpoint_ed_2".to_string()]),
    endpoint_types: Some(vec!["Endpoint".to_string(), "Endpoint".to_string()]),
};

// Convert to FHIR Location
let fhir_location: Location = domain_location.into();
```

## Clinical Use Cases

### 1. Hospital Emergency Department
```rust
let ed_location = DomainLocation {
    location_id: "loc_ed_001".to_string(),
    status: Some("active".to_string()),
    operational_status: Some("occupied".to_string()),
    operational_status_code: Some("O".to_string()),
    operational_status_system: Some("http://terminology.hl7.org/CodeSystem/v2-0116".to_string()),
    operational_status_display: Some("Occupied".to_string()),
    name: Some("Main Hospital - Emergency Department".to_string()),
    alias: Some(vec!["ED".to_string(), "Emergency Room".to_string(), "ER".to_string()]),
    description: Some("24/7 emergency department with 20 beds and trauma center".to_string()),
    mode: Some("instance".to_string()),
    types: Some(vec!["emergency-department".to_string(), "trauma-center".to_string()]),
    type_codes: Some(vec!["ED".to_string(), "TC".to_string()]),
    type_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string(), "http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string()]),
    type_displays: Some(vec!["Emergency Department".to_string(), "Trauma Center".to_string()]),
    contact_purpose: Some(vec!["general".to_string(), "emergency".to_string()]),
    contact_purpose_codes: Some(vec!["GENERAL".to_string(), "EMERGENCY".to_string()]),
    contact_purpose_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/contactentity-type".to_string(), "http://terminology.hl7.org/CodeSystem/contactentity-type".to_string()]),
    contact_purpose_displays: Some(vec!["General".to_string(), "Emergency".to_string()]),
    contact_name: Some(vec!["Emergency Department".to_string(), "Trauma Team".to_string()]),
    contact_telecom_system: Some(vec![vec!["phone".to_string(), "email".to_string()], vec!["phone".to_string(), "pager".to_string()]]),
    contact_telecom_value: Some(vec![vec!["+1-555-123-4567".to_string(), "ed@hospital.com".to_string()], vec!["+1-555-123-4568".to_string(), "trauma@hospital.com".to_string()]]),
    contact_telecom_use: Some(vec![vec!["work".to_string(), "work".to_string()], vec!["work".to_string(), "work".to_string()]]),
    contact_telecom_rank: Some(vec![vec![1, 2], vec![1, 2]]),
    contact_address_use: Some(vec!["work".to_string(), "work".to_string()]),
    contact_address_type: Some(vec!["physical".to_string(), "physical".to_string()]),
    contact_address_text: Some(vec!["123 Main St, Anytown, ST 12345, USA".to_string(), "123 Main St, Anytown, ST 12345, USA".to_string()]),
    contact_address_line: Some(vec![vec!["123 Main St".to_string()], vec!["123 Main St".to_string()]]),
    contact_address_city: Some(vec!["Anytown".to_string(), "Anytown".to_string()]),
    contact_address_state: Some(vec!["ST".to_string(), "ST".to_string()]),
    contact_address_postal_code: Some(vec!["12345".to_string(), "12345".to_string()]),
    contact_address_country: Some(vec!["USA".to_string(), "USA".to_string()]),
    contact_organization_id: Some(vec!["org_hospital".to_string(), "org_hospital".to_string()]),
    contact_organization_type: Some(vec!["Organization".to_string(), "Organization".to_string()]),
    contact_period_start: Some(vec!["2020-01-01T00:00:00Z".to_string(), "2020-01-01T00:00:00Z".to_string()]),
    contact_period_end: Some(vec!["2030-01-01T00:00:00Z".to_string(), "2030-01-01T00:00:00Z".to_string()]),
    address_use: Some("work".to_string()),
    address_type: Some("physical".to_string()),
    address_text: Some("123 Main St, Anytown, ST 12345, USA".to_string()),
    address_line: Some(vec!["123 Main St".to_string()]),
    address_city: Some("Anytown".to_string()),
    address_state: Some("ST".to_string()),
    address_postal_code: Some("12345".to_string()),
    address_country: Some("USA".to_string()),
    address_period_start: Some("2020-01-01T00:00:00Z".to_string()),
    address_period_end: Some("2030-01-01T00:00:00Z".to_string()),
    form: Some("building".to_string()),
    form_code: Some("bu".to_string()),
    form_system: Some("http://terminology.hl7.org/CodeSystem/location-physical-type".to_string()),
    form_display: Some("Building".to_string()),
    longitude: Some(-122.4194),
    latitude: Some(37.7749),
    altitude: Some(10.5),
    managing_organization_id: Some("org_hospital".to_string()),
    managing_organization_type: Some("Organization".to_string()),
    part_of_id: Some("loc_main_hospital".to_string()),
    part_of_type: Some("Location".to_string()),
    characteristics: Some(vec!["wheelchair-accessible".to_string(), "24-7-access".to_string()]),
    characteristic_codes: Some(vec!["WHEEL".to_string(), "24/7".to_string()]),
    characteristic_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/location-characteristic".to_string(), "http://terminology.hl7.org/CodeSystem/location-characteristic".to_string()]),
    characteristic_displays: Some(vec!["Wheelchair Accessible".to_string(), "24/7 Access".to_string()]),
    hours_of_operation_days_of_week: Some(vec![vec!["mon".to_string(), "tue".to_string(), "wed".to_string(), "thu".to_string(), "fri".to_string(), "sat".to_string(), "sun".to_string()]]),
    hours_of_operation_all_day: Some(vec![true]),
    hours_of_operation_opening_time: Some(vec!["00:00:00".to_string()]),
    hours_of_operation_closing_time: Some(vec!["23:59:59".to_string()]),
    virtual_service_channel_type: Some(vec!["video".to_string(), "audio".to_string()]),
    virtual_service_channel_type_codes: Some(vec!["video".to_string(), "audio".to_string()]),
    virtual_service_channel_type_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-EncounterChannel".to_string(), "http://terminology.hl7.org/CodeSystem/v3-EncounterChannel".to_string()]),
    virtual_service_channel_type_displays: Some(vec!["Video".to_string(), "Audio".to_string()]),
    virtual_service_address_url: Some(vec!["https://hospital.com/ed-video".to_string(), "https://hospital.com/ed-audio".to_string()]),
    endpoint_ids: Some(vec!["endpoint_ed_1".to_string(), "endpoint_ed_2".to_string()]),
    endpoint_types: Some(vec!["Endpoint".to_string(), "Endpoint".to_string()]),
    ..Default::default()
};
```

### 2. Outpatient Clinic
```rust
let clinic_location = DomainLocation {
    location_id: "loc_clinic_001".to_string(),
    status: Some("active".to_string()),
    operational_status: Some("available".to_string()),
    operational_status_code: Some("A".to_string()),
    operational_status_system: Some("http://terminology.hl7.org/CodeSystem/v2-0116".to_string()),
    operational_status_display: Some("Available".to_string()),
    name: Some("Downtown Family Practice Clinic".to_string()),
    alias: Some(vec!["Family Practice".to_string(), "Downtown Clinic".to_string()]),
    description: Some("Primary care clinic providing family medicine services".to_string()),
    mode: Some("instance".to_string()),
    types: Some(vec!["clinic".to_string(), "outpatient".to_string()]),
    type_codes: Some(vec!["CLINIC".to_string(), "OUTPATIENT".to_string()]),
    type_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string(), "http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string()]),
    type_displays: Some(vec!["Clinic".to_string(), "Outpatient".to_string()]),
    contact_purpose: Some(vec!["general".to_string(), "appointments".to_string()]),
    contact_purpose_codes: Some(vec!["GENERAL".to_string(), "APPOINTMENTS".to_string()]),
    contact_purpose_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/contactentity-type".to_string(), "http://terminology.hl7.org/CodeSystem/contactentity-type".to_string()]),
    contact_purpose_displays: Some(vec!["General".to_string(), "Appointments".to_string()]),
    contact_name: Some(vec!["Main Clinic".to_string(), "Appointment Desk".to_string()]),
    contact_telecom_system: Some(vec![vec!["phone".to_string(), "email".to_string()], vec!["phone".to_string(), "email".to_string()]]),
    contact_telecom_value: Some(vec![vec!["+1-555-987-6543".to_string(), "clinic@hospital.com".to_string()], vec!["+1-555-987-6544".to_string(), "appointments@hospital.com".to_string()]]),
    contact_telecom_use: Some(vec![vec!["work".to_string(), "work".to_string()], vec!["work".to_string(), "work".to_string()]]),
    contact_telecom_rank: Some(vec![vec![1, 2], vec![1, 2]]),
    contact_address_use: Some(vec!["work".to_string(), "work".to_string()]),
    contact_address_type: Some(vec!["physical".to_string(), "physical".to_string()]),
    contact_address_text: Some(vec!["456 Oak Ave, Anytown, ST 12346, USA".to_string(), "456 Oak Ave, Anytown, ST 12346, USA".to_string()]),
    contact_address_line: Some(vec![vec!["456 Oak Ave".to_string()], vec!["456 Oak Ave".to_string()]]),
    contact_address_city: Some(vec!["Anytown".to_string(), "Anytown".to_string()]),
    contact_address_state: Some(vec!["ST".to_string(), "ST".to_string()]),
    contact_address_postal_code: Some(vec!["12346".to_string(), "12346".to_string()]),
    contact_address_country: Some(vec!["USA".to_string(), "USA".to_string()]),
    contact_organization_id: Some(vec!["org_hospital".to_string(), "org_hospital".to_string()]),
    contact_organization_type: Some(vec!["Organization".to_string(), "Organization".to_string()]),
    contact_period_start: Some(vec!["2020-01-01T00:00:00Z".to_string(), "2020-01-01T00:00:00Z".to_string()]),
    contact_period_end: Some(vec!["2030-01-01T00:00:00Z".to_string(), "2030-01-01T00:00:00Z".to_string()]),
    address_use: Some("work".to_string()),
    address_type: Some("physical".to_string()),
    address_text: Some("456 Oak Ave, Anytown, ST 12346, USA".to_string()),
    address_line: Some(vec!["456 Oak Ave".to_string()]),
    address_city: Some("Anytown".to_string()),
    address_state: Some("ST".to_string()),
    address_postal_code: Some("12346".to_string()),
    address_country: Some("USA".to_string()),
    address_period_start: Some("2020-01-01T00:00:00Z".to_string()),
    address_period_end: Some("2030-01-01T00:00:00Z".to_string()),
    form: Some("building".to_string()),
    form_code: Some("bu".to_string()),
    form_system: Some("http://terminology.hl7.org/CodeSystem/location-physical-type".to_string()),
    form_display: Some("Building".to_string()),
    longitude: Some(-122.4194),
    latitude: Some(37.7749),
    altitude: Some(10.5),
    managing_organization_id: Some("org_hospital".to_string()),
    managing_organization_type: Some("Organization".to_string()),
    part_of_id: Some("loc_main_hospital".to_string()),
    part_of_type: Some("Location".to_string()),
    characteristics: Some(vec!["wheelchair-accessible".to_string(), "parking-available".to_string()]),
    characteristic_codes: Some(vec!["WHEEL".to_string(), "PARKING".to_string()]),
    characteristic_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/location-characteristic".to_string(), "http://terminology.hl7.org/CodeSystem/location-characteristic".to_string()]),
    characteristic_displays: Some(vec!["Wheelchair Accessible".to_string(), "Parking Available".to_string()]),
    hours_of_operation_days_of_week: Some(vec![vec!["mon".to_string(), "tue".to_string(), "wed".to_string(), "thu".to_string(), "fri".to_string()]]),
    hours_of_operation_all_day: Some(vec![false]),
    hours_of_operation_opening_time: Some(vec!["08:00:00".to_string()]),
    hours_of_operation_closing_time: Some(vec!["17:00:00".to_string()]),
    virtual_service_channel_type: Some(vec!["video".to_string()]),
    virtual_service_channel_type_codes: Some(vec!["video".to_string()]),
    virtual_service_channel_type_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v3-EncounterChannel".to_string()]),
    virtual_service_channel_type_displays: Some(vec!["Video".to_string()]),
    virtual_service_address_url: Some(vec!["https://hospital.com/clinic-video".to_string()]),
    endpoint_ids: Some(vec!["endpoint_clinic_1".to_string()]),
    endpoint_types: Some(vec!["Endpoint".to_string()]),
    ..Default::default()
};
```

## Next Steps

To complete the location implementation, you may want to:

1. **Add to Service Layer**: Integrate the location adapter into your gRPC service
2. **Add Database Adapters**: Create database adapters for storing/retrieving locations
3. **Add Validation**: Implement validation rules for location data
4. **Add Error Handling**: Enhance error handling for conversion failures
5. **Add Logging**: Add appropriate logging for location operations
6. **Add Geographic Services**: Integrate with mapping and geocoding services
7. **Add Capacity Management**: Implement bed/room capacity tracking
8. **Add Scheduling Integration**: Integrate with scheduling systems
9. **Add Analytics**: Analyze location utilization and performance
10. **Add Virtual Services**: Enhance virtual service capabilities

## Related Entities

This implementation follows the same pattern as the existing entities and integrates with:
- **Organization**: Locations are managed by organizations
- **Practitioner**: Practitioners work at specific locations
- **Patient**: Patients receive care at locations
- **Appointment**: Appointments are scheduled at locations
- **Encounter**: Encounters occur at specific locations
- **Endpoint**: Locations have technical endpoints

## Location Types Supported

The implementation supports various location types:
- **Hospitals**: Main hospitals, specialty hospitals
- **Clinics**: Outpatient clinics, specialty clinics
- **Emergency Departments**: EDs, trauma centers
- **Operating Rooms**: ORs, procedure rooms
- **Patient Rooms**: Inpatient rooms, ICU rooms
- **Laboratories**: Lab facilities, testing centers
- **Radiology**: Imaging centers, radiology departments
- **Pharmacy**: Hospital pharmacies, retail pharmacies
- **Administrative**: Offices, meeting rooms
- **Virtual**: Telehealth locations, virtual care

## Status Types

The implementation supports various status types:
- **Active**: Currently in use
- **Suspended**: Temporarily unavailable
- **Inactive**: No longer in use

## Operational Status Types

The implementation supports various operational status types:
- **Occupied**: Currently occupied
- **Available**: Available for use
- **Reserved**: Reserved for specific use
- **Unavailable**: Not available for use

## Mode Types

The implementation supports various mode types:
- **Instance**: Specific physical location
- **Kind**: Type of location (conceptual)

## Form Types

The implementation supports various form types:
- **Building**: Physical building
- **Room**: Individual room
- **Bed**: Hospital bed
- **Vehicle**: Ambulance, mobile unit
- **Virtual**: Virtual location

## Characteristics

The implementation supports various characteristics:
- **Accessibility**: Wheelchair accessible, ADA compliant
- **Availability**: 24/7 access, appointment only
- **Services**: Emergency services, specialty services
- **Amenities**: Parking, waiting areas, food services
- **Technology**: Telehealth capable, digital signage

## Hours of Operation

The implementation supports various hours of operation:
- **Days of Week**: Monday through Sunday
- **All Day**: 24/7 operation
- **Specific Times**: Opening and closing times
- **Exceptions**: Holiday hours, special schedules

## Virtual Services

The implementation supports various virtual services:
- **Video**: Video conferencing, telehealth
- **Audio**: Phone consultations, audio calls
- **Chat**: Text messaging, chat support
- **Email**: Email consultations, communication

## Geographic Information

The implementation supports various geographic information:
- **Coordinates**: Longitude, latitude, altitude
- **Address**: Full address with all components
- **Region**: State, province, country
- **Timezone**: Local timezone information

## Contact Information

The implementation supports various contact methods:
- **Phone**: Work phone, emergency phone
- **Email**: General email, emergency email
- **Fax**: Fax numbers
- **Pager**: Pager numbers
- **URL**: Website URLs
- **SMS**: SMS contact numbers
- **Other**: Other contact methods

## Address Types

The implementation supports various address types:
- **Work**: Professional address
- **Temporary**: Temporary address
- **Old**: Previous address
- **Billing**: Billing address

## Contact Purposes

The implementation supports various contact purposes:
- **General**: General information
- **Emergency**: Emergency contact
- **Billing**: Billing and financial
- **Appointments**: Scheduling and appointments
- **Technical**: Technical support
- **Administrative**: Administrative functions

## Endpoint Types

The implementation supports various endpoint types:
- **HL7 FHIR**: FHIR endpoints
- **HL7 v2**: HL7 v2 endpoints
- **DICOM**: DICOM endpoints
- **REST**: REST API endpoints
- **SOAP**: SOAP web service endpoints
- **Other**: Other endpoint types

This comprehensive location implementation provides a solid foundation for managing healthcare location information in your FHIR synchronization system, enabling location management, capacity tracking, and geographic services.
