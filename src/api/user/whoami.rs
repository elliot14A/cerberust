use std::sync::Arc;

use crate::{
    actions::user::details::get_user_by_id, error::Result, extractors::authenticator::Authenticated,
};
use axum::{extract::State, response::IntoResponse, Json};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};

use crate::error::ApiErrResp;

pub async fn whoami(
    Authenticated(session): Authenticated,
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;
    let user = get_user_by_id(&mut conn, session.user_id)
        .await?
        .ok_or_else(|| ApiErrResp::unauthorized(Some("User not found".to_string())))?;

    Ok(Json(user))
}
