mod forgot;
mod login;
mod logout;
mod refresh;
mod register;
mod resend;
mod verify;
pub mod whoami;

use std::sync::Arc;

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use register::register;
use serde::Deserialize;
use validator::Validate;

use self::{
    forgot::{forgot_password_send_email, reset_password},
    login::login,
    logout::logout,
    refresh::refesh,
    resend::resend_verification_email,
    verify::verify,
    whoami::whoami,
};

pub fn init_routes(pool: Pool<AsyncPgConnection>) -> Router {
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
        .with_state(Arc::new(pool))
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyOrResetRequestBody {
    #[validate(email)]
    email: String,
}

async fn health() -> impl IntoResponse {
    "OK"
}
