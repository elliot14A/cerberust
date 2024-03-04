use std::sync::Arc;

use crate::{
    actions::role::details::{get_privilegs_by_role_id, get_role_by_id},
    error::{ApiErrResp, Result},
    extractors::authenticator::Authenticated,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use uuid::Uuid;

pub async fn get_role_details_handler(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Authenticated(_): Authenticated,
    Path(role_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;
    let role = get_role_by_id(&mut conn, role_id).await?;
    Ok(Json(role))
}

pub async fn get_role_privileges_handler(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Authenticated(_): Authenticated,
    Path(role_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;

    let privileges = get_privilegs_by_role_id(&mut conn, role_id).await?;

    Ok(Json(privileges))
}
