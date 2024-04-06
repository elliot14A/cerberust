use std::sync::Arc;

use axum::extract::Path;
use axum::Json;
use axum::{extract::State, response::IntoResponse};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use hyper::StatusCode;
use uuid::Uuid;

use crate::actions::role::list::{get_all_default_roles, get_custom_roles};
use crate::error::{ApiErrResp, Result};
use crate::extractors::authenticator::Authenticated;
use crate::models::session::Session;
use crate::models::{READ, RESOURCE};
use crate::utils::db::check_has_privilege;

pub async fn list_default_roles_handler(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Authenticated(_): Authenticated,
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;
    let default_roles = get_all_default_roles(&mut conn).await?;

    Ok(Json(default_roles))
}

pub async fn list_custom_roles_handler(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Authenticated(Session { user_id, .. }): Authenticated,
    Path(resource_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;

    let has_privilege =
        check_has_privilege(&mut conn, user_id, resource_id, READ, RESOURCE, None, None).await?;

    if !has_privilege {
        return Err(ApiErrResp {
            code: StatusCode::FORBIDDEN,
            error: "FORBIDDEN".to_string(),
            message: "You do not have permission to view roles on this resource".to_string(),
        });
    }

    let custom_roles = get_custom_roles(&mut conn, resource_id).await?;

    Ok(Json(custom_roles))
}
