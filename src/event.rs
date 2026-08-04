use chrono::{DateTime, Utc};

use crate::domain::patient::DomainPatient;

/// Where a `SyncEvent` originated. See D1 in TASK_FEATURES_SPEC_OSCAR_SYNC.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    OscarBinlog,
    Webhook,
    Grpc,
    FhirHistory,
}

impl Source {
    pub fn as_str(&self) -> &'static str {
        match self {
            Source::OscarBinlog => "oscar_binlog",
            Source::Webhook => "webhook",
            Source::Grpc => "grpc",
            Source::FhirHistory => "fhir_history",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Upsert,
    Delete,
}

impl Op {
    pub fn as_str(&self) -> &'static str {
        match self {
            Op::Upsert => "upsert",
            Op::Delete => "delete",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Patient,
}

/// Canonical event envelope. Replaces the old ad-hoc `Event` enum
/// (`PatientUpsertAMT` / `PatientUpsertOscar`) per D1/F8.
#[derive(Debug, Clone)]
pub struct SyncEvent {
    pub source: Source,
    pub op: Op,
    pub resource_type: ResourceType,
    pub idempotency_key: String,
    pub payload: DomainPatient,
    pub occurred_at: DateTime<Utc>,
}
