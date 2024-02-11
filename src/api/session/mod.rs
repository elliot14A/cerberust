use std::sync::Arc;

use axum::{routing::post, Router};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};

use self::logout::logout;

mod logout;
mod refresh;

use refresh::refresh;

pub fn init_session_routes() -> Router<Arc<Pool<AsyncPgConnection>>> {
    Router::new()
        .route("/logout", post(logout))
        .route("/refresh", post(refresh))
}
