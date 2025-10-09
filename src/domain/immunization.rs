use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainImmunization {
    pub immunization_id: String,
    pub patient_demographic_no: String,
    
    // Basic immunization information
    pub status: Option<String>, // "completed" | "entered-in-error" | "not-done"
    pub status_reason: Option<String>, // Reason for current status
    pub status_reason_code: Option<String>, // Code for status reason
    pub status_reason_system: Option<String>, // Terminology system for status reason
    pub status_reason_display: Option<String>, // Display name for status reason
    
    // Vaccine information
    pub vaccine_code: Option<String>, // Vaccine administered
    pub vaccine_code_code: Option<String>, // Code for vaccine
    pub vaccine_code_system: Option<String>, // Terminology system for vaccine
    pub vaccine_code_display: Option<String>, // Display name for vaccine
    pub administered_product: Option<String>, // Product that was administered
    pub administered_product_code: Option<String>, // Code for administered product
    pub administered_product_system: Option<String>, // Terminology system for administered product
    pub administered_product_display: Option<String>, // Display name for administered product
    pub manufacturer: Option<String>, // Vaccine manufacturer
    pub manufacturer_code: Option<String>, // Code for manufacturer
    pub manufacturer_system: Option<String>, // Terminology system for manufacturer
    pub manufacturer_display: Option<String>, // Display name for manufacturer
    pub lot_number: Option<String>, // Vaccine lot number
    pub expiration_date: Option<String>, // ISO date string for vaccine expiration date
    
    // Temporal information
    pub occurrence_date: Option<String>, // ISO datetime string for vaccine administration date
    pub occurrence_string: Option<String>, // String description of occurrence
    
    // Context information
    pub primary_source: Option<bool>, // Indicates context the data was captured in
    pub information_source: Option<String>, // Indicates the source of a reported record
    pub information_source_code: Option<String>, // Code for information source
    pub information_source_system: Option<String>, // Terminology system for information source
    pub information_source_display: Option<String>, // Display name for information source
    
    // Location and administration details
    pub location_id: Option<String>, // Where immunization occurred
    pub site: Option<String>, // Body site vaccine was administered
    pub site_code: Option<String>, // Code for site
    pub site_system: Option<String>, // Terminology system for site
    pub site_display: Option<String>, // Display name for site
    pub route: Option<String>, // How vaccine entered body
    pub route_code: Option<String>, // Code for route
    pub route_system: Option<String>, // Terminology system for route
    pub route_display: Option<String>, // Display name for route
    pub dose_quantity_value: Option<f64>, // Amount of vaccine administered
    pub dose_quantity_unit: Option<String>, // Unit for dose quantity
    
    // References and relationships
    pub encounter_id: Option<String>, // Encounter immunization was part of
    pub based_on_ids: Option<Vec<String>>, // Authority that the immunization event is based on
    pub based_on_types: Option<Vec<String>>, // Types of based on references
    pub supporting_information_ids: Option<Vec<String>>, // Additional information in support of the immunization
    
    // Performers
    pub performer_function_codes: Option<Vec<String>>, // What type of performance was done
    pub performer_function_code_codes: Option<Vec<String>>, // Codes for performer functions
    pub performer_function_code_systems: Option<Vec<String>>, // Terminology systems for functions
    pub performer_function_code_displays: Option<Vec<String>>, // Display names for functions
    pub performer_actor_ids: Option<Vec<String>>, // Individual or organization who was performing
    pub performer_actor_types: Option<Vec<String>>, // Types of performers
    
    // Reasons
    pub reason_codes: Option<Vec<String>>, // Why immunization occurred
    pub reason_code_codes: Option<Vec<String>>, // Codes for reasons
    pub reason_code_systems: Option<Vec<String>>, // Terminology systems for reasons
    pub reason_code_displays: Option<Vec<String>>, // Display names for reasons
    pub reason_reference_ids: Option<Vec<String>>, // References for reasons
    pub reason_reference_types: Option<Vec<String>>, // Types of reason references
    
    // Potency information
    pub is_subpotent: Option<bool>, // Dose potency
    pub subpotent_reason_codes: Option<Vec<String>>, // Reason for being subpotent
    pub subpotent_reason_code_codes: Option<Vec<String>>, // Codes for subpotent reasons
    pub subpotent_reason_code_systems: Option<Vec<String>>, // Terminology systems for subpotent reasons
    pub subpotent_reason_code_displays: Option<Vec<String>>, // Display names for subpotent reasons
    
    // Program eligibility
    pub program_eligibility_programs: Option<Vec<String>>, // The program that eligibility is declared for
    pub program_eligibility_program_codes: Option<Vec<String>>, // Codes for programs
    pub program_eligibility_program_systems: Option<Vec<String>>, // Terminology systems for programs
    pub program_eligibility_program_displays: Option<Vec<String>>, // Display names for programs
    pub program_eligibility_statuses: Option<Vec<String>>, // The patient's eligibility status for the program
    pub program_eligibility_status_codes: Option<Vec<String>>, // Codes for statuses
    pub program_eligibility_status_systems: Option<Vec<String>>, // Terminology systems for statuses
    pub program_eligibility_status_displays: Option<Vec<String>>, // Display names for statuses
    
    // Funding source
    pub funding_source: Option<String>, // Funding source for the vaccine
    pub funding_source_code: Option<String>, // Code for funding source
    pub funding_source_system: Option<String>, // Terminology system for funding source
    pub funding_source_display: Option<String>, // Display name for funding source
    
    // Reactions
    pub reaction_dates: Option<Vec<String>>, // When reaction started (ISO datetime strings)
    pub reaction_manifestations: Option<Vec<String>>, // Additional information on reaction
    pub reaction_manifestation_codes: Option<Vec<String>>, // Codes for manifestations
    pub reaction_manifestation_systems: Option<Vec<String>>, // Terminology systems for manifestations
    pub reaction_manifestation_displays: Option<Vec<String>>, // Display names for manifestations
    pub reaction_reference_ids: Option<Vec<String>>, // References for manifestations
    pub reaction_reference_types: Option<Vec<String>>, // Types of reaction references
    pub reaction_reported: Option<Vec<bool>>, // Indicates self-reported reaction
    
    // Protocol applied
    pub protocol_series: Option<Vec<String>>, // Name of vaccine series
    pub protocol_authority_ids: Option<Vec<String>>, // Who is responsible for publishing the recommendations
    pub protocol_target_diseases: Option<Vec<String>>, // Vaccine preventable disease being targeted
    pub protocol_target_disease_codes: Option<Vec<String>>, // Codes for target diseases
    pub protocol_target_disease_systems: Option<Vec<String>>, // Terminology systems for target diseases
    pub protocol_target_disease_displays: Option<Vec<String>>, // Display names for target diseases
    pub protocol_dose_numbers: Option<Vec<String>>, // Dose number within series
    pub protocol_series_doses: Option<Vec<String>>, // Recommended number of doses for immunity
    
    // Additional information
    pub notes: Option<String>, // Additional immunization notes
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_domain_immunization_deserialization() {
        let json = r#"{
            "immunization_id": "imm_12345",
            "patient_demographic_no": "12345",
            "status": "completed",
            "status_reason": "routine",
            "status_reason_code": "routine",
            "status_reason_system": "http://terminology.hl7.org/CodeSystem/v3-ActReason",
            "status_reason_display": "Routine",
            "vaccine_code": "covid-19-vaccine",
            "vaccine_code_code": "207",
            "vaccine_code_system": "http://hl7.org/fhir/sid/cvx",
            "vaccine_code_display": "COVID-19, mRNA, LNP-S, PF, 100 mcg/0.5mL dose",
            "administered_product": "pfizer-biontech-covid-19-vaccine",
            "administered_product_code": "EU/1/20/1528",
            "administered_product_system": "http://ema.europa.eu/",
            "administered_product_display": "Comirnaty",
            "manufacturer": "pfizer",
            "manufacturer_code": "PFR",
            "manufacturer_system": "http://hl7.org/fhir/sid/mvx",
            "manufacturer_display": "Pfizer, Inc",
            "lot_number": "EW0165",
            "expiration_date": "2024-12-31",
            "occurrence_date": "2024-01-15T10:30:00Z",
            "primary_source": true,
            "information_source": "patient",
            "information_source_code": "patient",
            "information_source_system": "http://terminology.hl7.org/CodeSystem/information-source",
            "information_source_display": "Patient",
            "location_id": "loc_001",
            "site": "left-deltoid",
            "site_code": "LA",
            "site_system": "http://terminology.hl7.org/CodeSystem/v3-ActSite",
            "site_display": "Left deltoid",
            "route": "intramuscular",
            "route_code": "IM",
            "route_system": "http://terminology.hl7.org/CodeSystem/v3-RouteOfAdministration",
            "route_display": "Intramuscular",
            "dose_quantity_value": 0.5,
            "dose_quantity_unit": "mL",
            "encounter_id": "enc_001",
            "based_on_ids": ["sr_001"],
            "based_on_types": ["ServiceRequest"],
            "supporting_information_ids": ["obs_001"],
            "performer_function_codes": ["performer"],
            "performer_function_code_codes": ["performer"],
            "performer_function_code_systems": ["http://terminology.hl7.org/CodeSystem/v3-ParticipationType"],
            "performer_function_code_displays": ["Performer"],
            "performer_actor_ids": ["prac_001"],
            "performer_actor_types": ["Practitioner"],
            "reason_codes": ["routine"],
            "reason_code_codes": ["routine"],
            "reason_code_systems": ["http://terminology.hl7.org/CodeSystem/v3-ActReason"],
            "reason_code_displays": ["Routine"],
            "is_subpotent": false,
            "program_eligibility_programs": ["covid-19-vaccination"],
            "program_eligibility_program_codes": ["covid-19-vaccination"],
            "program_eligibility_program_systems": ["http://terminology.hl7.org/CodeSystem/vaccination-program"],
            "program_eligibility_program_displays": ["COVID-19 Vaccination Program"],
            "program_eligibility_statuses": ["eligible"],
            "program_eligibility_status_codes": ["eligible"],
            "program_eligibility_status_systems": ["http://terminology.hl7.org/CodeSystem/vaccination-program-status"],
            "program_eligibility_status_displays": ["Eligible"],
            "funding_source": "government",
            "funding_source_code": "government",
            "funding_source_system": "http://terminology.hl7.org/CodeSystem/vaccination-funding-source",
            "funding_source_display": "Government",
            "reaction_dates": ["2024-01-15T14:00:00Z"],
            "reaction_manifestations": ["fever"],
            "reaction_manifestation_codes": ["386661006"],
            "reaction_manifestation_systems": ["http://snomed.info/sct"],
            "reaction_manifestation_displays": ["Fever"],
            "reaction_reported": [true],
            "protocol_series": ["covid-19-primary-series"],
            "protocol_authority_ids": ["org_cdc"],
            "protocol_target_diseases": ["covid-19"],
            "protocol_target_disease_codes": ["840539006"],
            "protocol_target_disease_systems": ["http://snomed.info/sct"],
            "protocol_target_disease_displays": ["COVID-19"],
            "protocol_dose_numbers": ["1"],
            "protocol_series_doses": ["2"],
            "notes": "Patient tolerated vaccination well. No immediate adverse reactions observed."
        }"#;

        let immunization: DomainImmunization = serde_json::from_str(json).unwrap();
        
        assert_eq!(immunization.immunization_id, "imm_12345");
        assert_eq!(immunization.patient_demographic_no, "12345");
        assert_eq!(immunization.status, Some("completed".to_string()));
        assert_eq!(immunization.status_reason, Some("routine".to_string()));
        assert_eq!(immunization.status_reason_code, Some("routine".to_string()));
        assert_eq!(immunization.status_reason_system, Some("http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string()));
        assert_eq!(immunization.status_reason_display, Some("Routine".to_string()));
        assert_eq!(immunization.vaccine_code, Some("covid-19-vaccine".to_string()));
        assert_eq!(immunization.vaccine_code_code, Some("207".to_string()));
        assert_eq!(immunization.vaccine_code_system, Some("http://hl7.org/fhir/sid/cvx".to_string()));
        assert_eq!(immunization.vaccine_code_display, Some("COVID-19, mRNA, LNP-S, PF, 100 mcg/0.5mL dose".to_string()));
        assert_eq!(immunization.administered_product, Some("pfizer-biontech-covid-19-vaccine".to_string()));
        assert_eq!(immunization.administered_product_code, Some("EU/1/20/1528".to_string()));
        assert_eq!(immunization.administered_product_system, Some("http://ema.europa.eu/".to_string()));
        assert_eq!(immunization.administered_product_display, Some("Comirnaty".to_string()));
        assert_eq!(immunization.manufacturer, Some("pfizer".to_string()));
        assert_eq!(immunization.manufacturer_code, Some("PFR".to_string()));
        assert_eq!(immunization.manufacturer_system, Some("http://hl7.org/fhir/sid/mvx".to_string()));
        assert_eq!(immunization.manufacturer_display, Some("Pfizer, Inc".to_string()));
        assert_eq!(immunization.lot_number, Some("EW0165".to_string()));
        assert_eq!(immunization.expiration_date, Some("2024-12-31".to_string()));
        assert_eq!(immunization.occurrence_date, Some("2024-01-15T10:30:00Z".to_string()));
        assert_eq!(immunization.primary_source, Some(true));
        assert_eq!(immunization.information_source, Some("patient".to_string()));
        assert_eq!(immunization.information_source_code, Some("patient".to_string()));
        assert_eq!(immunization.information_source_system, Some("http://terminology.hl7.org/CodeSystem/information-source".to_string()));
        assert_eq!(immunization.information_source_display, Some("Patient".to_string()));
        assert_eq!(immunization.location_id, Some("loc_001".to_string()));
        assert_eq!(immunization.site, Some("left-deltoid".to_string()));
        assert_eq!(immunization.site_code, Some("LA".to_string()));
        assert_eq!(immunization.site_system, Some("http://terminology.hl7.org/CodeSystem/v3-ActSite".to_string()));
        assert_eq!(immunization.site_display, Some("Left deltoid".to_string()));
        assert_eq!(immunization.route, Some("intramuscular".to_string()));
        assert_eq!(immunization.route_code, Some("IM".to_string()));
        assert_eq!(immunization.route_system, Some("http://terminology.hl7.org/CodeSystem/v3-RouteOfAdministration".to_string()));
        assert_eq!(immunization.route_display, Some("Intramuscular".to_string()));
        assert_eq!(immunization.dose_quantity_value, Some(0.5));
        assert_eq!(immunization.dose_quantity_unit, Some("mL".to_string()));
        assert_eq!(immunization.encounter_id, Some("enc_001".to_string()));
        assert_eq!(immunization.based_on_ids, Some(vec!["sr_001".to_string()]));
        assert_eq!(immunization.based_on_types, Some(vec!["ServiceRequest".to_string()]));
        assert_eq!(immunization.supporting_information_ids, Some(vec!["obs_001".to_string()]));
        assert_eq!(immunization.performer_function_codes, Some(vec!["performer".to_string()]));
        assert_eq!(immunization.performer_function_code_codes, Some(vec!["performer".to_string()]));
        assert_eq!(immunization.performer_function_code_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ParticipationType".to_string()]));
        assert_eq!(immunization.performer_function_code_displays, Some(vec!["Performer".to_string()]));
        assert_eq!(immunization.performer_actor_ids, Some(vec!["prac_001".to_string()]));
        assert_eq!(immunization.performer_actor_types, Some(vec!["Practitioner".to_string()]));
        assert_eq!(immunization.reason_codes, Some(vec!["routine".to_string()]));
        assert_eq!(immunization.reason_code_codes, Some(vec!["routine".to_string()]));
        assert_eq!(immunization.reason_code_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string()]));
        assert_eq!(immunization.reason_code_displays, Some(vec!["Routine".to_string()]));
        assert_eq!(immunization.is_subpotent, Some(false));
        assert_eq!(immunization.program_eligibility_programs, Some(vec!["covid-19-vaccination".to_string()]));
        assert_eq!(immunization.program_eligibility_program_codes, Some(vec!["covid-19-vaccination".to_string()]));
        assert_eq!(immunization.program_eligibility_program_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/vaccination-program".to_string()]));
        assert_eq!(immunization.program_eligibility_program_displays, Some(vec!["COVID-19 Vaccination Program".to_string()]));
        assert_eq!(immunization.program_eligibility_statuses, Some(vec!["eligible".to_string()]));
        assert_eq!(immunization.program_eligibility_status_codes, Some(vec!["eligible".to_string()]));
        assert_eq!(immunization.program_eligibility_status_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/vaccination-program-status".to_string()]));
        assert_eq!(immunization.program_eligibility_status_displays, Some(vec!["Eligible".to_string()]));
        assert_eq!(immunization.funding_source, Some("government".to_string()));
        assert_eq!(immunization.funding_source_code, Some("government".to_string()));
        assert_eq!(immunization.funding_source_system, Some("http://terminology.hl7.org/CodeSystem/vaccination-funding-source".to_string()));
        assert_eq!(immunization.funding_source_display, Some("Government".to_string()));
        assert_eq!(immunization.reaction_dates, Some(vec!["2024-01-15T14:00:00Z".to_string()]));
        assert_eq!(immunization.reaction_manifestations, Some(vec!["fever".to_string()]));
        assert_eq!(immunization.reaction_manifestation_codes, Some(vec!["386661006".to_string()]));
        assert_eq!(immunization.reaction_manifestation_systems, Some(vec!["http://snomed.info/sct".to_string()]));
        assert_eq!(immunization.reaction_manifestation_displays, Some(vec!["Fever".to_string()]));
        assert_eq!(immunization.reaction_reported, Some(vec![true]));
        assert_eq!(immunization.protocol_series, Some(vec!["covid-19-primary-series".to_string()]));
        assert_eq!(immunization.protocol_authority_ids, Some(vec!["org_cdc".to_string()]));
        assert_eq!(immunization.protocol_target_diseases, Some(vec!["covid-19".to_string()]));
        assert_eq!(immunization.protocol_target_disease_codes, Some(vec!["840539006".to_string()]));
        assert_eq!(immunization.protocol_target_disease_systems, Some(vec!["http://snomed.info/sct".to_string()]));
        assert_eq!(immunization.protocol_target_disease_displays, Some(vec!["COVID-19".to_string()]));
        assert_eq!(immunization.protocol_dose_numbers, Some(vec!["1".to_string()]));
        assert_eq!(immunization.protocol_series_doses, Some(vec!["2".to_string()]));
        assert_eq!(immunization.notes, Some("Patient tolerated vaccination well. No immediate adverse reactions observed.".to_string()));
    }

    #[test]
    fn test_domain_immunization_minimal_deserialization() {
        let json = r#"{
            "immunization_id": "imm_67890",
            "patient_demographic_no": "67890"
        }"#;

        let immunization: DomainImmunization = serde_json::from_str(json).unwrap();
        
        assert_eq!(immunization.immunization_id, "imm_67890");
        assert_eq!(immunization.patient_demographic_no, "67890");
        assert_eq!(immunization.status, None);
        assert_eq!(immunization.status_reason, None);
        assert_eq!(immunization.status_reason_code, None);
        assert_eq!(immunization.status_reason_system, None);
        assert_eq!(immunization.status_reason_display, None);
        assert_eq!(immunization.vaccine_code, None);
        assert_eq!(immunization.vaccine_code_code, None);
        assert_eq!(immunization.vaccine_code_system, None);
        assert_eq!(immunization.vaccine_code_display, None);
        assert_eq!(immunization.administered_product, None);
        assert_eq!(immunization.administered_product_code, None);
        assert_eq!(immunization.administered_product_system, None);
        assert_eq!(immunization.administered_product_display, None);
        assert_eq!(immunization.manufacturer, None);
        assert_eq!(immunization.manufacturer_code, None);
        assert_eq!(immunization.manufacturer_system, None);
        assert_eq!(immunization.manufacturer_display, None);
        assert_eq!(immunization.lot_number, None);
        assert_eq!(immunization.expiration_date, None);
        assert_eq!(immunization.occurrence_date, None);
        assert_eq!(immunization.occurrence_string, None);
        assert_eq!(immunization.primary_source, None);
        assert_eq!(immunization.information_source, None);
        assert_eq!(immunization.information_source_code, None);
        assert_eq!(immunization.information_source_system, None);
        assert_eq!(immunization.information_source_display, None);
        assert_eq!(immunization.location_id, None);
        assert_eq!(immunization.site, None);
        assert_eq!(immunization.site_code, None);
        assert_eq!(immunization.site_system, None);
        assert_eq!(immunization.site_display, None);
        assert_eq!(immunization.route, None);
        assert_eq!(immunization.route_code, None);
        assert_eq!(immunization.route_system, None);
        assert_eq!(immunization.route_display, None);
        assert_eq!(immunization.dose_quantity_value, None);
        assert_eq!(immunization.dose_quantity_unit, None);
        assert_eq!(immunization.encounter_id, None);
        assert_eq!(immunization.based_on_ids, None);
        assert_eq!(immunization.based_on_types, None);
        assert_eq!(immunization.supporting_information_ids, None);
        assert_eq!(immunization.performer_function_codes, None);
        assert_eq!(immunization.performer_function_code_codes, None);
        assert_eq!(immunization.performer_function_code_systems, None);
        assert_eq!(immunization.performer_function_code_displays, None);
        assert_eq!(immunization.performer_actor_ids, None);
        assert_eq!(immunization.performer_actor_types, None);
        assert_eq!(immunization.reason_codes, None);
        assert_eq!(immunization.reason_code_codes, None);
        assert_eq!(immunization.reason_code_systems, None);
        assert_eq!(immunization.reason_code_displays, None);
        assert_eq!(immunization.reason_reference_ids, None);
        assert_eq!(immunization.reason_reference_types, None);
        assert_eq!(immunization.is_subpotent, None);
        assert_eq!(immunization.subpotent_reason_codes, None);
        assert_eq!(immunization.subpotent_reason_code_codes, None);
        assert_eq!(immunization.subpotent_reason_code_systems, None);
        assert_eq!(immunization.subpotent_reason_code_displays, None);
        assert_eq!(immunization.program_eligibility_programs, None);
        assert_eq!(immunization.program_eligibility_program_codes, None);
        assert_eq!(immunization.program_eligibility_program_systems, None);
        assert_eq!(immunization.program_eligibility_program_displays, None);
        assert_eq!(immunization.program_eligibility_statuses, None);
        assert_eq!(immunization.program_eligibility_status_codes, None);
        assert_eq!(immunization.program_eligibility_status_systems, None);
        assert_eq!(immunization.program_eligibility_status_displays, None);
        assert_eq!(immunization.funding_source, None);
        assert_eq!(immunization.funding_source_code, None);
        assert_eq!(immunization.funding_source_system, None);
        assert_eq!(immunization.funding_source_display, None);
        assert_eq!(immunization.reaction_dates, None);
        assert_eq!(immunization.reaction_manifestations, None);
        assert_eq!(immunization.reaction_manifestation_codes, None);
        assert_eq!(immunization.reaction_manifestation_systems, None);
        assert_eq!(immunization.reaction_manifestation_displays, None);
        assert_eq!(immunization.reaction_reference_ids, None);
        assert_eq!(immunization.reaction_reference_types, None);
        assert_eq!(immunization.reaction_reported, None);
        assert_eq!(immunization.protocol_series, None);
        assert_eq!(immunization.protocol_authority_ids, None);
        assert_eq!(immunization.protocol_target_diseases, None);
        assert_eq!(immunization.protocol_target_disease_codes, None);
        assert_eq!(immunization.protocol_target_disease_systems, None);
        assert_eq!(immunization.protocol_target_disease_displays, None);
        assert_eq!(immunization.protocol_dose_numbers, None);
        assert_eq!(immunization.protocol_series_doses, None);
        assert_eq!(immunization.notes, None);
    }

    #[test]
    fn test_domain_immunization_flu_vaccine() {
        let json = r#"{
            "immunization_id": "imm_flu_001",
            "patient_demographic_no": "12345",
            "status": "completed",
            "status_reason": "routine",
            "status_reason_code": "routine",
            "status_reason_system": "http://terminology.hl7.org/CodeSystem/v3-ActReason",
            "status_reason_display": "Routine",
            "vaccine_code": "influenza-vaccine",
            "vaccine_code_code": "140",
            "vaccine_code_system": "http://hl7.org/fhir/sid/cvx",
            "vaccine_code_display": "Influenza, seasonal, injectable",
            "administered_product": "fluzone-quadrivalent",
            "administered_product_code": "49281-0400-78",
            "administered_product_system": "http://www.fda.gov/",
            "administered_product_display": "Fluzone Quadrivalent",
            "manufacturer": "sanofi-pasteur",
            "manufacturer_code": "PMC",
            "manufacturer_system": "http://hl7.org/fhir/sid/mvx",
            "manufacturer_display": "Sanofi Pasteur",
            "lot_number": "FLU2024001",
            "expiration_date": "2025-06-30",
            "occurrence_date": "2024-10-15T09:00:00Z",
            "primary_source": true,
            "information_source": "practitioner",
            "information_source_code": "practitioner",
            "information_source_system": "http://terminology.hl7.org/CodeSystem/information-source",
            "information_source_display": "Practitioner",
            "location_id": "loc_002",
            "site": "left-deltoid",
            "site_code": "LA",
            "site_system": "http://terminology.hl7.org/CodeSystem/v3-ActSite",
            "site_display": "Left deltoid",
            "route": "intramuscular",
            "route_code": "IM",
            "route_system": "http://terminology.hl7.org/CodeSystem/v3-RouteOfAdministration",
            "route_display": "Intramuscular",
            "dose_quantity_value": 0.5,
            "dose_quantity_unit": "mL",
            "encounter_id": "enc_002",
            "performer_function_codes": ["performer"],
            "performer_function_code_codes": ["performer"],
            "performer_function_code_systems": ["http://terminology.hl7.org/CodeSystem/v3-ParticipationType"],
            "performer_function_code_displays": ["Performer"],
            "performer_actor_ids": ["prac_002"],
            "performer_actor_types": ["Practitioner"],
            "reason_codes": ["routine"],
            "reason_code_codes": ["routine"],
            "reason_code_systems": ["http://terminology.hl7.org/CodeSystem/v3-ActReason"],
            "reason_code_displays": ["Routine"],
            "is_subpotent": false,
            "program_eligibility_programs": ["influenza-vaccination"],
            "program_eligibility_program_codes": ["influenza-vaccination"],
            "program_eligibility_program_systems": ["http://terminology.hl7.org/CodeSystem/vaccination-program"],
            "program_eligibility_program_displays": ["Influenza Vaccination Program"],
            "program_eligibility_statuses": ["eligible"],
            "program_eligibility_status_codes": ["eligible"],
            "program_eligibility_status_systems": ["http://terminology.hl7.org/CodeSystem/vaccination-program-status"],
            "program_eligibility_status_displays": ["Eligible"],
            "funding_source": "insurance",
            "funding_source_code": "insurance",
            "funding_source_system": "http://terminology.hl7.org/CodeSystem/vaccination-funding-source",
            "funding_source_display": "Insurance",
            "protocol_series": ["influenza-annual"],
            "protocol_authority_ids": ["org_cdc"],
            "protocol_target_diseases": ["influenza"],
            "protocol_target_disease_codes": ["6142004"],
            "protocol_target_disease_systems": ["http://snomed.info/sct"],
            "protocol_target_disease_displays": ["Influenza"],
            "protocol_dose_numbers": ["1"],
            "protocol_series_doses": ["1"],
            "notes": "Annual influenza vaccination. Patient reported no adverse reactions."
        }"#;

        let immunization: DomainImmunization = serde_json::from_str(json).unwrap();
        
        assert_eq!(immunization.immunization_id, "imm_flu_001");
        assert_eq!(immunization.patient_demographic_no, "12345");
        assert_eq!(immunization.status, Some("completed".to_string()));
        assert_eq!(immunization.status_reason, Some("routine".to_string()));
        assert_eq!(immunization.status_reason_code, Some("routine".to_string()));
        assert_eq!(immunization.status_reason_system, Some("http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string()));
        assert_eq!(immunization.status_reason_display, Some("Routine".to_string()));
        assert_eq!(immunization.vaccine_code, Some("influenza-vaccine".to_string()));
        assert_eq!(immunization.vaccine_code_code, Some("140".to_string()));
        assert_eq!(immunization.vaccine_code_system, Some("http://hl7.org/fhir/sid/cvx".to_string()));
        assert_eq!(immunization.vaccine_code_display, Some("Influenza, seasonal, injectable".to_string()));
        assert_eq!(immunization.administered_product, Some("fluzone-quadrivalent".to_string()));
        assert_eq!(immunization.administered_product_code, Some("49281-0400-78".to_string()));
        assert_eq!(immunization.administered_product_system, Some("http://www.fda.gov/".to_string()));
        assert_eq!(immunization.administered_product_display, Some("Fluzone Quadrivalent".to_string()));
        assert_eq!(immunization.manufacturer, Some("sanofi-pasteur".to_string()));
        assert_eq!(immunization.manufacturer_code, Some("PMC".to_string()));
        assert_eq!(immunization.manufacturer_system, Some("http://hl7.org/fhir/sid/mvx".to_string()));
        assert_eq!(immunization.manufacturer_display, Some("Sanofi Pasteur".to_string()));
        assert_eq!(immunization.lot_number, Some("FLU2024001".to_string()));
        assert_eq!(immunization.expiration_date, Some("2025-06-30".to_string()));
        assert_eq!(immunization.occurrence_date, Some("2024-10-15T09:00:00Z".to_string()));
        assert_eq!(immunization.primary_source, Some(true));
        assert_eq!(immunization.information_source, Some("practitioner".to_string()));
        assert_eq!(immunization.information_source_code, Some("practitioner".to_string()));
        assert_eq!(immunization.information_source_system, Some("http://terminology.hl7.org/CodeSystem/information-source".to_string()));
        assert_eq!(immunization.information_source_display, Some("Practitioner".to_string()));
        assert_eq!(immunization.location_id, Some("loc_002".to_string()));
        assert_eq!(immunization.site, Some("left-deltoid".to_string()));
        assert_eq!(immunization.site_code, Some("LA".to_string()));
        assert_eq!(immunization.site_system, Some("http://terminology.hl7.org/CodeSystem/v3-ActSite".to_string()));
        assert_eq!(immunization.site_display, Some("Left deltoid".to_string()));
        assert_eq!(immunization.route, Some("intramuscular".to_string()));
        assert_eq!(immunization.route_code, Some("IM".to_string()));
        assert_eq!(immunization.route_system, Some("http://terminology.hl7.org/CodeSystem/v3-RouteOfAdministration".to_string()));
        assert_eq!(immunization.route_display, Some("Intramuscular".to_string()));
        assert_eq!(immunization.dose_quantity_value, Some(0.5));
        assert_eq!(immunization.dose_quantity_unit, Some("mL".to_string()));
        assert_eq!(immunization.encounter_id, Some("enc_002".to_string()));
        assert_eq!(immunization.performer_function_codes, Some(vec!["performer".to_string()]));
        assert_eq!(immunization.performer_function_code_codes, Some(vec!["performer".to_string()]));
        assert_eq!(immunization.performer_function_code_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ParticipationType".to_string()]));
        assert_eq!(immunization.performer_function_code_displays, Some(vec!["Performer".to_string()]));
        assert_eq!(immunization.performer_actor_ids, Some(vec!["prac_002".to_string()]));
        assert_eq!(immunization.performer_actor_types, Some(vec!["Practitioner".to_string()]));
        assert_eq!(immunization.reason_codes, Some(vec!["routine".to_string()]));
        assert_eq!(immunization.reason_code_codes, Some(vec!["routine".to_string()]));
        assert_eq!(immunization.reason_code_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/v3-ActReason".to_string()]));
        assert_eq!(immunization.reason_code_displays, Some(vec!["Routine".to_string()]));
        assert_eq!(immunization.is_subpotent, Some(false));
        assert_eq!(immunization.program_eligibility_programs, Some(vec!["influenza-vaccination".to_string()]));
        assert_eq!(immunization.program_eligibility_program_codes, Some(vec!["influenza-vaccination".to_string()]));
        assert_eq!(immunization.program_eligibility_program_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/vaccination-program".to_string()]));
        assert_eq!(immunization.program_eligibility_program_displays, Some(vec!["Influenza Vaccination Program".to_string()]));
        assert_eq!(immunization.program_eligibility_statuses, Some(vec!["eligible".to_string()]));
        assert_eq!(immunization.program_eligibility_status_codes, Some(vec!["eligible".to_string()]));
        assert_eq!(immunization.program_eligibility_status_systems, Some(vec!["http://terminology.hl7.org/CodeSystem/vaccination-program-status".to_string()]));
        assert_eq!(immunization.program_eligibility_status_displays, Some(vec!["Eligible".to_string()]));
        assert_eq!(immunization.funding_source, Some("insurance".to_string()));
        assert_eq!(immunization.funding_source_code, Some("insurance".to_string()));
        assert_eq!(immunization.funding_source_system, Some("http://terminology.hl7.org/CodeSystem/vaccination-funding-source".to_string()));
        assert_eq!(immunization.funding_source_display, Some("Insurance".to_string()));
        assert_eq!(immunization.protocol_series, Some(vec!["influenza-annual".to_string()]));
        assert_eq!(immunization.protocol_authority_ids, Some(vec!["org_cdc".to_string()]));
        assert_eq!(immunization.protocol_target_diseases, Some(vec!["influenza".to_string()]));
        assert_eq!(immunization.protocol_target_disease_codes, Some(vec!["6142004".to_string()]));
        assert_eq!(immunization.protocol_target_disease_systems, Some(vec!["http://snomed.info/sct".to_string()]));
        assert_eq!(immunization.protocol_target_disease_displays, Some(vec!["Influenza".to_string()]));
        assert_eq!(immunization.protocol_dose_numbers, Some(vec!["1".to_string()]));
        assert_eq!(immunization.protocol_series_doses, Some(vec!["1".to_string()]));
        assert_eq!(immunization.notes, Some("Annual influenza vaccination. Patient reported no adverse reactions.".to_string()));
    }

    #[test]
    fn test_domain_immunization_missing_required_field() {
        let json = r#"{
            "performer_actor_ids": ["prac_001"]
        }"#;

        // This should fail because immunization_id and patient_demographic_no are required
        let result: Result<DomainImmunization, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
