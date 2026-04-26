use crate::storage::in_memory::MetricStore;
use axum::{Json, extract::State};
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
        "replication_data": {
            "replay_lag_seconds": snapshot.replication_metrics.replay_lag_seconds,
            "receive_lag_seconds": snapshot.replication_metrics.receive_lag_seconds,
            "replay_lsn": snapshot.replication_metrics.replay_lsn,
            "lsn_gap_bytes": snapshot.replication_metrics.lsn_gap_bytes,
            "in_recovery": snapshot.replication_metrics.in_recovery,
            "collected_at": snapshot.replication_metrics.collected_at,
            "replication_clients": snapshot.primary_metrics.replication_clients.iter().map(|client| {
                json!({
                    "application_name": client.application_name,
                    "client_addr": client.client_addr,
                    "stage": client.stage,
                    "sent_lsn": client.sent_lsn,
                    "relay_lsn": client.relay_lsn,
                    "collected_at": snapshot.primary_metrics.collected_at
                })
            }).collect::<Vec<_>>(),
        },
        "collected_at": snapshot.collected_at
    }))
}
