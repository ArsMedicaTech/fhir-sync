use chrono::{DateTime, Utc};
use crate::domain::location::DomainLocation;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    // Note: Location struct may not be generated yet
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
    Address,
    Markdown,
    Decimal,
    Boolean,
    ExtendedContactDetail,
    ContactPoint,
    Availability,
    VirtualServiceDetail,
};

// Placeholder for Location until it's generated
// This will be replaced with the actual struct when available
pub struct Location {
    pub id: Option<Id>,
    pub identifier: Vec<Identifier>,
    pub status: Option<StatusCode>,
    pub operational_status: Option<Coding>,
    pub name: Option<String>,
    pub alias: Vec<String>,
    pub description: Option<Markdown>,
    pub mode: Option<ModeCode>,
    pub r#type: Vec<CodeableConcept>,
    pub contact: Vec<ExtendedContactDetail>,
    pub address: Option<Address>,
    pub form: Option<CodeableConcept>,
    pub position: Option<Position>,
    pub managing_organization: Option<Reference>,
    pub part_of: Option<Reference>,
    pub characteristic: Vec<CodeableConcept>,
    pub hours_of_operation: Vec<Availability>,
    pub virtual_service: Vec<VirtualServiceDetail>,
    pub endpoint: Vec<Reference>,
}

// Placeholder nested types
pub struct StatusCode {
    pub value: i32,
    pub id: Option<String>,
    pub extension: Vec<Extension>,
}

pub struct ModeCode {
    pub value: i32,
    pub id: Option<String>,
    pub extension: Vec<Extension>,
}

pub struct Position {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub longitude: Option<Decimal>,
    pub latitude: Option<Decimal>,
    pub altitude: Option<Decimal>,
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

impl Default for Location {
    fn default() -> Self {
        Self {
            id: None,
            identifier: Vec::new(),
            status: None,
            operational_status: None,
            name: None,
            alias: Vec::new(),
            description: None,
            mode: None,
            r#type: Vec::new(),
            contact: Vec::new(),
            address: None,
            form: None,
            position: None,
            managing_organization: None,
            part_of: None,
            characteristic: Vec::new(),
            hours_of_operation: Vec::new(),
            virtual_service: Vec::new(),
            endpoint: Vec::new(),
        }
    }
}

impl From<DomainLocation> for Location {
    fn from(src: DomainLocation) -> Self {
        // Start with a completely empty message
        let mut dest = Location::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.location_id.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:location_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.location_id.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 3. Status  --------------------------------------------------------
        if let Some(status) = src.status {
            let status_value = match status.to_lowercase().as_str() {
                "active" => 1,
                "suspended" => 2,
                "inactive" => 3,
                _ => 0,
            };

            dest.status = Some(StatusCode {
                value: status_value,
                id: None,
                extension: Vec::new(),
            });
        }

        // ------------------------------------------------------------------
        // 4. Operational Status  --------------------------------------------
        if let Some(operational_status) = src.operational_status {
            dest.operational_status = Some(Coding {
                system: src.operational_status_system.map(|system| Uri {
                    value: system,
                    ..Default::default()
                }).or_else(|| Some(Uri {
                    value: "http://terminology.hl7.org/CodeSystem/v2-0116".to_string(),
                    ..Default::default()
                })),
                code: src.operational_status_code.map(|code| Code {
                    value: code,
                    ..Default::default()
                }),
                display: src.operational_status_display.map(|display| String {
                    value: display,
                    ..Default::default()
                }),
                ..Default::default()
            });
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
        // 8. Mode  ----------------------------------------------------------
        if let Some(mode) = src.mode {
            let mode_value = match mode.to_lowercase().as_str() {
                "instance" => 1,
                "kind" => 2,
                _ => 0,
            };

            dest.mode = Some(ModeCode {
                value: mode_value,
                id: None,
                extension: Vec::new(),
            });
        }

        // ------------------------------------------------------------------
        // 9. Type  ----------------------------------------------------------
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
                                value: "http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string(),
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
        // 10. Contact  ------------------------------------------------------
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
        // 11. Address  ------------------------------------------------------
        if src.address_use.is_some() || src.address_type.is_some() || src.address_text.is_some() || 
           src.address_line.is_some() || src.address_city.is_some() || src.address_state.is_some() || 
           src.address_postal_code.is_some() || src.address_country.is_some() {
            
            let mut address = Address {
                ..Default::default()
            };

            if let Some(address_use) = src.address_use {
                address.use = Some(address_use);
            }

            if let Some(address_type) = src.address_type {
                address.r#type = Some(address_type);
            }

            if let Some(address_text) = src.address_text {
                address.text = Some(String {
                    value: address_text,
                    ..Default::default()
                });
            }

            if let Some(address_line) = src.address_line {
                for line in address_line {
                    address.line.push(String {
                        value: line,
                        ..Default::default()
                    });
                }
            }

            if let Some(address_city) = src.address_city {
                address.city = Some(String {
                    value: address_city,
                    ..Default::default()
                });
            }

            if let Some(address_district) = src.address_district {
                address.district = Some(String {
                    value: address_district,
                    ..Default::default()
                });
            }

            if let Some(address_state) = src.address_state {
                address.state = Some(String {
                    value: address_state,
                    ..Default::default()
                });
            }

            if let Some(address_postal_code) = src.address_postal_code {
                address.postal_code = Some(String {
                    value: address_postal_code,
                    ..Default::default()
                });
            }

            if let Some(address_country) = src.address_country {
                address.country = Some(String {
                    value: address_country,
                    ..Default::default()
                });
            }

            if src.address_period_start.is_some() || src.address_period_end.is_some() {
                let mut period = Period {
                    ..Default::default()
                };

                if let Some(period_start) = src.address_period_start {
                    if let Ok(dt) = DateTime::parse_from_rfc3339(&period_start) {
                        let fhir_dt = dt.with_timezone(&Utc);
                        period.start = Some(FhirDateTime {
                            value_us: fhir_dt.timestamp_micros(),
                            ..Default::default()
                        });
                    }
                }

                if let Some(period_end) = src.address_period_end {
                    if let Ok(dt) = DateTime::parse_from_rfc3339(&period_end) {
                        let fhir_dt = dt.with_timezone(&Utc);
                        period.end = Some(FhirDateTime {
                            value_us: fhir_dt.timestamp_micros(),
                            ..Default::default()
                        });
                    }
                }

                address.period = Some(period);
            }

            dest.address = Some(address);
        }

        // ------------------------------------------------------------------
        // 12. Form  ---------------------------------------------------------
        if let Some(form) = src.form {
            dest.form = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.form_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/location-physical-type".to_string(),
                        ..Default::default()
                    })),
                    code: src.form_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.form_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: form,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 13. Position  -----------------------------------------------------
        if src.longitude.is_some() || src.latitude.is_some() || src.altitude.is_some() {
            let mut position = Position {
                ..Default::default()
            };

            if let Some(longitude) = src.longitude {
                position.longitude = Some(Decimal {
                    value: longitude,
                    ..Default::default()
                });
            }

            if let Some(latitude) = src.latitude {
                position.latitude = Some(Decimal {
                    value: latitude,
                    ..Default::default()
                });
            }

            if let Some(altitude) = src.altitude {
                position.altitude = Some(Decimal {
                    value: altitude,
                    ..Default::default()
                });
            }

            dest.position = Some(position);
        }

        // ------------------------------------------------------------------
        // 14. Managing Organization  ----------------------------------------
        if let Some(managing_organization_id) = src.managing_organization_id {
            let managing_organization_type = src.managing_organization_type
                .unwrap_or_else(|| "Organization".to_string());

            dest.managing_organization = Some(Reference {
                reference: Some(String {
                    value: format!("{}/{}", managing_organization_type, managing_organization_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 15. Part Of  ------------------------------------------------------
        if let Some(part_of_id) = src.part_of_id {
            let part_of_type = src.part_of_type
                .unwrap_or_else(|| "Location".to_string());

            dest.part_of = Some(Reference {
                reference: Some(String {
                    value: format!("{}/{}", part_of_type, part_of_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 16. Characteristics  ----------------------------------------------
        if let Some(characteristics) = src.characteristics {
            for (i, characteristic) in characteristics.iter().enumerate() {
                dest.characteristic.push(CodeableConcept {
                    coding: vec![Coding {
                        system: src.characteristic_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/location-characteristic".to_string(),
                                ..Default::default()
                            })),
                        code: src.characteristic_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.characteristic_displays.as_ref()
                            .and_then(|displays| displays.get(i))
                            .map(|display| String {
                                value: display.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: characteristic.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 17. Hours of Operation  -------------------------------------------
        if let Some(hours_of_operation_days_of_week) = src.hours_of_operation_days_of_week {
            for (i, days_of_week) in hours_of_operation_days_of_week.iter().enumerate() {
                let mut availability = Availability {
                    ..Default::default()
                };

                for day in days_of_week {
                    availability.days_of_week.push(day.clone());
                }

                if let Some(all_day) = src.hours_of_operation_all_day.as_ref().and_then(|all_days| all_days.get(i)) {
                    availability.all_day = Some(Boolean {
                        value: *all_day,
                        ..Default::default()
                    });
                }

                if let Some(opening_time) = src.hours_of_operation_opening_time.as_ref().and_then(|times| times.get(i)) {
                    availability.opening_time = Some(String {
                        value: opening_time.clone(),
                        ..Default::default()
                    });
                }

                if let Some(closing_time) = src.hours_of_operation_closing_time.as_ref().and_then(|times| times.get(i)) {
                    availability.closing_time = Some(String {
                        value: closing_time.clone(),
                        ..Default::default()
                    });
                }

                dest.hours_of_operation.push(availability);
            }
        }

        // ------------------------------------------------------------------
        // 18. Virtual Service  ----------------------------------------------
        if let Some(virtual_service_channel_types) = src.virtual_service_channel_type {
            for (i, channel_type) in virtual_service_channel_types.iter().enumerate() {
                let mut virtual_service = VirtualServiceDetail {
                    ..Default::default()
                };

                virtual_service.channel_type = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.virtual_service_channel_type_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "http://terminology.hl7.org/CodeSystem/v3-EncounterChannel".to_string(),
                                ..Default::default()
                            })),
                        code: src.virtual_service_channel_type_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.virtual_service_channel_type_displays.as_ref()
                            .and_then(|displays| displays.get(i))
                            .map(|display| String {
                                value: display.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: channel_type.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                if let Some(address_url) = src.virtual_service_address_url.as_ref().and_then(|urls| urls.get(i)) {
                    virtual_service.address_url = Some(Uri {
                        value: address_url.clone(),
                        ..Default::default()
                    });
                }

                dest.virtual_service.push(virtual_service);
            }
        }

        // ------------------------------------------------------------------
        // 19. Endpoints  ----------------------------------------------------
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

        dest
    }
}
