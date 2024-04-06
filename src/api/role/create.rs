use std::sync::Arc;

use crate::actions::role::create::create_role;
use crate::error::{ApiErrResp, Result};
use crate::extractors::authenticator::Authenticated;
use crate::extractors::FromValidatedJson;
use crate::models::role::{NewRole, PrivilegeVec};
use crate::models::session::Session;
use crate::models::{CREATE, ROLE};
use crate::utils::db::{check_has_privilege, check_privileges_callback};
use crate::utils::helper::filter_privileges;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::AsyncPgConnection;
use hyper::StatusCode;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct RequestBody {
    #[validate(length(min = 3, max = 24))]
    pub name: String,
    #[validate(length(min = 8))]
    pub description: Option<String>,
    pub privileges: PrivilegeVec,
    pub resource_id: Uuid,
}

/// create a custom role on a resource
pub async fn create_custom_role_handler(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Authenticated(Session { user_id, .. }): Authenticated,
    FromValidatedJson(new_role): FromValidatedJson<RequestBody>,
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;

    if new_role.privileges.0.is_empty() {
        return Err(ApiErrResp {
            code: StatusCode::BAD_REQUEST,
            error: String::from("BAD_REQUEST"),
            message: String::from("Privileges cannot be empty"),
        });
    }

    let callback = check_privileges_callback();
    let privileges = filter_privileges(new_role.privileges.0);

    let has_privilege = check_has_privilege(
        &mut conn,
        user_id,
        new_role.resource_id,
        CREATE,
        ROLE,
        Some(callback),
        Some(privileges.0.clone()),
    )
    .await?;

    if !has_privilege {
        return Err(ApiErrResp {
            code: StatusCode::FORBIDDEN,
            error: String::from("FORBIDDEN"),
            message: String::from(
                "You don't have enough privileges to create a custom role on this resource",
            ),
        });
    }

    // check if the privileges are subset or equal to the user privileges
    // if not return an error

    let new_role = NewRole {
        name: new_role.name,
        description: new_role.description,
        privileges,
        resource_id: Some(new_role.resource_id),
        is_default: false,
    };

    let role = create_role(&mut conn, new_role).await?;

    Ok(Json(role))
}
