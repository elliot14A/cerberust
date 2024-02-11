mod auth;
mod password;
mod session;
mod user;

use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Router};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use serde::Deserialize;
use validator::Validate;

use self::{
    auth::init_auth_routes, password::init_password_routes, session::init_session_routes,
    user::init_user_routes,
};

pub fn init_routes(pool: Pool<AsyncPgConnection>) -> Router {
    Router::new()
        .route("/health", get(health))
        .nest("/auth", init_auth_routes())
        .nest("/password", init_password_routes())
        .nest("/user", init_user_routes())
        .nest("/session", init_session_routes())
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
