use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainCoverage {
    pub coverage_id: String,
    pub patient_demographic_no: String,
    
    // Basic coverage information
    pub status: Option<String>, // "active" | "cancelled" | "draft" | "entered-in-error"
    pub kind: Option<String>, // "insurance" | "self-pay" | "other"
    pub r#type: Option<String>, // Coverage category such as medical or accident
    pub type_code: Option<String>, // Code for coverage type
    pub type_system: Option<String>, // Terminology system for type
    pub type_display: Option<String>, // Display name for type
    
    // Policy information
    pub policy_holder_id: Option<String>, // Owner of the policy
    pub policy_holder_type: Option<String>, // Type of policy holder (Patient, RelatedPerson, Organization)
    pub subscriber_id: Option<String>, // Subscriber to the policy
    pub subscriber_type: Option<String>, // Type of subscriber (Patient, RelatedPerson)
    pub subscriber_identifier: Option<String>, // ID assigned to the subscriber
    pub beneficiary_id: Option<String>, // Plan beneficiary (Patient)
    pub dependent_number: Option<String>, // Dependent number
    pub relationship: Option<String>, // Beneficiary relationship to the subscriber
    pub relationship_code: Option<String>, // Code for relationship
    pub relationship_system: Option<String>, // Terminology system for relationship
    pub relationship_display: Option<String>, // Display name for relationship
    
    // Temporal information
    pub period_start: Option<String>, // ISO datetime string for coverage start
    pub period_end: Option<String>, // ISO datetime string for coverage end
    
    // Insurer information
    pub insurer_id: Option<String>, // Issuer of the policy
    pub network: Option<String>, // Insurer network
    
    // Coverage classifications
    pub class_types: Option<Vec<String>>, // Types of class such as 'group' or 'plan'
    pub class_values: Option<Vec<String>>, // Values associated with the types
    pub class_names: Option<Vec<String>>, // Human readable descriptions
    pub class_systems: Option<Vec<String>>, // Terminology systems for class types
    
    // Order and priority
    pub order: Option<u32>, // Relative order of the coverage
    
    // Cost information
    pub cost_types: Option<Vec<String>>, // Cost categories
    pub cost_categories: Option<Vec<String>>, // Benefit classifications
    pub cost_networks: Option<Vec<String>>, // In or out of network
    pub cost_units: Option<Vec<String>>, // Individual or family
    pub cost_terms: Option<Vec<String>>, // Annual or lifetime
    pub cost_values: Option<Vec<String>>, // The amount or percentage due from the beneficiary
    pub cost_value_types: Option<Vec<String>>, // Type of cost value (quantity, money)
    
    // Payment information
    pub payment_by_party_ids: Option<Vec<String>>, // Parties performing self-payment
    pub payment_by_party_types: Option<Vec<String>>, // Types of payment parties
    pub payment_by_responsibilities: Option<Vec<String>>, // Party's responsibility
    
    // Additional information
    pub subrogation: Option<bool>, // Reimbursement to insurer
    pub contract_ids: Option<Vec<String>>, // Contract details
    pub insurance_plan_id: Option<String>, // Insurance plan details
    pub notes: Option<String>, // Additional notes about the coverage
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_coverage_deserialization() {
        let json = r#"{
            "coverage_id": "cov_12345",
            "patient_demographic_no": "12345",
            "status": "active",
            "kind": "insurance",
            "type": "medical",
            "type_code": "MED",
            "type_system": "http://terminology.hl7.org/CodeSystem/v3-ActCode",
            "type_display": "Medical",
            "policy_holder_id": "pat_12345",
            "policy_holder_type": "Patient",
            "subscriber_id": "pat_12345",
            "subscriber_type": "Patient",
            "subscriber_identifier": "SUB123456789",
            "beneficiary_id": "pat_12345",
            "dependent_number": "01",
            "relationship": "self",
            "relationship_code": "self",
            "relationship_system": "http://terminology.hl7.org/CodeSystem/subscriber-relationship",
            "relationship_display": "Self",
            "period_start": "2024-01-01T00:00:00Z",
            "period_end": "2024-12-31T23:59:59Z",
            "insurer_id": "org_insurer_001",
            "network": "PPO Network",
            "class_types": ["group", "plan"],
            "class_values": ["GROUP001", "PLAN001"],
            "class_names": ["Employee Group", "Premium Plan"],
            "class_systems": ["http://terminology.hl7.org/CodeSystem/coverage-class", "http://terminology.hl7.org/CodeSystem/coverage-class"],
            "order": 1,
            "cost_types": ["copay", "deductible"],
            "cost_categories": ["primary", "primary"],
            "cost_networks": ["in-network", "in-network"],
            "cost_units": ["individual", "individual"],
            "cost_terms": ["annual", "annual"],
            "cost_values": ["$25.00", "$500.00"],
            "cost_value_types": ["money", "money"],
            "payment_by_party_ids": [],
            "payment_by_party_types": [],
            "payment_by_responsibilities": [],
            "subrogation": false,
            "contract_ids": ["contract_001"],
            "insurance_plan_id": "plan_001",
            "notes": "Primary insurance coverage for patient"
        }"#;

        let coverage: DomainCoverage = serde_json::from_str(json).unwrap();
        
        assert_eq!(coverage.coverage_id, "cov_12345");
        assert_eq!(coverage.patient_demographic_no, "12345");
        assert_eq!(coverage.status, Some("active".to_string()));
        assert_eq!(coverage.kind, Some("insurance".to_string()));
        assert_eq!(coverage.r#type, Some("medical".to_string()));
        assert_eq!(coverage.type_code, Some("MED".to_string()));
        assert_eq!(coverage.type_system, Some("http://terminology.hl7.org/CodeSystem/v3-ActCode".to_string()));
        assert_eq!(coverage.type_display, Some("Medical".to_string()));
        assert_eq!(coverage.policy_holder_id, Some("pat_12345".to_string()));
        assert_eq!(coverage.policy_holder_type, Some("Patient".to_string()));
        assert_eq!(coverage.subscriber_id, Some("pat_12345".to_string()));
        assert_eq!(coverage.subscriber_type, Some("Patient".to_string()));
        assert_eq!(coverage.subscriber_identifier, Some("SUB123456789".to_string()));
        assert_eq!(coverage.beneficiary_id, Some("pat_12345".to_string()));
        assert_eq!(coverage.dependent_number, Some("01".to_string()));
        assert_eq!(coverage.relationship, Some("self".to_string()));
        assert_eq!(coverage.relationship_code, Some("self".to_string()));
        assert_eq!(coverage.relationship_system, Some("http://terminology.hl7.org/CodeSystem/subscriber-relationship".to_string()));
        assert_eq!(coverage.relationship_display, Some("Self".to_string()));
        assert_eq!(coverage.period_start, Some("2024-01-01T00:00:00Z".to_string()));
        assert_eq!(coverage.period_end, Some("2024-12-31T23:59:59Z".to_string()));
        assert_eq!(coverage.insurer_id, Some("org_insurer_001".to_string()));
        assert_eq!(coverage.network, Some("PPO Network".to_string()));
        assert_eq!(coverage.class_types, Some(vec!["group".to_string(), "plan".to_string()]));
        assert_eq!(coverage.class_values, Some(vec!["GROUP001".to_string(), "PLAN001".to_string()]));
        assert_eq!(coverage.class_names, Some(vec!["Employee Group".to_string(), "Premium Plan".to_string()]));
        assert_eq!(coverage.class_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/coverage-class".to_string(), "http://terminology.hl7.org/CodeSystem/coverage-class".to_string()]));
        assert_eq!(coverage.order, Some(1));
        assert_eq!(coverage.cost_types, Some(vec!["copay".to_string(), "deductible".to_string()]));
        assert_eq!(coverage.cost_categories, Some(vec!["primary".to_string(), "primary".to_string()]));
        assert_eq!(coverage.cost_networks, Some(vec!["in-network".to_string(), "in-network".to_string()]));
        assert_eq!(coverage.cost_units, Some(vec!["individual".to_string(), "individual".to_string()]));
        assert_eq!(coverage.cost_terms, Some(vec!["annual".to_string(), "annual".to_string()]));
        assert_eq!(coverage.cost_values, Some(vec!["$25.00".to_string(), "$500.00".to_string()]));
        assert_eq!(coverage.cost_value_types, Some(vec!["money".to_string(), "money".to_string()]));
        assert_eq!(coverage.payment_by_party_ids, Some(vec![]));
        assert_eq!(coverage.payment_by_party_types, Some(vec![]));
        assert_eq!(coverage.payment_by_responsibilities, Some(vec![]));
        assert_eq!(coverage.subrogation, Some(false));
        assert_eq!(coverage.contract_ids, Some(vec!["contract_001".to_string()]));
        assert_eq!(coverage.insurance_plan_id, Some("plan_001".to_string()));
        assert_eq!(coverage.notes, Some("Primary insurance coverage for patient".to_string()));
    }

    #[test]
    fn test_domain_coverage_minimal_deserialization() {
        let json = r#"{
            "coverage_id": "cov_67890",
            "patient_demographic_no": "67890"
        }"#;

        let coverage: DomainCoverage = serde_json::from_str(json).unwrap();
        
        assert_eq!(coverage.coverage_id, "cov_67890");
        assert_eq!(coverage.patient_demographic_no, "67890");
        assert_eq!(coverage.status, None);
        assert_eq!(coverage.kind, None);
        assert_eq!(coverage.r#type, None);
        assert_eq!(coverage.type_code, None);
        assert_eq!(coverage.type_system, None);
        assert_eq!(coverage.type_display, None);
        assert_eq!(coverage.policy_holder_id, None);
        assert_eq!(coverage.policy_holder_type, None);
        assert_eq!(coverage.subscriber_id, None);
        assert_eq!(coverage.subscriber_type, None);
        assert_eq!(coverage.subscriber_identifier, None);
        assert_eq!(coverage.beneficiary_id, None);
        assert_eq!(coverage.dependent_number, None);
        assert_eq!(coverage.relationship, None);
        assert_eq!(coverage.relationship_code, None);
        assert_eq!(coverage.relationship_system, None);
        assert_eq!(coverage.relationship_display, None);
        assert_eq!(coverage.period_start, None);
        assert_eq!(coverage.period_end, None);
        assert_eq!(coverage.insurer_id, None);
        assert_eq!(coverage.network, None);
        assert_eq!(coverage.class_types, None);
        assert_eq!(coverage.class_values, None);
        assert_eq!(coverage.class_names, None);
        assert_eq!(coverage.class_systems, None);
        assert_eq!(coverage.order, None);
        assert_eq!(coverage.cost_types, None);
        assert_eq!(coverage.cost_categories, None);
        assert_eq!(coverage.cost_networks, None);
        assert_eq!(coverage.cost_units, None);
        assert_eq!(coverage.cost_terms, None);
        assert_eq!(coverage.cost_values, None);
        assert_eq!(coverage.cost_value_types, None);
        assert_eq!(coverage.payment_by_party_ids, None);
        assert_eq!(coverage.payment_by_party_types, None);
        assert_eq!(coverage.payment_by_responsibilities, None);
        assert_eq!(coverage.subrogation, None);
        assert_eq!(coverage.contract_ids, None);
        assert_eq!(coverage.insurance_plan_id, None);
        assert_eq!(coverage.notes, None);
    }

    #[test]
    fn test_domain_coverage_medicare() {
        let json = r#"{
            "coverage_id": "cov_medicare_001",
            "patient_demographic_no": "12345",
            "status": "active",
            "kind": "insurance",
            "type": "medicare",
            "type_code": "MEDICARE",
            "type_system": "http://terminology.hl7.org/CodeSystem/v3-ActCode",
            "type_display": "Medicare",
            "policy_holder_id": "pat_12345",
            "policy_holder_type": "Patient",
            "subscriber_id": "pat_12345",
            "subscriber_type": "Patient",
            "subscriber_identifier": "123456789A",
            "beneficiary_id": "pat_12345",
            "relationship": "self",
            "relationship_code": "self",
            "relationship_system": "http://terminology.hl7.org/CodeSystem/subscriber-relationship",
            "relationship_display": "Self",
            "period_start": "2024-01-01T00:00:00Z",
            "period_end": "2024-12-31T23:59:59Z",
            "insurer_id": "org_medicare",
            "network": "Medicare Network",
            "class_types": ["group", "plan"],
            "class_values": ["MEDICARE", "PART_B"],
            "class_names": ["Medicare", "Part B"],
            "class_systems": ["http://terminology.hl7.org/CodeSystem/coverage-class", "http://terminology.hl7.org/CodeSystem/coverage-class"],
            "order": 1,
            "cost_types": ["copay", "deductible"],
            "cost_categories": ["primary", "primary"],
            "cost_networks": ["in-network", "in-network"],
            "cost_units": ["individual", "individual"],
            "cost_terms": ["annual", "annual"],
            "cost_values": ["$0.00", "$240.00"],
            "cost_value_types": ["money", "money"],
            "subrogation": false,
            "contract_ids": ["contract_medicare_001"],
            "insurance_plan_id": "plan_medicare_001",
            "notes": "Medicare Part B coverage for patient"
        }"#;

        let coverage: DomainCoverage = serde_json::from_str(json).unwrap();
        
        assert_eq!(coverage.coverage_id, "cov_medicare_001");
        assert_eq!(coverage.patient_demographic_no, "12345");
        assert_eq!(coverage.status, Some("active".to_string()));
        assert_eq!(coverage.kind, Some("insurance".to_string()));
        assert_eq!(coverage.r#type, Some("medicare".to_string()));
        assert_eq!(coverage.type_code, Some("MEDICARE".to_string()));
        assert_eq!(coverage.type_system, Some("http://terminology.hl7.org/CodeSystem/v3-ActCode".to_string()));
        assert_eq!(coverage.type_display, Some("Medicare".to_string()));
        assert_eq!(coverage.policy_holder_id, Some("pat_12345".to_string()));
        assert_eq!(coverage.policy_holder_type, Some("Patient".to_string()));
        assert_eq!(coverage.subscriber_id, Some("pat_12345".to_string()));
        assert_eq!(coverage.subscriber_type, Some("Patient".to_string()));
        assert_eq!(coverage.subscriber_identifier, Some("123456789A".to_string()));
        assert_eq!(coverage.beneficiary_id, Some("pat_12345".to_string()));
        assert_eq!(coverage.relationship, Some("self".to_string()));
        assert_eq!(coverage.relationship_code, Some("self".to_string()));
        assert_eq!(coverage.relationship_system, Some("http://terminology.hl7.org/CodeSystem/subscriber-relationship".to_string()));
        assert_eq!(coverage.relationship_display, Some("Self".to_string()));
        assert_eq!(coverage.period_start, Some("2024-01-01T00:00:00Z".to_string()));
        assert_eq!(coverage.period_end, Some("2024-12-31T23:59:59Z".to_string()));
        assert_eq!(coverage.insurer_id, Some("org_medicare".to_string()));
        assert_eq!(coverage.network, Some("Medicare Network".to_string()));
        assert_eq!(coverage.class_types, Some(vec!["group".to_string(), "plan".to_string()]));
        assert_eq!(coverage.class_values, Some(vec!["MEDICARE".to_string(), "PART_B".to_string()]));
        assert_eq!(coverage.class_names, Some(vec!["Medicare".to_string(), "Part B".to_string()]));
        assert_eq!(coverage.class_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/coverage-class".to_string(), "http://terminology.hl7.org/CodeSystem/coverage-class".to_string()]));
        assert_eq!(coverage.order, Some(1));
        assert_eq!(coverage.cost_types, Some(vec!["copay".to_string(), "deductible".to_string()]));
        assert_eq!(coverage.cost_categories, Some(vec!["primary".to_string(), "primary".to_string()]));
        assert_eq!(coverage.cost_networks, Some(vec!["in-network".to_string(), "in-network".to_string()]));
        assert_eq!(coverage.cost_units, Some(vec!["individual".to_string(), "individual".to_string()]));
        assert_eq!(coverage.cost_terms, Some(vec!["annual".to_string(), "annual".to_string()]));
        assert_eq!(coverage.cost_values, Some(vec!["$0.00".to_string(), "$240.00".to_string()]));
        assert_eq!(coverage.cost_value_types, Some(vec!["money".to_string(), "money".to_string()]));
        assert_eq!(coverage.subrogation, Some(false));
        assert_eq!(coverage.contract_ids, Some(vec!["contract_medicare_001".to_string()]));
        assert_eq!(coverage.insurance_plan_id, Some("plan_medicare_001".to_string()));
        assert_eq!(coverage.notes, Some("Medicare Part B coverage for patient".to_string()));
    }

    #[test]
    fn test_domain_coverage_self_pay() {
        let json = r#"{
            "coverage_id": "cov_self_pay_001",
            "patient_demographic_no": "12345",
            "status": "active",
            "kind": "self-pay",
            "type": "self-pay",
            "type_code": "SELF",
            "type_system": "http://terminology.hl7.org/CodeSystem/v3-ActCode",
            "type_display": "Self Pay",
            "policy_holder_id": "pat_12345",
            "policy_holder_type": "Patient",
            "subscriber_id": "pat_12345",
            "subscriber_type": "Patient",
            "beneficiary_id": "pat_12345",
            "relationship": "self",
            "relationship_code": "self",
            "relationship_system": "http://terminology.hl7.org/CodeSystem/subscriber-relationship",
            "relationship_display": "Self",
            "period_start": "2024-01-01T00:00:00Z",
            "period_end": "2024-12-31T23:59:59Z",
            "class_types": ["plan"],
            "class_values": ["SELF_PAY"],
            "class_names": ["Self Pay Plan"],
            "class_systems": ["http://terminology.hl7.org/CodeSystem/coverage-class"],
            "order": 1,
            "cost_types": ["full-payment"],
            "cost_categories": ["primary"],
            "cost_networks": ["self-pay"],
            "cost_units": ["individual"],
            "cost_terms": ["per-service"],
            "cost_values": ["100%"],
            "cost_value_types": ["quantity"],
            "payment_by_party_ids": ["pat_12345"],
            "payment_by_party_types": ["Patient"],
            "payment_by_responsibilities": ["Full payment responsibility"],
            "subrogation": false,
            "notes": "Patient responsible for full payment of services"
        }"#;

        let coverage: DomainCoverage = serde_json::from_str(json).unwrap();
        
        assert_eq!(coverage.coverage_id, "cov_self_pay_001");
        assert_eq!(coverage.patient_demographic_no, "12345");
        assert_eq!(coverage.status, Some("active".to_string()));
        assert_eq!(coverage.kind, Some("self-pay".to_string()));
        assert_eq!(coverage.r#type, Some("self-pay".to_string()));
        assert_eq!(coverage.type_code, Some("SELF".to_string()));
        assert_eq!(coverage.type_system, Some("http://terminology.hl7.org/CodeSystem/v3-ActCode".to_string()));
        assert_eq!(coverage.type_display, Some("Self Pay".to_string()));
        assert_eq!(coverage.policy_holder_id, Some("pat_12345".to_string()));
        assert_eq!(coverage.policy_holder_type, Some("Patient".to_string()));
        assert_eq!(coverage.subscriber_id, Some("pat_12345".to_string()));
        assert_eq!(coverage.subscriber_type, Some("Patient".to_string()));
        assert_eq!(coverage.beneficiary_id, Some("pat_12345".to_string()));
        assert_eq!(coverage.relationship, Some("self".to_string()));
        assert_eq!(coverage.relationship_code, Some("self".to_string()));
        assert_eq!(coverage.relationship_system, Some("http://terminology.hl7.org/CodeSystem/subscriber-relationship".to_string()));
        assert_eq!(coverage.relationship_display, Some("Self".to_string()));
        assert_eq!(coverage.period_start, Some("2024-01-01T00:00:00Z".to_string()));
        assert_eq!(coverage.period_end, Some("2024-12-31T23:59:59Z".to_string()));
        assert_eq!(coverage.class_types, Some(vec!["plan".to_string()]));
        assert_eq!(coverage.class_values, Some(vec!["SELF_PAY".to_string()]));
        assert_eq!(coverage.class_names, Some(vec!["Self Pay Plan".to_string()]));
        assert_eq!(coverage.class_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/coverage-class".to_string()]));
        assert_eq!(coverage.order, Some(1));
        assert_eq!(coverage.cost_types, Some(vec!["full-payment".to_string()]));
        assert_eq!(coverage.cost_categories, Some(vec!["primary".to_string()]));
        assert_eq!(coverage.cost_networks, Some(vec!["self-pay".to_string()]));
        assert_eq!(coverage.cost_units, Some(vec!["individual".to_string()]));
        assert_eq!(coverage.cost_terms, Some(vec!["per-service".to_string()]));
        assert_eq!(coverage.cost_values, Some(vec!["100%".to_string()]));
        assert_eq!(coverage.cost_value_types, Some(vec!["quantity".to_string()]));
        assert_eq!(coverage.payment_by_party_ids, Some(vec!["pat_12345".to_string()]));
        assert_eq!(coverage.payment_by_party_types, Some(vec!["Patient".to_string()]));
        assert_eq!(coverage.payment_by_responsibilities, Some(vec!["Full payment responsibility".to_string()]));
        assert_eq!(coverage.subrogation, Some(false));
        assert_eq!(coverage.notes, Some("Patient responsible for full payment of services".to_string()));
    }

    #[test]
    fn test_domain_coverage_dependent() {
        let json = r#"{
            "coverage_id": "cov_dependent_001",
            "patient_demographic_no": "12345",
            "status": "active",
            "kind": "insurance",
            "type": "medical",
            "type_code": "MED",
            "type_system": "http://terminology.hl7.org/CodeSystem/v3-ActCode",
            "type_display": "Medical",
            "policy_holder_id": "pat_parent_001",
            "policy_holder_type": "Patient",
            "subscriber_id": "pat_parent_001",
            "subscriber_type": "Patient",
            "subscriber_identifier": "SUB123456789",
            "beneficiary_id": "pat_12345",
            "dependent_number": "01",
            "relationship": "child",
            "relationship_code": "child",
            "relationship_system": "http://terminology.hl7.org/CodeSystem/subscriber-relationship",
            "relationship_display": "Child",
            "period_start": "2024-01-01T00:00:00Z",
            "period_end": "2024-12-31T23:59:59Z",
            "insurer_id": "org_insurer_001",
            "network": "PPO Network",
            "class_types": ["group", "plan"],
            "class_values": ["GROUP001", "PLAN001"],
            "class_names": ["Employee Group", "Premium Plan"],
            "class_systems": ["http://terminology.hl7.org/CodeSystem/coverage-class", "http://terminology.hl7.org/CodeSystem/coverage-class"],
            "order": 1,
            "cost_types": ["copay", "deductible"],
            "cost_categories": ["primary", "primary"],
            "cost_networks": ["in-network", "in-network"],
            "cost_units": ["individual", "individual"],
            "cost_terms": ["annual", "annual"],
            "cost_values": ["$25.00", "$500.00"],
            "cost_value_types": ["money", "money"],
            "subrogation": false,
            "contract_ids": ["contract_001"],
            "insurance_plan_id": "plan_001",
            "notes": "Dependent coverage under parent's insurance"
        }"#;

        let coverage: DomainCoverage = serde_json::from_str(json).unwrap();
        
        assert_eq!(coverage.coverage_id, "cov_dependent_001");
        assert_eq!(coverage.patient_demographic_no, "12345");
        assert_eq!(coverage.status, Some("active".to_string()));
        assert_eq!(coverage.kind, Some("insurance".to_string()));
        assert_eq!(coverage.r#type, Some("medical".to_string()));
        assert_eq!(coverage.type_code, Some("MED".to_string()));
        assert_eq!(coverage.type_system, Some("http://terminology.hl7.org/CodeSystem/v3-ActCode".to_string()));
        assert_eq!(coverage.type_display, Some("Medical".to_string()));
        assert_eq!(coverage.policy_holder_id, Some("pat_parent_001".to_string()));
        assert_eq!(coverage.policy_holder_type, Some("Patient".to_string()));
        assert_eq!(coverage.subscriber_id, Some("pat_parent_001".to_string()));
        assert_eq!(coverage.subscriber_type, Some("Patient".to_string()));
        assert_eq!(coverage.subscriber_identifier, Some("SUB123456789".to_string()));
        assert_eq!(coverage.beneficiary_id, Some("pat_12345".to_string()));
        assert_eq!(coverage.dependent_number, Some("01".to_string()));
        assert_eq!(coverage.relationship, Some("child".to_string()));
        assert_eq!(coverage.relationship_code, Some("child".to_string()));
        assert_eq!(coverage.relationship_system, Some("http://terminology.hl7.org/CodeSystem/subscriber-relationship".to_string()));
        assert_eq!(coverage.relationship_display, Some("Child".to_string()));
        assert_eq!(coverage.period_start, Some("2024-01-01T00:00:00Z".to_string()));
        assert_eq!(coverage.period_end, Some("2024-12-31T23:59:59Z".to_string()));
        assert_eq!(coverage.insurer_id, Some("org_insurer_001".to_string()));
        assert_eq!(coverage.network, Some("PPO Network".to_string()));
        assert_eq!(coverage.class_types, Some(vec!["group".to_string(), "plan".to_string()]));
        assert_eq!(coverage.class_values, Some(vec!["GROUP001".to_string(), "PLAN001".to_string()]));
        assert_eq!(coverage.class_names, Some(vec!["Employee Group".to_string(), "Premium Plan".to_string()]));
        assert_eq!(coverage.class_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/coverage-class".to_string(), "http://terminology.hl7.org/CodeSystem/coverage-class".to_string()]));
        assert_eq!(coverage.order, Some(1));
        assert_eq!(coverage.cost_types, Some(vec!["copay".to_string(), "deductible".to_string()]));
        assert_eq!(coverage.cost_categories, Some(vec!["primary".to_string(), "primary".to_string()]));
        assert_eq!(coverage.cost_networks, Some(vec!["in-network".to_string(), "in-network".to_string()]));
        assert_eq!(coverage.cost_units, Some(vec!["individual".to_string(), "individual".to_string()]));
        assert_eq!(coverage.cost_terms, Some(vec!["annual".to_string(), "annual".to_string()]));
        assert_eq!(coverage.cost_values, Some(vec!["$25.00".to_string(), "$500.00".to_string()]));
        assert_eq!(coverage.cost_value_types, Some(vec!["money".to_string(), "money".to_string()]));
        assert_eq!(coverage.subrogation, Some(false));
        assert_eq!(coverage.contract_ids, Some(vec!["contract_001".to_string()]));
        assert_eq!(coverage.insurance_plan_id, Some("plan_001".to_string()));
        assert_eq!(coverage.notes, Some("Dependent coverage under parent's insurance".to_string()));
    }

    #[test]
    fn test_domain_coverage_missing_required_field() {
        let json = r#"{
            "insurer_id": "org_001"
        }"#;

        // This should fail because coverage_id and patient_demographic_no are required
        let result: Result<DomainCoverage, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
