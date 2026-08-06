/// Oscar-side representation of the CareTeam that links a patient to their
/// most-responsible provider (MRP). See `TASK_FEATURES_SPEC_CARE_TEAM.md`.
#[derive(Debug, Clone)]
pub struct DomainCareTeam {
    pub demographic_no: String,
    pub provider_no: String,
}
