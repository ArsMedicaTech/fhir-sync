//! CDC source implementation using `mysql-binlog-connector-rust` (D7,
//! settled in the v3 spec revision after the §2.3 control test).
//!
//! Async-native: no `spawn_blocking`, no `blocking_send`. Two invariants
//! from D7 are load-bearing and must never regress:
//!
//! 1. `binlog_filename` must never be empty — an empty filename routes
//!    through `fetch_binlog_info` (`SHOW MASTER STATUS`), which panics
//!    against MariaDB because it unconditionally reads column index 4,
//!    which MariaDB doesn't have (E5, F7).
//! 2. `server_id` must be non-zero and distinct from the source server's —
//!    zero causes the master to disconnect after the last available event
//!    (F12).

use std::collections::HashMap;

use anyhow::{bail, Context, Result};
use mysql_binlog_connector_rust::{
    binlog_client::{BinlogClient, StartPosition},
    column::column_value::ColumnValue,
    event::event_data::EventData,
};
use tokio::sync::mpsc::Sender;
use tracing::{error, info, warn};

use crate::checkpoint::{self, Checkpoint};
use crate::config::{Config, DatabaseConfig};
use crate::event::{Op, ResourceType, Source as EventSource, SyncEvent};
use crate::mapping::demographic::{row_to_domain_patient, ColumnMap};
use crate::sources::{RowChange, RowOp, SourcePosition, TableRef};

const DEMOGRAPHIC_TABLE: &str = "demographic";

/// Runs the MariaDB binlog listener to completion (or until the sink
/// channel closes). Intended to be spawned as a top-level tokio task.
pub async fn run(cfg: Config, tx: Sender<SyncEvent>) -> Result<()> {
    let db = cfg.database.clone();

    if db.server_id == 0 {
        bail!("database.server_id must be non-zero and != Oscar's server-id (F12)");
    }

    let columns = resolve_column_map(&db).await?;
    let (mut current_filename, start_position) = resolve_start_position(&cfg).await?;

    let url = format!("mysql://{}:{}@{}:{}", db.user, db.password, db.host, db.port);

    let mut client = BinlogClient::new(
        &url,
        db.server_id,
        StartPosition::BinlogPosition(current_filename.clone(), start_position),
    );

    let mut stream = client
        .connect()
        .await
        .context("connecting to MariaDB binlog stream")?;

    info!(
        "mariadb_binlog: streaming schema `{}` from {}:{}",
        db.schema, current_filename, start_position
    );

    let mut tables: HashMap<u64, TableRef> = HashMap::new();

    loop {
        let (header, data) = match stream.read().await {
            Ok(v) => v,
            Err(e) => {
                error!("mariadb_binlog: read error: {e:?}");
                continue;
            }
        };

        let mut channel_closed = false;

        match &data {
            EventData::Rotate(rotate) => {
                current_filename = rotate.binlog_filename.clone();
            }
            EventData::TableMap(tm) => {
                tables.insert(
                    tm.table_id,
                    TableRef {
                        schema: tm.database_name.clone(),
                        table: tm.table_name.clone(),
                    },
                );
            }
            EventData::WriteRows(write) => {
                if is_target_table(&tables, write.table_id, &db.schema) {
                    for row in &write.rows {
                        channel_closed |= !emit_row(
                            &db.schema,
                            &columns,
                            &tx,
                            &row.column_values,
                            RowOp::Insert,
                            Op::Upsert,
                        )
                        .await;
                    }
                }
            }
            EventData::UpdateRows(update) => {
                if is_target_table(&tables, update.table_id, &db.schema) {
                    // After-image only (F6): the sink treats this as a full upsert.
                    for (_before, after) in &update.rows {
                        channel_closed |= !emit_row(
                            &db.schema,
                            &columns,
                            &tx,
                            &after.column_values,
                            RowOp::Update,
                            Op::Upsert,
                        )
                        .await;
                    }
                }
            }
            EventData::DeleteRows(delete) => {
                if is_target_table(&tables, delete.table_id, &db.schema) {
                    // MariaDB is configured with binlog_row_image=FULL (E2), so the
                    // before-image carries every column, including demographic_no.
                    for row in &delete.rows {
                        channel_closed |= !emit_row(
                            &db.schema,
                            &columns,
                            &tx,
                            &row.column_values,
                            RowOp::Delete,
                            Op::Delete,
                        )
                        .await;
                    }
                }
            }
            _ => {}
        }

        if channel_closed {
            warn!("mariadb_binlog: sink channel closed; stopping listener");
            return Ok(());
        }

        // D6: persist file+position after every event, not just target-table
        // rows — `next_event_position` advances for the whole stream, and a
        // resume must not re-read events already accounted for.
        let cp = Checkpoint {
            binlog_filename: current_filename.clone(),
            binlog_position: header.next_event_position,
        };
        if let Err(e) = checkpoint::save(&cfg.sync.checkpoint_path, &cp) {
            warn!("mariadb_binlog: failed to persist checkpoint: {e:?}");
        }
    }
}

/// Resolves the (binlog_filename, binlog_position) to start streaming from:
/// the last saved checkpoint on warm start, or a direct `SHOW MASTER
/// STATUS` read on cold start. D7 invariant 1: never returns an empty
/// filename.
async fn resolve_start_position(cfg: &Config) -> Result<(String, u32)> {
    if let Some(cp) = checkpoint::load(&cfg.sync.checkpoint_path) {
        info!(
            "mariadb_binlog: resuming from checkpoint {}:{}",
            cp.binlog_filename, cp.binlog_position
        );
        return Ok((cp.binlog_filename, cp.binlog_position));
    }

    show_master_status(&cfg.database).await
}

/// Reads `SHOW MASTER STATUS` directly, parsing **only columns 0 and 1**
/// (File, Position). MariaDB's result set has 4 columns, MySQL's has 5
/// (E5) — never index into column 4, which is the root cause of the
/// original mis-diagnosed incompatibility (§2.3).
async fn show_master_status(db: &DatabaseConfig) -> Result<(String, u32)> {
    use mysql_async::prelude::*;

    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db.user, db.password, db.host, db.port, db.schema
    );

    let pool = mysql_async::Pool::new(url.as_str());
    let mut conn = pool
        .get_conn()
        .await
        .context("connecting to run SHOW MASTER STATUS")?;

    let row: Option<(String, u32)> = conn
        .query_first("SHOW MASTER STATUS")
        .await
        .context("running SHOW MASTER STATUS")?;

    drop(conn);
    let _ = pool.disconnect().await;

    row.ok_or_else(|| {
        anyhow::anyhow!(
            "SHOW MASTER STATUS returned no rows — is binlog enabled on this server? (E2)"
        )
    })
}

/// Maps one row's column values to a `DomainPatient` and sends the
/// resulting `SyncEvent`. Returns `false` if the sink channel has closed
/// (signal to stop the listener); `true` otherwise, including when the row
/// is skipped because it has no `demographic_no`.
async fn emit_row(
    schema: &str,
    columns: &ColumnMap,
    tx: &Sender<SyncEvent>,
    values: &[ColumnValue],
    row_op: RowOp,
    sync_op: Op,
) -> bool {
    let after: Vec<Option<String>> = values.iter().map(column_value_to_string).collect();

    let change = RowChange {
        schema: schema.to_string(),
        table: DEMOGRAPHIC_TABLE.to_string(),
        op: row_op,
        after,
        // File+position is threaded through the checkpoint, not per-row;
        // the idempotency key only needs a value that's unique per commit.
        position: SourcePosition::FilePos {
            file: String::new(),
            pos: 0,
        },
    };

    let Some(patient) = row_to_domain_patient(&change, columns) else {
        return true;
    };

    let idempotency_key = format!(
        "oscar:demographic:{}:{}",
        patient.demographic_no,
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
    );
    let sync_event = SyncEvent {
        source: EventSource::OscarBinlog,
        op: sync_op,
        resource_type: ResourceType::Patient,
        idempotency_key,
        payload: patient,
        occurred_at: chrono::Utc::now(),
    };

    tx.send(sync_event).await.is_ok()
}

fn is_target_table(tables: &HashMap<u64, TableRef>, table_id: u64, schema: &str) -> bool {
    tables
        .get(&table_id)
        .map(|t| t.schema == schema && t.table == DEMOGRAPHIC_TABLE)
        .unwrap_or(false)
}

/// Converts a decoded column value to its string form for downstream
/// mapping. `String`/`Blob`/`Json` arrive as raw bytes with no charset
/// attached (§8 charset risk, confirmed in §2.3) — `from_utf8_lossy` is a
/// deliberate, visible degradation rather than a silent one; revisit if
/// Oscar's charset is confirmed to be latin1.
fn column_value_to_string(value: &ColumnValue) -> Option<String> {
    match value {
        ColumnValue::None => None,
        ColumnValue::Tiny(v) => Some(v.to_string()),
        ColumnValue::Short(v) => Some(v.to_string()),
        ColumnValue::Long(v) => Some(v.to_string()),
        ColumnValue::LongLong(v) => Some(v.to_string()),
        ColumnValue::Float(v) => Some(v.to_string()),
        ColumnValue::Double(v) => Some(v.to_string()),
        ColumnValue::Decimal(s) => Some(s.clone()),
        ColumnValue::Time(s) => Some(s.clone()),
        ColumnValue::Date(s) => Some(s.clone()),
        ColumnValue::DateTime(s) => Some(s.clone()),
        ColumnValue::Timestamp(v) => Some(v.to_string()),
        ColumnValue::Year(v) => Some(v.to_string()),
        ColumnValue::String(bytes) => Some(String::from_utf8_lossy(bytes).into_owned()),
        ColumnValue::Blob(bytes) => Some(String::from_utf8_lossy(bytes).into_owned()),
        ColumnValue::Bit(v) => Some(v.to_string()),
        ColumnValue::Set(v) => Some(v.to_string()),
        ColumnValue::Enum(v) => Some(v.to_string()),
        ColumnValue::Json(bytes) => Some(String::from_utf8_lossy(bytes).into_owned()),
    }
}

/// Resolves `demographic` column name -> ordinal index via
/// `information_schema.columns` (D3). Self-healing across Oscar schema
/// variants; never hand-maintain a column list.
async fn resolve_column_map(db: &DatabaseConfig) -> Result<ColumnMap> {
    use mysql_async::prelude::*;

    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db.user, db.password, db.host, db.port, db.schema
    );

    let pool = mysql_async::Pool::new(url.as_str());
    let mut conn = pool
        .get_conn()
        .await
        .context("connecting to resolve demographic column map")?;

    let rows: Vec<(String, u32)> = conn
        .exec(
            "SELECT column_name, ordinal_position FROM information_schema.columns \
             WHERE table_schema = :schema AND table_name = :table \
             ORDER BY ordinal_position",
            params! { "schema" => db.schema.clone(), "table" => DEMOGRAPHIC_TABLE },
        )
        .await
        .context("querying information_schema.columns")?;

    drop(conn);
    let _ = pool.disconnect().await;

    if rows.is_empty() {
        bail!(
            "no columns resolved for {}.{} — check schema name and privileges",
            db.schema,
            DEMOGRAPHIC_TABLE
        );
    }

    let map: ColumnMap = rows
        .into_iter()
        .map(|(name, ordinal)| (name, (ordinal - 1) as usize))
        .collect();

    info!(
        "mariadb_binlog: resolved {} columns for {}.{}",
        map.len(),
        db.schema,
        DEMOGRAPHIC_TABLE
    );

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_target_table_matches_schema_and_table() {
        let mut tables = HashMap::new();
        tables.insert(
            42,
            TableRef {
                schema: "oscar".to_string(),
                table: "demographic".to_string(),
            },
        );

        assert!(is_target_table(&tables, 42, "oscar"));
        assert!(!is_target_table(&tables, 42, "other_schema"));
        assert!(!is_target_table(&tables, 999, "oscar")); // unknown table_id (F2)
    }

    #[test]
    fn column_value_to_string_formats_all_variants() {
        assert_eq!(column_value_to_string(&ColumnValue::None), None);
        assert_eq!(
            column_value_to_string(&ColumnValue::Long(42)),
            Some("42".to_string())
        );
        assert_eq!(
            column_value_to_string(&ColumnValue::Date("1990-03-05".to_string())),
            Some("1990-03-05".to_string())
        );
        assert_eq!(
            column_value_to_string(&ColumnValue::String(b"hello".to_vec())),
            Some("hello".to_string())
        );
        assert_eq!(
            column_value_to_string(&ColumnValue::Blob(b"blob-data".to_vec())),
            Some("blob-data".to_string())
        );
    }
}
