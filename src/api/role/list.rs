use std::sync::Arc;

use axum::Json;
use axum::{extract::State, response::IntoResponse};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};

use crate::actions::role::list::{get_all_default_roles, get_custom_roles};
use crate::error::{ApiErrResp, Result};
use crate::extractors::authenticator::Authenticated;
use crate::models::session::Session;

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
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;

    let custom_roles = get_custom_roles(&mut conn, user_id).await?;

    Ok(Json(custom_roles))
}
