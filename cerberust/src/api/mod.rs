mod register;
mod verify;

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use register::register;
use repositories::DatabaseRepository;

use self::verify::verify;

pub fn init_routes<H: DatabaseRepository>() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/verify/:token", get(verify::<H>))
        .route("/verify/:token", post(verify::<H>))
        .route("/register", post(register::<H>))
}

async fn health() -> impl IntoResponse {
    "OK"
}
