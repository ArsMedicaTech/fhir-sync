use chrono::NaiveDate;
//use prost::Message;

use crate::domain::patient::DomainPatient;

// Generated modules ---------------------------------------------
// (path depends on how you configured `tonic_build`)
use crate::proto::google::fhir::proto::r5::core::{
    Patient, // proto message we’re producing
    Id,
    Identifier,
    Uri,
    String,
    HumanName,
    ContactPoint,
    Date,
    Address,
};

use crate::proto::google::fhir::proto::r5::core::contact_point_system_code::Value as ContactPointSystemCode;
use crate::proto::google::fhir::proto::r5::core::administrative_gender_code::Value as AdministrativeGenderCode;
use crate::proto::google::fhir::proto::r5::core::contact_point::SystemCode;

use crate::proto::google::fhir::proto::r5::core::patient;

// Shorthand for nested message that lives *inside* Patient.
type GenderCode = patient::GenderCode;

impl From<DomainPatient> for Patient {
    fn from(src: DomainPatient) -> Self {
        // Start with a completely empty message
        let mut dest = Patient::default();

        // ------------------------------------------------------------------
        // 1. Logical ID  ----------------------------------------------------
        dest.id = Some(Id {
            value: src.demographic_no.clone(),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 2. Identifier  ----------------------------------------------------
        dest.identifier.push(Identifier {
            system: Some(Uri {
                value: "urn:arsmedicatech:demographic_no".to_string(),
                ..Default::default()
            }),
            value: Some(String {
                value: src.demographic_no.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });

        // ------------------------------------------------------------------
        // 3. Name  ----------------------------------------------------------
        if src.first_name.is_some() || src.last_name.is_some() {
            let mut name = HumanName::default();

            if let Some(fam) = src.last_name {
                name.family = Some(String { value: fam, ..Default::default() });
            }
            if let Some(giv) = src.first_name {
                name.given.push(String { value: giv, ..Default::default() });
            }

            dest.name.push(name);
        }

        // ------------------------------------------------------------------
        // 4. Telecom  (phone & e-mail) --------------------------------------
        if let Some(phone) = src.phone {
            dest.telecom.push(ContactPoint {
                system: Some(SystemCode {
                    value: ContactPointSystemCode::Phone as i32,
                    ..Default::default()
                }),
                value: Some(String { value: phone, ..Default::default() }),
                ..Default::default()
            });
        }
        if let Some(email) = src.email {
            dest.telecom.push(ContactPoint {
                system: Some(SystemCode {
                    value: ContactPointSystemCode::Email as i32,
                    ..Default::default()
                }),
                value: Some(String { value: email, ..Default::default() }),
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 5. Gender  --------------------------------------------------------
        if let Some(sex_raw) = src.sex {
            let code_val = match sex_raw.to_lowercase().as_str() {
                "male" | "m"      => AdministrativeGenderCode::Male,
                "female" | "f"    => AdministrativeGenderCode::Female,
                "other"           => AdministrativeGenderCode::Other,
                _                 => AdministrativeGenderCode::Unknown,
            };

            dest.gender = Some(GenderCode {
                value: code_val as i32,
                ..Default::default()
            });
        }

        // ------------------------------------------------------------------
        // 6. Birth date  ----------------------------------------------------
        if let Some(dob) = src.date_of_birth {
            if let Ok(date) = NaiveDate::parse_from_str(&dob, "%Y-%m-%d") {
                // FHIR primitive Date uses microseconds since Unix epoch
                let us = date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp() * 1_000_000;
                dest.birth_date = Some(Date { value_us: us, ..Default::default() });
            }
        }

        // ------------------------------------------------------------------
        // 7. Address  -------------------------------------------------------
        if let Some((city, province, country, postal)) = src.location {
            dest.address.push(Address {
                city:        Some(String { value: city,      ..Default::default() }),
                state:       Some(String { value: province,  ..Default::default() }),
                country:     Some(String { value: country,   ..Default::default() }),
                postal_code: Some(String { value: postal,    ..Default::default() }),
                ..Default::default()
            });
        }

        dest
    }
}
