use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct ReplicationMetrics {
    pub replay_lag_seconds: Option<i64>,
    pub receive_lag_seconds: Option<i64>,
    pub replay_lsn: Option<String>,
    pub lsn_gap_bytes: Option<i64>,
    pub in_recovery: bool,
    pub collected_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ReplicationClient {
    pub application_name: String,
    pub client_addr: Option<String>,
    pub stage: String,
    pub sent_lsn: Option<String>,
    pub relay_lsn: Option<String>,
}

#[derive(Debug)]
pub struct PrimaryMetrics {
    pub replication_clients: Vec<ReplicationClient>,
    pub collected_at: DateTime<Utc>,
}
