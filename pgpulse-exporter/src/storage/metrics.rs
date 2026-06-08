use prometheus::{
    Encoder, GaugeVec, IntGaugeVec, TextEncoder, register_gauge_vec, register_int_gauge_vec,
};

pub struct Metrics {
    pub replication_lag_seconds: GaugeVec,
    pub lsn_gap_bytes: GaugeVec,
    pub health_status: IntGaugeVec, // 0=Healthy 1=Warning 2=Critical
    pub long_running_query_count: prometheus::IntGauge,
}

impl Metrics {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            replication_lag_seconds: register_gauge_vec!(
                "pgpulse_replication_lag_seconds",
                "Replay lag of each replica in seconds",
                &["replica_name"]
            )?,
            lsn_gap_bytes: register_gauge_vec!(
                "pgpulse_lsn_gap_bytes",
                "LSN gap between primary and replica in bytes",
                &["replica_name"]
            )?,
            health_status: register_int_gauge_vec!(
                "pgpulse_health_status",
                "Current health status: 0=Healthy 1=Warning 2=Critical",
                &["node"]
            )?,
            long_running_query_count: prometheus::register_int_gauge!(
                "pgpulse_long_running_queries",
                "Number of queries exceeding the long-running threshold"
            )?,
        })
    }
}

/// Gathers all registered Prometheus metrics and encodes them as UTF-8 text.
pub fn gather_as_text() -> anyhow::Result<String> {
    let encoder = TextEncoder::new();
    let families = prometheus::gather();
    let mut buf = Vec::new();
    encoder.encode(&families, &mut buf)?;
    Ok(String::from_utf8(buf)?)
}
