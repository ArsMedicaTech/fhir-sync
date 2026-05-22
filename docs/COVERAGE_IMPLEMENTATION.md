# Coverage Entity Implementation

This document describes the implementation of the `Coverage` entity for the gRPC FHIR synchronization project.

## Files Created/Modified

### 1. Domain Model
- **File**: `src/domain/coverage.rs`
- **Purpose**: Defines the `DomainCoverage` struct that represents insurance/medical plan coverage data in our domain model
- **Key Fields**:
  - `coverage_id`: String (required) - Unique identifier for the coverage
  - `patient_demographic_no`: String (required) - Reference to the patient
  - `status`: Option<String> - Status (active, cancelled, draft, entered-in-error)
  - `kind`: Option<String> - Kind (insurance, self-pay, other)
  - `type`: Option<String> - Coverage category such as medical or accident
  - `type_code`: Option<String> - Code for coverage type
  - `type_system`: Option<String> - Terminology system for type
  - `type_display`: Option<String> - Display name for type
  - `policy_holder_id`: Option<String> - Owner of the policy
  - `policy_holder_type`: Option<String> - Type of policy holder (Patient, RelatedPerson, Organization)
  - `subscriber_id`: Option<String> - Subscriber to the policy
  - `subscriber_type`: Option<String> - Type of subscriber (Patient, RelatedPerson)
  - `subscriber_identifier`: Option<String> - ID assigned to the subscriber
  - `beneficiary_id`: Option<String> - Plan beneficiary (Patient)
  - `dependent_number`: Option<String> - Dependent number
  - `relationship`: Option<String> - Beneficiary relationship to the subscriber
  - `relationship_code`: Option<String> - Code for relationship
  - `relationship_system`: Option<String> - Terminology system for relationship
  - `relationship_display`: Option<String> - Display name for relationship
  - `period_start`: Option<String> - ISO datetime string for coverage start
  - `period_end`: Option<String> - ISO datetime string for coverage end
  - `insurer_id`: Option<String> - Issuer of the policy
  - `network`: Option<String> - Insurer network
  - `class_types`: Option<Vec<String>> - Types of class such as 'group' or 'plan'
  - `class_values`: Option<Vec<String>> - Values associated with the types
  - `class_names`: Option<Vec<String>> - Human readable descriptions
  - `class_systems`: Option<Vec<String>> - Terminology systems for class types
  - `order`: Option<u32> - Relative order of the coverage
  - `cost_types`: Option<Vec<String>> - Cost categories
  - `cost_categories`: Option<Vec<String>> - Benefit classifications
  - `cost_networks`: Option<Vec<String>> - In or out of network
  - `cost_units`: Option<Vec<String>> - Individual or family
  - `cost_terms`: Option<Vec<String>> - Annual or lifetime
  - `cost_values`: Option<Vec<String>> - The amount or percentage due from the beneficiary
  - `cost_value_types`: Option<Vec<String>> - Type of cost value (quantity, money)
  - `payment_by_party_ids`: Option<Vec<String>> - Parties performing self-payment
  - `payment_by_party_types`: Option<Vec<String>> - Types of payment parties
  - `payment_by_responsibilities`: Option<Vec<String>> - Party's responsibility
  - `subrogation`: Option<bool> - Reimbursement to insurer
  - `contract_ids`: Option<Vec<String>> - Contract details
  - `insurance_plan_id`: Option<String> - Insurance plan details
  - `notes`: Option<String> - Additional notes about the coverage

### 2. FHIR Adapter
- **File**: `src/adapters/entities/coverage.rs`
- **Purpose**: Implements the conversion from `DomainCoverage` to FHIR `Coverage` proto message
- **Key Features**:
  - Maps domain fields to FHIR Coverage structure
  - Handles status and kind conversion with proper codes
  - Converts datetime strings to FHIR DateTime types
  - Creates proper FHIR references for patient, organization, and other resources
  - Handles coverage classifications and cost information
  - Maps payment information and subrogation details
  - Creates proper FHIR cost-to-beneficiary structures

### 3. Module Updates
- **File**: `src/domain/mod.rs` - Added `pub mod coverage;`
- **File**: `src/adapters/entities/mod.rs` - Added `pub mod coverage;`

## FHIR Mapping Details

### Status
The adapter maps string status values to FHIR status codes:
- "active" → 1
- "cancelled" → 2
- "draft" → 3
- "entered-in-error" → 4

### Kind
The adapter maps string kind values to FHIR kind codes:
- "insurance" → 1
- "self-pay" → 2
- "other" → 3

### Type
The adapter maps string type values to FHIR CodeableConcept:
- System: Defaults to `http://terminology.hl7.org/CodeSystem/v3-ActCode` or uses provided system
- Code: The provided type code
- Display: The human-readable type name
- Text: The provided type string

### References
- Policy holder reference: `{policy_holder_type}/{policy_holder_id}`
- Subscriber reference: `{subscriber_type}/{subscriber_id}`
- Beneficiary reference: `Patient/{beneficiary_id}`
- Insurer reference: `Organization/{insurer_id}`
- Contract references: `Contract/{contract_id}`
- Insurance plan reference: `InsurancePlan/{insurance_plan_id}`

### Identifiers
- System: `urn:arsmedicatech:coverage_id`
- Value: The coverage ID

### Period
The adapter handles temporal information:
- Start: ISO datetime strings are converted to FHIR DateTime
- End: ISO datetime strings are converted to FHIR DateTime

### Class
The adapter handles coverage classifications:
- Type: The class type (e.g., "group", "plan")
- Value: The class value identifier
- Name: Human-readable description

### Cost to Beneficiary
The adapter handles cost information:
- Type: Cost category (e.g., "copay", "deductible")
- Category: Benefit classification
- Network: In or out of network
- Unit: Individual or family
- Term: Annual or lifetime
- Value: Amount or percentage (supports both quantity and money)

### Payment By
The adapter handles self-payment information:
- Party: Reference to the party performing self-payment
- Responsibility: Description of the party's responsibility

### Relationship
The adapter maps relationship information:
- System: Defaults to `http://terminology.hl7.org/CodeSystem/subscriber-relationship` or uses provided system
- Code: The provided relationship code
- Display: The human-readable relationship name
- Text: The provided relationship string

## Testing

The implementation includes comprehensive unit tests covering:
- Full deserialization with all fields
- Minimal deserialization with only required fields
- Medicare coverage scenario
- Self-pay coverage scenario
- Dependent coverage scenario
- Error handling for missing required fields

## Usage Example

```rust
use crate::domain::coverage::DomainCoverage;
use crate::adapters::entities::coverage::*;

// Create a domain coverage
let domain_coverage = DomainCoverage {
    coverage_id: "cov_12345".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("active".to_string()),
    kind: Some("insurance".to_string()),
    r#type: Some("medical".to_string()),
    type_code: Some("MED".to_string()),
    type_system: Some("http://terminology.hl7.org/CodeSystem/v3-ActCode".to_string()),
    type_display: Some("Medical".to_string()),
    policy_holder_id: Some("pat_12345".to_string()),
    policy_holder_type: Some("Patient".to_string()),
    subscriber_id: Some("pat_12345".to_string()),
    subscriber_type: Some("Patient".to_string()),
    subscriber_identifier: Some("SUB123456789".to_string()),
    beneficiary_id: Some("pat_12345".to_string()),
    dependent_number: Some("01".to_string()),
    relationship: Some("self".to_string()),
    relationship_code: Some("self".to_string()),
    relationship_system: Some("http://terminology.hl7.org/CodeSystem/subscriber-relationship".to_string()),
    relationship_display: Some("Self".to_string()),
    period_start: Some("2024-01-01T00:00:00Z".to_string()),
    period_end: Some("2024-12-31T23:59:59Z".to_string()),
    insurer_id: Some("org_insurer_001".to_string()),
    network: Some("PPO Network".to_string()),
    class_types: Some(vec!["group".to_string(), "plan".to_string()]),
    class_values: Some(vec!["GROUP001".to_string(), "PLAN001".to_string()]),
    class_names: Some(vec!["Employee Group".to_string(), "Premium Plan".to_string()]),
    class_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/coverage-class".to_string(), "http://terminology.hl7.org/CodeSystem/coverage-class".to_string()]),
    order: Some(1),
    cost_types: Some(vec!["copay".to_string(), "deductible".to_string()]),
    cost_categories: Some(vec!["primary".to_string(), "primary".to_string()]),
    cost_networks: Some(vec!["in-network".to_string(), "in-network".to_string()]),
    cost_units: Some(vec!["individual".to_string(), "individual".to_string()]),
    cost_terms: Some(vec!["annual".to_string(), "annual".to_string()]),
    cost_values: Some(vec!["$25.00".to_string(), "$500.00".to_string()]),
    cost_value_types: Some(vec!["money".to_string(), "money".to_string()]),
    payment_by_party_ids: Some(vec![]),
    payment_by_party_types: Some(vec![]),
    payment_by_responsibilities: Some(vec![]),
    subrogation: Some(false),
    contract_ids: Some(vec!["contract_001".to_string()]),
    insurance_plan_id: Some("plan_001".to_string()),
    notes: Some("Primary insurance coverage for patient".to_string()),
};

// Convert to FHIR Coverage
let fhir_coverage: Coverage = domain_coverage.into();
```

## Clinical Use Cases

### 1. Commercial Insurance Coverage
```rust
let commercial_coverage = DomainCoverage {
    coverage_id: "cov_commercial_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("active".to_string()),
    kind: Some("insurance".to_string()),
    r#type: Some("medical".to_string()),
    type_code: Some("MED".to_string()),
    type_system: Some("http://terminology.hl7.org/CodeSystem/v3-ActCode".to_string()),
    type_display: Some("Medical".to_string()),
    policy_holder_id: Some("pat_12345".to_string()),
    policy_holder_type: Some("Patient".to_string()),
    subscriber_id: Some("pat_12345".to_string()),
    subscriber_type: Some("Patient".to_string()),
    subscriber_identifier: Some("SUB123456789".to_string()),
    beneficiary_id: Some("pat_12345".to_string()),
    relationship: Some("self".to_string()),
    relationship_code: Some("self".to_string()),
    relationship_system: Some("http://terminology.hl7.org/CodeSystem/subscriber-relationship".to_string()),
    relationship_display: Some("Self".to_string()),
    period_start: Some("2024-01-01T00:00:00Z".to_string()),
    period_end: Some("2024-12-31T23:59:59Z".to_string()),
    insurer_id: Some("org_insurer_001".to_string()),
    network: Some("PPO Network".to_string()),
    class_types: Some(vec!["group".to_string(), "plan".to_string()]),
    class_values: Some(vec!["GROUP001".to_string(), "PLAN001".to_string()]),
    class_names: Some(vec!["Employee Group".to_string(), "Premium Plan".to_string()]),
    class_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/coverage-class".to_string(), "http://terminology.hl7.org/CodeSystem/coverage-class".to_string()]),
    order: Some(1),
    cost_types: Some(vec!["copay".to_string(), "deductible".to_string()]),
    cost_categories: Some(vec!["primary".to_string(), "primary".to_string()]),
    cost_networks: Some(vec!["in-network".to_string(), "in-network".to_string()]),
    cost_units: Some(vec!["individual".to_string(), "individual".to_string()]),
    cost_terms: Some(vec!["annual".to_string(), "annual".to_string()]),
    cost_values: Some(vec!["$25.00".to_string(), "$500.00".to_string()]),
    cost_value_types: Some(vec!["money".to_string(), "money".to_string()]),
    subrogation: Some(false),
    contract_ids: Some(vec!["contract_001".to_string()]),
    insurance_plan_id: Some("plan_001".to_string()),
    notes: Some("Primary insurance coverage for patient".to_string()),
    ..Default::default()
};
```

### 2. Medicare Coverage
```rust
let medicare_coverage = DomainCoverage {
    coverage_id: "cov_medicare_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("active".to_string()),
    kind: Some("insurance".to_string()),
    r#type: Some("medicare".to_string()),
    type_code: Some("MEDICARE".to_string()),
    type_system: Some("http://terminology.hl7.org/CodeSystem/v3-ActCode".to_string()),
    type_display: Some("Medicare".to_string()),
    policy_holder_id: Some("pat_12345".to_string()),
    policy_holder_type: Some("Patient".to_string()),
    subscriber_id: Some("pat_12345".to_string()),
    subscriber_type: Some("Patient".to_string()),
    subscriber_identifier: Some("123456789A".to_string()),
    beneficiary_id: Some("pat_12345".to_string()),
    relationship: Some("self".to_string()),
    relationship_code: Some("self".to_string()),
    relationship_system: Some("http://terminology.hl7.org/CodeSystem/subscriber-relationship".to_string()),
    relationship_display: Some("Self".to_string()),
    period_start: Some("2024-01-01T00:00:00Z".to_string()),
    period_end: Some("2024-12-31T23:59:59Z".to_string()),
    insurer_id: Some("org_medicare".to_string()),
    network: Some("Medicare Network".to_string()),
    class_types: Some(vec!["group".to_string(), "plan".to_string()]),
    class_values: Some(vec!["MEDICARE".to_string(), "PART_B".to_string()]),
    class_names: Some(vec!["Medicare".to_string(), "Part B".to_string()]),
    class_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/coverage-class".to_string(), "http://terminology.hl7.org/CodeSystem/coverage-class".to_string()]),
    order: Some(1),
    cost_types: Some(vec!["copay".to_string(), "deductible".to_string()]),
    cost_categories: Some(vec!["primary".to_string(), "primary".to_string()]),
    cost_networks: Some(vec!["in-network".to_string(), "in-network".to_string()]),
    cost_units: Some(vec!["individual".to_string(), "individual".to_string()]),
    cost_terms: Some(vec!["annual".to_string(), "annual".to_string()]),
    cost_values: Some(vec!["$0.00".to_string(), "$240.00".to_string()]),
    cost_value_types: Some(vec!["money".to_string(), "money".to_string()]),
    subrogation: Some(false),
    contract_ids: Some(vec!["contract_medicare_001".to_string()]),
    insurance_plan_id: Some("plan_medicare_001".to_string()),
    notes: Some("Medicare Part B coverage for patient".to_string()),
    ..Default::default()
};
```

### 3. Self-Pay Coverage
```rust
let self_pay_coverage = DomainCoverage {
    coverage_id: "cov_self_pay_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("active".to_string()),
    kind: Some("self-pay".to_string()),
    r#type: Some("self-pay".to_string()),
    type_code: Some("SELF".to_string()),
    type_system: Some("http://terminology.hl7.org/CodeSystem/v3-ActCode".to_string()),
    type_display: Some("Self Pay".to_string()),
    policy_holder_id: Some("pat_12345".to_string()),
    policy_holder_type: Some("Patient".to_string()),
    subscriber_id: Some("pat_12345".to_string()),
    subscriber_type: Some("Patient".to_string()),
    beneficiary_id: Some("pat_12345".to_string()),
    relationship: Some("self".to_string()),
    relationship_code: Some("self".to_string()),
    relationship_system: Some("http://terminology.hl7.org/CodeSystem/subscriber-relationship".to_string()),
    relationship_display: Some("Self".to_string()),
    period_start: Some("2024-01-01T00:00:00Z".to_string()),
    period_end: Some("2024-12-31T23:59:59Z".to_string()),
    class_types: Some(vec!["plan".to_string()]),
    class_values: Some(vec!["SELF_PAY".to_string()]),
    class_names: Some(vec!["Self Pay Plan".to_string()]),
    class_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/coverage-class".to_string()]),
    order: Some(1),
    cost_types: Some(vec!["full-payment".to_string()]),
    cost_categories: Some(vec!["primary".to_string()]),
    cost_networks: Some(vec!["self-pay".to_string()]),
    cost_units: Some(vec!["individual".to_string()]),
    cost_terms: Some(vec!["per-service".to_string()]),
    cost_values: Some(vec!["100%".to_string()]),
    cost_value_types: Some(vec!["quantity".to_string()]),
    payment_by_party_ids: Some(vec!["pat_12345".to_string()]),
    payment_by_party_types: Some(vec!["Patient".to_string()]),
    payment_by_responsibilities: Some(vec!["Full payment responsibility".to_string()]),
    subrogation: Some(false),
    notes: Some("Patient responsible for full payment of services".to_string()),
    ..Default::default()
};
```

### 4. Dependent Coverage
```rust
let dependent_coverage = DomainCoverage {
    coverage_id: "cov_dependent_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("active".to_string()),
    kind: Some("insurance".to_string()),
    r#type: Some("medical".to_string()),
    type_code: Some("MED".to_string()),
    type_system: Some("http://terminology.hl7.org/CodeSystem/v3-ActCode".to_string()),
    type_display: Some("Medical".to_string()),
    policy_holder_id: Some("pat_parent_001".to_string()),
    policy_holder_type: Some("Patient".to_string()),
    subscriber_id: Some("pat_parent_001".to_string()),
    subscriber_type: Some("Patient".to_string()),
    subscriber_identifier: Some("SUB123456789".to_string()),
    beneficiary_id: Some("pat_12345".to_string()),
    dependent_number: Some("01".to_string()),
    relationship: Some("child".to_string()),
    relationship_code: Some("child".to_string()),
    relationship_system: Some("http://terminology.hl7.org/CodeSystem/subscriber-relationship".to_string()),
    relationship_display: Some("Child".to_string()),
    period_start: Some("2024-01-01T00:00:00Z".to_string()),
    period_end: Some("2024-12-31T23:59:59Z".to_string()),
    insurer_id: Some("org_insurer_001".to_string()),
    network: Some("PPO Network".to_string()),
    class_types: Some(vec!["group".to_string(), "plan".to_string()]),
    class_values: Some(vec!["GROUP001".to_string(), "PLAN001".to_string()]),
    class_names: Some(vec!["Employee Group".to_string(), "Premium Plan".to_string()]),
    class_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/coverage-class".to_string(), "http://terminology.hl7.org/CodeSystem/coverage-class".to_string()]),
    order: Some(1),
    cost_types: Some(vec!["copay".to_string(), "deductible".to_string()]),
    cost_categories: Some(vec!["primary".to_string(), "primary".to_string()]),
    cost_networks: Some(vec!["in-network".to_string(), "in-network".to_string()]),
    cost_units: Some(vec!["individual".to_string(), "individual".to_string()]),
    cost_terms: Some(vec!["annual".to_string(), "annual".to_string()]),
    cost_values: Some(vec!["$25.00".to_string(), "$500.00".to_string()]),
    cost_value_types: Some(vec!["money".to_string(), "money".to_string()]),
    subrogation: Some(false),
    contract_ids: Some(vec!["contract_001".to_string()]),
    insurance_plan_id: Some("plan_001".to_string()),
    notes: Some("Dependent coverage under parent's insurance".to_string()),
    ..Default::default()
};
```

### 5. Medicaid Coverage
```rust
let medicaid_coverage = DomainCoverage {
    coverage_id: "cov_medicaid_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("active".to_string()),
    kind: Some("insurance".to_string()),
    r#type: Some("medicaid".to_string()),
    type_code: Some("MEDICAID".to_string()),
    type_system: Some("http://terminology.hl7.org/CodeSystem/v3-ActCode".to_string()),
    type_display: Some("Medicaid".to_string()),
    policy_holder_id: Some("pat_12345".to_string()),
    policy_holder_type: Some("Patient".to_string()),
    subscriber_id: Some("pat_12345".to_string()),
    subscriber_type: Some("Patient".to_string()),
    subscriber_identifier: Some("MED123456789".to_string()),
    beneficiary_id: Some("pat_12345".to_string()),
    relationship: Some("self".to_string()),
    relationship_code: Some("self".to_string()),
    relationship_system: Some("http://terminology.hl7.org/CodeSystem/subscriber-relationship".to_string()),
    relationship_display: Some("Self".to_string()),
    period_start: Some("2024-01-01T00:00:00Z".to_string()),
    period_end: Some("2024-12-31T23:59:59Z".to_string()),
    insurer_id: Some("org_medicaid".to_string()),
    network: Some("Medicaid Network".to_string()),
    class_types: Some(vec!["group".to_string(), "plan".to_string()]),
    class_values: Some(vec!["MEDICAID".to_string(), "STANDARD".to_string()]),
    class_names: Some(vec!["Medicaid".to_string(), "Standard Plan".to_string()]),
    class_systems: Some(vec!["http://terminology.hl7.org/CodeSystem/coverage-class".to_string(), "http://terminology.hl7.org/CodeSystem/coverage-class".to_string()]),
    order: Some(1),
    cost_types: Some(vec!["copay".to_string(), "deductible".to_string()]),
    cost_categories: Some(vec!["primary".to_string(), "primary".to_string()]),
    cost_networks: Some(vec!["in-network".to_string(), "in-network".to_string()]),
    cost_units: Some(vec!["individual".to_string(), "individual".to_string()]),
    cost_terms: Some(vec!["annual".to_string(), "annual".to_string()]),
    cost_values: Some(vec!["$0.00".to_string(), "$0.00".to_string()]),
    cost_value_types: Some(vec!["money".to_string(), "money".to_string()]),
    subrogation: Some(false),
    contract_ids: Some(vec!["contract_medicaid_001".to_string()]),
    insurance_plan_id: Some("plan_medicaid_001".to_string()),
    notes: Some("Medicaid coverage for patient".to_string()),
    ..Default::default()
};
```

## Next Steps

To complete the coverage implementation, you may want to:

1. **Add to Service Layer**: Integrate the coverage adapter into your gRPC service
2. **Add Database Adapters**: Create database adapters for storing/retrieving coverage
3. **Add Validation**: Implement validation rules for coverage data
4. **Add Error Handling**: Enhance error handling for conversion failures
5. **Add Logging**: Add appropriate logging for coverage operations
6. **Add Coverage Verification**: Implement coverage verification and eligibility checking
7. **Add Prior Authorization**: Integrate with prior authorization systems
8. **Add Claims Processing**: Support claims submission and processing
9. **Add Coverage Analytics**: Analyze coverage patterns and utilization
10. **Add Coverage Workflows**: Integrate with billing and payment systems

## Related Entities

This implementation follows the same pattern as the existing entities and integrates with:
- **Patient**: The beneficiary of the coverage
- **Organization**: The insurer and policy holder
- **Contract**: Coverage contract details
- **InsurancePlan**: Insurance plan information
- **RelatedPerson**: Policy holders and subscribers

## Coverage Types

The implementation supports various coverage types:
- **Medical**: General medical coverage
- **Dental**: Dental care coverage
- **Vision**: Vision care coverage
- **Pharmacy**: Prescription drug coverage
- **Mental Health**: Mental health coverage
- **Accident**: Accident coverage
- **Disability**: Disability coverage
- **Life**: Life insurance coverage
- **Medicare**: Medicare coverage
- **Medicaid**: Medicaid coverage
- **Self-Pay**: Self-payment coverage

## Coverage Status Lifecycle

The implementation supports the following coverage statuses:
- **Active**: Coverage is currently active
- **Cancelled**: Coverage has been cancelled
- **Draft**: Coverage is being prepared
- **Entered-in-Error**: Coverage was created by mistake

## Coverage Kinds

The implementation supports the following coverage kinds:
- **Insurance**: Traditional insurance coverage
- **Self-Pay**: Patient pays directly
- **Other**: Other types of coverage

## Cost Types

The implementation supports various cost types:
- **Copay**: Fixed amount per service
- **Deductible**: Amount before coverage begins
- **Coinsurance**: Percentage of cost shared
- **Out-of-Pocket**: Maximum amount patient pays
- **Premium**: Regular payment for coverage
- **Full-Payment**: Patient pays full amount

## Network Types

The implementation supports various network types:
- **In-Network**: Covered providers
- **Out-of-Network**: Non-covered providers
- **Self-Pay**: Direct payment
- **Preferred**: Preferred providers

## Relationship Types

The implementation supports various relationship types:
- **Self**: Patient is the subscriber
- **Child**: Patient is a child of subscriber
- **Spouse**: Patient is a spouse of subscriber
- **Parent**: Patient is a parent of subscriber
- **Other**: Other relationship

This comprehensive coverage implementation provides a solid foundation for managing insurance and medical plan coverage in your FHIR synchronization system, enabling accurate billing, claims processing, and coverage verification.
