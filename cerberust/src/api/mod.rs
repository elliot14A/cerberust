mod register;
mod resend;
mod verify;

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use register::register;
use repositories::DatabaseRepository;

use self::{resend::resend_verification_email, verify::verify};

pub fn init_routes<H: DatabaseRepository>() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/resend", post(resend_verification_email::<H>))
        .route("/register", post(register::<H>))
        .route("/verify/:token", get(verify::<H>))
        .route("/verify/:token", post(verify::<H>))
}

async fn health() -> impl IntoResponse {
    "OK"
}
