use std::sync::Arc;

use crate::{
    actions::resource::list::{get_resource_children, get_user_resources},
    error::{ApiErrResp, Result},
    extractors::authenticator::Authenticated,
    models::{session::Session, READ, RESOURCE},
    utils::db::check_has_privilege,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use hyper::StatusCode;
use uuid::Uuid;

pub async fn list_resources_handler(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Authenticated(Session { user_id, .. }): Authenticated,
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;
    let role_resources = get_user_resources(&mut conn, user_id).await?;

    Ok(Json(role_resources))
}

pub async fn list_child_resources_handler(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Authenticated(Session { user_id, .. }): Authenticated,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;

    if !check_has_privilege(&mut conn, user_id, id, READ, RESOURCE, None, None).await? {
        Err(ApiErrResp {
            code: StatusCode::FORBIDDEN,
            error: "Forbidden".to_string(),
            message: "You don't have permission to access this resource".to_string(),
        })?;
    }

    // if the user has read access to the parent
    // then he also have read access to the children as well
    let child_resources = get_resource_children(&mut conn, id).await?;

    Ok(Json(child_resources))
}
