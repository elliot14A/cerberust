use std::sync::Arc;

use crate::{
    actions::resource::delete::delete_resource,
    error::{ApiErrResp, Result},
    extractors::authenticator::Authenticated,
    models::{session::Session, DELETE, RESOURCE},
    utils::db::check_has_privilege,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use hyper::StatusCode;
use uuid::Uuid;

pub async fn delete_resource_hadler(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Authenticated(Session { user_id, .. }): Authenticated,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;

    let has_privilege =
        check_has_privilege(&mut conn, user_id, id, DELETE, RESOURCE, None, None).await?;

    if !has_privilege {
        return Err(ApiErrResp::forbidden());
    }

    delete_resource(&mut conn, id).await?;

    Ok(StatusCode::NO_CONTENT)
}
