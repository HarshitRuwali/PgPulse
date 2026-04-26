use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Default)]
pub struct ReplicationMetrics {
    pub replay_lag_seconds: Option<i64>,
    pub receive_lag_seconds: Option<i64>,
    pub replay_lsn: Option<String>,
    pub lsn_gap_bytes: Option<i64>,
    pub in_recovery: bool,
    pub collected_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct ReplicationClient {
    pub application_name: String,
    pub client_addr: Option<String>,
    pub stage: String,
    pub sent_lsn: Option<String>,
    pub relay_lsn: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PrimaryMetrics {
    pub replication_clients: Vec<ReplicationClient>,
    pub collected_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, PartialEq, Eq, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    #[default]
    Healthy,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Default)]
pub struct MetricSnapshot {
    pub replication_metrics: ReplicationMetrics,
    pub primary_metrics: PrimaryMetrics,
    pub health_status: HealthStatus,
    pub collected_at: DateTime<Utc>,
}
