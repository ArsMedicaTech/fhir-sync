//! `--backfill` snapshot mode.
//!
//! Captures the current binlog position *before* scanning and persists it
//! as the checkpoint immediately, so the streaming run that follows resumes
//! from that position rather than "now" — nothing written during the scan
//! is missed, and nothing already covered by the snapshot is re-read.
//! Batch-SELECTs the dependency-ordered tables (`provider`, `demographic`,
//! `appointment`) through the same sink path as live CDC
//! (`row_to_*` -> `SyncEvent` -> conditional PUT / Bundle), so it is
//! idempotent and safe to re-run (spec acceptance: running twice changes
//! nothing).

use std::collections::HashMap;

use anyhow::{Context, Result};
use mysql_async::{prelude::*, Conn, Row, Value};
use tokio::sync::mpsc::Sender;
use tracing::{info, warn};

use crate::checkpoint::{self, Checkpoint};
use crate::config::{Config, DatabaseConfig};
use crate::domain::resource::DomainResource;
use crate::event::{Op, Source as EventSource, SyncEvent};
use crate::mapping::appointment::row_to_domain_appointment;
use crate::mapping::demographic::{row_to_domain_patient, ColumnMap};
use crate::mapping::provider::row_to_domain_practitioner;
use crate::metrics::SharedMetrics;
use crate::sources::mariadb_binlog::{self, resolve_column_map_for_table};
use crate::sources::{RowChange, RowOp, SourcePosition};

const BATCH_SIZE: u64 = 500;

/// Dependency order for the multi-resource backfill. Resources with outgoing
/// conditional references (appointment) are scanned last so their targets have
/// already been sent to the sink.
type Mapper = fn(&RowChange, &ColumnMap) -> Option<DomainResource>;
const BACKFILL_STEPS: &[(&str, &str, Mapper)] = &[
    ("provider", "provider_no", practitioner_mapper),
    ("demographic", "demographic_no", patient_mapper),
    ("appointment", "appointment_no", appointment_mapper),
];

/// Runs one dependency-ordered backfill pass, sending every row through `tx`
/// as an `Upsert` `SyncEvent`. Returns the total number of resources sent.
pub async fn run(
    cfg: &Config,
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

    let mut column_maps = HashMap::new();
    for (table, _, _) in BACKFILL_STEPS {
        column_maps.insert(
            table.to_string(),
            resolve_column_map_for_table(db, table).await?,
        );
    }

    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db.user, db.password, db.host, db.port, db.schema
    );
    let pool = mysql_async::Pool::new(url.as_str());
    let mut conn = pool
        .get_conn()
        .await
        .context("connecting for backfill scan")?;

    let mut total = 0usize;

    for (table, order_col, mapper) in BACKFILL_STEPS {
        let columns = column_maps.get(*table).expect("resolved column map");
        total += scan_table(&mut conn, db, *table, *order_col, columns, tx, metrics, *mapper)
            .await?;
    }

    drop(conn);
    let _ = pool.disconnect().await;

    info!("backfill: complete, {total} resources sent to sink");
    Ok(total)
}

fn patient_mapper(change: &RowChange, columns: &ColumnMap) -> Option<DomainResource> {
    row_to_domain_patient(change, columns).map(DomainResource::Patient)
}

fn practitioner_mapper(change: &RowChange, columns: &ColumnMap) -> Option<DomainResource> {
    row_to_domain_practitioner(change, columns).map(DomainResource::Practitioner)
}

fn appointment_mapper(change: &RowChange, columns: &ColumnMap) -> Option<DomainResource> {
    row_to_domain_appointment(change, columns).map(DomainResource::Appointment)
}

async fn scan_table(
    conn: &mut Conn,
    db: &DatabaseConfig,
    table: &str,
    order_col: &str,
    columns: &ColumnMap,
    tx: &Sender<SyncEvent>,
    metrics: &SharedMetrics,
    mapper: fn(&RowChange, &ColumnMap) -> Option<DomainResource>,
) -> Result<usize> {
    let mut offset: u64 = 0;
    let mut total = 0usize;

    loop {
        let sql = format!(
            "SELECT * FROM {table} ORDER BY {order_col} LIMIT {BATCH_SIZE} OFFSET {offset}"
        );
        let rows: Vec<Row> = conn
            .query(sql)
            .await
            .with_context(|| format!("scanning {table} batch"))?;

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
                table: table.to_string(),
                op: RowOp::Insert,
                after,
                position: SourcePosition::FilePos {
                    file: String::new(),
                    pos: 0,
                },
            };

            let Some(resource) = mapper(&change, columns) else {
                continue;
            };

            let sync_event = SyncEvent::new(
                EventSource::OscarBackfill { table: table.to_string() },
                Op::Upsert,
                resource,
                chrono::Utc::now(),
            );

            metrics.inc_received();
            if tx.send(sync_event).await.is_err() {
                warn!("backfill: sink channel closed mid-scan, stopping early");
                return Ok(total);
            }
            total += 1;
        }

        offset += batch_len;
    }

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
            let total_hours = days * 24 + u32::from(*hours);
            Some(format!("{sign}{total_hours:02}:{minutes:02}:{seconds:02}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backfill_steps_are_in_dependency_order() {
        let names: Vec<_> = BACKFILL_STEPS.iter().map(|(t, _, _)| *t).collect();
        assert_eq!(names, vec!["provider", "demographic", "appointment"]);
    }

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
        assert_eq!(
            mysql_value_to_string(&Value::Time(false, 0, 9, 0, 0, 0)),
            Some("09:00:00".to_string())
        );
        assert_eq!(
            mysql_value_to_string(&Value::Time(true, 0, 14, 30, 0, 0)),
            Some("-14:30:00".to_string())
        );
    }
}
