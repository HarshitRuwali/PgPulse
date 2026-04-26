use axum::{Router, routing::get};
mod routes;

use crate::models::MetricSnapshot;

pub fn create_router(snapshot: MetricSnapshot) -> Router {
    Router::new()
        .route("/health", get(routes::health_handler))
        .route(
            "/replication_status",
            get(routes::replication_status_handler),
        )
        .with_state(snapshot)
}
