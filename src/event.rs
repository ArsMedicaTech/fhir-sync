use chrono::{DateTime, Utc};

use crate::domain::resource::DomainResource;

/// Where a `SyncEvent` originated. See D1 in TASK_FEATURES_SPEC_OSCAR_1.md.
///
/// The `OscarBinlog` and `OscarBackfill` variants carry the source Oscar table
/// name, which is needed for the `oscar:{table}:{source_id}` idempotency-key
/// convention.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    OscarBinlog { table: String },
    OscarBackfill { table: String },
    Webhook,
    Grpc,
    FhirHistory,
}

impl Source {
    pub fn as_str(&self) -> &'static str {
        match self {
            Source::OscarBinlog { .. } => "oscar_binlog",
            Source::OscarBackfill { .. } => "oscar_backfill",
            Source::Webhook => "webhook",
            Source::Grpc => "grpc",
            Source::FhirHistory => "fhir_history",
        }
    }

    /// Returns the source Oscar table when this is a binlog or backfill event.
    pub fn table(&self) -> Option<&str> {
        match self {
            Source::OscarBinlog { table } | Source::OscarBackfill { table } => Some(table),
            _ => None,
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
    Practitioner,
    Appointment,
    Encounter,
    DocumentReference,
    Condition,
    FamilyMemberHistory,
    CareTeam,
}

impl ResourceType {
    /// FHIR URL path segment for this resource type.
    pub fn as_path(&self) -> &'static str {
        match self {
            ResourceType::Patient => "Patient",
            ResourceType::Practitioner => "Practitioner",
            ResourceType::Appointment => "Appointment",
            ResourceType::Encounter => "Encounter",
            ResourceType::DocumentReference => "DocumentReference",
            ResourceType::Condition => "Condition",
            ResourceType::FamilyMemberHistory => "FamilyMemberHistory",
            ResourceType::CareTeam => "CareTeam",
        }
    }

    /// Lower-case name used in idempotency and dispatch payloads where required.
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceType::Patient => "Patient",
            ResourceType::Practitioner => "Practitioner",
            ResourceType::Appointment => "Appointment",
            ResourceType::Encounter => "Encounter",
            ResourceType::DocumentReference => "DocumentReference",
            ResourceType::Condition => "Condition",
            ResourceType::FamilyMemberHistory => "FamilyMemberHistory",
            ResourceType::CareTeam => "CareTeam",
        }
    }
}

/// Canonical multi-resource event envelope (D1).
///
/// Construction must go through `SyncEvent::new` so that `resource_type` and
/// `idempotency_key` always agree with `payload`. Fields are `pub(crate)` so
/// tests can read them but external modules cannot construct inconsistent
/// events.
#[derive(Debug, Clone)]
pub struct SyncEvent {
    pub(crate) source: Source,
    pub(crate) op: Op,
    pub(crate) resource_type: ResourceType,
    pub(crate) idempotency_key: String,
    pub(crate) payload: DomainResource,
    pub(crate) occurred_at: DateTime<Utc>,
}

impl SyncEvent {
    /// Creates a `SyncEvent`, deriving `resource_type` and `idempotency_key`
    /// from the `payload` and `source`.
    ///
    /// Idempotency-key convention:
    /// - `oscar:{table}:{resource_type}:{source_id}` for streaming
    /// - `oscar:{table}:backfill:{resource_type}:{source_id}` for backfill
    /// - `webhook:{source_table}:{source_id}`, `grpc:{source_table}:{source_id}`,
    ///   `fhir_history:{source_table}:{source_id}` for other sources.
    ///
    /// `resource_type` is included because a single Oscar row (e.g.
    /// `casemgmt_note`) can produce multiple FHIR resource types keyed on the
    /// same natural id.
    pub fn new(source: Source, op: Op, payload: DomainResource, occurred_at: DateTime<Utc>) -> Self {
        let resource_type = payload.resource_type();
        let source_id = payload.source_id();
        let rt = resource_type.as_str();
        let idempotency_key = match &source {
            Source::OscarBinlog { table } => format!("oscar:{table}:{rt}:{source_id}"),
            Source::OscarBackfill { table } => format!("oscar:{table}:backfill:{rt}:{source_id}"),
            Source::Webhook => format!("webhook:{}:{source_id}", payload.source_table()),
            Source::Grpc => format!("grpc:{}:{source_id}", payload.source_table()),
            Source::FhirHistory => format!("fhir_history:{}:{source_id}", payload.source_table()),
        };

        Self {
            source,
            op,
            resource_type,
            idempotency_key,
            payload,
            occurred_at,
        }
    }

    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn op(&self) -> Op {
        self.op
    }

    pub fn resource_type(&self) -> ResourceType {
        self.resource_type
    }

    pub fn idempotency_key(&self) -> &str {
        &self.idempotency_key
    }

    pub fn payload(&self) -> &DomainResource {
        &self.payload
    }

    pub fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}
