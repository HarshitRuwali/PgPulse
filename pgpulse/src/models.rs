use heapless;
use pgrx::prelude::*;

use serde::Serialize;

#[derive(Debug, Clone, Default)]
pub struct ReplicationMetrics {
    pub replay_lag_seconds: Option<i64>,
    pub receive_lag_seconds: Option<i64>,
    pub replay_lsn: Option<heapless::String<32>>,
    pub lsn_gap_bytes: Option<i64>,
    pub in_recovery: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ReplicationClient {
    pub application_name: heapless::String<64>,
    pub client_addr: Option<heapless::String<64>>,
    pub state: Option<heapless::String<32>>,
    pub sent_lsn: Option<heapless::String<32>>,
    pub write_lsn: Option<heapless::String<32>>,
    pub flush_lsn: Option<heapless::String<32>>,
    pub replay_lsn: Option<heapless::String<32>>,
    pub write_lag_seconds: Option<f64>,
    pub flush_lag_seconds: Option<f64>,
    pub replay_lag_seconds: Option<f64>,
    pub lsn_gap_bytes: Option<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct PrimaryMetrics {
    pub replication_clients: Vec<ReplicationClient>,
}

#[derive(Debug, Serialize, PartialEq, Eq, Clone, Default, PostgresEnum)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    #[default]
    Healthy,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Default)]
pub struct LongRunningQuery {
    pub query: heapless::String<256>,
    pub duration: f64,
}

#[derive(Debug, Clone, Default)]
pub struct MetricSnapshot {
    // pub replication_metrics: ReplicationMetrics,
    pub replication_clients: heapless::Vec<ReplicationClient, 16>,
    pub health_status: HealthStatus,
    pub collected_at: i64, // Unix timestamp
    pub long_running_queries: heapless::Vec<LongRunningQuery, 16>,
    pub replica_replay_lag_seconds: Option<f64>,
}

// SAFETY: MetricSnapshot is only accessed via PgLwLock which provides
// exclusive locking. All fields must remain in the shared memory segment.
unsafe impl pgrx::PGRXSharedMemory for MetricSnapshot {}
