use std::sync::Arc;

use axum::{routing::post, Router};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};

use self::create::create_resource_handler;

mod create;

pub fn init_resource_routes() -> Router<Arc<Pool<AsyncPgConnection>>> {
    Router::new().route("/create", post(create_resource_handler))
}
