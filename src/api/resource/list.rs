use std::sync::Arc;

use crate::{
    actions::resource::list::get_user_resources,
    error::{ApiErrResp, Result},
    extractors::authenticator::Authenticated,
    models::session::Session,
};
use axum::{extract::State, response::IntoResponse, Json};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};

pub async fn list(
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
