//! CDC source implementation using the `mysql_cdc` crate (D7).
//!
//! `mysql-binlog-connector-rust` panics during handshake parsing against
//! MariaDB (see TASK_FEATURES_SPEC_OSCAR_SYNC.md §2.3); `mysql_cdc` is the
//! confirmed replacement. Its `replicate()` API is synchronous, so the
//! listener loop runs inside `spawn_blocking` and pushes into the async
//! mpsc channel via `blocking_send` (D7.1).

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use mysql_cdc::{
    binlog_client::BinlogClient,
    binlog_options::BinlogOptions,
    events::{binlog_event::BinlogEvent, row_events::mysql_value::MySqlValue},
    replica_options::ReplicaOptions,
    ssl_mode::SslMode,
};
use tokio::sync::mpsc::Sender;
use tracing::{error, info, warn};

use crate::config::{Config, DatabaseConfig};
use crate::event::{Op, ResourceType, Source as EventSource, SyncEvent};
use crate::mapping::demographic::{row_to_domain_patient, ColumnMap};
use crate::sources::{RowChange, RowOp, SourcePosition, TableRef};

const DEMOGRAPHIC_TABLE: &str = "demographic";

/// Runs the MariaDB CDC listener to completion (or until the channel closes).
/// Intended to be spawned as a top-level tokio task.
pub async fn run(cfg: Config, tx: Sender<SyncEvent>) -> Result<()> {
    let columns = Arc::new(resolve_column_map(&cfg.database).await?);
    let db = cfg.database.clone();

    tokio::task::spawn_blocking(move || run_blocking(db, columns, tx))
        .await
        .context("mariadb_cdc listener task panicked")?
}

fn run_blocking(db: DatabaseConfig, columns: Arc<ColumnMap>, tx: Sender<SyncEvent>) -> Result<()> {
    let options = ReplicaOptions {
        hostname: db.host,
        port: db.port,
        username: db.user,
        password: db.password,
        ssl_mode: SslMode::Disabled, // D7.3: connector has no SSL support
        blocking: true,
        binlog: BinlogOptions::from_end(),
        ..Default::default()
    };

    let mut client = BinlogClient::new(options);
    let mut tables: HashMap<u64, TableRef> = HashMap::new();

    info!("mariadb_cdc: starting replication for schema `{}`", db.schema);

    let events = client
        .replicate()
        .map_err(|e| anyhow::anyhow!("mariadb_cdc replicate() failed: {e:?}"))?;

    for result in events {
        let (header, event) = match result {
            Ok(v) => v,
            Err(e) => {
                error!("mariadb_cdc: read error: {e:?}");
                continue;
            }
        };

        let mut channel_closed = false;

        match &event {
            BinlogEvent::TableMapEvent(tm) => {
                tables.insert(
                    tm.table_id,
                    TableRef {
                        schema: tm.database_name.clone(),
                        table: tm.table_name.clone(),
                    },
                );
            }
            BinlogEvent::WriteRowsEvent(write) => {
                if is_target_table(&tables, write.table_id, &db.schema) {
                    for row in &write.rows {
                        channel_closed |= !emit_row(
                            &db.schema,
                            &columns,
                            &tx,
                            &row.cells,
                            header.next_event_position,
                            RowOp::Insert,
                            Op::Upsert,
                        );
                    }
                }
            }
            BinlogEvent::UpdateRowsEvent(update) => {
                if is_target_table(&tables, update.table_id, &db.schema) {
                    // After-image only (F6): the sink treats this as a full upsert.
                    for row in &update.rows {
                        channel_closed |= !emit_row(
                            &db.schema,
                            &columns,
                            &tx,
                            &row.after_update.cells,
                            header.next_event_position,
                            RowOp::Update,
                            Op::Upsert,
                        );
                    }
                }
            }
            BinlogEvent::DeleteRowsEvent(delete) => {
                if is_target_table(&tables, delete.table_id, &db.schema) {
                    // MariaDB is configured with binlog_row_image=FULL (E2), so the
                    // before-image carries every column, including demographic_no.
                    for row in &delete.rows {
                        channel_closed |= !emit_row(
                            &db.schema,
                            &columns,
                            &tx,
                            &row.cells,
                            header.next_event_position,
                            RowOp::Delete,
                            Op::Delete,
                        );
                    }
                }
            }
            _ => {}
        }

        if channel_closed {
            warn!("mariadb_cdc: sink channel closed; stopping listener");
            return Ok(());
        }

        client.commit(&header, &event);
    }

    Ok(())
}

/// Maps one row's cells to a `DomainPatient` and sends the resulting
/// `SyncEvent`. Returns `false` if the sink channel has closed (signal to
/// stop the listener); `true` otherwise, including when the row is skipped
/// because it has no `demographic_no`.
fn emit_row(
    schema: &str,
    columns: &ColumnMap,
    tx: &Sender<SyncEvent>,
    cells: &[Option<MySqlValue>],
    position: u32,
    row_op: RowOp,
    sync_op: Op,
) -> bool {
    let after: Vec<Option<String>> = cells
        .iter()
        .map(|c| c.as_ref().map(mysql_value_to_string))
        .collect();

    let change = RowChange {
        schema: schema.to_string(),
        table: DEMOGRAPHIC_TABLE.to_string(),
        op: row_op,
        after,
        position: SourcePosition::FilePos {
            file: String::new(),
            pos: position,
        },
    };

    let Some(patient) = row_to_domain_patient(&change, columns) else {
        return true;
    };

    let idempotency_key = format!("oscar:demographic:{}:{}", patient.demographic_no, position);
    let sync_event = SyncEvent {
        source: EventSource::OscarBinlog,
        op: sync_op,
        resource_type: ResourceType::Patient,
        idempotency_key,
        payload: patient,
        occurred_at: chrono::Utc::now(),
    };

    tx.blocking_send(sync_event).is_ok()
}

fn is_target_table(tables: &HashMap<u64, TableRef>, table_id: u64, schema: &str) -> bool {
    tables
        .get(&table_id)
        .map(|t| t.schema == schema && t.table == DEMOGRAPHIC_TABLE)
        .unwrap_or(false)
}

fn mysql_value_to_string(value: &MySqlValue) -> String {
    match value {
        MySqlValue::TinyInt(v) => v.to_string(),
        MySqlValue::SmallInt(v) => v.to_string(),
        MySqlValue::MediumInt(v) => v.to_string(),
        MySqlValue::Int(v) => v.to_string(),
        MySqlValue::BigInt(v) => v.to_string(),
        MySqlValue::Float(v) => v.to_string(),
        MySqlValue::Double(v) => v.to_string(),
        MySqlValue::Decimal(s) => s.clone(),
        MySqlValue::String(s) => s.clone(),
        MySqlValue::Enum(v) => v.to_string(),
        MySqlValue::Set(v) => v.to_string(),
        MySqlValue::Year(v) => v.to_string(),
        MySqlValue::Date(d) => format!("{:04}-{:02}-{:02}", d.year, d.month, d.day),
        MySqlValue::Time(t) => format!("{:02}:{:02}:{:02}", t.hour, t.minute, t.second),
        MySqlValue::DateTime(dt) => format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second
        ),
        MySqlValue::Timestamp(v) => v.to_string(),
        MySqlValue::Bit(bits) => bits.iter().map(|b| if *b { '1' } else { '0' }).collect(),
        MySqlValue::Blob(bytes) => String::from_utf8_lossy(bytes).to_string(),
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
        "mariadb_cdc: resolved {} columns for {}.{}",
        map.len(),
        db.schema,
        DEMOGRAPHIC_TABLE
    );

    Ok(map)
}
