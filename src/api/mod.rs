mod routes;

use crate::storage::in_memory::MetricStore;
use axum::{Router, routing::get};

pub fn create_router(snapshot: MetricStore) -> Router {
    Router::new()
        .route("/health", get(routes::health_handler))
        .route("/metrics", get(routes::metrics_handler))
        .route(
            "/replication_status",
            get(routes::replication_status_handler),
        )
        .with_state(snapshot)
}
