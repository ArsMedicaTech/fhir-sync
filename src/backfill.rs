//! `--backfill` snapshot mode.
//!
//! Captures the current binlog position *before* scanning and persists it
//! as the checkpoint immediately, so the streaming run that follows résumes
//! from that position rather than "now" — nothing written during the scan
//! is missed, and nothing already covered by the snapshot is re-read.
//! Batch-SELECTs the entire `demographic` table through the same sink path
//! as live CDC (`row_to_domain_patient` -> `SyncEvent` -> conditional PUT),
//! so it is idempotent and safe to re-run (spec acceptance: running twice
//! changes nothing).

use anyhow::{Context, Result};
use mysql_async::{prelude::*, Row, Value};
use tokio::sync::mpsc::Sender;
use tracing::{info, warn};

use crate::checkpoint::{self, Checkpoint};
use crate::config::Config;
use crate::domain::resource::DomainResource;
use crate::event::{Op, Source as EventSource, SyncEvent};
use crate::mapping::demographic::{row_to_domain_patient, ColumnMap};
use crate::metrics::SharedMetrics;
use crate::sources::mariadb_binlog;
use crate::sources::{RowChange, RowOp, SourcePosition};

const DEMOGRAPHIC_TABLE: &str = "demographic";
const BATCH_SIZE: u64 = 500;

/// Runs one backfill pass over `demographic`, sending every row through
/// `tx` as an `Upsert` `SyncEvent`. Returns the number of patients sent.
pub async fn run(
    cfg: &Config,
    columns: &ColumnMap,
    tx: &Sender<SyncEvent>,
    metrics: &SharedMetrics,
) -> Result<usize> {
    let db = &cfg.database;

    // Capture + persist the pre-snapshot position first (D6) — if the scan
    // is interrupted partway, the next streaming run still starts from
    // before the scan began rather than losing the gap entirely.
    let (filename, position) = mariadb_binlog::capture_binlog_position(db).await?;
    info!("backfill: captured pre-snapshot position {filename}:{position}");
    checkpoint::save(
        &cfg.sync.checkpoint_path,
        &Checkpoint {
            binlog_filename: filename,
            binlog_position: position,
        },
    )?;

    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db.user, db.password, db.host, db.port, db.schema
    );
    let pool = mysql_async::Pool::new(url.as_str());
    let mut conn = pool
        .get_conn()
        .await
        .context("connecting for backfill scan")?;

    let mut offset: u64 = 0;
    let mut total = 0usize;

    loop {
        let sql = format!(
            "SELECT * FROM {DEMOGRAPHIC_TABLE} ORDER BY demographic_no LIMIT {BATCH_SIZE} OFFSET {offset}"
        );
        let rows: Vec<Row> = conn
            .query(sql)
            .await
            .context("scanning demographic batch")?;

        if rows.is_empty() {
            break;
        }

        let batch_len = rows.len() as u64;

        for row in rows {
            let after: Vec<Option<String>> = row
                .unwrap()
                .into_iter()
                .map(|v| mysql_value_to_string(&v))
                .collect();

            let change = RowChange {
                schema: db.schema.clone(),
                table: DEMOGRAPHIC_TABLE.to_string(),
                op: RowOp::Insert,
                after,
                position: SourcePosition::FilePos {
                    file: String::new(),
                    pos: 0,
                },
            };

            let Some(patient) = row_to_domain_patient(&change, columns) else {
                continue;
            };

            let sync_event = SyncEvent::new(
                EventSource::OscarBackfill { table: DEMOGRAPHIC_TABLE.to_string() },
                Op::Upsert,
                DomainResource::Patient(patient),
                chrono::Utc::now(),
            );

            metrics.inc_received();
            if tx.send(sync_event).await.is_err() {
                warn!("backfill: sink channel closed mid-scan, stopping early");
                drop(conn);
                let _ = pool.disconnect().await;
                return Ok(total);
            }
            total += 1;
        }

        offset += batch_len;
    }

    drop(conn);
    let _ = pool.disconnect().await;

    info!("backfill: complete, {total} patients sent to sink");
    Ok(total)
}

/// Converts a raw `mysql_async::Value` to its string form, mirroring
/// `mariadb_binlog::column_value_to_string` for the binlog path — the two
/// must treat the same underlying columns consistently.
fn mysql_value_to_string(value: &Value) -> Option<String> {
    match value {
        Value::NULL => None,
        Value::Bytes(b) => Some(String::from_utf8_lossy(b).into_owned()),
        Value::Int(i) => Some(i.to_string()),
        Value::UInt(u) => Some(u.to_string()),
        Value::Float(f) => Some(f.to_string()),
        Value::Double(d) => Some(d.to_string()),
        Value::Date(year, month, day, hour, minute, second, micro) => {
            if *hour == 0 && *minute == 0 && *second == 0 && *micro == 0 {
                Some(format!("{year:04}-{month:02}-{day:02}"))
            } else {
                Some(format!(
                    "{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}"
                ))
            }
        }
        Value::Time(negative, days, hours, minutes, seconds, _micro) => {
            let sign = if *negative { "-" } else { "" };
            Some(format!("{sign}{days}d{hours:02}:{minutes:02}:{seconds:02}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mysql_value_to_string_formats_common_variants() {
        assert_eq!(mysql_value_to_string(&Value::NULL), None);
        assert_eq!(
            mysql_value_to_string(&Value::Bytes(b"Alice".to_vec())),
            Some("Alice".to_string())
        );
        assert_eq!(mysql_value_to_string(&Value::Int(1990)), Some("1990".to_string()));
        assert_eq!(
            mysql_value_to_string(&Value::Date(1990, 3, 5, 0, 0, 0, 0)),
            Some("1990-03-05".to_string())
        );
    }
}
