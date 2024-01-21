mod register;

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use register::register;
use repositories::DatabaseRepository;

pub fn init_routes<H: DatabaseRepository>() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/register", post(register::<H>))
}

async fn health() -> impl IntoResponse {
    "OK"
}
