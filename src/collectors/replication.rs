use crate::models::ReplicationMetrics;
use chrono::Utc;
use tokio_postgres::Client;

pub async fn collect_replica_metrics(client: &Client) -> anyhow::Result<ReplicationMetrics> {
    let result = client.query_one(
        "SELECT
            EXTRACT(EPOCH FROM (now() - pg_last_xact_replay_timestamp()))::BIGINT AS replay_lag_seconds,
            NULL::BIGINT AS receive_lag_seconds,
            pg_last_wal_replay_lsn()::text AS replay_lsn,
            pg_wal_lsn_diff(pg_last_wal_receive_lsn(), pg_last_wal_replay_lsn())::BIGINT AS lsn_gap_bytes,
            pg_is_in_recovery() AS in_recovery
        ",
        &[]
    ).await?;

    Ok(ReplicationMetrics {
        replay_lag_seconds: result.get("replay_lag_seconds"),
        receive_lag_seconds: result.get("receive_lag_seconds"),
        replay_lsn: result.get("replay_lsn"),
        lsn_gap_bytes: result.get("lsn_gap_bytes"),
        in_recovery: result.get("in_recovery"),
        collected_at: Utc::now(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_collect_replica_metrics() -> anyhow::Result<()> {
        // TODO: Test case
        let config = crate::config::load_config("config.yaml")?;
        let replica_client = crate::db::replica::connect(&config.replica)
            .await
            .expect("Failed to connect to replica database");
        let metrics = collect_replica_metrics(&replica_client).await?;
        println!("Replication Metrics: {:?}", metrics);
        Ok(())
    }
}
