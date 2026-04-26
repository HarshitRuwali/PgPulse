use crate::config::ThresholdConfig;
use crate::models::ReplicationMetrics;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
}

pub fn evaluate_health(replica: &ReplicationMetrics, threshold: &ThresholdConfig) -> HealthStatus {
    // let Some(metrics) = replica else {
    //     return HealthStatus::Critical; // If metrics are missing, consider it critical
    // };

    match replica.replay_lag_seconds {
        Some(lag)
            if lag >= threshold.replay_lag_warning_seconds as i64
                && lag < threshold.replay_lag_critical_seconds as i64 =>
        {
            HealthStatus::Warning
        }
        Some(lag) if lag >= threshold.replay_lag_critical_seconds as i64 => HealthStatus::Critical,
        _ => HealthStatus::Healthy,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ReplicationMetrics;

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
        let health_status = evaluate_health(&replica_metrics, &threshold);
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

        let health_status = evaluate_health(&replica_metrics, &threshold);
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
        let health_status = evaluate_health(&replica_metrics, &threshold);
        assert_eq!(health_status, HealthStatus::Critical);
    }
}
