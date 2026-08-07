//! Pure FHIR → Oscar row mappers.
//!
//! These functions are designed to be unit-testable without a database.

pub mod appointment;
pub mod note;
pub mod patient;

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
        }
    }
}

impl std::error::Error for MappingError {}
