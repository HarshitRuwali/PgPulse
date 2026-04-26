use axum::{Router, routing::get};
mod routes;

pub fn create_router() -> Router {
    Router::new().route("/health", get(routes::health_handler))
}
