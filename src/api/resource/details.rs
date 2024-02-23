use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use hyper::StatusCode;
use uuid::Uuid;

use crate::{actions::resource::details::get_resource_by_id, error::ApiErrResp};
use crate::{
    error::Result,
    extractors::authenticator::Authenticated,
    models::{session::Session, READ, RESOURCE},
    utils::db::check_has_privilege,
};

pub async fn details_handler(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Authenticated(Session { user_id, .. }): Authenticated,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;

    if !check_has_privilege(&mut conn, user_id, id, READ, RESOURCE).await? {
        Err(ApiErrResp {
            code: StatusCode::FORBIDDEN,
            error: "Forbidden".to_string(),
            message: "You don't have permission to access this resource".to_string(),
        })?;
    }

    let resource = get_resource_by_id(&mut conn, id).await?;
    if resource.is_none() {
        return Err(ApiErrResp {
            code: StatusCode::NOT_FOUND,
            error: "Not Found".to_string(),
            message: format!("Resource with id {} not found", id),
        });
    }
    Ok(Json(resource))
}
