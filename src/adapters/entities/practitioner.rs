use chrono::{DateTime, Utc};
use crate::domain::practitioner::DomainPractitioner;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    // Note: Practitioner struct may not be generated yet
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
    Date,
    Period,
    HumanName,
    ContactPoint,
    Address,
    Attachment,
    Boolean,
};

// Placeholder for Practitioner until it's generated
// This will be replaced with the actual struct when available
pub struct Practitioner {
    pub id: Option<Id>,
    pub identifier: Vec<Identifier>,
    pub active: Option<Boolean>,
    pub name: Vec<HumanName>,
    pub telecom: Vec<ContactPoint>,
    pub gender: Option<GenderCode>,
    pub birth_date: Option<Date>,
    pub deceased: Option<DeceasedX>,
    pub address: Vec<Address>,
    pub photo: Vec<Attachment>,
    pub qualification: Vec<Qualification>,
    pub communication: Vec<Communication>,
}

// Placeholder nested types
pub struct GenderCode {
    pub value: i32,
    pub id: Option<String>,
    pub extension: Vec<Extension>,
}

pub struct DeceasedX {
    pub choice: Option<DeceasedChoice>,
}

pub enum DeceasedChoice {
    Boolean(Boolean),
    DateTime(FhirDateTime),
}

pub struct Qualification {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub identifier: Vec<Identifier>,
    pub code: Option<CodeableConcept>,
    pub period: Option<Period>,
    pub issuer: Option<Reference>,
}

pub struct Communication {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub language: Option<CodeableConcept>,
    pub preferred: Option<Boolean>,
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

impl Default for Practitioner {
    fn default() -> Self {
        Self {
            id: None,
            identifier: Vec::new(),
            active: None,
            name: Vec::new(),
            telecom: Vec::new(),
            gender: None,
            birth_date: None,
            deceased: None,
            address: Vec::new(),
            photo: Vec::new(),
            qualification: Vec::new(),
            communication: Vec::new(),
        }
    }
}

impl From<DomainPractitioner> for Practitioner {
    fn from(src: DomainPractitioner) -> Self {
        // Start with a completely empty message
        let mut dest = Practitioner::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.practitioner_id.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:practitioner_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.practitioner_id.clone(),
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
        // 4. Name  ----------------------------------------------------------
        if src.family_name.is_some() || src.given_names.is_some() || src.prefix.is_some() || src.suffix.is_some() || src.text.is_some() {
            let mut human_name = HumanName {
                ..Default::default()
            };

            // Use
            if let Some(use_code) = src.use_code {
                human_name.use = Some(use_code);
            }

            // Family
            if let Some(family_name) = src.family_name {
                human_name.family = Some(String {
                    value: family_name,
                    ..Default::default()
                });
            }

            // Given
            if let Some(given_names) = src.given_names {
                for given_name in given_names {
                    human_name.given.push(String {
                        value: given_name,
                        ..Default::default()
                    });
                }
            }

            // Prefix
            if let Some(prefix) = src.prefix {
                for prefix_item in prefix {
                    human_name.prefix.push(String {
                        value: prefix_item,
                        ..Default::default()
                    });
                }
            }

            // Suffix
            if let Some(suffix) = src.suffix {
                for suffix_item in suffix {
                    human_name.suffix.push(String {
                        value: suffix_item,
                        ..Default::default()
                    });
                }
            }

            // Text
            if let Some(text) = src.text {
                human_name.text = Some(String {
                    value: text,
                    ..Default::default()
                });
            }

            dest.name.push(human_name);
        }

        // ------------------------------------------------------------------
        // 5. Telecom  -------------------------------------------------------
        if let Some(telecom_systems) = src.telecom_system {
            for (i, system) in telecom_systems.iter().enumerate() {
                let mut contact_point = ContactPoint {
                    ..Default::default()
                };

                // System
                contact_point.system = Some(system.clone());

                // Value
                if let Some(telecom_values) = &src.telecom_value {
                    if let Some(value) = telecom_values.get(i) {
                        contact_point.value = Some(String {
                            value: value.clone(),
                            ..Default::default()
                        });
                    }
                }

                // Use
                if let Some(telecom_uses) = &src.telecom_use {
                    if let Some(use) = telecom_uses.get(i) {
                        contact_point.r#use = Some(use.clone());
                    }
                }

                // Rank
                if let Some(telecom_ranks) = &src.telecom_rank {
                    if let Some(rank) = telecom_ranks.get(i) {
                        contact_point.rank = Some(*rank);
                    }
                }

                // Period
                if src.telecom_period_start.is_some() || src.telecom_period_end.is_some() {
                    let mut period = Period {
                        ..Default::default()
                    };

                    if let Some(period_start) = src.telecom_period_start.as_ref().and_then(|starts| starts.get(i)) {
                        if let Ok(dt) = DateTime::parse_from_rfc3339(period_start) {
                            let fhir_dt = dt.with_timezone(&Utc);
                            period.start = Some(FhirDateTime {
                                value_us: fhir_dt.timestamp_micros(),
                                ..Default::default()
                            });
                        }
                    }

                    if let Some(period_end) = src.telecom_period_end.as_ref().and_then(|ends| ends.get(i)) {
                        if let Ok(dt) = DateTime::parse_from_rfc3339(period_end) {
                            let fhir_dt = dt.with_timezone(&Utc);
                            period.end = Some(FhirDateTime {
                                value_us: fhir_dt.timestamp_micros(),
                                ..Default::default()
                            });
                        }
                    }

                    contact_point.period = Some(period);
                }

                dest.telecom.push(contact_point);
            }
        }

        // ------------------------------------------------------------------
        // 6. Gender  --------------------------------------------------------
        if let Some(gender) = src.gender {
            let gender_value = match gender.to_lowercase().as_str() {
                "male" => 1,
                "female" => 2,
                "other" => 3,
                "unknown" => 4,
                _ => 0,
            };

            dest.gender = Some(GenderCode {
                value: gender_value,
                id: None,
                extension: Vec::new(),
            });
        }

        // ------------------------------------------------------------------
        // 7. Birth Date  ----------------------------------------------------
        if let Some(birth_date) = src.birth_date {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(&birth_date, "%Y-%m-%d") {
                dest.birth_date = Some(Date {
                    value: date.format("%Y-%m-%d").to_string(),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 8. Deceased  ------------------------------------------------------
        if let Some(deceased) = src.deceased {
            if deceased {
                if let Some(deceased_date) = src.deceased_date {
                    if let Ok(dt) = DateTime::parse_from_rfc3339(&deceased_date) {
                        let fhir_dt = dt.with_timezone(&Utc);
                        dest.deceased = Some(DeceasedX {
                            choice: Some(DeceasedChoice::DateTime(FhirDateTime {
                                value_us: fhir_dt.timestamp_micros(),
                                ..Default::default()
                            })),
                        });
                    }
                } else {
                    dest.deceased = Some(DeceasedX {
                        choice: Some(DeceasedChoice::Boolean(Boolean {
                            value: true,
                            ..Default::default()
                        })),
                    });
                }
            } else {
                dest.deceased = Some(DeceasedX {
                    choice: Some(DeceasedChoice::Boolean(Boolean {
                        value: false,
                        ..Default::default()
                    })),
                });
            }
        }

        // ------------------------------------------------------------------
        // 9. Address  -------------------------------------------------------
        if src.address_use.is_some() || src.address_type.is_some() || src.address_text.is_some() || 
           src.address_line.is_some() || src.address_city.is_some() || src.address_state.is_some() || 
           src.address_postal_code.is_some() || src.address_country.is_some() {
            
            let mut address = Address {
                ..Default::default()
            };

            // Use
            if let Some(address_uses) = &src.address_use {
                if let Some(use) = address_uses.get(0) {
                    address.use = Some(use.clone());
                }
            }

            // Type
            if let Some(address_types) = &src.address_type {
                if let Some(r#type) = address_types.get(0) {
                    address.r#type = Some(r#type.clone());
                }
            }

            // Text
            if let Some(address_texts) = &src.address_text {
                if let Some(text) = address_texts.get(0) {
                    address.text = Some(String {
                        value: text.clone(),
                        ..Default::default()
                    });
                }
            }

            // Line
            if let Some(address_lines) = &src.address_line {
                if let Some(lines) = address_lines.get(0) {
                    for line in lines {
                        address.line.push(String {
                            value: line.clone(),
                            ..Default::default()
                        });
                    }
                }
            }

            // City
            if let Some(address_cities) = &src.address_city {
                if let Some(city) = address_cities.get(0) {
                    address.city = Some(String {
                        value: city.clone(),
                        ..Default::default()
                    });
                }
            }

            // District
            if let Some(address_districts) = &src.address_district {
                if let Some(district) = address_districts.get(0) {
                    address.district = Some(String {
                        value: district.clone(),
                        ..Default::default()
                    });
                }
            }

            // State
            if let Some(address_states) = &src.address_state {
                if let Some(state) = address_states.get(0) {
                    address.state = Some(String {
                        value: state.clone(),
                        ..Default::default()
                    });
                }
            }

            // Postal Code
            if let Some(address_postal_codes) = &src.address_postal_code {
                if let Some(postal_code) = address_postal_codes.get(0) {
                    address.postal_code = Some(String {
                        value: postal_code.clone(),
                        ..Default::default()
                    });
                }
            }

            // Country
            if let Some(address_countries) = &src.address_country {
                if let Some(country) = address_countries.get(0) {
                    address.country = Some(String {
                        value: country.clone(),
                        ..Default::default()
                    });
                }
            }

            // Period
            if src.address_period_start.is_some() || src.address_period_end.is_some() {
                let mut period = Period {
                    ..Default::default()
                };

                if let Some(period_start) = src.address_period_start.as_ref().and_then(|starts| starts.get(0)) {
                    if let Ok(dt) = DateTime::parse_from_rfc3339(period_start) {
                        let fhir_dt = dt.with_timezone(&Utc);
                        period.start = Some(FhirDateTime {
                            value_us: fhir_dt.timestamp_micros(),
                            ..Default::default()
                        });
                    }
                }

                if let Some(period_end) = src.address_period_end.as_ref().and_then(|ends| ends.get(0)) {
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

            dest.address.push(address);
        }

        // ------------------------------------------------------------------
        // 10. Photo  --------------------------------------------------------
        if src.photo_content_type.is_some() || src.photo_url.is_some() || src.photo_title.is_some() {
            let mut attachment = Attachment {
                ..Default::default()
            };

            // Content Type
            if let Some(photo_content_types) = &src.photo_content_type {
                if let Some(content_type) = photo_content_types.get(0) {
                    attachment.content_type = Some(String {
                        value: content_type.clone(),
                        ..Default::default()
                    });
                }
            }

            // Language
            if let Some(photo_languages) = &src.photo_language {
                if let Some(language) = photo_languages.get(0) {
                    attachment.language = Some(String {
                        value: language.clone(),
                        ..Default::default()
                    });
                }
            }

            // Data
            if let Some(photo_data) = &src.photo_data {
                if let Some(data) = photo_data.get(0) {
                    attachment.data = Some(String {
                        value: data.clone(),
                        ..Default::default()
                    });
                }
            }

            // URL
            if let Some(photo_urls) = &src.photo_url {
                if let Some(url) = photo_urls.get(0) {
                    attachment.url = Some(Uri {
                        value: url.clone(),
                        ..Default::default()
                    });
                }
            }

            // Size
            if let Some(photo_sizes) = &src.photo_size {
                if let Some(size) = photo_sizes.get(0) {
                    attachment.size = Some(*size);
                }
            }

            // Hash
            if let Some(photo_hashes) = &src.photo_hash {
                if let Some(hash) = photo_hashes.get(0) {
                    attachment.hash = Some(String {
                        value: hash.clone(),
                        ..Default::default()
                    });
                }
            }

            // Title
            if let Some(photo_titles) = &src.photo_title {
                if let Some(title) = photo_titles.get(0) {
                    attachment.title = Some(String {
                        value: title.clone(),
                        ..Default::default()
                    });
                }
            }

            // Creation
            if let Some(photo_creations) = &src.photo_creation {
                if let Some(creation) = photo_creations.get(0) {
                    if let Ok(dt) = DateTime::parse_from_rfc3339(creation) {
                        let fhir_dt = dt.with_timezone(&Utc);
                        attachment.creation = Some(FhirDateTime {
                            value_us: fhir_dt.timestamp_micros(),
                            ..Default::default()
                        });
                    }
                }
            }

            dest.photo.push(attachment);
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
                                value: "http://terminology.hl7.org/CodeSystem/v2-0360".to_string(),
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

        // ------------------------------------------------------------------
        // 12. Communication  ------------------------------------------------
        if let Some(communication_languages) = src.communication_languages {
            for (i, language) in communication_languages.iter().enumerate() {
                let mut communication = Communication {
                    ..Default::default()
                };

                // Language
                communication.language = Some(CodeableConcept {
                    coding: vec![Coding {
                        system: src.communication_language_systems.as_ref()
                            .and_then(|systems| systems.get(i))
                            .map(|system| Uri {
                                value: system.clone(),
                                ..Default::default()
                            })
                            .or_else(|| Some(Uri {
                                value: "urn:ietf:bcp:47".to_string(),
                                ..Default::default()
                            })),
                        code: src.communication_language_codes.as_ref()
                            .and_then(|codes| codes.get(i))
                            .map(|code| Code {
                                value: code.clone(),
                                ..Default::default()
                            }),
                        display: src.communication_language_displays.as_ref()
                            .and_then(|displays| displays.get(i))
                            .map(|display| String {
                                value: display.clone(),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }],
                    text: Some(String {
                        value: language.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                // Preferred
                if let Some(communication_preferred) = &src.communication_preferred {
                    if let Some(preferred) = communication_preferred.get(i) {
                        communication.preferred = Some(Boolean {
                            value: *preferred,
                            ..Default::default()
                        });
                    }
                }

                dest.communication.push(communication);
            }
        }

        dest
    }
}
