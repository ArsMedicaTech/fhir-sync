use std::collections::HashMap;
use std::sync::OnceLock;

use anyhow::{Context, Result};
use mysql_async::prelude::*;
use tracing::{info, warn};

use crate::config::{DatabaseConfig, OscarConfig};
use crate::domain::condition::DomainCondition;
use crate::sources::RowChange;

pub type ColumnMap = HashMap<String, usize>;

static DIAGNOSTIC_CODES: OnceLock<HashMap<String, String>> = OnceLock::new();

/// Loads `diagnosticcode` descriptions into a startup cache and initialises the
/// lookup table used by `row_to_domain_condition` (D7).
pub async fn load_diagnostic_codes(db: &DatabaseConfig, oscar: &OscarConfig) -> Result<()> {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db.user, db.password, db.host, db.port, db.schema
    );
    let pool = mysql_async::Pool::new(url.as_str());
    let mut conn = pool
        .get_conn()
        .await
        .context("connecting to load diagnosticcode")?;

    let rows: Vec<(String, String)> = conn
        .query("SELECT code, description FROM diagnosticcode")
        .await
        .context("selecting diagnosticcode")?;

    drop(conn);
    let _ = pool.disconnect().await;

    let mut map = HashMap::with_capacity(rows.len());
    for (code, description) in rows {
        map.insert(code, description);
    }

    info!("loaded {} diagnosticcode rows", map.len());
    let _ = DIAGNOSTIC_CODES.set(map);
    Ok(())
}

#[cfg(test)]
pub fn set_diagnostic_codes_for_test(codes: HashMap<String, String>) {
    let _ = DIAGNOSTIC_CODES.set(codes);
}

fn lookup<'a>(change: &'a RowChange, columns: &ColumnMap, name: &str) -> Option<&'a str> {
    let idx = *columns.get(name)?;
    change
        .after
        .get(idx)
        .and_then(|v| v.as_deref())
        .filter(|s| !s.is_empty())
}

/// Maps one `dxresearch` row to a `DomainCondition`.
///
/// - Empty `coding_system` produces `Condition.code.text` only, no `Coding`.
/// - `status` outside `A`/`C`/`D` yields `None` (caller should dead-letter).
/// - `D` status yields only `verificationStatus = entered-in-error` with no
///   `clinicalStatus` (R4 con-5).
pub fn row_to_domain_condition(change: &RowChange, columns: &ColumnMap) -> Option<DomainCondition> {
    let dxresearch_id = lookup(change, columns, "dxresearch_no")
        .or_else(|| lookup(change, columns, "dxresearchNo"))?;
    let demographic_no = lookup(change, columns, "demographic_no")?;
    let code = lookup(change, columns, "dxresearch_code")?;
    let coding_system = lookup(change, columns, "coding_system").map(str::to_string);
    let status = lookup(change, columns, "status")?;
    let start_date = lookup(change, columns, "start_date").map(str::to_string);
    let provider_no = lookup(change, columns, "providerNo").map(str::to_string);
    let update_date = lookup(change, columns, "update_date").map(str::to_string);

    let (clinical_status, verification_status) = match status {
        "A" => (Some("active".to_string()), Some("confirmed".to_string())),
        "C" => (Some("resolved".to_string()), Some("confirmed".to_string())),
        "D" => (None, Some("entered-in-error".to_string())),
        _ => {
            warn!("unmapped_dxresearch_status: '{}' for dxresearch_no={dxresearch_id}", status);
            return None;
        }
    };

    let normalized_code = normalize_icd9(code);
    let display = if coding_system.as_deref() == Some("icd9") {
        build_display(&normalized_code)
    } else {
        None
    };

    Some(DomainCondition {
        source_id: dxresearch_id.to_string(),
        source_table: "dxresearch".to_string(),
        demographic_no: demographic_no.to_string(),
        code: Some(normalized_code),
        coding_system,
        display,
        clinical_status,
        verification_status,
        onset_date: start_date,
        abatement_date: None,
        onset_age: None,
        note: None,
        recorded_date: update_date,
        recorder: provider_no,
        problem_description: None,
        treatment: None,
        exposure_details: None,
        hide_cpp: None,
    })
}

/// Inserts the ICD-9 decimal per D7/E6.
pub fn normalize_icd9(code: &str) -> String {
    let code = code.trim();
    if code.len() <= 3 {
        return code.to_string();
    }
    if code.starts_with('V') || code.starts_with('v') {
        return format!("{:.4}.{}", &code[..4].to_ascii_uppercase(), &code[4..]);
    }
    if code.starts_with('E') || code.starts_with('e') {
        return format!("{:.4}.{}", &code[..4].to_ascii_uppercase(), &code[4..]);
    }
    format!("{}.{}", &code[..3], &code[3..])
}

/// Reconstructs a display string by walking `diagnosticcode` descriptions up
/// the 3/4/5 character hierarchy (D7).
fn build_display(normalized_code: &str) -> Option<String> {
    let codes = DIAGNOSTIC_CODES.get()?;
    let raw: String = normalized_code.chars().filter(|c| *c != '.').collect();

    match raw.len() {
        3 => codes.get(&raw).cloned(),
        4 => {
            let base3 = &raw[..3];
            let desc3 = codes.get(base3)?;
            let desc4 = codes.get(&raw)?;
            Some(format!("{desc3} — {desc4}"))
        }
        5 => {
            let base3 = &raw[..3];
            let base4 = &raw[..4];
            let desc3 = codes.get(base3)?;
            let desc4 = codes.get(base4)?;
            let desc5 = codes.get(&raw)?;
            Some(format!("{desc3} — {desc4} — {desc5}"))
        }
        _ => None,
    }
}
