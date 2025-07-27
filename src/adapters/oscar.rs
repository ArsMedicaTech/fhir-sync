use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct OscarPatient {
    pub demographic_no: String,
    pub first_name:    Option<String>,
    pub last_name:     Option<String>,
    pub date_of_birth: Option<String>, // ISO "YYYY-MM-DD"
    pub location:      Option<(String, String, String, String)>, // city, province, country, postal
    pub sex:           Option<String>, // "male" | "female" | "other" | ...
    pub phone:         Option<String>,
    pub email:         Option<String>,
}
