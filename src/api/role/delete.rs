use std::sync::Arc;

use crate::{
    actions::role::delete::delete_role,
    error::{ApiErrResp, Result},
    extractors::authenticator::Authenticated,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use hyper::StatusCode;
use uuid::Uuid;

pub async fn delete_handler(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Authenticated(_): Authenticated,
    Path(role_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;

    delete_role(&mut conn, role_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
