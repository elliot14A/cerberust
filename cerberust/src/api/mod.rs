mod forgot;
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
use serde::Deserialize;

use self::{
    forgot::{forgot_password_send_email, reset_password},
    resend::resend_verification_email,
    verify::verify,
};

pub fn init_routes<H: DatabaseRepository>() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/resend", post(resend_verification_email::<H>))
        .route("/register", post(register::<H>))
        .route("/verify/:token", get(verify::<H>))
        .route("/verify/:token", post(verify::<H>))
        .route("/forgot_password", post(forgot_password_send_email::<H>))
        .route("/reset_password/:token", post(reset_password::<H>))
}

#[derive(Debug, Deserialize)]
pub struct VerifyOrResetRequestBody {
    email: String,
}

async fn health() -> impl IntoResponse {
    "OK"
}
