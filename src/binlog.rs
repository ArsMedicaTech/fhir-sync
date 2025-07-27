use anyhow::Result;
use mysql_binlog_connector_rust::{
    binlog_client::BinlogClient,
    event::event_data::EventData,
    event::write_rows_event::WriteRowsEvent,
};
use tokio::sync::mpsc::Sender;

//use crate::adapters::entities::patient::Patient;
//use crate::proto::google::fhir::proto::r5::core::Patient as ProtoPatient;
//use crate::adapters::oscar::OscarPatient;
use crate::domain::patient::DomainPatient;
use crate::{Event};
use crate::ext::ColumnValueExt;
use crate::config::load_config;


/// Run inside a Tokio task
pub async fn run_binlog_listener(tx: Sender<Event>) -> Result<()> {
    let config = load_config()?;

    let db = config.database;
    let url = format!(
        "mysql://{}:{}@{}:{}",
        db.user, db.password, db.host, db.port
    );

    // ----------- build & connect -------------------------------------------------
    let mut client = BinlogClient {
        url,
        // pick up current master position automatically:
        ..Default::default()
    };

    let mut stream = client.connect().await?;     // returns BinlogStream
    println!("✅ connected to binlog; streaming...");

    // ----------- main loop -------------------------------------------------------
    loop {
        // `read()` is async-blocking until the next event chunk arrives
        let (_header, data) = stream.read().await?;
        match data {
            EventData::WriteRows(write) => handle_write_rows(write, &tx).await?,
            EventData::UpdateRows(_update) => { 
                // same idea, but for UpdateRowsEvent
                // convert DTO → google.fhir.r5.core.Patient → send to peer
             }
            EventData::DeleteRows(_delete) => { 
                // … same for DeleteRowsEvent
                // convert DTO → google.fhir.r5.core.Patient → send to peer
             }
            _ /* heartbeat, rotate, ... */ => {}
        }
    }
}

/// Map binlog row → DTO
async fn handle_write_rows(event: WriteRowsEvent, tx: &Sender<Event>) -> Result<()> {
    // The `WriteRowsEvent` already carries the table name (if a TableMapEvent was
    // previously received).  Ignore tables that aren't `demographic`.
    if event.table_name.as_deref() != Some("demographic") {
        return Ok(());
    }

    // no field `columns` on type `&RowEvent`

    for row in &event.rows {
        // NOTE: Adjust column indexes to match OSCAR's schema.       

        let col1 = &row.column_values[0]; // force type binding
        col1.as_str(); // <-- this should now work if trait is in scope

        let col2 = &row.column_values[1];
        let col3 = &row.column_values[2];
        let col4 = &row.column_values[3];
        let col5 = &row.column_values[4];

        let dto = DomainPatient {
            //demographic_no: row.column_values[0].as_str().unwrap_or_default().to_owned(),
            // use associated function syntax instead: `ColumnValue::parse()`
            demographic_no: col1.as_str().unwrap_or_default().to_owned(),
            first_name:     Some(col2.as_str().unwrap_or_default().to_owned()),
            last_name:      Some(col3.as_str().unwrap_or_default().to_owned()),
            //birth_date:     col4.as_str().unwrap_or_default(),
            date_of_birth:  col4.as_str().map(|s| s.to_owned()), // Option<String>
            //gender:         col5.as_str().unwrap_or_default(),
            sex:            col5.as_str().map(|s| s.to_owned()), // Option<String>
            // ... fill the rest
            email:          Some("email".to_owned()),
            phone:          Some("phone".to_owned()),
            location:       Some((
                "location".to_owned(),
                "location".to_owned(),
                "location".to_owned(),
                "location".to_owned()
            )),
        };

        // push through your mpsc channel
        tx.send(Event::PatientUpsertAMT(dto)).await.ok();
    }
    Ok(())
}