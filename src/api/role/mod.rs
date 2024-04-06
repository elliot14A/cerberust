use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};

use self::{
    create::create_custom_role_handler,
    delete::delete_handler,
    details::{get_role_details_handler, get_role_privileges_handler},
    list::{list_custom_roles_handler, list_default_roles_handler},
};

mod create;
mod delete;
mod details;
mod grant;
mod list;
mod revoke;
mod update;

pub fn init_role_routes() -> Router<Arc<Pool<AsyncPgConnection>>> {
    Router::new()
        .route("/custom", post(create_custom_role_handler))
        .route("/custom/:resource_id", get(list_custom_roles_handler))
        .route("/default", get(list_default_roles_handler))
        .route("/grant", post(grant::grant_role_handler))
        .route("/revoke", post(revoke::revoke_role_handler))
        .route(
            "/:role_id",
            get(get_role_details_handler).delete(delete_handler),
        )
        .route("/:role_id/privileges", get(get_role_privileges_handler))
}
