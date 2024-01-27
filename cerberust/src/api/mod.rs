mod forgot;
mod login;
mod logout;
mod refresh;
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
    login::login,
    logout::logout,
    refresh::refesh,
    resend::resend_verification_email,
    verify::verify,
};

pub fn init_routes<H: DatabaseRepository>() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/resend", post(resend_verification_email::<H>))
        .route("/register", post(register::<H>))
        .route("/verify/:token", get(verify::<H>).post(verify::<H>))
        .route("/forgot_password", post(forgot_password_send_email::<H>))
        .route("/reset_password/:token", post(reset_password::<H>))
        .route("/login", post(login::<H>))
        .route("/logout", post(logout::<H>))
        .route("/refresh", post(refesh::<H>))
}

#[derive(Debug, Deserialize)]
pub struct VerifyOrResetRequestBody {
    email: String,
}

async fn health() -> impl IntoResponse {
    "OK"
}
