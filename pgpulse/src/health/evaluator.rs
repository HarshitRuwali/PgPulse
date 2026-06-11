// use crate::config::ThresholdConfig;
use crate::guc;
use crate::models::{HealthStatus, MetricSnapshot};

pub fn evaluate_health(snapshot: &MetricSnapshot) -> HealthStatus {
    // threshold config now will be read from the GUC's instead of the config
    let lsn_gap_critical = guc::LSN_GAP_CRITICAL_BYTES.get();
    let lsn_gap_warning = guc::LSN_GAP_WARNING_BYTES.get();
    let replay_lag_critical = guc::REPLAY_LAG_CRITICAL_SECONDS.get() as f64;
    let replay_lag_warning = guc::REPLAY_LAG_WARNING_SECONDS.get() as f64;

    // No replicas connected — warn but don't critical (replica may not exist yet)
    if snapshot.replication_clients.is_empty() {
        return HealthStatus::Warning;
    }

    let mut worst = HealthStatus::Healthy;

    // Evaluate per-client byte lag from pg_stat_replication
    for client in &snapshot.replication_clients {
        let gap_status = match client.lsn_gap_bytes {
            Some(gap) if gap >= lsn_gap_critical as i64 => HealthStatus::Critical,
            Some(gap) if gap >= lsn_gap_warning as i64 => HealthStatus::Warning,
            _ => HealthStatus::Healthy,
        };
        if gap_status == HealthStatus::Critical {
            return HealthStatus::Critical;
        }
        if gap_status == HealthStatus::Warning {
            worst = HealthStatus::Warning;
        }
    }

    // Evaluate time-based lag from replica (if available)
    if let Some(lag) = snapshot.replica_replay_lag_seconds {
        if lag >= replay_lag_critical {
            return HealthStatus::Critical;
        }
        if lag >= replay_lag_warning {
            worst = HealthStatus::Warning;
        }
    }
    worst
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{HealthStatus, MetricSnapshot, ReplicationClient};

    fn snapshot_with_gap(lsn_gap_bytes: Option<i64>) -> MetricSnapshot {
        let mut s = MetricSnapshot::default();
        let mut client = ReplicationClient::default();
        client.lsn_gap_bytes = lsn_gap_bytes;
        s.replication_clients.push(client).ok();
        s
    }

    #[test]
    fn test_healthy() {
        let s = snapshot_with_gap(Some(1024));
        assert_eq!(evaluate_health(&s), HealthStatus::Healthy);
    }

    #[test]
    fn test_warning_lsn_gap() {
        let s = snapshot_with_gap(Some(10 * 1024 * 1024)); // 10 MB — at warning threshold
        assert_eq!(evaluate_health(&s), HealthStatus::Warning);
    }

    #[test]
    fn test_critical_lsn_gap() {
        let s = snapshot_with_gap(Some(200 * 1024 * 1024)); // 200 MB — above critical threshold
        assert_eq!(evaluate_health(&s), HealthStatus::Critical);
    }

    #[test]
    fn test_no_clients_is_warning() {
        assert_eq!(
            evaluate_health(&MetricSnapshot::default()),
            HealthStatus::Warning
        );
    }

    #[test]
    fn test_critical_replay_lag() {
        let mut s = snapshot_with_gap(Some(0));
        s.replica_replay_lag_seconds = Some(120.0); // above 60s critical default
        assert_eq!(evaluate_health(&s), HealthStatus::Critical);
    }
}
