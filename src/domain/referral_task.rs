#[derive(Debug, Clone)]
pub struct DomainReferralTask {
    pub request_id: String,
    pub demographic_no: String,
    pub provider_no: Option<String>,
    pub oscar_status: String,
}
