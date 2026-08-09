use std::collections::HashMap;

use anyhow::{Context, Result};
use mysql_async::prelude::*;
use tracing::{info, warn};

use crate::config::DatabaseConfig;
use crate::domain::service_request::DomainServiceRequest;
use crate::mapping::syncable_provider;
use crate::sources::RowChange;

/// Column name -> zero-based index in `RowChange.after`, as resolved from
/// `information_schema.columns` at startup (D3). Never hardcode indexes.
pub type ColumnMap = HashMap<String, usize>;

fn lookup<'a>(change: &'a RowChange, columns: &ColumnMap, name: &str) -> Option<&'a str> {
    let idx = *columns.get(name)?;
    change
        .after
        .get(idx)
        .and_then(|v| v.as_deref())
        .filter(|s| !s.is_empty())
}

/// Maps one `consultationRequests` row to a `DomainServiceRequest`.
///
/// This is the reverse of `writeback/mappers/service_request.rs`: AMT
/// originates the `ServiceRequest`, but Oscar's own consult workflow (a
/// human changing the request's Status in Oscar's UI) needs to resync back
/// into AMT so the corresponding task reflects progress/completion there.
///
/// Oscar does not update the AMT-originated row in place when its status
/// changes -- "Update Consultation Request" always inserts a *new*
/// `consultationRequests` row with a blank `source` (confirmed empirically:
/// completing a request spawns a sibling row, not a mutation). So the
/// changed row itself is never the one carrying `source = 'AMT-eReferral'`
/// or the `amt.fhirServiceRequestId` ext link -- those live on the original
/// row. This function therefore ignores the changed row's own `source` and
/// instead resolves the target `ServiceRequest` by correlating on
/// `demographicNo` against `consultationRequestExt`, mirroring the D1
/// heuristic in `consultation_response.rs`'s `resolve_based_on`: exactly one
/// AMT-originated request for that patient must exist, or the row is
/// skipped (dead-lettered via `Ok(None)`, never guessed).
///
/// The resulting `DomainServiceRequest.request_id` is the *original*
/// AMT-originated `requestId` (which is what the FHIR identifier was built
/// from at write-back time) -- not the id of the row that actually changed.
/// Status/reason/etc. are taken from the changed row, since that carries the
/// current real-world state.
pub async fn row_to_domain_service_request(
    change: &RowChange,
    columns: &ColumnMap,
    db: &DatabaseConfig,
) -> Result<Option<DomainServiceRequest>> {
    let changed_request_id = match lookup(change, columns, "requestId") {
        Some(id) => id.to_string(),
        None => {
            info!("consultationRequests mapping: skipping row with no requestId");
            return Ok(None);
        }
    };

    let demographic_no = match lookup(change, columns, "demographicNo") {
        Some(d) => d.to_string(),
        None => {
            info!("consultationRequests mapping: skipping requestId={changed_request_id} (no demographicNo)");
            return Ok(None);
        }
    };

    let amt_request_id = match resolve_amt_request_id(db, &demographic_no).await? {
        Some(id) => id,
        None => return Ok(None), // no/ambiguous AMT lineage already logged
    };

    let provider_no = syncable_provider(lookup(change, columns, "providerNo"));

    Ok(Some(DomainServiceRequest {
        request_id: amt_request_id,
        demographic_no: Some(demographic_no),
        provider_no,
        reason: lookup(change, columns, "reason").map(str::to_string),
        clinical_info: lookup(change, columns, "clinicalInfo").map(str::to_string),
        referal_date: lookup(change, columns, "referalDate").map(str::to_string),
        urgency: lookup(change, columns, "urgency").map(str::to_string),
        status: lookup(change, columns, "status").map(str::to_string),
    }))
}

/// Finds the single AMT-originated `consultationRequests.requestId` for a
/// patient, i.e. the row `write_consultation_request` created and tagged
/// with `source = 'AMT-eReferral'` and an `amt.fhirServiceRequestId` ext
/// row. Returns `Ok(None)` (already logged) if there are zero or more than
/// one candidates -- never guesses which lineage a status change belongs to.
async fn resolve_amt_request_id(db: &DatabaseConfig, demographic_no: &str) -> Result<Option<String>> {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db.user, db.password, db.host, db.port, db.schema
    );
    let pool = mysql_async::Pool::new(url.as_str());
    let mut conn = pool
        .get_conn()
        .await
        .context("connecting to resolve AMT-originated consultation request")?;

    let rows: Vec<(i64,)> = conn
        .exec(
            "SELECT DISTINCT cr.requestId \
             FROM consultationRequests cr \
             JOIN consultationRequestExt ext \
               ON ext.requestId = cr.requestId AND ext.name = 'amt.fhirServiceRequestId' \
             WHERE cr.demographicNo = :demographic_no AND cr.source = 'AMT-eReferral'",
            params! { "demographic_no" => demographic_no },
        )
        .await
        .context("selecting AMT-originated consultation request")?;

    drop(conn);
    let _ = pool.disconnect().await;

    if rows.is_empty() {
        info!(
            "consultationRequests mapping: no AMT-originated consultationRequest for demographic_no={demographic_no}; skipping"
        );
        return Ok(None);
    }

    if rows.len() > 1 {
        let ids: Vec<String> = rows.iter().map(|(id,)| id.to_string()).collect();
        warn!(
            "consultationRequests mapping: ambiguous AMT-originated consultationRequest for demographic_no={demographic_no}: {ids:?}; skipping"
        );
        return Ok(None);
    }

    Ok(Some(rows[0].0.to_string()))
}
