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
use std::time::Duration;

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
use crate::domain::resource::DomainResource;
use crate::event::{Op, Source as EventSource, SyncEvent};
use crate::mapping::appointment::row_to_domain_appointment;
use crate::mapping::demographic::{row_to_domain_patient, row_to_merged_patient, ColumnMap};
use crate::mapping::provider::row_to_domain_practitioner;
use crate::metrics::SharedMetrics;
use crate::sources::{RowChange, RowOp, SourcePosition, TableRef};

const DEMOGRAPHIC_TABLE: &str = "demographic";
const DEMOGRAPHIC_MERGED_TABLE: &str = "demographic_merged";
const PROVIDER_TABLE: &str = "provider";
const APPOINTMENT_TABLE: &str = "appointment";
const MAX_CONSECUTIVE_READ_ERRORS: u32 = 5;
const READ_ERROR_BACKOFF_MS: u64 = 100;

/// Runs the MariaDB binlog listener to completion (or until the sink
/// channel closes). Intended to be spawned as a top-level tokio task.
pub async fn run(cfg: Config, tx: Sender<SyncEvent>, metrics: SharedMetrics) -> Result<()> {
    let db = cfg.database.clone();

    if db.server_id == 0 {
        bail!("database.server_id must be non-zero and != Oscar's server-id (F12)");
    }

    let mut column_maps = HashMap::new();
    column_maps.insert(
        DEMOGRAPHIC_TABLE.to_string(),
        resolve_column_map_for_table(&db, DEMOGRAPHIC_TABLE).await?,
    );
    column_maps.insert(
        DEMOGRAPHIC_MERGED_TABLE.to_string(),
        resolve_column_map_for_table(&db, DEMOGRAPHIC_MERGED_TABLE).await?,
    );
    column_maps.insert(
        PROVIDER_TABLE.to_string(),
        resolve_column_map_for_table(&db, PROVIDER_TABLE).await?,
    );
    column_maps.insert(
        APPOINTMENT_TABLE.to_string(),
        resolve_column_map_for_table(&db, APPOINTMENT_TABLE).await?,
    );

    let (mut current_filename, start_position) = resolve_start_position(&cfg).await?;

    let url = format!("mysql://{}:{}@{}:{}", db.user, db.password, db.host, db.port);

    let mut client = BinlogClient::new(
        &url,
        db.server_id,
        StartPosition::BinlogPosition(current_filename.clone(), start_position),
    )
    .with_master_heartbeat(Duration::from_secs(30));

    let mut stream = client
        .connect()
        .await
        .context("connecting to MariaDB binlog stream")?;

    info!(
        "mariadb_binlog: streaming schema `{}` from {}:{}",
        db.schema, current_filename, start_position
    );

    let mut tables: HashMap<u64, TableRef> = HashMap::new();
    let mut consecutive_errors = 0u32;

    loop {
        let (header, data) = match stream.read().await {
            Ok(v) => {
                consecutive_errors = 0;
                v
            }
            Err(e) => {
                consecutive_errors += 1;
                error!(
                    "mariadb_binlog: read error ({}/{}): {e:?}",
                    consecutive_errors, MAX_CONSECUTIVE_READ_ERRORS
                );
                if consecutive_errors >= MAX_CONSECUTIVE_READ_ERRORS {
                    bail!("mariadb_binlog: too many consecutive read errors");
                }
                tokio::time::sleep(Duration::from_millis(READ_ERROR_BACKOFF_MS)).await;
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
                if let Some(table) = is_target_table(&tables, write.table_id, &db.schema) {
                    for row in &write.rows {
                        channel_closed |= !emit_row(
                            &db.schema,
                            &table,
                            &column_maps,
                            &tx,
                            &metrics,
                            &row.column_values,
                            RowOp::Insert,
                            Op::Upsert,
                            &current_filename,
                            header.next_event_position,
                        )
                        .await;
                    }
                }
            }
            EventData::UpdateRows(update) => {
                if let Some(table) = is_target_table(&tables, update.table_id, &db.schema) {
                    // After-image only (F6): the sink treats this as a full upsert.
                    for (_before, after) in &update.rows {
                        channel_closed |= !emit_row(
                            &db.schema,
                            &table,
                            &column_maps,
                            &tx,
                            &metrics,
                            &after.column_values,
                            RowOp::Update,
                            Op::Upsert,
                            &current_filename,
                            header.next_event_position,
                        )
                        .await;
                    }
                }
            }
            EventData::DeleteRows(delete) => {
                if let Some(table) = is_target_table(&tables, delete.table_id, &db.schema) {
                    // MariaDB is configured with binlog_row_image=FULL (E2), so the
                    // before-image carries every column, including demographic_no.
                    for row in &delete.rows {
                        channel_closed |= !emit_row(
                            &db.schema,
                            &table,
                            &column_maps,
                            &tx,
                            &metrics,
                            &row.column_values,
                            RowOp::Delete,
                            Op::Delete,
                            &current_filename,
                            header.next_event_position,
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
        metrics.set_position(format!("{}:{}", cp.binlog_filename, cp.binlog_position));
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

    capture_binlog_position(&cfg.database).await
}

/// Reads `SHOW MASTER STATUS` directly, parsing **only columns 0 and 1**
/// (File, Position). MariaDB's result set has 4 columns, MySQL's has 5
/// (E5) — never index into column 4, which is the root cause of the
/// original mis-diagnosed incompatibility (§2.3).
pub(crate) async fn capture_binlog_position(db: &DatabaseConfig) -> Result<(String, u32)> {
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

    let row: Option<mysql_async::Row> = conn
        .query_first("SHOW MASTER STATUS")
        .await
        .context("running SHOW MASTER STATUS")?;

    drop(conn);
    let _ = pool.disconnect().await;

    let row = row.ok_or_else(|| {
        anyhow::anyhow!(
            "SHOW MASTER STATUS returned no rows — is binlog enabled on this server? (E2)"
        )
    })?;

    // Positional, not tuple-typed: MariaDB returns 4 columns, MySQL 5.
    // Only columns 0 and 1 are ever read.
    let file: String = row
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("SHOW MASTER STATUS: missing File column"))?;
    let pos_raw: String = row
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("SHOW MASTER STATUS: missing Position column"))?;
    let pos: u32 = pos_raw
        .parse()
        .with_context(|| format!("SHOW MASTER STATUS: unparseable Position `{pos_raw}`"))?;

    Ok((file, pos))
}

/// Maps one row's column values to a `DomainPatient` and sends the
/// resulting `SyncEvent`. Returns `false` if the sink channel has closed
/// (signal to stop the listener); `true` otherwise, including when the row
/// is skipped because it has no natural key.
async fn emit_row(
    schema: &str,
    table: &str,
    column_maps: &HashMap<String, ColumnMap>,
    tx: &Sender<SyncEvent>,
    metrics: &SharedMetrics,
    values: &[ColumnValue],
    row_op: RowOp,
    sync_op: Op,
    file: &str,
    pos: u32,
) -> bool {
    let Some(columns) = column_maps.get(table) else {
        warn!("mariadb_binlog: no column map resolved for {table}");
        return true;
    };

    let after: Vec<Option<String>> = values.iter().map(column_value_to_string).collect();

    let change = RowChange {
        schema: schema.to_string(),
        table: table.to_string(),
        op: row_op,
        after,
        position: SourcePosition::FilePos {
            file: file.to_string(),
            pos,
        },
    };

    let resource = match table {
        DEMOGRAPHIC_TABLE => row_to_domain_patient(&change, columns).map(DomainResource::Patient),
        DEMOGRAPHIC_MERGED_TABLE => row_to_merged_patient(&change, columns).map(DomainResource::Patient),
        PROVIDER_TABLE => row_to_domain_practitioner(&change, columns).map(DomainResource::Practitioner),
        APPOINTMENT_TABLE => row_to_domain_appointment(&change, columns).map(DomainResource::Appointment),
        _ => return true,
    };

    let Some(resource) = resource else {
        return true;
    };

    let sync_event = SyncEvent::new(
        EventSource::OscarBinlog { table: table.to_string() },
        sync_op,
        resource,
        chrono::Utc::now(),
    );

    metrics.inc_received();
    tx.send(sync_event).await.is_ok()
}

fn is_target_table(tables: &HashMap<u64, TableRef>, table_id: u64, schema: &str) -> Option<String> {
    tables.get(&table_id).and_then(|t| {
        if t.schema == schema
            && (t.table == DEMOGRAPHIC_TABLE
                || t.table == DEMOGRAPHIC_MERGED_TABLE
                || t.table == PROVIDER_TABLE
                || t.table == APPOINTMENT_TABLE)
        {
            Some(t.table.clone())
        } else {
            None
        }
    })
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

/// Resolves a single Oscar table's column name -> ordinal index.
/// Backwards-compatible alias for the original single-table call.
pub(crate) async fn resolve_column_map(db: &DatabaseConfig) -> Result<ColumnMap> {
    resolve_column_map_for_table(db, DEMOGRAPHIC_TABLE).await
}

/// Resolves `table` column name -> ordinal index via `information_schema.columns` (D3).
/// Self-healing across Oscar schema variants; never hand-maintain a column list.
pub(crate) async fn resolve_column_map_for_table(
    db: &DatabaseConfig,
    table: &str,
) -> Result<ColumnMap> {
    use mysql_async::prelude::*;

    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db.user, db.password, db.host, db.port, db.schema
    );

    let pool = mysql_async::Pool::new(url.as_str());
    let mut conn = pool
        .get_conn()
        .await
        .with_context(|| format!("connecting to resolve {table} column map"))?;

    let rows: Vec<(String, u32)> = conn
        .exec(
            "SELECT column_name, ordinal_position FROM information_schema.columns \
             WHERE table_schema = :schema AND table_name = :table \
             ORDER BY ordinal_position",
            params! { "schema" => db.schema.clone(), "table" => table },
        )
        .await
        .with_context(|| format!("querying information_schema.columns for {table}"))?;

    drop(conn);
    let _ = pool.disconnect().await;

    if rows.is_empty() {
        bail!(
            "no columns resolved for {}.{} — check schema name and privileges",
            db.schema,
            table
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
        table
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
        tables.insert(
            43,
            TableRef {
                schema: "oscar".to_string(),
                table: "demographic_merged".to_string(),
            },
        );
        tables.insert(
            44,
            TableRef {
                schema: "oscar".to_string(),
                table: "provider".to_string(),
            },
        );
        tables.insert(
            45,
            TableRef {
                schema: "oscar".to_string(),
                table: "appointment".to_string(),
            },
        );

        assert_eq!(is_target_table(&tables, 42, "oscar").as_deref(), Some("demographic"));
        assert_eq!(is_target_table(&tables, 43, "oscar").as_deref(), Some("demographic_merged"));
        assert_eq!(is_target_table(&tables, 44, "oscar").as_deref(), Some("provider"));
        assert_eq!(is_target_table(&tables, 45, "oscar").as_deref(), Some("appointment"));
        assert!(is_target_table(&tables, 42, "other_schema").is_none());
        assert!(is_target_table(&tables, 999, "oscar").is_none()); // unknown table_id (F2)
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
