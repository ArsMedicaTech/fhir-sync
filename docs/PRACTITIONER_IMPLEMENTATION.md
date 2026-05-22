# Practitioner Entity Implementation

This document describes the implementation of the `Practitioner` entity for the gRPC FHIR synchronization project.

## Files Created/Modified

### 1. Domain Model
- **File**: `src/domain/practitioner.rs`
- **Purpose**: Defines the `DomainPractitioner` struct that represents practitioner data in our domain model
- **Key Fields**:
  - `practitioner_id`: String (required) - Unique identifier for the practitioner
  - `active`: Option<bool> - Whether this practitioner's record is in active use
  - `family_name`: Option<String> - Family name (surname)
  - `given_names`: Option<Vec<String>> - Given names (first names)
  - `prefix`: Option<Vec<String>> - Prefixes (Dr., Prof., etc.)
  - `suffix`: Option<Vec<String>> - Suffixes (Jr., Sr., III, etc.)
  - `use_code`: Option<String> - Name use (usual, official, temp, nickname, anonymous, old, maiden)
  - `text`: Option<String> - Text representation of the full name
  - `telecom_system`: Option<Vec<String>> - Contact system (phone, fax, email, pager, url, sms, other)
  - `telecom_value`: Option<Vec<String>> - Contact value
  - `telecom_use`: Option<Vec<String>> - Contact use (home, work, temp, old, mobile)
  - `telecom_rank`: Option<Vec<u32>> - Specify preferred order of use (1 = highest)
  - `telecom_period_start`: Option<Vec<String>> - Time period when the contact point was/is in use (ISO datetime)
  - `telecom_period_end`: Option<Vec<String>> - Time period when the contact point was/is in use (ISO datetime)
  - `gender`: Option<String> - "male" | "female" | "other" | "unknown"
  - `gender_code`: Option<String> - Code for gender
  - `gender_system`: Option<String> - Terminology system for gender
  - `gender_display`: Option<String> - Display name for gender
  - `birth_date`: Option<String> - ISO date string for birth date
  - `deceased`: Option<bool> - Indicates if the practitioner is deceased or not
  - `deceased_date`: Option<String> - ISO datetime string for death date
  - `address_use`: Option<Vec<String>> - Address use (home, work, temp, old, billing)
  - `address_type`: Option<Vec<String>> - Address type (postal, physical, both)
  - `address_text`: Option<Vec<String>> - Text representation of the address
  - `address_line`: Option<Vec<Vec<String>>> - Street address lines
  - `address_city`: Option<Vec<String>> - City name
  - `address_district`: Option<Vec<String>> - District name (sublocality)
  - `address_state`: Option<Vec<String>> - State name
  - `address_postal_code`: Option<Vec<String>> - Postal code
  - `address_country`: Option<Vec<String>> - Country name
  - `address_period_start`: Option<Vec<String>> - Time period when address was/is in use (ISO datetime)
  - `address_period_end`: Option<Vec<String>> - Time period when address was/is in use (ISO datetime)
  - `photo_content_type`: Option<Vec<String>> - Mime type of the content
  - `photo_language`: Option<Vec<String>> - Human language of the content
  - `photo_data`: Option<Vec<String>> - Data inline, base64ed
  - `photo_url`: Option<Vec<String>> - Uri where the data can be found
  - `photo_size`: Option<Vec<u64>> - Number of bytes of content
  - `photo_hash`: Option<Vec<String>> - Hash of the data (sha-1, base64ed)
  - `photo_title`: Option<Vec<String>> - Label to display in place of the data
  - `photo_creation`: Option<Vec<String>> - Date attachment was first created (ISO datetime)
  - `qualification_identifiers`: Option<Vec<Vec<String>>> - An identifier for this qualification for the practitioner
  - `qualification_identifier_systems`: Option<Vec<Vec<String>>> - Terminology systems for qualification identifiers
  - `qualification_identifier_values`: Option<Vec<Vec<String>>> - Values for qualification identifiers
  - `qualification_identifier_uses`: Option<Vec<Vec<String>>> - Uses for qualification identifiers
  - `qualification_identifier_periods_start`: Option<Vec<Vec<String>>> - Periods for qualification identifiers (start)
  - `qualification_identifier_periods_end`: Option<Vec<Vec<String>>> - Periods for qualification identifiers (end)
  - `qualification_codes`: Option<Vec<String>> - Coded representation of the qualification
  - `qualification_code_codes`: Option<Vec<String>> - Codes for qualifications
  - `qualification_code_systems`: Option<Vec<String>> - Terminology systems for qualifications
  - `qualification_code_displays`: Option<Vec<String>> - Display names for qualifications
  - `qualification_periods_start`: Option<Vec<String>> - Period during which the qualification is valid (start)
  - `qualification_periods_end`: Option<Vec<String>> - Period during which the qualification is valid (end)
  - `qualification_issuer_ids`: Option<Vec<String>> - Organization that regulates and issues the qualification
  - `qualification_issuer_types`: Option<Vec<String>> - Types of qualification issuers
  - `communication_languages`: Option<Vec<String>> - The language code used to communicate with the practitioner
  - `communication_language_codes`: Option<Vec<String>> - Codes for communication languages
  - `communication_language_systems`: Option<Vec<String>> - Terminology systems for communication languages
  - `communication_language_displays`: Option<Vec<String>> - Display names for communication languages
  - `communication_preferred`: Option<Vec<bool>> - Language preference indicator

### 2. FHIR Adapter
- **File**: `src/adapters/entities/practitioner.rs`
- **Purpose**: Implements the conversion from `DomainPractitioner` to FHIR `Practitioner` proto message
- **Key Features**:
  - Maps domain fields to FHIR Practitioner structure
  - Handles name information with prefixes, suffixes, and use codes
  - Converts contact information with systems, uses, and periods
  - Maps gender with proper codes
  - Handles birth date and deceased status
  - Maps address information with all components
  - Handles photo attachments with metadata
  - Maps qualifications with identifiers, codes, and periods
  - Handles communication languages with preferences

### 3. Module Updates
- **File**: `src/domain/mod.rs` - Added `pub mod practitioner;`
- **File**: `src/adapters/entities/mod.rs` - Added `pub mod practitioner;`

## FHIR Mapping Details

### Gender
The adapter maps string gender values to FHIR gender codes:
- "male" → 1
- "female" → 2
- "other" → 3
- "unknown" → 4

### Name
The adapter maps name information to FHIR HumanName:
- Use: The provided use code
- Family: The family name
- Given: Array of given names
- Prefix: Array of prefixes
- Suffix: Array of suffixes
- Text: The full text representation

### Telecom
The adapter maps contact information to FHIR ContactPoint:
- System: The contact system (phone, email, etc.)
- Value: The contact value
- Use: The contact use (home, work, etc.)
- Rank: The preferred order of use
- Period: Time period when the contact point was/is in use

### Address
The adapter maps address information to FHIR Address:
- Use: Address use (home, work, etc.)
- Type: Address type (postal, physical, both)
- Text: Text representation of the address
- Line: Array of street address lines
- City: City name
- District: District name (sublocality)
- State: State name
- Postal Code: Postal code
- Country: Country name
- Period: Time period when address was/is in use

### Photo
The adapter maps photo information to FHIR Attachment:
- Content Type: Mime type of the content
- Language: Human language of the content
- Data: Data inline, base64ed
- URL: Uri where the data can be found
- Size: Number of bytes of content
- Hash: Hash of the data (sha-1, base64ed)
- Title: Label to display in place of the data
- Creation: Date attachment was first created

### Qualification
The adapter maps qualification information to FHIR Qualification:
- Identifier: Array of identifiers for the qualification
- Code: Coded representation of the qualification
- Period: Period during which the qualification is valid
- Issuer: Organization that regulates and issues the qualification

### Communication
The adapter maps communication information to FHIR Communication:
- Language: The language code used to communicate with the practitioner
- Preferred: Language preference indicator

### References
- Qualification issuer references: `{issuer_type}/{issuer_id}`

### Identifiers
- System: `urn:arsmedicatech:practitioner_id`
- Value: The practitioner ID

## Testing

The implementation includes comprehensive unit tests covering:
- Full deserialization with all fields
- Minimal deserialization with only required fields
- Doctor scenario with medical qualifications
- Nurse scenario with nursing qualifications
- Error handling for missing required fields

## Usage Example

```rust
use crate::domain::practitioner::DomainPractitioner;
use crate::adapters::entities::practitioner::*;

// Create a domain practitioner
let domain_practitioner = DomainPractitioner {
    practitioner_id: "prac_12345".to_string(),
    active: Some(true),
    family_name: Some("Smith".to_string()),
    given_names: Some(vec!["John".to_string(), "Michael".to_string()]),
    prefix: Some(vec!["Dr.".to_string()]),
    suffix: Some(vec!["MD".to_string()]),
    use_code: Some("official".to_string()),
    text: Some("Dr. John Michael Smith, MD".to_string()),
    telecom_system: Some(vec!["phone".to_string(), "email".to_string()]),
    telecom_value: Some(vec!["+1-555-123-4567".to_string(), "john.smith@hospital.com".to_string()]),
    telecom_use: Some(vec!["work".to_string(), "work".to_string()]),
    telecom_rank: Some(vec![1, 2]),
    gender: Some("male".to_string()),
    gender_code: Some("M".to_string()),
    gender_system: Some("http://hl7.org/fhir/administrative-gender".to_string()),
    gender_display: Some("Male".to_string()),
    birth_date: Some("1980-05-15".to_string()),
    deceased: Some(false),
    address_use: Some(vec!["work".to_string()]),
    address_type: Some(vec!["physical".to_string()]),
    address_text: Some(vec!["123 Main St, Suite 100, Anytown, ST 12345, USA".to_string()]),
    address_line: Some(vec![vec!["123 Main St".to_string(), "Suite 100".to_string()]]),
    address_city: Some(vec!["Anytown".to_string()]),
    address_state: Some(vec!["ST".to_string()]),
    address_postal_code: Some(vec!["12345".to_string()]),
    address_country: Some(vec!["USA".to_string()]),
    photo_content_type: Some(vec!["image/jpeg".to_string()]),
    photo_url: Some(vec!["https://hospital.com/photos/john_smith.jpg".to_string()]),
    photo_title: Some(vec!["Dr. John Smith - Headshot".to_string()]),
    qualification_codes: Some(vec!["MD".to_string(), "Internal Medicine".to_string()]),
    qualification_code_codes: Some(vec!["MD".to_string(), "IM".to_string()]),
    qualification_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v2-0360".to_string(), "http://terminology.hl7.org/CodeSystem/v2-0402".to_string()]),
    qualification_code_displays: Some(vec!["Doctor of Medicine".to_string(), "Internal Medicine".to_string()]),
    qualification_periods_start: Some(vec!["2010-06-01T00:00:00Z".to_string()]),
    qualification_periods_end: Some(vec!["2030-06-01T00:00:00Z".to_string()]),
    qualification_issuer_ids: Some(vec!["org_medical_board".to_string()]),
    qualification_issuer_types: Some(vec!["Organization".to_string()]),
    communication_languages: Some(vec!["English".to_string(), "Spanish".to_string()]),
    communication_language_codes: Some(vec!["en".to_string(), "es".to_string()]),
    communication_language_systems: Some(vec!["urn:ietf:bcp:47".to_string(), "urn:ietf:bcp:47".to_string()]),
    communication_language_displays: Some(vec!["English".to_string(), "Spanish".to_string()]),
    communication_preferred: Some(vec![true, false]),
};

// Convert to FHIR Practitioner
let fhir_practitioner: Practitioner = domain_practitioner.into();
```

## Clinical Use Cases

### 1. Doctor
```rust
let doctor = DomainPractitioner {
    practitioner_id: "prac_doctor_001".to_string(),
    active: Some(true),
    family_name: Some("Smith".to_string()),
    given_names: Some(vec!["John".to_string(), "Michael".to_string()]),
    prefix: Some(vec!["Dr.".to_string()]),
    suffix: Some(vec!["MD".to_string()]),
    use_code: Some("official".to_string()),
    text: Some("Dr. John Michael Smith, MD".to_string()),
    telecom_system: Some(vec!["phone".to_string(), "email".to_string()]),
    telecom_value: Some(vec!["+1-555-123-4567".to_string(), "john.smith@hospital.com".to_string()]),
    telecom_use: Some(vec!["work".to_string(), "work".to_string()]),
    telecom_rank: Some(vec![1, 2]),
    gender: Some("male".to_string()),
    gender_code: Some("M".to_string()),
    gender_system: Some("http://hl7.org/fhir/administrative-gender".to_string()),
    gender_display: Some("Male".to_string()),
    birth_date: Some("1980-05-15".to_string()),
    deceased: Some(false),
    address_use: Some(vec!["work".to_string()]),
    address_type: Some(vec!["physical".to_string()]),
    address_text: Some(vec!["123 Main St, Suite 100, Anytown, ST 12345, USA".to_string()]),
    address_line: Some(vec![vec!["123 Main St".to_string(), "Suite 100".to_string()]]),
    address_city: Some(vec!["Anytown".to_string()]),
    address_state: Some(vec!["ST".to_string()]),
    address_postal_code: Some(vec!["12345".to_string()]),
    address_country: Some(vec!["USA".to_string()]),
    photo_content_type: Some(vec!["image/jpeg".to_string()]),
    photo_url: Some(vec!["https://hospital.com/photos/john_smith.jpg".to_string()]),
    photo_title: Some(vec!["Dr. John Smith - Headshot".to_string()]),
    qualification_codes: Some(vec!["MD".to_string(), "Internal Medicine".to_string()]),
    qualification_code_codes: Some(vec!["MD".to_string(), "IM".to_string()]),
    qualification_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v2-0360".to_string(), "http://terminology.hl7.org/CodeSystem/v2-0402".to_string()]),
    qualification_code_displays: Some(vec!["Doctor of Medicine".to_string(), "Internal Medicine".to_string()]),
    qualification_periods_start: Some(vec!["2010-06-01T00:00:00Z".to_string()]),
    qualification_periods_end: Some(vec!["2030-06-01T00:00:00Z".to_string()]),
    qualification_issuer_ids: Some(vec!["org_medical_board".to_string()]),
    qualification_issuer_types: Some(vec!["Organization".to_string()]),
    communication_languages: Some(vec!["English".to_string(), "Spanish".to_string()]),
    communication_language_codes: Some(vec!["en".to_string(), "es".to_string()]),
    communication_language_systems: Some(vec!["urn:ietf:bcp:47".to_string(), "urn:ietf:bcp:47".to_string()]),
    communication_language_displays: Some(vec!["English".to_string(), "Spanish".to_string()]),
    communication_preferred: Some(vec![true, false]),
    ..Default::default()
};
```

### 2. Nurse
```rust
let nurse = DomainPractitioner {
    practitioner_id: "prac_nurse_001".to_string(),
    active: Some(true),
    family_name: Some("Johnson".to_string()),
    given_names: Some(vec!["Sarah".to_string(), "Elizabeth".to_string()]),
    prefix: Some(vec!["RN".to_string()]),
    suffix: Some(vec!["BSN".to_string()]),
    use_code: Some("official".to_string()),
    text: Some("Sarah Elizabeth Johnson, RN, BSN".to_string()),
    telecom_system: Some(vec!["phone".to_string(), "email".to_string()]),
    telecom_value: Some(vec!["+1-555-987-6543".to_string(), "sarah.johnson@hospital.com".to_string()]),
    telecom_use: Some(vec!["work".to_string(), "work".to_string()]),
    telecom_rank: Some(vec![1, 2]),
    gender: Some("female".to_string()),
    gender_code: Some("F".to_string()),
    gender_system: Some("http://hl7.org/fhir/administrative-gender".to_string()),
    gender_display: Some("Female".to_string()),
    birth_date: Some("1985-08-22".to_string()),
    deceased: Some(false),
    address_use: Some(vec!["home".to_string()]),
    address_type: Some(vec!["physical".to_string()]),
    address_text: Some(vec!["456 Oak Ave, Apartment 2B, Anytown, ST 12346, USA".to_string()]),
    address_line: Some(vec![vec!["456 Oak Ave".to_string(), "Apartment 2B".to_string()]]),
    address_city: Some(vec!["Anytown".to_string()]),
    address_state: Some(vec!["ST".to_string()]),
    address_postal_code: Some(vec!["12346".to_string()]),
    address_country: Some(vec!["USA".to_string()]),
    photo_content_type: Some(vec!["image/jpeg".to_string()]),
    photo_url: Some(vec!["https://hospital.com/photos/sarah_johnson.jpg".to_string()]),
    photo_title: Some(vec!["Sarah Johnson - Professional Photo".to_string()]),
    qualification_codes: Some(vec!["RN".to_string(), "BSN".to_string(), "Critical Care".to_string()]),
    qualification_code_codes: Some(vec!["RN".to_string(), "BSN".to_string(), "CC".to_string()]),
    qualification_code_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/v2-0360".to_string(), "http://terminology.hl7.org/CodeSystem/v2-0360".to_string(), "http://terminology.hl7.org/CodeSystem/v2-0402".to_string()]),
    qualification_code_displays: Some(vec!["Registered Nurse".to_string(), "Bachelor of Science in Nursing".to_string(), "Critical Care".to_string()]),
    qualification_periods_start: Some(vec!["2008-05-01T00:00:00Z".to_string(), "2008-05-01T00:00:00Z".to_string(), "2015-01-01T00:00:00Z".to_string()]),
    qualification_periods_end: Some(vec!["2028-05-01T00:00:00Z".to_string(), "2028-05-01T00:00:00Z".to_string(), "2025-01-01T00:00:00Z".to_string()]),
    qualification_issuer_ids: Some(vec!["org_nursing_board".to_string(), "org_nursing_board".to_string(), "org_critical_care_cert".to_string()]),
    qualification_issuer_types: Some(vec!["Organization".to_string(), "Organization".to_string(), "Organization".to_string()]),
    communication_languages: Some(vec!["English".to_string(), "French".to_string()]),
    communication_language_codes: Some(vec!["en".to_string(), "fr".to_string()]),
    communication_language_systems: Some(vec!["urn:ietf:bcp:47".to_string(), "urn:ietf:bcp:47".to_string()]),
    communication_language_displays: Some(vec!["English".to_string(), "French".to_string()]),
    communication_preferred: Some(vec![true, false]),
    ..Default::default()
};
```

## Next Steps

To complete the practitioner implementation, you may want to:

1. **Add to Service Layer**: Integrate the practitioner adapter into your gRPC service
2. **Add Database Adapters**: Create database adapters for storing/retrieving practitioners
3. **Add Validation**: Implement validation rules for practitioner data
4. **Add Error Handling**: Enhance error handling for conversion failures
5. **Add Logging**: Add appropriate logging for practitioner operations
6. **Add Credential Management**: Integrate with credential management systems
7. **Add License Verification**: Implement license verification and renewal tracking
8. **Add Role Management**: Create role-based access control for practitioners
9. **Add Scheduling Integration**: Integrate with scheduling systems
10. **Add Analytics**: Analyze practitioner performance and utilization

## Related Entities

This implementation follows the same pattern as the existing entities and integrates with:
- **Patient**: Practitioners provide care to patients
- **Appointment**: Practitioners are scheduled for appointments
- **Encounter**: Practitioners participate in healthcare encounters
- **Organization**: Practitioners are affiliated with organizations
- **Location**: Practitioners work at specific locations
- **Qualification**: Practitioners have various qualifications and certifications

## Practitioner Types Supported

The implementation supports various practitioner types:
- **Doctors**: Physicians, specialists, surgeons
- **Nurses**: Registered nurses, nurse practitioners, clinical nurse specialists
- **Allied Health**: Physical therapists, occupational therapists, pharmacists
- **Technicians**: Laboratory technicians, radiology technicians, respiratory therapists
- **Administrative**: Healthcare administrators, managers, coordinators
- **Other**: Other healthcare professionals and support staff

## Name Components

The implementation supports various name components:
- **Family Name**: Surname or last name
- **Given Names**: First names and middle names
- **Prefixes**: Professional titles (Dr., Prof., etc.)
- **Suffixes**: Generational suffixes (Jr., Sr., III, etc.)
- **Use Codes**: Name usage (usual, official, temp, nickname, anonymous, old, maiden)

## Contact Information

The implementation supports various contact methods:
- **Phone**: Work phone, home phone, mobile phone
- **Email**: Work email, personal email
- **Fax**: Fax numbers
- **Pager**: Pager numbers
- **URL**: Website URLs
- **SMS**: SMS contact numbers
- **Other**: Other contact methods

## Address Types

The implementation supports various address types:
- **Home**: Residential address
- **Work**: Professional address
- **Temporary**: Temporary address
- **Old**: Previous address
- **Billing**: Billing address

## Qualification Types

The implementation supports various qualification types:
- **Medical Degrees**: MD, DO, MBBS, etc.
- **Nursing Degrees**: RN, BSN, MSN, DNP, etc.
- **Specialties**: Internal Medicine, Surgery, Pediatrics, etc.
- **Certifications**: Board certifications, specialty certifications
- **Licenses**: Professional licenses, state licenses
- **Training**: Residency, fellowship, continuing education

## Communication Languages

The implementation supports various communication languages:
- **Primary Language**: Practitioner's primary language
- **Secondary Languages**: Additional languages spoken
- **Preferred Language**: Preferred language for communication
- **Language Codes**: Standard language codes (ISO 639-1, ISO 639-2)

## Photo Management

The implementation supports various photo types:
- **Professional Headshots**: Official professional photos
- **ID Photos**: Identification photos
- **Profile Photos**: Profile pictures for systems
- **Other Photos**: Other relevant photos

## Gender Support

The implementation supports various gender options:
- **Male**: Male gender
- **Female**: Female gender
- **Other**: Other gender identity
- **Unknown**: Unknown or unspecified gender

## Active Status

The implementation supports various active statuses:
- **Active**: Currently practicing
- **Inactive**: Not currently practicing
- **Retired**: Retired from practice
- **Deceased**: Deceased practitioner

## Qualification Management

The implementation supports various qualification management features:
- **Identifier Management**: Multiple identifiers per qualification
- **Period Management**: Validity periods for qualifications
- **Issuer Management**: Organizations that issue qualifications
- **Code Management**: Standardized codes for qualifications
- **Display Management**: Human-readable names for qualifications

## Communication Preferences

The implementation supports various communication preferences:
- **Language Preferences**: Preferred languages for communication
- **Contact Preferences**: Preferred contact methods
- **Availability**: Availability for communication
- **Time Zones**: Time zone preferences

This comprehensive practitioner implementation provides a solid foundation for managing healthcare practitioner information in your FHIR synchronization system, enabling credential management, role-based access control, and practitioner analytics.
