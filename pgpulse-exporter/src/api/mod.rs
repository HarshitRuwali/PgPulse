use std::sync::Arc;
mod routes;
// mod storage;
use crate::storage::metrics;
use axum::{Router, routing::get};

pub fn create_router(
    metrics: Arc<metrics::Metrics>,
    client: Arc<tokio_postgres::Client>,
) -> Router {
    Router::new()
        .route("/health", get(routes::health_handler))
        .route("/metrics", get(routes::metrics_handler))
        .route(
            "/replication_status",
            get(routes::replication_status_handler),
        )
        .with_state((metrics, client))
}
