mod forgot;
use std::sync::Arc;

use axum::{routing::post, Router};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use forgot::{forgot_password_send_email, reset_password};

pub fn init_password_routes() -> Router<Arc<Pool<AsyncPgConnection>>> {
    Router::new()
        .route("/forgot", post(forgot_password_send_email))
        .route("/reset/:token", post(reset_password))
}
