use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainMedication {
    pub medication_id: String,
    
    // Basic medication information
    pub status: Option<String>, // "active" | "inactive" | "entered-in-error"
    
    // Identification and coding
    pub code: Option<String>, // Codes that identify this medication
    pub code_system: Option<String>, // Terminology system for code
    pub code_display: Option<String>, // Display name for code
    
    // Marketing authorization
    pub marketing_authorization_holder_id: Option<String>, // Organization that has authorization to market medication
    pub marketing_authorization_holder_type: Option<String>, // Type of marketing authorization holder
    
    // Dose form
    pub dose_form: Option<String>, // powder | tablets | capsule +
    pub dose_form_code: Option<String>, // Code for dose form
    pub dose_form_system: Option<String>, // Terminology system for dose form
    pub dose_form_display: Option<String>, // Display name for dose form
    
    // Total volume
    pub total_volume_value: Option<f64>, // Specific amount of drug in the product
    pub total_volume_unit: Option<String>, // Unit for total volume
    pub total_volume_system: Option<String>, // System for total volume unit
    pub total_volume_code: Option<String>, // Code for total volume unit
    
    // Ingredients
    pub ingredient_item_codes: Option<Vec<String>>, // The ingredient (substance or medication)
    pub ingredient_item_systems: Option<Vec<String>>, // Terminology systems for ingredient items
    pub ingredient_item_displays: Option<Vec<String>>, // Display names for ingredient items
    pub ingredient_item_reference_ids: Option<Vec<String>>, // Reference IDs for ingredient items
    pub ingredient_item_reference_types: Option<Vec<String>>, // Types of ingredient item references
    pub ingredient_is_active: Option<Vec<bool>>, // Active ingredient indicator
    pub ingredient_strength_ratio_numerator_value: Option<Vec<f64>>, // Strength ratio numerator value
    pub ingredient_strength_ratio_numerator_unit: Option<Vec<String>>, // Strength ratio numerator unit
    pub ingredient_strength_ratio_numerator_system: Option<Vec<String>>, // Strength ratio numerator system
    pub ingredient_strength_ratio_numerator_code: Option<Vec<String>>, // Strength ratio numerator code
    pub ingredient_strength_ratio_denominator_value: Option<Vec<f64>>, // Strength ratio denominator value
    pub ingredient_strength_ratio_denominator_unit: Option<Vec<String>>, // Strength ratio denominator unit
    pub ingredient_strength_ratio_denominator_system: Option<Vec<String>>, // Strength ratio denominator system
    pub ingredient_strength_ratio_denominator_code: Option<Vec<String>>, // Strength ratio denominator code
    pub ingredient_strength_codeable_concept_codes: Option<Vec<String>>, // Strength codeable concept codes
    pub ingredient_strength_codeable_concept_systems: Option<Vec<String>>, // Strength codeable concept systems
    pub ingredient_strength_codeable_concept_displays: Option<Vec<String>>, // Strength codeable concept displays
    pub ingredient_strength_quantity_value: Option<Vec<f64>>, // Strength quantity value
    pub ingredient_strength_quantity_unit: Option<Vec<String>>, // Strength quantity unit
    pub ingredient_strength_quantity_system: Option<Vec<String>>, // Strength quantity system
    pub ingredient_strength_quantity_code: Option<Vec<String>>, // Strength quantity code
    
    // Batch information
    pub batch_lot_number: Option<String>, // Identifier assigned to batch
    pub batch_expiration_date: Option<String>, // When batch will expire (ISO datetime)
    
    // Knowledge reference
    pub definition_id: Option<String>, // Knowledge about this medication
    pub definition_type: Option<String>, // Type of definition reference
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_medication_deserialization() {
        let json = r#"{
            "medication_id": "med_12345",
            "status": "active",
            "code": "acetaminophen",
            "code_system": "http://www.nlm.nih.gov/research/umls/rxnorm",
            "code_display": "Acetaminophen",
            "marketing_authorization_holder_id": "org_pharma_001",
            "marketing_authorization_holder_type": "Organization",
            "dose_form": "tablet",
            "dose_form_code": "TAB",
            "dose_form_system": "http://terminology.hl7.org/CodeSystem/v3-orderableDrugForm",
            "dose_form_display": "Tablet",
            "total_volume_value": 500.0,
            "total_volume_unit": "mg",
            "total_volume_system": "http://unitsofmeasure.org",
            "total_volume_code": "mg",
            "ingredient_item_codes": ["acetaminophen", "inactive-ingredients"],
            "ingredient_item_systems": ["http://www.nlm.nih.gov/research/umls/rxnorm", "http://terminology.hl7.org/CodeSystem/v3-orderableDrugForm"],
            "ingredient_item_displays": ["Acetaminophen", "Inactive Ingredients"],
            "ingredient_item_reference_ids": ["sub_acetaminophen", "sub_inactive"],
            "ingredient_item_reference_types": ["Substance", "Substance"],
            "ingredient_is_active": [true, false],
            "ingredient_strength_ratio_numerator_value": [500.0, 100.0],
            "ingredient_strength_ratio_numerator_unit": ["mg", "mg"],
            "ingredient_strength_ratio_numerator_system": ["http://unitsofmeasure.org", "http://unitsofmeasure.org"],
            "ingredient_strength_ratio_numerator_code": ["mg", "mg"],
            "ingredient_strength_ratio_denominator_value": [1.0, 1.0],
            "ingredient_strength_ratio_denominator_unit": ["1", "1"],
            "ingredient_strength_ratio_denominator_system": ["http://unitsofmeasure.org", "http://unitsofmeasure.org"],
            "ingredient_strength_ratio_denominator_code": ["1", "1"],
            "ingredient_strength_codeable_concept_codes": ["500mg", "100mg"],
            "ingredient_strength_codeable_concept_systems": ["http://terminology.hl7.org/CodeSystem/v3-orderableDrugForm", "http://terminology.hl7.org/CodeSystem/v3-orderableDrugForm"],
            "ingredient_strength_codeable_concept_displays": ["500mg", "100mg"],
            "ingredient_strength_quantity_value": [500.0, 100.0],
            "ingredient_strength_quantity_unit": ["mg", "mg"],
            "ingredient_strength_quantity_system": ["http://unitsofmeasure.org", "http://unitsofmeasure.org"],
            "ingredient_strength_quantity_code": ["mg", "mg"],
            "batch_lot_number": "LOT123456",
            "batch_expiration_date": "2025-12-31T23:59:59Z",
            "definition_id": "med_knowledge_acetaminophen",
            "definition_type": "MedicationKnowledge"
        }"#;

        let medication: DomainMedication = serde_json::from_str(json).unwrap();
        
        assert_eq!(medication.medication_id, "med_12345");
        assert_eq!(medication.status, Some("active".to_string()));
        assert_eq!(medication.code, Some("acetaminophen".to_string()));
        assert_eq!(medication.code_system, Some("http://www.nlm.nih.gov/research/umls/rxnorm".to_string()));
        assert_eq!(medication.code_display, Some("Acetaminophen".to_string()));
        assert_eq!(medication.marketing_authorization_holder_id, Some("org_pharma_001".to_string()));
        assert_eq!(medication.marketing_authorization_holder_type, Some("Organization".to_string()));
        assert_eq!(medication.dose_form, Some("tablet".to_string()));
        assert_eq!(medication.dose_form_code, Some("TAB".to_string()));
        assert_eq!(medication.dose_form_system, Some("http://terminology.hl7.org/CodeSystem/v3-orderableDrugForm".to_string()));
        assert_eq!(medication.dose_form_display, Some("Tablet".to_string()));
        assert_eq!(medication.total_volume_value, Some(500.0));
        assert_eq!(medication.total_volume_unit, Some("mg".to_string()));
        assert_eq!(medication.total_volume_system, Some("http://unitsofmeasure.org".to_string()));
        assert_eq!(medication.total_volume_code, Some("mg".to_string()));
        assert_eq!(medication.ingredient_item_codes, Some(vec!["acetaminophen".to_string(), "inactive-ingredients".to_string()]));
        assert_eq!(medication.ingredient_item_systems, Some(vec!["http://www.nlm.nih.gov/research/umls/rxnorm".to_string(), "http://terminology.hl7.org/CodeSystem/v3-orderableDrugForm".to_string()]));
        assert_eq!(medication.ingredient_item_displays, Some(vec!["Acetaminophen".to_string(), "Inactive Ingredients".to_string()]));
        assert_eq!(medication.ingredient_item_reference_ids, Some(vec!["sub_acetaminophen".to_string(), "sub_inactive".to_string()]));
        assert_eq!(medication.ingredient_item_reference_types, Some(vec!["Substance".to_string(), "Substance".to_string()]));
        assert_eq!(medication.ingredient_is_active, Some(vec![true, false]));
        assert_eq!(medication.ingredient_strength_ratio_numerator_value, Some(vec![500.0, 100.0]));
        assert_eq!(medication.ingredient_strength_ratio_numerator_unit, Some(vec!["mg".to_string(), "mg".to_string()]));
        assert_eq!(medication.ingredient_strength_ratio_numerator_system, Some(vec!["http://unitsofmeasure.org".to_string(), "http://unitsofmeasure.org".to_string()]));
        assert_eq!(medication.ingredient_strength_ratio_numerator_code, Some(vec!["mg".to_string(), "mg".to_string()]));
        assert_eq!(medication.ingredient_strength_ratio_denominator_value, Some(vec![1.0, 1.0]));
        assert_eq!(medication.ingredient_strength_ratio_denominator_unit, Some(vec!["1".to_string(), "1".to_string()]));
        assert_eq!(medication.ingredient_strength_ratio_denominator_system, Some(vec!["http://unitsofmeasure.org".to_string(), "http://unitsofmeasure.org".to_string()]));
        assert_eq!(medication.ingredient_strength_ratio_denominator_code, Some(vec!["1".to_string(), "1".to_string()]));
        assert_eq!(medication.ingredient_strength_codeable_concept_codes, Some(vec!["500mg".to_string(), "100mg".to_string()]));
        assert_eq!(medication.ingredient_strength_codeable_concept_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/v3-orderableDrugForm".to_string(), "http://terminology.hl7.org/CodeSystem/v3-orderableDrugForm".to_string()]));
        assert_eq!(medication.ingredient_strength_codeable_concept_displays, Some(vec!["500mg".to_string(), "100mg".to_string()]));
        assert_eq!(medication.ingredient_strength_quantity_value, Some(vec![500.0, 100.0]));
        assert_eq!(medication.ingredient_strength_quantity_unit, Some(vec!["mg".to_string(), "mg".to_string()]));
        assert_eq!(medication.ingredient_strength_quantity_system, Some(vec!["http://unitsofmeasure.org".to_string(), "http://unitsofmeasure.org".to_string()]));
        assert_eq!(medication.ingredient_strength_quantity_code, Some(vec!["mg".to_string(), "mg".to_string()]));
        assert_eq!(medication.batch_lot_number, Some("LOT123456".to_string()));
        assert_eq!(medication.batch_expiration_date, Some("2025-12-31T23:59:59Z".to_string()));
        assert_eq!(medication.definition_id, Some("med_knowledge_acetaminophen".to_string()));
        assert_eq!(medication.definition_type, Some("MedicationKnowledge".to_string()));
    }

    #[test]
    fn test_domain_medication_minimal_deserialization() {
        let json = r#"{
            "medication_id": "med_67890"
        }"#;

        let medication: DomainMedication = serde_json::from_str(json).unwrap();
        
        assert_eq!(medication.medication_id, "med_67890");
        assert_eq!(medication.status, None);
        assert_eq!(medication.code, None);
        assert_eq!(medication.code_system, None);
        assert_eq!(medication.code_display, None);
        assert_eq!(medication.marketing_authorization_holder_id, None);
        assert_eq!(medication.marketing_authorization_holder_type, None);
        assert_eq!(medication.dose_form, None);
        assert_eq!(medication.dose_form_code, None);
        assert_eq!(medication.dose_form_system, None);
        assert_eq!(medication.dose_form_display, None);
        assert_eq!(medication.total_volume_value, None);
        assert_eq!(medication.total_volume_unit, None);
        assert_eq!(medication.total_volume_system, None);
        assert_eq!(medication.total_volume_code, None);
        assert_eq!(medication.ingredient_item_codes, None);
        assert_eq!(medication.ingredient_item_systems, None);
        assert_eq!(medication.ingredient_item_displays, None);
        assert_eq!(medication.ingredient_item_reference_ids, None);
        assert_eq!(medication.ingredient_item_reference_types, None);
        assert_eq!(medication.ingredient_is_active, None);
        assert_eq!(medication.ingredient_strength_ratio_numerator_value, None);
        assert_eq!(medication.ingredient_strength_ratio_numerator_unit, None);
        assert_eq!(medication.ingredient_strength_ratio_numerator_system, None);
        assert_eq!(medication.ingredient_strength_ratio_numerator_code, None);
        assert_eq!(medication.ingredient_strength_ratio_denominator_value, None);
        assert_eq!(medication.ingredient_strength_ratio_denominator_unit, None);
        assert_eq!(medication.ingredient_strength_ratio_denominator_system, None);
        assert_eq!(medication.ingredient_strength_ratio_denominator_code, None);
        assert_eq!(medication.ingredient_strength_codeable_concept_codes, None);
        assert_eq!(medication.ingredient_strength_codeable_concept_systems, None);
        assert_eq!(medication.ingredient_strength_codeable_concept_displays, None);
        assert_eq!(medication.ingredient_strength_quantity_value, None);
        assert_eq!(medication.ingredient_strength_quantity_unit, None);
        assert_eq!(medication.ingredient_strength_quantity_system, None);
        assert_eq!(medication.ingredient_strength_quantity_code, None);
        assert_eq!(medication.batch_lot_number, None);
        assert_eq!(medication.batch_expiration_date, None);
        assert_eq!(medication.definition_id, None);
        assert_eq!(medication.definition_type, None);
    }

    #[test]
    fn test_domain_medication_missing_required_field() {
        let json = r#"{
            "status": "active"
        }"#;

        // This should fail because medication_id is required
        let result: Result<DomainMedication, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
