# CarePlan Entity Implementation

This document describes the implementation of the `CarePlan` entity for the gRPC FHIR synchronization project.

## Files Created/Modified

### 1. Domain Model
- **File**: `src/domain/care_plan.rs`
- **Purpose**: Defines the `DomainCarePlan` struct that represents care plan data in our domain model
- **Key Fields**:
  - `care_plan_id`: String (required) - Unique identifier for the care plan
  - `patient_demographic_no`: String (required) - Reference to the patient
  - `status`: Option<String> - Status (draft, active, on-hold, revoked, completed, entered-in-error, unknown)
  - `intent`: Option<String> - Intent (proposal, plan, order, option, directive)
  - `category`: Option<String> - Type of plan (e.g., "diabetes-management", "post-surgical-care")
  - `title`: Option<String> - Human-friendly name for the care plan
  - `description`: Option<String> - Summary of nature of plan
  - `period_start`: Option<String> - ISO datetime string for plan start
  - `period_end`: Option<String> - ISO datetime string for plan end
  - `created_date`: Option<String> - ISO datetime string when first recorded
  - `encounter_id`: Option<String> - Encounter during which this CarePlan was created
  - `custodian_id`: Option<String> - Who is the designated responsible party
  - `custodian_type`: Option<String> - Type of custodian (Patient, Practitioner, etc.)
  - `contributor_ids`: Option<Vec<String>> - Who provided the content of the care plan
  - `contributor_types`: Option<Vec<String>> - Types of contributors
  - `care_team_ids`: Option<Vec<String>> - Care team members involved
  - `based_on_ids`: Option<Vec<String>> - References to other care plans or requests
  - `based_on_types`: Option<Vec<String>> - Types of based-on references
  - `replaces_ids`: Option<Vec<String>> - Care plans replaced by this one
  - `part_of_ids`: Option<Vec<String>> - Care plans this is part of
  - `addresses_codes`: Option<Vec<String>> - Health issues this plan addresses (codes)
  - `addresses_descriptions`: Option<Vec<String>> - Health issues descriptions
  - `supporting_info_ids`: Option<Vec<String>> - Information considered as part of plan
  - `goal_ids`: Option<Vec<String>> - Desired outcomes of plan
  - `activity_descriptions`: Option<Vec<String>> - Descriptions of activities
  - `activity_codes`: Option<Vec<String>> - Codes for activities
  - `activity_references`: Option<Vec<String>> - References to planned activities
  - `activity_reference_types`: Option<Vec<String>> - Types of activity references
  - `activity_progress_notes`: Option<Vec<String>> - Progress notes for activities
  - `notes`: Option<String> - Comments about the plan
  - `instantiates_canonical`: Option<Vec<String>> - FHIR protocol or definition references
  - `instantiates_uri`: Option<Vec<String>> - External protocol or definition references

### 2. FHIR Adapter
- **File**: `src/adapters/entities/care_plan.rs`
- **Purpose**: Implements the conversion from `DomainCarePlan` to FHIR `CarePlan` proto message
- **Key Features**:
  - Maps domain fields to FHIR CarePlan structure
  - Handles status and intent conversion with proper codes
  - Converts datetime strings to FHIR DateTime types
  - Creates proper FHIR references for patient, practitioner, encounter, and other resources
  - Handles care plan relationships (based on, replaces, part of)
  - Maps health issues with proper terminology systems
  - Handles activities with progress notes and references
  - Creates proper FHIR contributors and care team references

### 3. Module Updates
- **File**: `src/domain/mod.rs` - Added `pub mod care_plan;`
- **File**: `src/adapters/entities/mod.rs` - Added `pub mod care_plan;`

## FHIR Mapping Details

### Status
The adapter maps string status values to FHIR status codes:
- "draft" → 1
- "active" → 2
- "on-hold" → 3
- "revoked" → 4
- "completed" → 5
- "entered-in-error" → 6
- "unknown" → 7

### Intent
The adapter maps string intent values to FHIR intent codes:
- "proposal" → 1
- "plan" → 2
- "order" → 3
- "option" → 4
- "directive" → 5

### Category
The adapter maps string category values to FHIR CodeableConcept:
- Text: The provided category string
- System: Custom system for care plan categories

### Period
The adapter handles temporal information:
- Start: ISO datetime strings are converted to FHIR DateTime
- End: ISO datetime strings are converted to FHIR DateTime

### References
- Patient reference: `Patient/{demographic_no}`
- Encounter reference: `Encounter/{encounter_id}`
- Custodian reference: `{custodian_type}/{custodian_id}`
- Contributor references: `{contributor_type}/{contributor_id}`
- Care team references: `CareTeam/{care_team_id}`
- Based on references: `{based_on_type}/{based_on_id}`
- Replaces references: `CarePlan/{replaces_id}`
- Part of references: `CarePlan/{part_of_id}`

### Identifiers
- System: `urn:arsmedicatech:care_plan_id`
- Value: The care plan ID

### Health Issues (Addresses)
- System: `http://hl7.org/fhir/sid/icd-10-cm`
- Code: The provided health issue code
- Display: The human-readable description

### Activities
- **Performed Activity**: Mapped to activity.performed_activity as CodeableReference
- **Progress Notes**: Mapped to activity.progress as Annotation
- **Planned Activity References**: Mapped to activity.planned_activity_reference as Reference

### Instantiates
- **Canonical**: FHIR protocol or definition references
- **URI**: External protocol or definition references

## Testing

The implementation includes comprehensive unit tests covering:
- Full deserialization with all fields
- Minimal deserialization with only required fields
- Diabetes management care plan scenario
- Post-surgical care plan scenario
- Palliative care plan scenario
- Error handling for missing required fields

## Usage Example

```rust
use crate::domain::care_plan::DomainCarePlan;
use crate::adapters::entities::care_plan::*;

// Create a domain care plan
let domain_care_plan = DomainCarePlan {
    care_plan_id: "cp_12345".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("active".to_string()),
    intent: Some("plan".to_string()),
    category: Some("diabetes-management".to_string()),
    title: Some("Diabetes Management Plan".to_string()),
    description: Some("Comprehensive care plan for type 2 diabetes management".to_string()),
    period_start: Some("2024-01-01T00:00:00Z".to_string()),
    period_end: Some("2024-12-31T23:59:59Z".to_string()),
    created_date: Some("2024-01-01T10:00:00Z".to_string()),
    encounter_id: Some("enc_001".to_string()),
    custodian_id: Some("prac_001".to_string()),
    custodian_type: Some("Practitioner".to_string()),
    contributor_ids: Some(vec!["prac_002".to_string(), "nurse_001".to_string()]),
    contributor_types: Some(vec!["Practitioner".to_string(), "Practitioner".to_string()]),
    care_team_ids: Some(vec!["team_001".to_string(), "team_002".to_string()]),
    based_on_ids: Some(vec!["cp_00001".to_string()]),
    based_on_types: Some(vec!["CarePlan".to_string()]),
    replaces_ids: Some(vec![]),
    part_of_ids: Some(vec![]),
    addresses_codes: Some(vec!["E11.9".to_string(), "I10".to_string()]),
    addresses_descriptions: Some(vec!["Type 2 diabetes mellitus without complications".to_string(), "Essential hypertension".to_string()]),
    supporting_info_ids: Some(vec!["obs_001".to_string(), "obs_002".to_string()]),
    goal_ids: Some(vec!["goal_001".to_string(), "goal_002".to_string()]),
    activity_descriptions: Some(vec!["Blood glucose monitoring".to_string(), "Medication adherence".to_string(), "Dietary counseling".to_string()]),
    activity_codes: Some(vec!["glucose-monitoring".to_string(), "medication-adherence".to_string(), "dietary-counseling".to_string()]),
    activity_references: Some(vec!["apt_001".to_string(), "med_001".to_string(), "task_001".to_string()]),
    activity_reference_types: Some(vec!["Appointment".to_string(), "MedicationRequest".to_string(), "Task".to_string()]),
    activity_progress_notes: Some(vec!["Patient monitoring glucose daily".to_string(), "Medications taken as prescribed".to_string(), "Following dietary guidelines".to_string()]),
    notes: Some("Patient is motivated and engaged in self-care. Regular follow-up scheduled.".to_string()),
    instantiates_canonical: Some(vec!["http://example.org/fhir/PlanDefinition/diabetes-management".to_string()]),
    instantiates_uri: Some(vec!["https://example.org/protocols/diabetes-care".to_string()]),
};

// Convert to FHIR CarePlan
let fhir_care_plan: CarePlan = domain_care_plan.into();
```

## Clinical Use Cases

### 1. Diabetes Management Plan
```rust
let diabetes_care_plan = DomainCarePlan {
    care_plan_id: "cp_diabetes_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("active".to_string()),
    intent: Some("plan".to_string()),
    category: Some("diabetes-management".to_string()),
    title: Some("Diabetes Management Plan".to_string()),
    description: Some("Comprehensive care plan for type 2 diabetes management".to_string()),
    period_start: Some("2024-01-01T00:00:00Z".to_string()),
    period_end: Some("2024-12-31T23:59:59Z".to_string()),
    created_date: Some("2024-01-01T10:00:00Z".to_string()),
    encounter_id: Some("enc_001".to_string()),
    custodian_id: Some("prac_001".to_string()),
    custodian_type: Some("Practitioner".to_string()),
    contributor_ids: Some(vec!["prac_002".to_string(), "nurse_001".to_string()]),
    contributor_types: Some(vec!["Practitioner".to_string(), "Practitioner".to_string()]),
    care_team_ids: Some(vec!["team_001".to_string(), "team_002".to_string()]),
    addresses_codes: Some(vec!["E11.9".to_string(), "I10".to_string()]),
    addresses_descriptions: Some(vec!["Type 2 diabetes mellitus without complications".to_string(), "Essential hypertension".to_string()]),
    supporting_info_ids: Some(vec!["obs_001".to_string(), "obs_002".to_string()]),
    goal_ids: Some(vec!["goal_001".to_string(), "goal_002".to_string()]),
    activity_descriptions: Some(vec!["Blood glucose monitoring".to_string(), "Medication adherence".to_string(), "Dietary counseling".to_string()]),
    activity_codes: Some(vec!["glucose-monitoring".to_string(), "medication-adherence".to_string(), "dietary-counseling".to_string()]),
    activity_references: Some(vec!["apt_001".to_string(), "med_001".to_string(), "task_001".to_string()]),
    activity_reference_types: Some(vec!["Appointment".to_string(), "MedicationRequest".to_string(), "Task".to_string()]),
    activity_progress_notes: Some(vec!["Patient monitoring glucose daily".to_string(), "Medications taken as prescribed".to_string(), "Following dietary guidelines".to_string()]),
    notes: Some("Patient is motivated and engaged in self-care. Regular follow-up scheduled.".to_string()),
    instantiates_canonical: Some(vec!["http://example.org/fhir/PlanDefinition/diabetes-management".to_string()]),
    instantiates_uri: Some(vec!["https://example.org/protocols/diabetes-care".to_string()]),
    ..Default::default()
};
```

### 2. Post-Surgical Care Plan
```rust
let post_surgical_care_plan = DomainCarePlan {
    care_plan_id: "cp_surgical_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("active".to_string()),
    intent: Some("order".to_string()),
    category: Some("post-surgical-care".to_string()),
    title: Some("Post-Surgical Recovery Plan".to_string()),
    description: Some("Comprehensive recovery plan following appendectomy".to_string()),
    period_start: Some("2024-02-15T00:00:00Z".to_string()),
    period_end: Some("2024-03-15T23:59:59Z".to_string()),
    created_date: Some("2024-02-15T14:30:00Z".to_string()),
    encounter_id: Some("enc_surgical_001".to_string()),
    custodian_id: Some("prac_surgeon_001".to_string()),
    custodian_type: Some("Practitioner".to_string()),
    contributor_ids: Some(vec!["nurse_001".to_string(), "pt_001".to_string()]),
    contributor_types: Some(vec!["Practitioner".to_string(), "Practitioner".to_string()]),
    care_team_ids: Some(vec!["team_surgical_001".to_string()]),
    addresses_codes: Some(vec!["K35.9".to_string()]),
    addresses_descriptions: Some(vec!["Acute appendicitis, unspecified".to_string()]),
    supporting_info_ids: Some(vec!["proc_001".to_string(), "obs_surgical_001".to_string()]),
    goal_ids: Some(vec!["goal_recovery_001".to_string(), "goal_mobility_001".to_string()]),
    activity_descriptions: Some(vec!["Pain management".to_string(), "Wound care".to_string(), "Physical therapy".to_string(), "Follow-up appointments".to_string()]),
    activity_codes: Some(vec!["pain-management".to_string(), "wound-care".to_string(), "physical-therapy".to_string(), "follow-up".to_string()]),
    activity_references: Some(vec!["med_pain_001".to_string(), "task_wound_001".to_string(), "apt_pt_001".to_string(), "apt_followup_001".to_string()]),
    activity_reference_types: Some(vec!["MedicationRequest".to_string(), "Task".to_string(), "Appointment".to_string(), "Appointment".to_string()]),
    activity_progress_notes: Some(vec!["Pain well controlled".to_string(), "Wound healing normally".to_string(), "PT sessions going well".to_string(), "Follow-up scheduled".to_string()]),
    notes: Some("Patient recovering well. No complications noted. Continue current plan.".to_string()),
    instantiates_canonical: Some(vec!["http://example.org/fhir/PlanDefinition/post-surgical-care".to_string()]),
    ..Default::default()
};
```

### 3. Palliative Care Plan
```rust
let palliative_care_plan = DomainCarePlan {
    care_plan_id: "cp_palliative_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("active".to_string()),
    intent: Some("plan".to_string()),
    category: Some("palliative-care".to_string()),
    title: Some("Palliative Care Plan".to_string()),
    description: Some("Comprehensive palliative care plan for end-of-life comfort".to_string()),
    period_start: Some("2024-03-01T00:00:00Z".to_string()),
    period_end: None, // Ongoing care
    created_date: Some("2024-03-01T09:00:00Z".to_string()),
    custodian_id: Some("prac_palliative_001".to_string()),
    custodian_type: Some("Practitioner".to_string()),
    contributor_ids: Some(vec!["nurse_palliative_001".to_string(), "social_worker_001".to_string()]),
    contributor_types: Some(vec!["Practitioner".to_string(), "Practitioner".to_string()]),
    care_team_ids: Some(vec!["team_palliative_001".to_string()]),
    addresses_codes: Some(vec!["C78.00".to_string()]),
    addresses_descriptions: Some(vec!["Secondary malignant neoplasm of unspecified lung".to_string()]),
    supporting_info_ids: Some(vec!["obs_pain_001".to_string(), "obs_quality_001".to_string()]),
    goal_ids: Some(vec!["goal_comfort_001".to_string(), "goal_quality_001".to_string()]),
    activity_descriptions: Some(vec!["Pain management".to_string(), "Symptom control".to_string(), "Family support".to_string(), "Spiritual care".to_string()]),
    activity_codes: Some(vec!["pain-management".to_string(), "symptom-control".to_string(), "family-support".to_string(), "spiritual-care".to_string()]),
    activity_references: Some(vec!["med_pain_002".to_string(), "task_symptoms_001".to_string(), "task_family_001".to_string(), "task_spiritual_001".to_string()]),
    activity_reference_types: Some(vec!["MedicationRequest".to_string(), "Task".to_string(), "Task".to_string(), "Task".to_string()]),
    activity_progress_notes: Some(vec!["Pain well managed".to_string(), "Symptoms controlled".to_string(), "Family meetings scheduled".to_string(), "Spiritual needs addressed".to_string()]),
    notes: Some("Patient and family comfortable with care plan. Focus on comfort and quality of life.".to_string()),
    instantiates_canonical: Some(vec!["http://example.org/fhir/PlanDefinition/palliative-care".to_string()]),
    ..Default::default()
};
```

### 4. Chronic Disease Management Plan
```rust
let chronic_disease_care_plan = DomainCarePlan {
    care_plan_id: "cp_chronic_001".to_string(),
    patient_demographic_no: "12345".to_string(),
    status: Some("active".to_string()),
    intent: Some("plan".to_string()),
    category: Some("chronic-disease-management".to_string()),
    title: Some("Chronic Disease Management Plan".to_string()),
    description: Some("Comprehensive care plan for multiple chronic conditions".to_string()),
    period_start: Some("2024-01-01T00:00:00Z".to_string()),
    period_end: Some("2024-12-31T23:59:59Z".to_string()),
    created_date: Some("2024-01-01T10:00:00Z".to_string()),
    encounter_id: Some("enc_001".to_string()),
    custodian_id: Some("prac_001".to_string()),
    custodian_type: Some("Practitioner".to_string()),
    contributor_ids: Some(vec!["prac_002".to_string(), "nurse_001".to_string(), "pharmacist_001".to_string()]),
    contributor_types: Some(vec!["Practitioner".to_string(), "Practitioner".to_string(), "Practitioner".to_string()]),
    care_team_ids: Some(vec!["team_chronic_001".to_string()]),
    addresses_codes: Some(vec!["E11.9".to_string(), "I10".to_string(), "M79.3".to_string()]),
    addresses_descriptions: Some(vec!["Type 2 diabetes mellitus without complications".to_string(), "Essential hypertension".to_string(), "Panniculitis, unspecified".to_string()]),
    supporting_info_ids: Some(vec!["obs_001".to_string(), "obs_002".to_string(), "obs_003".to_string()]),
    goal_ids: Some(vec!["goal_001".to_string(), "goal_002".to_string(), "goal_003".to_string()]),
    activity_descriptions: Some(vec!["Regular monitoring".to_string(), "Medication management".to_string(), "Lifestyle counseling".to_string(), "Specialist referrals".to_string()]),
    activity_codes: Some(vec!["regular-monitoring".to_string(), "medication-management".to_string(), "lifestyle-counseling".to_string(), "specialist-referrals".to_string()]),
    activity_references: Some(vec!["apt_001".to_string(), "med_001".to_string(), "task_001".to_string(), "ref_001".to_string()]),
    activity_reference_types: Some(vec!["Appointment".to_string(), "MedicationRequest".to_string(), "Task".to_string(), "ServiceRequest".to_string()]),
    activity_progress_notes: Some(vec!["Patient monitoring regularly".to_string(), "Medications taken as prescribed".to_string(), "Following lifestyle recommendations".to_string(), "Specialist appointments scheduled".to_string()]),
    notes: Some("Patient managing multiple chronic conditions well. Regular monitoring and coordination between specialists.".to_string()),
    instantiates_canonical: Some(vec!["http://example.org/fhir/PlanDefinition/chronic-disease-management".to_string()]),
    ..Default::default()
};
```

## Next Steps

To complete the care plan implementation, you may want to:

1. **Add to Service Layer**: Integrate the care plan adapter into your gRPC service
2. **Add Database Adapters**: Create database adapters for storing/retrieving care plans
3. **Add Validation**: Implement validation rules for care plan data
4. **Add Error Handling**: Enhance error handling for conversion failures
5. **Add Logging**: Add appropriate logging for care plan operations
6. **Add Care Plan Templates**: Create reusable care plan templates
7. **Add Progress Tracking**: Track care plan progress and outcomes
8. **Add Care Plan Versioning**: Handle care plan updates and versioning
9. **Add Care Plan Analytics**: Analyze care plan effectiveness and outcomes
10. **Add Care Plan Collaboration**: Enable multi-provider care plan collaboration

## Related Entities

This implementation follows the same pattern as the existing entities and integrates with:
- **Patient**: The subject of the care plan
- **Encounter**: When the care plan was created
- **Practitioner**: Who is responsible for the care plan
- **CareTeam**: Team members involved in care
- **Goal**: Desired outcomes of the care plan
- **Activity**: Actions to be performed as part of the plan
- **Condition**: Health issues the plan addresses

## Care Plan Types

The implementation supports various care plan types:
- **Diabetes Management**: Blood glucose monitoring, medication adherence, dietary counseling
- **Post-Surgical Care**: Pain management, wound care, physical therapy, follow-up
- **Palliative Care**: Pain management, symptom control, family support, spiritual care
- **Chronic Disease Management**: Regular monitoring, medication management, lifestyle counseling
- **Preventive Care**: Screening, vaccinations, health maintenance
- **Mental Health Care**: Therapy, medication management, support groups
- **Rehabilitation Care**: Physical therapy, occupational therapy, speech therapy

## Care Plan Status Lifecycle

The implementation supports the following care plan statuses:
- **Draft**: Care plan is being developed
- **Active**: Care plan is currently being followed
- **On-Hold**: Care plan is temporarily paused
- **Revoked**: Care plan has been cancelled
- **Completed**: Care plan has been finished
- **Entered-in-Error**: Care plan was created by mistake
- **Unknown**: Status is not known

## Care Plan Intent

The implementation supports the following care plan intents:
- **Proposal**: A proposed care plan
- **Plan**: A planned care plan
- **Order**: An ordered care plan
- **Option**: An optional care plan
- **Directive**: A directive care plan

## Activity Types

The implementation supports various activity types:
- **Appointment**: Scheduled healthcare appointments
- **MedicationRequest**: Medication prescriptions
- **Task**: Specific tasks to be performed
- **ServiceRequest**: Service requests
- **CommunicationRequest**: Communication tasks
- **DeviceRequest**: Device-related tasks
- **NutritionOrder**: Nutrition-related tasks
- **VisionPrescription**: Vision-related tasks
- **ImmunizationRecommendation**: Immunization tasks
- **SupplyRequest**: Supply-related tasks

This comprehensive care plan implementation provides a solid foundation for managing patient care plans in your FHIR synchronization system, enabling coordinated, evidence-based care delivery across healthcare teams.
