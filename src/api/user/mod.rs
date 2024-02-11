mod whoami;
use std::sync::Arc;

use axum::{routing::post, Router};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};

use self::whoami::whoami;

pub fn init_user_routes() -> Router<Arc<Pool<AsyncPgConnection>>> {
    Router::new().route("/whoami", post(whoami))
}
