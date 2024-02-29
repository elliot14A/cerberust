use std::sync::Arc;

use axum::{routing::post, Router};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};

use self::create::create_custom_role_handler;

mod create;
mod details;
mod grant;
mod list;
mod revoke;
mod update;

pub fn init_role_routes() -> Router<Arc<Pool<AsyncPgConnection>>> {
    Router::new().route("/custom", post(create_custom_role_handler))
}
