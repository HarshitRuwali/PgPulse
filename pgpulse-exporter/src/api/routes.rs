use crate::storage::metrics;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::{Value, json};
use std::sync::Arc;
use tokio_postgres::Client;
use tracing::info;

pub async fn health_handler() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "message": "PgPulse is running"
    }))
}

pub async fn replication_status_handler(
    State((_, client)): State<(Arc<metrics::Metrics>, Arc<Client>)>,

) -> impl IntoResponse {
    let query = "SELECT * FROM pgpulse.replication_status";

    let rows = match client.query(query, &[]).await {
        Ok(rows) => rows,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("failed to query replication status: {}", e) })),
            )
                .into_response();
        }
    };

    let mut result = Vec::new();

    for row in rows {
        let replica_name: String = row.get("replica_name");
        let replay_lag_seconds: Option<f64> = row.get("replay_lag_seconds");
        let lsn_gap_bytes: Option<i64> = row.get("lsn_gap_bytes");
        let health_status: i32 = row.get("health_status");

        result.push(json!({
            "replica_name": replica_name,
            "replay_lag_seconds": replay_lag_seconds,
            "lsn_gap_bytes": lsn_gap_bytes,
            "health_status": health_status,
        }));
    }
    (
        StatusCode::OK,
        Json(json!({ "replication_status": result })),
    )
        .into_response()
}

pub async fn metrics_handler(
    State((metrics, client)): State<(Arc<metrics::Metrics>, Arc<Client>)>,
) -> impl IntoResponse {
    info!("Received request for replication metrics from Prometheus");

    let query =
        "SELECT replica_name, replay_lag_seconds, lsn_gap_bytes FROM pgpulse.replication_status";

    let rows = match client.query(query, &[]).await {
        Ok(rows) => rows,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("failed to query replication metrics: {}", e) })),
            )
                .into_response();
        }
    };

    for row in rows {
        let name: &str = row.get("replica_name");
        let lag: f64 = row.get("replay_lag_seconds");
        let lsn_gap: f64 = row.get("lsn_gap_bytes");

        metrics
            .replication_lag_seconds
            .with_label_values(&[&name])
            .set(lag);
        metrics
            .lsn_gap_bytes
            .with_label_values(&[&name])
            .set(lsn_gap);
    }

    // for health status
    let health_query = "SELECT pgpulse_health_status()";
    match client.query_one(health_query, &[]).await {
        Ok(row) => {
            let status: String = row.get(0);
            let code = match status.as_str() {
                "Warning" => 1,
                "Critical" => 2,
                _ => 0,
            };
            metrics
                .health_status
                .with_label_values(&["primary"])
                .set(code);
        }
        Err(e) => {
            eprintln!("Failed to query health status: {}", e);
            metrics.health_status.with_label_values(&["primary"]).set(2); // Critical if we can't determine health
        }
    }

    if let Ok(rows) = client
        .query("SELECT query FROM pgpulse.long_running_queries", &[])
        .await
    {
        metrics.long_running_query_count.set(rows.len() as i64);
    }

    match metrics::gather_as_text() {
        Ok(body) => (StatusCode::OK, body).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "error": format!("failed to encode metrics: {}", e) }).to_string(),
        )
            .into_response(),
    }
}
