use crate::guc;
use crate::models::{PrimaryMetrics, ReplicationClient};
use chrono::Utc;
use postgres::{Client, NoTls};

pub fn collect_primary_metrics() -> anyhow::Result<PrimaryMetrics> {
    let query = "SELECT 
            application_name, 
            client_addr::text, 
            state, 
            sent_lsn::text,
            write_lsn::text,
            flush_lsn::text,
            replay_lsn::text,
            EXTRACT(EPOCH FROM write_lag)::float8   AS write_lag_seconds,
            EXTRACT(EPOCH FROM flush_lag)::float8   AS flush_lag_seconds,
            EXTRACT(EPOCH FROM replay_lag)::float8  AS replay_lag_seconds,
            -- For async replication replay_lsn is NULL; fall back to flush_lsn then write_lsn
            pg_wal_lsn_diff(
                pg_current_wal_lsn(),
                COALESCE(replay_lsn, flush_lsn, write_lsn)
            )::bigint AS lsn_gap_bytes
         FROM pg_stat_replication";
    // let result = client
    //     .query(

    //         &[],
    //     )
    //     .await?;

    let host = guc::PEER_HOST
        .get()
        .ok_or_else(|| anyhow::anyhow!("pgpulse.host is not configured"))?
        .to_str()?
        .to_string();

    let port = guc::PEER_PORT.get();
    let dbname = guc::PEER_DBNAME
        .get()
        .ok_or_else(|| anyhow::anyhow!("pgpulse.dbname is not configured"))?
        .to_str()?
        .to_string();
    let user = guc::PEER_USER
        .get()
        .ok_or_else(|| anyhow::anyhow!("pgpulse.user is not configured"))?
        .to_str()?
        .to_string();
    let password = guc::PEER_PASSWORD
        .get()
        .ok_or_else(|| anyhow::anyhow!("pgpulse.password is not configured"))?
        .to_str()?
        .to_string();
    let ssl_mode = guc::PEER_SSL_MODE.get();

    let _tls_mode = if ssl_mode {
        postgres::config::SslMode::Require
    } else {
        postgres::config::SslMode::Disable
    };

    let mut client = Client::connect(&format!("host={host} port={port} dbname={dbname} user={user} password={password} sslmode={ssl_mode}"), NoTls)?;

    let result = client.query(query, &[])?;

    let mut clients = Vec::new();

    for row in result.iter() {
        clients.push(ReplicationClient {
            application_name: row.get("application_name"),
            client_addr: row.get("client_addr"),
            state: row.get("state"),
            sent_lsn: row.get("sent_lsn"),
            write_lsn: row.get("write_lsn"),
            flush_lsn: row.get("flush_lsn"),
            replay_lsn: row.get("replay_lsn"),
            write_lag_seconds: row.get("write_lag_seconds"),
            flush_lag_seconds: row.get("flush_lag_seconds"),
            replay_lag_seconds: row.get("replay_lag_seconds"),
            lsn_gap_bytes: row.get("lsn_gap_bytes"),
        })
    }

    Ok(PrimaryMetrics {
        replication_clients: clients,
        collected_at: Utc::now(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_primary_metrics_requires_peer_host_guc() {
        let err = collect_primary_metrics().expect_err("expected missing GUC error");
        assert!(err.to_string().contains("pgpulse.host is not configured"));
    }
}
