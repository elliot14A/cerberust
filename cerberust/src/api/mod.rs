use axum::{response::IntoResponse, routing::get, Router};
use repositories::DatabaseRepository;

pub fn init_routes<H: DatabaseRepository>() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> impl IntoResponse {
    "OK"
}
