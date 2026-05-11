use crate::config::ThresholdConfig;
use crate::models::{HealthStatus, PrimaryMetrics, ReplicationMetrics};

pub fn evaluate_health(
    replica: &ReplicationMetrics,
    primary: &PrimaryMetrics,
    threshold: &ThresholdConfig,
) -> HealthStatus {
    // Primary node: replica-side lag functions may not be meaningful, so evaluate via pg_stat_replication.
    if !replica.in_recovery {
        if primary.replication_clients.is_empty() {
            return HealthStatus::Warning;
        }
        let mut worst_health: HealthStatus = HealthStatus::Healthy;
        for client in &primary.replication_clients {
            let lag_status = match client.replay_lag_seconds {
                Some(lag) if lag >= threshold.replay_lag_critical_seconds as f64 => {
                    HealthStatus::Critical
                }
                Some(lag) if lag >= threshold.replay_lag_warning_seconds as f64 => {
                    HealthStatus::Warning
                }
                // NULL lag = async replication, rely on lsn_gap_bytes instead
                _ => HealthStatus::Healthy,
            };

            let gap_status = match client.lsn_gap_bytes {
                Some(gap) if gap >= threshold.lsn_gap_critical_bytes as i64 => {
                    HealthStatus::Critical
                }
                Some(gap) if gap >= threshold.lsn_gap_warning_bytes as i64 => HealthStatus::Warning,
                _ => HealthStatus::Healthy,
            };

            if lag_status == HealthStatus::Critical || gap_status == HealthStatus::Critical {
                return HealthStatus::Critical;
            }
            if lag_status == HealthStatus::Warning || gap_status == HealthStatus::Warning {
                worst_health = HealthStatus::Warning;
            }
        }
        return worst_health;
    }

    // Only valid where db is heavy loaded and replica is actively replaying WAL;
    // Otherwise lag can be near zero even if replication is stalled.
    // match replica.replay_lag_seconds {
    //     Some(lag)
    //         if lag >= threshold.replay_lag_warning_seconds as i64
    //             && lag < threshold.replay_lag_critical_seconds as i64 =>
    //     {
    //         HealthStatus::Warning
    //     }
    //     Some(lag) if lag >= threshold.replay_lag_critical_seconds as i64 => HealthStatus::Critical,
    //     None => HealthStatus::Critical, // Replica is in recovery but has no replay lag — likely stalled
    //     _ => HealthStatus::Healthy,
    // }

    // LSN gap is more reliable for detecting replication stalls,
    // especially for async replicas where replay lag is NULL
    match replica.lsn_gap_bytes {
        Some(gap)
            if gap >= threshold.lsn_gap_warning_bytes as i64
                && gap < threshold.lsn_gap_critical_bytes as i64 =>
        {
            HealthStatus::Warning
        }
        Some(gap) if gap >= threshold.lsn_gap_critical_bytes as i64 => HealthStatus::Critical,
        None => HealthStatus::Critical, // Replica is in recovery but has no LSN gap info — likely stalled
        _ => HealthStatus::Healthy,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ReplicationMetrics;

    fn make_primary_metrics(clients: usize) -> crate::models::PrimaryMetrics {
        crate::models::PrimaryMetrics {
            replication_clients: (0..clients)
                .map(|_| crate::models::ReplicationClient::default())
                .collect(),
            collected_at: chrono::Utc::now(),
        }
    }

    #[test]
    fn test_healthy_evaluate_health() {
        let config = crate::config::load_config("config.yaml").expect("Failed to load config");
        let threshold = config.threshold;
        let replica_metrics = ReplicationMetrics {
            replay_lag_seconds: Some(5),
            receive_lag_seconds: None,
            replay_lsn: None,
            lsn_gap_bytes: None,
            in_recovery: true,
            collected_at: chrono::Utc::now(),
        };
        let health_status = evaluate_health(&replica_metrics, &make_primary_metrics(0), &threshold);
        assert_eq!(health_status, HealthStatus::Healthy);
    }

    #[test]
    fn test_warning_evaluate_health() {
        let config = crate::config::load_config("config.yaml").expect("Failed to load config");
        let threshold = config.threshold;
        let replica_metrics = ReplicationMetrics {
            replay_lag_seconds: Some(15),
            receive_lag_seconds: None,
            replay_lsn: None,
            lsn_gap_bytes: None,
            in_recovery: true,
            collected_at: chrono::Utc::now(),
        };
        let health_status = evaluate_health(&replica_metrics, &make_primary_metrics(0), &threshold);
        assert_eq!(health_status, HealthStatus::Warning);
    }

    #[test]
    fn test_critical_evaluate_health() {
        let config = crate::config::load_config("config.yaml").expect("Failed to load config");
        let threshold = config.threshold;
        let replica_metrics = ReplicationMetrics {
            replay_lag_seconds: Some(65),
            receive_lag_seconds: None,
            replay_lsn: None,
            lsn_gap_bytes: None,
            in_recovery: true,
            collected_at: chrono::Utc::now(),
        };
        let health_status = evaluate_health(&replica_metrics, &make_primary_metrics(0), &threshold);
        assert_eq!(health_status, HealthStatus::Critical);
    }
}
