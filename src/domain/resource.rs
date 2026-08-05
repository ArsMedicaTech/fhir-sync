use crate::domain::appointment::DomainAppointment;
use crate::domain::patient::DomainPatient;
use crate::domain::practitioner::DomainPractitioner;
use crate::event::ResourceType;

/// Multi-resource payload carried by `SyncEvent`.
#[derive(Debug, Clone)]
pub enum DomainResource {
    Patient(DomainPatient),
    Practitioner(DomainPractitioner),
    Appointment(DomainAppointment),
}

impl DomainResource {
    /// Returns the FHIR resource type for this payload.
    pub fn resource_type(&self) -> ResourceType {
        match self {
            DomainResource::Patient(_) => ResourceType::Patient,
            DomainResource::Practitioner(_) => ResourceType::Practitioner,
            DomainResource::Appointment(_) => ResourceType::Appointment,
        }
    }

    /// Returns the Oscar-side natural key used for conditional identifiers and
    /// idempotency.
    pub fn source_id(&self) -> &str {
        match self {
            DomainResource::Patient(p) => &p.demographic_no,
            DomainResource::Practitioner(p) => &p.provider_no,
            DomainResource::Appointment(a) => &a.appointment_no,
        }
    }

    /// Returns the source Oscar table for this resource.
    pub fn source_table(&self) -> &'static str {
        match self {
            DomainResource::Patient(_) => "demographic",
            DomainResource::Practitioner(_) => "provider",
            DomainResource::Appointment(_) => "appointment",
        }
    }
}
