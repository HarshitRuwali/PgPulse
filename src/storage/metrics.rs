use crate::models::MetricSnapshot;
use prometheus::{Encoder, Gauge, TextEncoder, register_gauge};
use std::sync::LazyLock;

static REPLICA_REPLAY_LAG_SECONDS: LazyLock<Gauge> = LazyLock::new(|| {
    register_gauge!(
        "pg_replica_replay_lag_seconds",
        "Replica WAL replay lag in seconds"
    )
    .expect("Failed to register pg_replica_replay_lag_seconds")
});

static REPLICA_RECEIVE_REPLAY_LSN_GAP_BYTES: LazyLock<Gauge> = LazyLock::new(|| {
    register_gauge!(
        "pg_replica_receive_replay_lsn_gap_bytes",
        "Replica receive vs replay LSN gap in bytes"
    )
    .expect("Failed to register pg_replica_receive_replay_lsn_gap_bytes")
});

static PRIMARY_REPLICATION_CLIENTS: LazyLock<Gauge> = LazyLock::new(|| {
    register_gauge!(
        "pg_primary_replication_clients",
        "Number of replication clients connected to the primary"
    )
    .expect("Failed to register pg_primary_replication_clients")
});

static REPLICA_LONG_RUNNING_QUERIES: LazyLock<Gauge> = LazyLock::new(|| {
    register_gauge!(
        "pg_replica_long_running_queries",
        "Number of long running queries on the replica"
    )
    .expect("Failed to register pg_replica_long_running_queries")
});

static REPLICA_HEALTH_STATUS: LazyLock<Gauge> = LazyLock::new(|| {
    register_gauge!(
        "pg_replica_health_status",
        "Replication health status: 0=healthy, 1=warning, 2=critical"
    )
    .expect("Failed to register pg_replica_health_status")
});

static PRIMARY_MAX_CLIENT_LSN_GAP_BYTES: LazyLock<Gauge> = LazyLock::new(|| {
    register_gauge!(
        "pg_primary_max_client_lsn_gap_bytes",
        "Maximum per-client primary LSN gap in bytes"
    )
    .expect("Failed to register pg_primary_max_client_lsn_gap_bytes")
});

static REPLICA_IN_RECOVERY: LazyLock<Gauge> = LazyLock::new(|| {
    register_gauge!(
        "pg_replica_in_recovery",
        "Whether configured replica is in recovery mode (1=true, 0=false)"
    )
    .expect("Failed to register pg_replica_in_recovery")
});

pub fn update_from_snapshot(snapshot: &MetricSnapshot) {
    REPLICA_REPLAY_LAG_SECONDS
        .set(snapshot.replication_metrics.replay_lag_seconds.unwrap_or(0) as f64);
    REPLICA_RECEIVE_REPLAY_LSN_GAP_BYTES
        .set(snapshot.replication_metrics.lsn_gap_bytes.unwrap_or(0) as f64);
    PRIMARY_REPLICATION_CLIENTS.set(snapshot.primary_metrics.replication_clients.len() as f64);
    REPLICA_LONG_RUNNING_QUERIES.set(snapshot.long_running_queries.len() as f64);
    REPLICA_IN_RECOVERY.set(if snapshot.replication_metrics.in_recovery {
        1.0
    } else {
        0.0
    });

    let health_code = match snapshot.health_status {
        crate::models::HealthStatus::Healthy => 0.0,
        crate::models::HealthStatus::Warning => 1.0,
        crate::models::HealthStatus::Critical => 2.0,
    };
    REPLICA_HEALTH_STATUS.set(health_code);

    let max_client_gap = snapshot
        .primary_metrics
        .replication_clients
        .iter()
        .filter_map(|client| client.lsn_gap_bytes)
        .max()
        .unwrap_or(0);
    PRIMARY_MAX_CLIENT_LSN_GAP_BYTES.set(max_client_gap as f64);
}

pub fn gather_as_text() -> anyhow::Result<String> {
    let metric_families = prometheus::gather();
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer)?;
    let text = String::from_utf8(buffer)?;
    Ok(text)
}
