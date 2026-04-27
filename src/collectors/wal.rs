use crate::models::{PrimaryMetrics, ReplicationClient};
use chrono::Utc;
use tokio_postgres::Client;

pub async fn collect_primary_metrics(client: &Client) -> anyhow::Result<PrimaryMetrics> {
    let result = client
        .query(
            "SELECT 
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
         FROM pg_stat_replication",
            &[],
        )
        .await?;

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

    #[tokio::test]
    async fn test_collect_primary_metrics() -> anyhow::Result<()> {
        let config = crate::config::load_config("config.yaml")?;
        let primary_client = crate::db::primary::connect(&config.primary)
            .await
            .expect("Failed to connect to primary database");
        let metrics = collect_primary_metrics(&primary_client).await?;
        println!("Primary Metrics: {:?}", metrics);
        Ok(())
    }
}
