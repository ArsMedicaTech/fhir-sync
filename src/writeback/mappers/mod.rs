//! Pure FHIR → Oscar row mappers.
//!
//! These functions are designed to be unit-testable without a database.

pub mod appointment;
pub mod note;
pub mod patient;
pub mod service_request;

pub use appointment::{AppointmentRow, fhir_appointment_to_row};
pub use note::{NoteRow, fhir_document_reference_to_row};
pub use patient::{DemographicRow, fhir_patient_to_row};

use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum MappingError {
    UnmappedGender(String),
    UnmappedAppointmentStatus(String),
    NonexistentLocalTime(String),
    AmbiguousLocalTime(String),
    MissingField(String),
    InvalidValue { field: String, value: String },
    PlaceholderPatient,
    MergeTombstone,
}

impl fmt::Display for MappingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MappingError::UnmappedGender(v) => write!(f, "unmapped gender value: {v}"),
            MappingError::UnmappedAppointmentStatus(v) => {
                write!(f, "unmapped appointment status: {v}")
            }
            MappingError::NonexistentLocalTime(v) => write!(f, "nonexistent local time: {v}"),
            MappingError::AmbiguousLocalTime(v) => write!(f, "ambiguous local time: {v}"),
            MappingError::MissingField(v) => write!(f, "missing required field: {v}"),
            MappingError::InvalidValue { field, value } => {
                write!(f, "invalid value for {field}: {value}")
            }
            MappingError::PlaceholderPatient => {
                write!(f, "demographic_no 0 is not a valid patient")
            }
            MappingError::MergeTombstone => {
                write!(f, "patient has a link element (merge tombstone)")
            }
        }
    }
}

impl std::error::Error for MappingError {}
