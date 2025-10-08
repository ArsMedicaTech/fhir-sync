use chrono::{DateTime, Utc};
use crate::domain::organization::DomainOrganization;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    // Note: Organization struct may not be generated yet
    // We'll use a placeholder for now and update when the struct is available
    Id,
    Identifier,
    Uri,
    String,
    CodeableConcept,
    Coding,
    Code,
    Reference,
    DateTime as FhirDateTime,
    Markdown,
    Boolean,
    ExtendedContactDetail,
    ContactPoint,
    Address,
    Period,
};

// Placeholder for Organization until it's generated
// This will be replaced with the actual struct when available
pub struct Organization {
    pub id: Option<Id>,
    pub identifier: Vec<Identifier>,
    pub active: Option<Boolean>,
    pub r#type: Vec<CodeableConcept>,
    pub name: Option<String>,
    pub alias: Vec<String>,
    pub description: Option<Markdown>,
    pub contact: Vec<ExtendedContactDetail>,
    pub part_of: Option<Reference>,
    pub endpoint: Vec<Reference>,
    pub qualification: Vec<Qualification>,
}

// Placeholder nested types
pub struct Qualification {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub identifier: Vec<Identifier>,
    pub code: Option<CodeableConcept>,
    pub period: Option<Period>,
    pub issuer: Option<Reference>,
}

pub struct Extension {
    pub id: Option<String>,
    pub url: String,
    pub value: Option<ExtensionValue>,
}

pub enum ExtensionValue {
    String(String),
    CodeableConcept(CodeableConcept),
    // Add other types as needed
}

impl Default for Organization {
    fn default() -> Self {
        Self {
            id: None,
            identifier: Vec::new(),
            active: None,
            r#type: Vec::new(),
            name: None,
            alias: Vec::new(),
            description: None,
            contact: Vec::new(),
            part_of: None,
            endpoint: Vec::new(),
            qualification: Vec::new(),
        }
    }
}

impl From<DomainOrganization> for Organization {
    fn from(src: DomainOrganization) -> Self {
        // Start with a completely empty message
        let mut dest = Organization::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.organization_id.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:organization_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.organization_id.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 3. Active  --------------------------------------------------------
        if let Some(active) = src.active {
            dest.active = Some(Boolean {
                value: active,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 4. Type  ----------------------------------------------------------
        if let Some(types) = src.types {
            for (i, r#type) in types.iter().enumerate() {
                dest.r#type.push(CodeableConcept {
                    coding: vec![Coding {
                        system: src.type_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/organization-type".to_string(),
                                ..Default::default()
                            })),
                        code: src.type_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.type_displays.as_ref()
                            .and_then(|displays| displays.get(i))
                            .map(|display| String {
                                value: display.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: r#type.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 5. Name  ----------------------------------------------------------
        if let Some(name) = src.name {
            dest.name = Some(String {
                value: name,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 6. Alias  ---------------------------------------------------------
        if let Some(alias) = src.alias {
            for alias_item in alias {
                dest.alias.push(String {
                    value: alias_item,
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 7. Description  ---------------------------------------------------
        if let Some(description) = src.description {
            dest.description = Some(Markdown {
                value: description,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 8. Contact  -------------------------------------------------------
        if let Some(contact_purposes) = src.contact_purpose {
            for (i, purpose) in contact_purposes.iter().enumerate() {
                let mut contact = ExtendedContactDetail {
                    ..Default::default()
                };

                // Purpose
                contact.purpose = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.contact_purpose_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/contactentity-type".to_string(),
                                ..Default::default()
                            })),
                        code: src.contact_purpose_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.contact_purpose_displays.as_ref()
                            .and_then(|displays| displays.get(i))
                            .map(|display| String {
                                value: display.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: purpose.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                // Name
                if let Some(contact_names) = &src.contact_name {
                    if let Some(name) = contact_names.get(i) {
                        contact.name = Some(String {
                            value: name.clone(),
                            ..Default::default()
                        });
                    }
                }

                // Telecom
                if let Some(telecom_systems) = &src.contact_telecom_system {
                    if let Some(systems) = telecom_systems.get(i) {
                        for (j, system) in systems.iter().enumerate() {
                            let mut contact_point = ContactPoint {
                                ..Default::default()
                            };

                            contact_point.system = Some(system.clone());

                            if let Some(telecom_values) = &src.contact_telecom_value {
                                if let Some(values) = telecom_values.get(i) {
                                    if let Some(value) = values.get(j) {
                                        contact_point.value = Some(String {
                                            value: value.clone(),
                                            ..Default::default()
                                        });
                                    }
                                }
                            }

                            if let Some(telecom_uses) = &src.contact_telecom_use {
                                if let Some(uses) = telecom_uses.get(i) {
                                    if let Some(use) = uses.get(j) {
                                        contact_point.r#use = Some(use.clone());
                                    }
                                }
                            }

                            if let Some(telecom_ranks) = &src.contact_telecom_rank {
                                if let Some(ranks) = telecom_ranks.get(i) {
                                    if let Some(rank) = ranks.get(j) {
                                        contact_point.rank = Some(*rank);
                                    }
                                }
                            }

                            contact.telecom.push(contact_point);
                        }
                    }
                }

                // Address
                if src.contact_address_use.is_some() || src.contact_address_type.is_some() || 
                   src.contact_address_text.is_some() || src.contact_address_line.is_some() || 
                   src.contact_address_city.is_some() || src.contact_address_state.is_some() || 
                   src.contact_address_postal_code.is_some() || src.contact_address_country.is_some() {
                    
                    let mut address = Address {
                        ..Default::default()
                    };

                    if let Some(address_uses) = &src.contact_address_use {
                        if let Some(use) = address_uses.get(i) {
                            address.use = Some(use.clone());
                        }
                    }

                    if let Some(address_types) = &src.contact_address_type {
                        if let Some(r#type) = address_types.get(i) {
                            address.r#type = Some(r#type.clone());
                        }
                    }

                    if let Some(address_texts) = &src.contact_address_text {
                        if let Some(text) = address_texts.get(i) {
                            address.text = Some(String {
                                value: text.clone(),
                                ..Default::default()
                            });
                        }
                    }

                    if let Some(address_lines) = &src.contact_address_line {
                        if let Some(lines) = address_lines.get(i) {
                            for line in lines {
                                address.line.push(String {
                                    value: line.clone(),
                                    ..Default::default()
                                });
                            }
                        }
                    }

                    if let Some(address_cities) = &src.contact_address_city {
                        if let Some(city) = address_cities.get(i) {
                            address.city = Some(String {
                                value: city.clone(),
                                ..Default::default()
                            });
                        }
                    }

                    if let Some(address_districts) = &src.contact_address_district {
                        if let Some(district) = address_districts.get(i) {
                            address.district = Some(String {
                                value: district.clone(),
                                ..Default::default()
                            });
                        }
                    }

                    if let Some(address_states) = &src.contact_address_state {
                        if let Some(state) = address_states.get(i) {
                            address.state = Some(String {
                                value: state.clone(),
                                ..Default::default()
                            });
                        }
                    }

                    if let Some(address_postal_codes) = &src.contact_address_postal_code {
                        if let Some(postal_code) = address_postal_codes.get(i) {
                            address.postal_code = Some(String {
                                value: postal_code.clone(),
                                ..Default::default()
                            });
                        }
                    }

                    if let Some(address_countries) = &src.contact_address_country {
                        if let Some(country) = address_countries.get(i) {
                            address.country = Some(String {
                                value: country.clone(),
                                ..Default::default()
                            });
                        }
                    }

                    if src.contact_period_start.is_some() || src.contact_period_end.is_some() {
                        let mut period = Period {
                            ..Default::default()
                        };

                        if let Some(period_start) = src.contact_period_start.as_ref().and_then(|starts| starts.get(i)) {
                            if let Ok(dt) = DateTime::parse_from_rfc3339(period_start) {
                                let fhir_dt = dt.with_timezone(&Utc);
                                period.start = Some(FhirDateTime {
                                    value_us: fhir_dt.timestamp_micros(),
                                    ..Default::default()
                                });
                            }
                        }

                        if let Some(period_end) = src.contact_period_end.as_ref().and_then(|ends| ends.get(i)) {
                            if let Ok(dt) = DateTime::parse_from_rfc3339(period_end) {
                                let fhir_dt = dt.with_timezone(&Utc);
                                period.end = Some(FhirDateTime {
                                    value_us: fhir_dt.timestamp_micros(),
                                    ..Default::default()
                                });
                            }
                        }

                        address.period = Some(period);
                    }

                    contact.address = Some(address);
                }

                // Organization
                if let Some(organization_ids) = &src.contact_organization_id {
                    if let Some(organization_id) = organization_ids.get(i) {
                        let organization_type = src.contact_organization_type.as_ref()
                            .and_then(|types| types.get(i))
                            .cloned()
                            .unwrap_or_else(|| "Organization".to_string());

                        contact.organization = Some(Reference {
                            reference: Some(String {
                                value: format!("{}/{}", organization_type, organization_id),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                dest.contact.push(contact);
            }
        }

        // ------------------------------------------------------------------
        // 9. Part Of  ------------------------------------------------------
        if let Some(part_of_id) = src.part_of_id {
            let part_of_type = src.part_of_type
                .unwrap_or_else(|| "Organization".to_string());

            dest.part_of = Some(Reference {
                reference: Some(String {
                    value: format!("{}/{}", part_of_type, part_of_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 10. Endpoints  ----------------------------------------------------
        if let Some(endpoint_ids) = src.endpoint_ids {
            for (i, endpoint_id) in endpoint_ids.iter().enumerate() {
                let endpoint_type = src.endpoint_types.as_ref()
                    .and_then(|types| types.get(i))
                    .cloned()
                    .unwrap_or_else(|| "Endpoint".to_string());

                dest.endpoint.push(Reference {
                    reference: Some(String {
                        value: format!("{}/{}", endpoint_type, endpoint_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 11. Qualification  ------------------------------------------------
        if let Some(qualification_codes) = src.qualification_codes {
            for (i, qualification_code) in qualification_codes.iter().enumerate() {
                let mut qualification = Qualification {
                    ..Default::default()
                };

                // Identifier
                if let Some(qualification_identifiers) = &src.qualification_identifiers {
                    if let Some(identifiers) = qualification_identifiers.get(i) {
                        for (j, identifier) in identifiers.iter().enumerate() {
                            let mut identifier_obj = Identifier {
                                ..Default::default()
                            };

                            // System
                            if let Some(identifier_systems) = &src.qualification_identifier_systems {
                                if let Some(systems) = identifier_systems.get(i) {
                                    if let Some(system) = systems.get(j) {
                                        identifier_obj.system = Some(Uri {
                                            value: system.clone(),
                                            ..Default::default()
                                        });
                                    }
                                }
                            }

                            // Value
                            identifier_obj.value = Some(String {
                                value: identifier.clone(),
                                ..Default::default()
                            });

                            // Use
                            if let Some(identifier_uses) = &src.qualification_identifier_uses {
                                if let Some(uses) = identifier_uses.get(i) {
                                    if let Some(use) = uses.get(j) {
                                        identifier_obj.use = Some(use.clone());
                                    }
                                }
                            }

                            // Period
                            if src.qualification_identifier_periods_start.is_some() || src.qualification_identifier_periods_end.is_some() {
                                let mut period = Period {
                                    ..Default::default()
                                };

                                if let Some(period_starts) = &src.qualification_identifier_periods_start {
                                    if let Some(starts) = period_starts.get(i) {
                                        if let Some(period_start) = starts.get(j) {
                                            if let Ok(dt) = DateTime::parse_from_rfc3339(period_start) {
                                                let fhir_dt = dt.with_timezone(&Utc);
                                                period.start = Some(FhirDateTime {
                                                    value_us: fhir_dt.timestamp_micros(),
                                                    ..Default::default()
                                                });
                                            }
                                        }
                                    }
                                }

                                if let Some(period_ends) = &src.qualification_identifier_periods_end {
                                    if let Some(ends) = period_ends.get(i) {
                                        if let Some(period_end) = ends.get(j) {
                                            if let Ok(dt) = DateTime::parse_from_rfc3339(period_end) {
                                                let fhir_dt = dt.with_timezone(&Utc);
                                                period.end = Some(FhirDateTime {
                                                    value_us: fhir_dt.timestamp_micros(),
                                                    ..Default::default()
                                                });
                                            }
                                        }
                                    }
                                }

                                identifier_obj.period = Some(period);
                            }

                            qualification.identifier.push(identifier_obj);
                        }
                    }
                }

                // Code
                qualification.code = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.qualification_code_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/organization-qualification".to_string(),
                                ..Default::default()
                            })),
                        code: src.qualification_code_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.qualification_code_displays.as_ref()
                            .and_then(|displays| displays.get(i))
                            .map(|display| String {
                                value: display.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: qualification_code.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                // Period
                if src.qualification_periods_start.is_some() || src.qualification_periods_end.is_some() {
                    let mut period = Period {
                        ..Default::default()
                    };

                    if let Some(period_start) = src.qualification_periods_start.as_ref().and_then(|starts| starts.get(i)) {
                        if let Ok(dt) = DateTime::parse_from_rfc3339(period_start) {
                            let fhir_dt = dt.with_timezone(&Utc);
                            period.start = Some(FhirDateTime {
                                value_us: fhir_dt.timestamp_micros(),
                                ..Default::default()
                            });
                        }
                    }

                    if let Some(period_end) = src.qualification_periods_end.as_ref().and_then(|ends| ends.get(i)) {
                        if let Ok(dt) = DateTime::parse_from_rfc3339(period_end) {
                            let fhir_dt = dt.with_timezone(&Utc);
                            period.end = Some(FhirDateTime {
                                value_us: fhir_dt.timestamp_micros(),
                                ..Default::default()
                            });
                        }
                    }

                    qualification.period = Some(period);
                }

                // Issuer
                if let Some(issuer_ids) = &src.qualification_issuer_ids {
                    if let Some(issuer_id) = issuer_ids.get(i) {
                        let issuer_type = src.qualification_issuer_types.as_ref()
                            .and_then(|types| types.get(i))
                            .cloned()
                            .unwrap_or_else(|| "Organization".to_string());

                        qualification.issuer = Some(Reference {
                            reference: Some(String {
                                value: format!("{}/{}", issuer_type, issuer_id),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                dest.qualification.push(qualification);
            }
        }

        dest
    }
}
