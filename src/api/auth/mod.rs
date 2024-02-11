use std::sync::Arc;

use axum::{routing::post, Router};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};

mod login;
mod register;
mod resend;
mod verify;
use register::register;
use verify::verify;

use self::{login::login, resend::resend_verification_email};

pub fn init_auth_routes() -> Router<Arc<Pool<AsyncPgConnection>>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/resend", post(resend_verification_email))
        .route("/verify/:token", post(verify).get(verify))
}
