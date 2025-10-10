use chrono::{DateTime, Utc};
use crate::domain::coverage::DomainCoverage;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    // Note: Coverage struct may not be generated yet
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
    Period,
    Annotation,
    Money,
    SimpleQuantity,
    PositiveInt,
    Boolean,
};

// Placeholder for Coverage until it's generated
// This will be replaced with the actual struct when available
pub struct Coverage {
    pub id: Option<Id>,
    pub identifier: Vec<Identifier>,
    pub status: Option<StatusCode>,
    pub kind: Option<KindCode>,
    pub payment_by: Vec<PaymentBy>,
    pub r#type: Option<CodeableConcept>,
    pub policy_holder: Option<Reference>,
    pub subscriber: Option<Reference>,
    pub subscriber_id: Vec<Identifier>,
    pub beneficiary: Option<Reference>,
    pub dependent: Option<String>,
    pub relationship: Option<CodeableConcept>,
    pub period: Option<Period>,
    pub insurer: Option<Reference>,
    pub class_value: Vec<Class>,
    pub order: Option<PositiveInt>,
    pub network: Option<String>,
    pub cost_to_beneficiary: Vec<CostToBeneficiary>,
    pub subrogation: Option<Boolean>,
    pub contract: Vec<Reference>,
    pub insurance_plan: Option<Reference>,
}

// Placeholder nested types
pub struct StatusCode {
    pub value: i32,
    pub id: Option<String>,
    pub extension: Vec<Extension>,
}

pub struct KindCode {
    pub value: i32,
    pub id: Option<String>,
    pub extension: Vec<Extension>,
}

pub struct PaymentBy {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub party: Option<Reference>,
    pub responsibility: Option<String>,
}

pub struct Class {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub r#type: Option<CodeableConcept>,
    pub value: Option<Identifier>,
    pub name: Option<String>,
}

pub struct CostToBeneficiary {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub r#type: Option<CodeableConcept>,
    pub category: Option<CodeableConcept>,
    pub network: Option<CodeableConcept>,
    pub unit: Option<CodeableConcept>,
    pub term: Option<CodeableConcept>,
    pub value: Option<ValueX>,
    pub exception: Vec<Exception>,
}

pub struct ValueX {
    pub choice: Option<ValueChoice>,
}

pub enum ValueChoice {
    Quantity(SimpleQuantity),
    Money(Money),
}

pub struct Exception {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub r#type: Option<CodeableConcept>,
    pub period: Option<Period>,
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

impl Default for Coverage {
    fn default() -> Self {
        Self {
            id: None,
            identifier: Vec::new(),
            status: None,
            kind: None,
            payment_by: Vec::new(),
            r#type: None,
            policy_holder: None,
            subscriber: None,
            subscriber_id: Vec::new(),
            beneficiary: None,
            dependent: None,
            relationship: None,
            period: None,
            insurer: None,
            class_value: Vec::new(),
            order: None,
            network: None,
            cost_to_beneficiary: Vec::new(),
            subrogation: None,
            contract: Vec::new(),
            insurance_plan: None,
        }
    }
}

impl From<DomainCoverage> for Coverage {
    fn from(src: DomainCoverage) -> Self {
        // Start with a completely empty message
        let mut dest = Coverage::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.coverage_id.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:coverage_id".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.coverage_id.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 3. Status  --------------------------------------------------------
        if let Some(status) = src.status {
            let status_value = match status.to_lowercase().as_str() {
                "active" => 1,
                "cancelled" => 2,
                "draft" => 3,
                "entered-in-error" => 4,
                _ => 0,
            };

            dest.status = Some(StatusCode {
                value: status_value,
                id: None,
                extension: Vec::new(),
            });
        }

        // ------------------------------------------------------------------
        // 4. Kind  ----------------------------------------------------------
        if let Some(kind) = src.kind {
            let kind_value = match kind.to_lowercase().as_str() {
                "insurance" => 1,
                "self-pay" => 2,
                "other" => 3,
                _ => 0,
            };

            dest.kind = Some(KindCode {
                value: kind_value,
                id: None,
                extension: Vec::new(),
            });
        }

        // ------------------------------------------------------------------
        // 5. Type  ----------------------------------------------------------
        if let Some(coverage_type) = src.r#type {
            dest.r#type = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.type_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/v3-ActCode".to_string(),
                        ..Default::default()
                    })),
                    code: src.type_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.type_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: coverage_type,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 6. Policy Holder  -------------------------------------------------
        if let Some(policy_holder_id) = src.policy_holder_id {
            let policy_holder_type = src.policy_holder_type.unwrap_or_else(|| "Patient".to_string());
            dest.policy_holder = Some(Reference {
                reference: Some(String {
                    value: format!("{}/{}", policy_holder_type, policy_holder_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 7. Subscriber  ----------------------------------------------------
        if let Some(subscriber_id) = src.subscriber_id {
            let subscriber_type = src.subscriber_type.unwrap_or_else(|| "Patient".to_string());
            dest.subscriber = Some(Reference {
                reference: Some(String {
                    value: format!("{}/{}", subscriber_type, subscriber_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 8. Subscriber ID  -------------------------------------------------
        if let Some(subscriber_identifier) = src.subscriber_identifier {
            dest.subscriber_id.push(Identifier {
                value: Some(String {
                    value: subscriber_identifier,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 9. Beneficiary  ---------------------------------------------------
        if let Some(beneficiary_id) = src.beneficiary_id {
            dest.beneficiary = Some(Reference {
                reference: Some(String {
                    value: format!("Patient/{}", beneficiary_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 10. Dependent  -----------------------------------------------------
        if let Some(dependent_number) = src.dependent_number {
            dest.dependent = Some(String {
                value: dependent_number,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 11. Relationship  -------------------------------------------------
        if let Some(relationship) = src.relationship {
            dest.relationship = Some(CodeableConcept {
                coding: vec![Coding {
                    system: src.relationship_system.map(|system| Uri {
                        value: system,
                        ..Default::default()
                    }).or_else(|| Some(Uri {
                        value: "http://terminology.hl7.org/CodeSystem/subscriber-relationship".to_string(),
                        ..Default::default()
                    })),
                    code: src.relationship_code.map(|code| Code {
                        value: code,
                        ..Default::default()
                    }),
                    display: src.relationship_display.map(|display| String {
                        value: display,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                text: Some(String {
                    value: relationship,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 12. Period  --------------------------------------------------------
        if src.period_start.is_some() || src.period_end.is_some() {
            let mut period = Period {
                ..Default::default()
            };

            if let Some(period_start) = src.period_start {
                if let Ok(dt) = DateTime::parse_from_rfc3339(&period_start) {
                    let fhir_dt = dt.with_timezone(&Utc);
                    period.start = Some(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    });
                }
            }

            if let Some(period_end) = src.period_end {
                if let Ok(dt) = DateTime::parse_from_rfc3339(&period_end) {
                    let fhir_dt = dt.with_timezone(&Utc);
                    period.end = Some(FhirDateTime {
                        value_us: fhir_dt.timestamp_micros(),
                        ..Default::default()
                    });
                }
            }

            dest.period = Some(period);
        }

        // ------------------------------------------------------------------
        // 13. Insurer  -------------------------------------------------------
        if let Some(insurer_id) = src.insurer_id {
            dest.insurer = Some(Reference {
                reference: Some(String {
                    value: format!("Organization/{}", insurer_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 14. Network  -------------------------------------------------------
        if let Some(network) = src.network {
            dest.network = Some(String {
                value: network,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 15. Class  ---------------------------------------------------------
        if let Some(class_types) = src.class_types {
            for (i, class_type) in class_types.iter().enumerate() {
                let mut class_item = Class {
                    ..Default::default()
                };

                // Type
                class_item.r#type = Some(CodeableConcept {
                    text: Some(String {
                        value: class_type.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                // Value
                if let Some(class_values) = &src.class_values {
                    if let Some(class_value) = class_values.get(i) {
                        class_item.value = Some(Identifier {
                            value: Some(String {
                                value: class_value.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                // Name
                if let Some(class_names) = &src.class_names {
                    if let Some(class_name) = class_names.get(i) {
                        class_item.name = Some(String {
                            value: class_name.clone(),
                            ..Default::default()
                        });
                    }
                }

                dest.class_value.push(class_item);
            }
        }

        // ------------------------------------------------------------------
        // 16. Order  ---------------------------------------------------------
        if let Some(order) = src.order {
            dest.order = Some(PositiveInt {
                value: order,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 17. Cost to Beneficiary  -------------------------------------------
        if let Some(cost_types) = src.cost_types {
            for (i, cost_type) in cost_types.iter().enumerate() {
                let mut cost_item = CostToBeneficiary {
                    ..Default::default()
                };

                // Type
                cost_item.r#type = Some(CodeableConcept {
                    text: Some(String {
                        value: cost_type.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                // Category
                if let Some(cost_categories) = &src.cost_categories {
                    if let Some(cost_category) = cost_categories.get(i) {
                        cost_item.category = Some(CodeableConcept {
                            text: Some(String {
                                value: cost_category.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                // Network
                if let Some(cost_networks) = &src.cost_networks {
                    if let Some(cost_network) = cost_networks.get(i) {
                        cost_item.network = Some(CodeableConcept {
                            text: Some(String {
                                value: cost_network.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                // Unit
                if let Some(cost_units) = &src.cost_units {
                    if let Some(cost_unit) = cost_units.get(i) {
                        cost_item.unit = Some(CodeableConcept {
                            text: Some(String {
                                value: cost_unit.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                // Term
                if let Some(cost_terms) = &src.cost_terms {
                    if let Some(cost_term) = cost_terms.get(i) {
                        cost_item.term = Some(CodeableConcept {
                            text: Some(String {
                                value: cost_term.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                    }
                }

                // Value
                if let Some(cost_values) = &src.cost_values {
                    if let Some(cost_value) = cost_values.get(i) {
                        let value_type = src.cost_value_types.as_ref()
                            .and_then(|types| types.get(i))
                            .cloned()
                            .unwrap_or_else(|| "money".to_string());

                        cost_item.value = Some(ValueX {
                            choice: Some(match value_type.as_str() {
                                "quantity" => ValueChoice::Quantity(SimpleQuantity {
                                    value: cost_value.clone(),
                                    ..Default::default()
                                }),
                                "money" => ValueChoice::Money(Money {
                                    value: cost_value.clone(),
                                    ..Default::default()
                                }),
                                _ => ValueChoice::Money(Money {
                                    value: cost_value.clone(),
                                    ..Default::default()
                                }),
                            }),
                        });
                    }
                }

                dest.cost_to_beneficiary.push(cost_item);
            }
        }

        // ------------------------------------------------------------------
        // 18. Payment By  ----------------------------------------------------
        if let Some(payment_by_party_ids) = src.payment_by_party_ids {
            for (i, party_id) in payment_by_party_ids.iter().enumerate() {
                let party_type = src.payment_by_party_types.as_ref()
                    .and_then(|types| types.get(i))
                    .cloned()
                    .unwrap_or_else(|| "Patient".to_string());

                let mut payment_by = PaymentBy {
                    ..Default::default()
                };

                payment_by.party = Some(Reference {
                    reference: Some(String {
                        value: format!("{}/{}", party_type, party_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                if let Some(responsibilities) = &src.payment_by_responsibilities {
                    if let Some(responsibility) = responsibilities.get(i) {
                        payment_by.responsibility = Some(String {
                            value: responsibility.clone(),
                            ..Default::default()
                        });
                    }
                }

                dest.payment_by.push(payment_by);
            }
        }

        // ------------------------------------------------------------------
        // 19. Subrogation  ---------------------------------------------------
        if let Some(subrogation) = src.subrogation {
            dest.subrogation = Some(Boolean {
                value: subrogation,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 20. Contract  ------------------------------------------------------
        if let Some(contract_ids) = src.contract_ids {
            for contract_id in contract_ids {
                dest.contract.push(Reference {
                    reference: Some(String {
                        value: format!("Contract/{}", contract_id),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
            }
        }

        // ------------------------------------------------------------------
        // 21. Insurance Plan  ------------------------------------------------
        if let Some(insurance_plan_id) = src.insurance_plan_id {
            dest.insurance_plan = Some(Reference {
                reference: Some(String {
                    value: format!("InsurancePlan/{}", insurance_plan_id),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        dest
    }
}
