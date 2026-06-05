// use crate::models::ReplicationMetrics;
use crate::guc;
use crate::models::ReplicationClient;
use heapless;
use pgrx::spi::{Spi, SpiError};
use pq_sys::{
    ConnStatusType, ExecStatusType, PGconn, PQclear, PQconnectdb, PQerrorMessage, PQexec, PQfinish,
    PQgetvalue, PQnfields, PQntuples, PQresultStatus, PQstatus,
};
use std::ffi::{CStr, CString};

// use postgres::{Client, NoTls};
/// Use spi instead of tokio_postgres to query the replica metrics from within the PostgreSQL extension

pub fn collect_replication_clients(
) -> Result<(heapless::Vec<ReplicationClient, 16>, usize), SpiError> {
    let query = "
        SELECT
            application_name,
            client_addr::text,
            state,
            sent_lsn::text,
            write_lsn::text,
            flush_lsn::text,
            replay_lsn::text,
            EXTRACT(EPOCH FROM write_lag)::float8  AS write_lag_seconds,
            EXTRACT(EPOCH FROM flush_lag)::float8  AS flush_lag_seconds,
            EXTRACT(EPOCH FROM replay_lag)::float8 AS replay_lag_seconds,
            pg_wal_lsn_diff(
                pg_current_wal_lsn(),
                COALESCE(replay_lsn, flush_lsn, write_lsn)
            )::bigint AS lsn_gap_bytes
        FROM pg_stat_replication";

    Spi::connect(|client| {
        let rows = client.select(query, None, &[])?;
        // let mut clients = [ReplicationClient::default(); MAX_REPLICATION_CLIENTS];
        let mut clients = heapless::Vec::new();
        let mut count = 0;

        for row in rows {
            let app: Option<String> = row.get_by_name("application_name")?;
            let addr: Option<String> = row.get_by_name("client_addr")?;
            let state: Option<String> = row.get_by_name("state")?;
            let sent: Option<String> = row.get_by_name("sent_lsn")?;
            let write: Option<String> = row.get_by_name("write_lsn")?;
            let flush: Option<String> = row.get_by_name("flush_lsn")?;
            let replay: Option<String> = row.get_by_name("replay_lsn")?;

            clients[count] = ReplicationClient {
                application_name: heapless::String::<64>::try_from(app.as_deref().unwrap_or(""))
                    .unwrap_or_default(),
                client_addr: Some(
                    heapless::String::<64>::try_from(addr.as_deref().unwrap_or(""))
                        .unwrap_or_default(),
                ),
                state: Some(
                    heapless::String::<32>::try_from(state.as_deref().unwrap_or(""))
                        .unwrap_or_default(),
                ),
                sent_lsn: Some(
                    heapless::String::<32>::try_from(sent.as_deref().unwrap_or(""))
                        .unwrap_or_default(),
                ),
                write_lsn: Some(
                    heapless::String::<32>::try_from(write.as_deref().unwrap_or(""))
                        .unwrap_or_default(),
                ),
                flush_lsn: Some(
                    heapless::String::<32>::try_from(flush.as_deref().unwrap_or(""))
                        .unwrap_or_default(),
                ),
                replay_lsn: Some(
                    heapless::String::<32>::try_from(replay.as_deref().unwrap_or(""))
                        .unwrap_or_default(),
                ),
                write_lag_seconds: row.get_by_name("write_lag_seconds")?,
                flush_lag_seconds: row.get_by_name("flush_lag_seconds")?,
                replay_lag_seconds: row.get_by_name("replay_lag_seconds")?,
                lsn_gap_bytes: row.get_by_name("lsn_gap_bytes")?,
            };
            count += 1;
        }
        Ok((clients, count))
    })
}

pub fn collect_replica_time_lag() -> Option<f64> {
    let host = guc::REPLICA_HOST.get()?.to_str().ok()?.to_string();
    let port = guc::REPLICA_PORT.get();
    let dbname = guc::REPLICA_DBNAME.get()?.to_str().ok()?.to_string();
    let user = guc::REPLICA_USER.get()?.to_str().ok()?.to_string();
    let password = guc::REPLICA_PASSWORD.get()?.to_str().ok()?.to_string();
    let ssl = if guc::REPLICA_SSL_MODE.get() {
        "require"
    } else {
        "disable"
    };
    // Cant use postgres crate in BGW as it spawns its own threads and postgres crate is not designed to be used in a BGW
    // use dblink or libpq to connect to the replica and run the query to get the replay lag

    let conn_str = CString::new(format!(
        "host={} port={} dbname={} user={} password={} sslmode={}",
        host, port, dbname, user, password, ssl
    ))
    .expect("Failed to create CString");

    unsafe {
        let conn: *mut PGconn = PQconnectdb(conn_str.as_ptr());
        // Verify connection safety status
        if PQstatus(conn) != ConnStatusType::CONNECTION_OK {
            let err_ptr = PQerrorMessage(conn);
            let err_message = CStr::from_ptr(err_ptr).to_string_lossy();
            eprintln!("Connection failed: {}", err_message);

            PQfinish(conn);
            return None;
        }
        println!("Connected successfully to PostgreSQL via raw libpq!");

        let query = c"SELECT EXTRACT(EPOCH FROM (now() - pg_last_xact_replay_timestamp()))::float8 AS replay_lag_seconds";

        let result = PQexec(conn, query.as_ptr());

        let replay_lag_seconds = if PQresultStatus(result) == ExecStatusType::PGRES_TUPLES_OK
            && PQntuples(result) > 0
            && PQnfields(result) > 0
        {
            let val_ptr: *mut i8 = PQgetvalue(result, 0, 0);
            CStr::from_ptr(val_ptr)
                .to_str()
                .ok()
                .and_then(|s| s.parse::<f64>().ok())
        } else {
            None
        };

        PQclear(result);
        PQfinish(conn);
        return replay_lag_seconds;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    async fn test_collect_local_metrics() -> anyhow::Result<()> {
        // TODO: Test case
        // let config = crate::config::load_config("config.yaml")?;
        // let replica_client = crate::db::replica::connect(&config.replica)
        //     .await
        //     .expect("Failed to connect to replica database");
        // let metrics = collect_replica_metrics(&replica_client).await?;
        // println!("Replication Metrics: {:?}", metrics);
        Ok(())
    }
}
