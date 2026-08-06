pub mod appointment;
pub mod care_team;
pub mod casemgmt_note;
pub mod casemgmt_note_ext;
pub mod demographic;
pub mod dxresearch;
pub mod provider;

/// Oscar's system actor (`provider_no = '-1'`) is never synced as a
/// Practitioner (D3/D5), so any FHIR reference to it is unsatisfiable and
/// fails at the sink with HAPI-1091. Every mapper that reads a provider
/// column routes it through here rather than re-implementing the check.
///
/// Also drops empty strings: several provider-bearing columns in real Oscar
/// are `NOT NULL DEFAULT ''`.
pub fn syncable_provider(raw: Option<&str>) -> Option<String> {
    match raw.map(str::trim) {
        None | Some("") | Some("-1") => None,
        Some(v) => Some(v.to_string()),
    }
}
