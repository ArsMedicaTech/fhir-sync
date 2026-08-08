use crate::domain::appointment::DomainAppointment;
use crate::domain::care_team::DomainCareTeam;
use crate::domain::condition::{DomainCondition, DomainFamilyMemberHistory};
use crate::domain::diagnostic_report::DomainDiagnosticReport;
use crate::domain::document_reference::DomainDocumentReference;
use crate::domain::encounter::DomainEncounter;
use crate::domain::patient::DomainPatient;
use crate::domain::practitioner::DomainPractitioner;
use crate::event::ResourceType;

/// Multi-resource payload carried by `SyncEvent`.
#[derive(Debug, Clone)]
pub enum DomainResource {
    Patient(DomainPatient),
    Practitioner(DomainPractitioner),
    Appointment(DomainAppointment),
    Encounter(DomainEncounter),
    DocumentReference(DomainDocumentReference),
    DiagnosticReport(DomainDiagnosticReport),
    Condition(DomainCondition),
    FamilyMemberHistory(DomainFamilyMemberHistory),
    CareTeam(DomainCareTeam),
}

impl DomainResource {
    /// Returns the FHIR resource type for this payload.
    pub fn resource_type(&self) -> ResourceType {
        match self {
            DomainResource::Patient(_) => ResourceType::Patient,
            DomainResource::Practitioner(_) => ResourceType::Practitioner,
            DomainResource::Appointment(_) => ResourceType::Appointment,
            DomainResource::Encounter(_) => ResourceType::Encounter,
            DomainResource::DocumentReference(_) => ResourceType::DocumentReference,
            DomainResource::DiagnosticReport(_) => ResourceType::DiagnosticReport,
            DomainResource::Condition(_) => ResourceType::Condition,
            DomainResource::FamilyMemberHistory(_) => ResourceType::FamilyMemberHistory,
            DomainResource::CareTeam(_) => ResourceType::CareTeam,
        }
    }

    /// Returns the Oscar-side natural key used for conditional identifiers and
    /// idempotency.
    pub fn source_id(&self) -> &str {
        match self {
            DomainResource::Patient(p) => &p.demographic_no,
            DomainResource::Practitioner(p) => &p.provider_no,
            DomainResource::Appointment(a) => &a.appointment_no,
            DomainResource::Encounter(e) => e.uuid.as_deref().unwrap_or(&e.note_id),
            DomainResource::DocumentReference(d) => d.uuid.as_deref().unwrap_or(&d.note_id),
            DomainResource::DiagnosticReport(r) => &r.response_id,
            DomainResource::Condition(c) => &c.source_id,
            DomainResource::FamilyMemberHistory(f) => &f.note_id,
            DomainResource::CareTeam(c) => &c.demographic_no,
        }
    }

    /// Returns the source Oscar table for this resource.
    pub fn source_table(&self) -> &'static str {
        match self {
            DomainResource::Patient(_) => "demographic",
            DomainResource::Practitioner(_) => "provider",
            DomainResource::Appointment(_) => "appointment",
            DomainResource::Encounter(_) => "casemgmt_note",
            DomainResource::DocumentReference(_) => "casemgmt_note",
            DomainResource::DiagnosticReport(_) => "consultationResponse",
            DomainResource::Condition(c) => match c.source_table.as_str() {
                "dxresearch" => "dxresearch",
                _ => "casemgmt_note",
            },
            DomainResource::FamilyMemberHistory(_) => "casemgmt_note",
            DomainResource::CareTeam(_) => "demographic",
        }
    }
}
