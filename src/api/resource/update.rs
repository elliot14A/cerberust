use std::sync::Arc;

use crate::{
    actions::resource::update::update_resource,
    error::{ApiErrResp, Result},
    extractors::{authenticator::Authenticated, FromValidatedJson},
    models::{session::Session, RESOURCE, UPDATE},
    utils::db::check_has_privilege,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use hyper::StatusCode;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub(crate) struct RequestBody {
    #[validate(length(min = 3, max = 24))]
    name: Option<String>,
    #[validate(length(min = 3, max = 255))]
    description: Option<String>,
}

pub async fn update_handler(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Authenticated(Session { user_id, .. }): Authenticated,
    Path(id): Path<Uuid>,
    FromValidatedJson(body): FromValidatedJson<RequestBody>,
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;

    if !check_has_privilege(&mut conn, user_id, id, UPDATE, RESOURCE, None, None).await? {
        Err(ApiErrResp {
            code: StatusCode::FORBIDDEN,
            error: "Forbidden".to_string(),
            message: "You don't have permission to access this resource".to_string(),
        })?;
    }

    let update_resource = update_resource(&mut conn, id, body.name, body.description).await?;

    if update_resource.is_none() {
        return Err(ApiErrResp {
            code: StatusCode::NOT_FOUND,
            error: "Not Found".to_string(),
            message: format!("Resource with id {} not found", id),
        });
    }

    Ok(Json(update_resource))
}
