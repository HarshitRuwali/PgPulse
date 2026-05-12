use crate::models::ReplicationMetrics;
use chrono::Utc;
use pgrx::spi::{Spi, SpiError, SpiTupleTable};
/// Use spi instead of tokio_postgres to query the replica metrics from within the PostgreSQL extension

pub async fn collect_replica_metrics() -> Result<ReplicationMetrics, SpiError> {
    let query = "
        SELECT
        EXTRACT(EPOCH FROM (now() - pg_last_xact_replay_timestamp()))::BIGINT AS replay_lag_seconds,
        NULL::BIGINT AS receive_lag_seconds,
        pg_last_wal_replay_lsn()::text AS replay_lsn,
        pg_wal_lsn_diff(pg_last_wal_receive_lsn(), pg_last_wal_replay_lsn())::BIGINT AS lsn_gap_bytes,
        pg_is_in_recovery() AS in_recovery";

    Spi::connect(|client| {
        let row: SpiTupleTable = client.select(query, None, &[])?.first();
        // row is now SpiTupbleTable object, we can extract the columns by name
        Ok(ReplicationMetrics {
            replay_lag_seconds: row.get_by_name("replay_lag_seconds")?,
            receive_lag_seconds: row.get_by_name("receive_lag_seconds")?,
            replay_lsn: row.get_by_name("replay_lsn")?,
            lsn_gap_bytes: row.get_by_name("lsn_gap_bytes")?,
            in_recovery: row.get_by_name("in_recovery")?.unwrap_or(false),
            collected_at: Utc::now(),
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    async fn test_collect_replica_metrics() -> anyhow::Result<()> {
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
