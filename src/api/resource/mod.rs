use std::sync::Arc;

use axum::{
    routing::{delete, post},
    Router,
};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};

use self::{
    create::{create_child_resource_handler, create_resource_handler},
    delete::delete_resource_hadler,
    details::details_handler,
    list::list,
    update::update_handler,
};

mod create;
mod delete;
mod details;
mod list;
mod update;

pub fn init_resource_routes() -> Router<Arc<Pool<AsyncPgConnection>>> {
    Router::new()
        .route("/", post(create_resource_handler).get(list))
        .route(
            "/:id",
            delete(delete_resource_hadler)
                .get(details_handler)
                .patch(update_handler),
        )
        .route("/child", post(create_child_resource_handler))
}
