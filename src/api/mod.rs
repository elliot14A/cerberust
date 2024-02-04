mod forgot;
mod login;
mod logout;
mod refresh;
mod register;
mod resend;
mod verify;
pub mod whoami;

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use register::register;
use serde::Deserialize;

use self::{
    forgot::{forgot_password_send_email, reset_password},
    login::login,
    logout::logout,
    refresh::refesh,
    resend::resend_verification_email,
    verify::verify,
    whoami::whoami,
};

pub fn init_routes() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/resend", post(resend_verification_email))
        .route("/register", post(register))
        .route("/verify/:token", get(verify).post(verify))
        .route("/forgot_password", post(forgot_password_send_email))
        .route("/reset_password/:token", post(reset_password))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refesh))
        .route("/whoami", get(whoami).post(whoami))
}

#[derive(Debug, Deserialize)]
pub struct VerifyOrResetRequestBody {
    email: String,
}

async fn health() -> impl IntoResponse {
    "OK"
}
