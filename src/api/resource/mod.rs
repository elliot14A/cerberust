use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};

use self::{
    create::{create_child_resource_handler, create_resource_handler},
    list::list,
};

mod create;
mod delete;
mod details;
mod list;
mod update;

pub fn init_resource_routes() -> Router<Arc<Pool<AsyncPgConnection>>> {
    Router::new()
        .route("/", post(create_resource_handler))
        .route("/", get(list))
        .route("/child", post(create_child_resource_handler))
}
