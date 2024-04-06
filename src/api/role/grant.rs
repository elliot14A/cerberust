use std::sync::Arc;

use crate::{
    actions::{relation::create::create_relation, role::details::get_role_by_id},
    error::{ApiErrResp, Result},
    extractors::authenticator::Authenticated,
    models::{relation::NewRelation, session::Session},
    utils::db::check_privileges_callback,
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

pub async fn grant_role_handler(
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

    let role = get_role_by_id(&mut conn, role_id).await?;

    if role.is_none() {
        return Err(ApiErrResp {
            code: StatusCode::NOT_FOUND,
            error: "NOT_FOUND".to_string(),
            message: format!("Role with id {} not found", role_id),
        });
    }
    let role = role.unwrap();

    if !role.is_default {
        if role.resource_id != Some(resource_id) {
            return Err(ApiErrResp {
                code: StatusCode::BAD_REQUEST,
                error: "BAD_REQUEST".to_string(),
                message: "Role does not exist on the resource".to_string(),
            });
        }
    }

    let callback = check_privileges_callback();
    let has_privilege = crate::utils::db::check_has_privilege(
        &mut conn,
        user_id,
        resource_id,
        "grant",
        "role",
        Some(callback),
        Some(role.privileges.0),
    )
    .await?;

    if !has_privilege {
        return Err(ApiErrResp::unauthorized(Some(
            "You don't have privilege to grant role".to_owned(),
        )));
    }

    let new_relation = NewRelation {
        user_id: assignee,
        role_id,
        resource_id,
    };

    let relation = create_relation(&mut conn, new_relation).await?;
    Ok(Json(relation))
}
