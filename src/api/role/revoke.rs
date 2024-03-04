use std::sync::Arc;

use crate::{
    actions::{relation::delete::delete_relation, role::details::get_role_by_id},
    error::{ApiErrResp, Result},
    extractors::authenticator::Authenticated,
    models::{session::Session, REVOKE, ROLE},
    utils::db::{check_has_privilege, check_privileges_callback},
};
use axum::{extract::State, response::IntoResponse, Json};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use hyper::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    user_id: Uuid,
    role_id: Uuid,
    resource_id: Uuid,
}

pub async fn revoke_role_handler(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Authenticated(Session { user_id, .. }): Authenticated,
    Json(RequestBody {
        user_id: assignee,
        role_id,
        resource_id,
    }): Json<RequestBody>,
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;

    let callback = check_privileges_callback();
    let role = get_role_by_id(&mut conn, role_id).await?;

    if role.is_none() {
        return Err(ApiErrResp {
            code: StatusCode::NOT_FOUND,
            error: "NOT_FOUND".to_string(),
            message: format!("Role with id {} not found", role_id),
        });
    }

    let role = role.unwrap();

    let privileges = role.privileges.0;

    let has_privilege = check_has_privilege(
        &mut conn,
        user_id,
        resource_id,
        REVOKE,
        ROLE,
        Some(callback),
        Some(privileges.clone()),
    )
    .await?;

    if !has_privilege {
        return Err(ApiErrResp {
            code: StatusCode::FORBIDDEN,
            error: "FORBIDDEN".to_string(),
            message: "You don't have permission to revoke this role".to_string(),
        });
    }

    // delete assignee role relation
    delete_relation(&mut conn, assignee, resource_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
