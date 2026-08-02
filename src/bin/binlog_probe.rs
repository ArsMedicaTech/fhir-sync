//! Probe: does mysql-binlog-connector-rust 0.3.2 parse MariaDB 10.6 binlogs?
//!
//! Place at: examples/binlog_probe.rs
//! Run with: cargo run --example binlog_probe
//!
//! Env overrides (defaults target the local Oscar compose stack):
//!   PROBE_URL   default mysql://fhirsync:fhirsyncpw@127.0.0.1:3316
//!
//! This file is throwaway diagnostics. It does not import anything from
//! fhir-sync's own modules, so it cannot affect the main build.

use mysql_binlog_connector_rust::{
    binlog_client::BinlogClient,
    event::event_data::EventData,
};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = env::var("PROBE_URL")
        .unwrap_or_else(|_| "mysql://fhirsync:fhirsyncpw@127.0.0.1:3316".to_string());

    let server_id: u64 = env::var("PROBE_SERVER_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4321);

    // Empty filename => crate calls fetch_binlog_info() (SHOW MASTER STATUS),
    // which panics on MariaDB. Setting it explicitly skips that path entirely.
    let binlog_filename = env::var("PROBE_BINLOG_FILE").unwrap_or_default();
    let binlog_position: u32 = env::var("PROBE_BINLOG_POS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4);

    println!(
        "connecting to {url} (server_id={server_id}, file={:?}, pos={binlog_position})",
        binlog_filename
    );

    let mut client = BinlogClient {
        url,
        server_id,
        binlog_filename,
        binlog_position,
        ..Default::default()
    };

    let mut stream = match client.connect().await {
        Ok(s) => {
            println!("CONNECTED — handshake + COM_BINLOG_DUMP accepted");
            s
        }
        Err(e) => {
            eprintln!("CONNECT FAILED: {e:?}");
            eprintln!("  -> auth plugin, privileges, or protocol-level incompatibility");
            return Err(e.into());
        }
    };

    println!("streaming... now go modify a demographic row (see checklist)");
    println!("---");

    let mut event_count = 0usize;

    loop {
        match stream.read().await {
            Ok((header, data)) => {
                event_count += 1;

                // Print the discriminant for every event so we can see whether
                // MariaDB-specific events (e.g. ANNOTATE_ROWS = 160) show up
                // and whether the parser chokes on them.
                match &data {
                    EventData::TableMap(e) => {
                        println!(
                            "[{event_count}] TableMap  table_id={} db={} table={} n_cols={}",
                            e.table_id,
                            e.database_name,
                            e.table_name,
                            e.column_types.len()
                        );
                    }
                    EventData::WriteRows(e) => {
                        println!("[{event_count}] WriteRows table_id={} rows={}", e.table_id, e.rows.len());
                        for row in &e.rows {
                            println!("    {:?}", row.column_values);
                        }
                    }
                    EventData::UpdateRows(e) => {
                        println!("[{event_count}] UpdateRows table_id={} rows={}", e.table_id, e.rows.len());
                        for (before, after) in &e.rows {
                            println!("    BEFORE {:?}", before.column_values);
                            println!("    AFTER  {:?}", after.column_values);
                        }
                    }
                    EventData::DeleteRows(e) => {
                        println!("[{event_count}] DeleteRows table_id={} rows={}", e.table_id, e.rows.len());
                    }
                    other => {
                        // Rotate / FormatDescription / Query / Xid / Gtid / heartbeat etc.
                        println!(
                            "[{event_count}] other event_type={} -> {:?}",
                            header.event_type,
                            std::mem::discriminant(other)
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!("PARSE/READ FAILED after {event_count} events: {e:?}");
                eprintln!("  -> this is the MariaDB-incompatibility signal");
                return Err(e.into());
            }
        }
    }
}
