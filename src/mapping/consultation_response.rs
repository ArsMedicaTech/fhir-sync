use std::collections::HashMap;

use anyhow::{Context, Result};
use mysql_async::prelude::*;
use mysql_async::Row;
use tracing::{warn};

use crate::config::DatabaseConfig;
use crate::domain::diagnostic_report::DomainDiagnosticReport;
use crate::mapping::syncable_provider;
use crate::sources::RowChange;

pub type ColumnMap = HashMap<String, usize>;

fn lookup<'a>(change: &'a RowChange, columns: &ColumnMap, name: &str) -> Option<&'a str> {
    let idx = *columns.get(name)?;
    change
        .after
        .get(idx)
        .and_then(|v| v.as_deref())
        .filter(|s| !s.is_empty())
}

fn lookup_any<'a>(change: &'a RowChange, columns: &ColumnMap, names: &[&str]) -> Option<&'a str> {
    names.iter().find_map(|n| lookup(change, columns, n))
}

/// Maps one `consultationResponse` row to a `DomainDiagnosticReport`.
///
/// This requires a live correlation query against `consultationRequests` and
/// `consultationRequestExt` to resolve `basedOn`. If the originating request
/// cannot be found unambiguously, `Ok(None)` is returned.
pub async fn row_to_domain_diagnostic_report(
    change: &RowChange,
    columns: &ColumnMap,
    db: &DatabaseConfig,
) -> Result<Option<DomainDiagnosticReport>> {
    let response_id = lookup_any(change, columns, &["responseId", "response_id"]).map(|s| s.to_string());
    let demographic_no = lookup_any(change, columns, &["demographicNo", "demographic_no"]).map(|s| s.to_string());
    let referral_date = lookup_any(change, columns, &["referralDate", "referral_date"]).map(|s| s.to_string());
    let provider_no = syncable_provider(lookup_any(change, columns, &["providerNo", "provider_no"]));
    let response_date = lookup_any(change, columns, &["responseDate", "response_date"]).map(|s| s.to_string());
    let status = lookup_any(change, columns, &["status"]).map(|s| s.trim().to_string());
    let examination = lookup_any(change, columns, &["examination"]).map(|s| s.to_string());
    let impression = lookup_any(change, columns, &["impression"]).map(|s| s.to_string());
    let plan = lookup_any(change, columns, &["plan"]).map(|s| s.to_string());
    let referral_reason = lookup_any(change, columns, &["referralReason", "referral_reason"]).map(|s| s.to_string());

    let response_id = match response_id {
        Some(id) => id,
        None => {
            warn!("consultation_response mapping: skipping row with no responseId");
            return Ok(None);
        }
    };
    let demographic_no = match demographic_no {
        Some(d) => d,
        None => {
            warn!("consultation_response mapping: skipping response_id={response_id} (no demographicNo)");
            return Ok(None);
        }
    };

    let based_on = match resolve_based_on(db, &demographic_no, referral_date.as_deref()).await? {
        Some(id) => Some(id),
        None => {
            // No/ambiguous originating request already logged.
            return Ok(None);
        }
    };

    Ok(Some(DomainDiagnosticReport {
        response_id,
        demographic_no,
        provider_no,
        response_date,
        referral_date,
        status,
        examination,
        impression,
        plan,
        referral_reason,
        based_on,
    }))
}

async fn resolve_based_on(
    db: &DatabaseConfig,
    demographic_no: &str,
    referral_date: Option<&str>,
) -> Result<Option<String>> {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db.user, db.password, db.host, db.port, db.schema
    );
    let pool = mysql_async::Pool::new(url.as_str());
    let mut conn = pool
        .get_conn()
        .await
        .context("connecting to resolve originating consultation request")?;

    let request_rows: Vec<(i64,)> = if let Some(date) = referral_date {
        conn.exec(
            "SELECT requestId FROM consultationRequests \
             WHERE demographicNo = :demographic_no \
               AND source = 'AMT-eReferral' \
               AND referalDate = :referral_date \
             ORDER BY requestId DESC",
            params! {
                "demographic_no" => demographic_no,
                "referral_date" => date,
            },
        )
        .await
        .context("selecting originating consultation request by referral date")?
    } else {
        conn.exec(
            "SELECT requestId FROM consultationRequests \
             WHERE demographicNo = :demographic_no \
               AND source = 'AMT-eReferral' \
             ORDER BY requestId DESC",
            params! {
                "demographic_no" => demographic_no,
            },
        )
        .await
        .context("selecting originating consultation request without referral date")?
    };

    if request_rows.is_empty() {
        warn!(
            "consultation_response mapping: no AMT-eReferral consultationRequest for demographic_no={demographic_no} referalDate={referral_date:?}"
        );
        drop(conn);
        let _ = pool.disconnect().await;
        return Ok(None);
    }

    if request_rows.len() > 1 {
        let ids: Vec<String> = request_rows.iter().map(|(id,)| id.to_string()).collect();
        warn!(
            "consultation_response mapping: ambiguous originating consultationRequest for demographic_no={demographic_no} referalDate={referral_date:?}: {ids:?}"
        );
        drop(conn);
        let _ = pool.disconnect().await;
        return Ok(None);
    }

    let request_id = request_rows[0].0;

    let ext_rows: Vec<Row> = conn
        .exec(
            "SELECT `value` FROM consultationRequestExt \
             WHERE requestId = :request_id AND `key` = 'amt.fhirServiceRequestId'",
            params! {
                "request_id" => request_id,
            },
        )
        .await
        .context("selecting amt.fhirServiceRequestId ext row")?;

    drop(conn);
    let _ = pool.disconnect().await;

    if ext_rows.is_empty() {
        warn!(
            "consultation_response mapping: no amt.fhirServiceRequestId ext row for request_id={request_id}"
        );
        return Ok(None);
    }

    let fhir_service_request_id: Option<String> =
        ext_rows.first().and_then(|r| r.get(0));

    match fhir_service_request_id.as_deref().map(str::trim) {
        Some(id) if !id.is_empty() => Ok(Some(format!("ServiceRequest/{id}"))),
        _ => {
            warn!(
                "consultation_response mapping: empty amt.fhirServiceRequestId for request_id={request_id}"
            );
            Ok(None)
        }
    }
}
