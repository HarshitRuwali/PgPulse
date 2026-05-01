use crate::storage::{in_memory::MetricStore, metrics};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::{Value, json};
use tracing::info;

pub async fn health_handler() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "message": "PgPulse is running"
    }))
}

pub async fn replication_status_handler(State(snapshot): State<MetricStore>) -> Json<Value> {
    info!("Received request for replication status");
    let snapshot = snapshot.read_snapshot().await;
    Json(json!({
        "status": snapshot.health_status,
        "in_recovery": snapshot.replication_metrics.in_recovery,
        "replica_metrics": {
            "replay_lag_seconds": snapshot.replication_metrics.replay_lag_seconds,
            "receive_lag_seconds": snapshot.replication_metrics.receive_lag_seconds,
            "replay_lsn": snapshot.replication_metrics.replay_lsn,
            "lsn_gap_bytes": snapshot.replication_metrics.lsn_gap_bytes,
            "collected_at": snapshot.replication_metrics.collected_at,
        },
        "replication_clients": snapshot.primary_metrics.replication_clients.iter().map(|client| {
            json!({
                "application_name": client.application_name,
                "client_addr": client.client_addr,
                "state": client.state,
                "sent_lsn": client.sent_lsn,
                "write_lsn": client.write_lsn,
                "flush_lsn": client.flush_lsn,
                "replay_lsn": client.replay_lsn,
                "write_lag_seconds": client.write_lag_seconds,
                "flush_lag_seconds": client.flush_lag_seconds,
                "replay_lag_seconds": client.replay_lag_seconds,
                "lsn_gap_bytes": client.lsn_gap_bytes,
                "collected_at": snapshot.primary_metrics.collected_at
            })
        }).collect::<Vec<_>>(),
        "long_running_queries": snapshot.long_running_queries.iter().map(|query| {
            json!({
                "query": query.query,
                "duration": query.duration,
            })
        }).collect::<Vec<_>>(),
        "collected_at": snapshot.collected_at
    }))
}

pub async fn metrics_handler() -> impl IntoResponse {
    info!("Received request for replication metrics from Prometheus");
    match metrics::gather_as_text() {
        Ok(body) => (
            StatusCode::OK,
            body,
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "error": format!("failed to encode metrics: {}", e) }).to_string(),
        )
            .into_response(),
    }
}
