use std::sync::Arc;

use crate::actions::relation::create::create_relation;
use crate::actions::role::create::create_role;
use crate::actions::role::details::get_role_id_by_name;
use crate::error::{ApiErrResp, Result};
use crate::extractors::authenticator::Authenticated;
use crate::extractors::FromValidatedJson;
use crate::models::relation::NewRelation;
use crate::models::role::{NewRole, PrivilegeVec, Role};
use crate::models::session::Session;
use crate::models::{CREATE, ROLE, ROOT_ROLE};
use crate::utils::db::{check_has_privilege, check_privileges_callback};
use crate::utils::helper::filter_privileges;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, AsyncPgConnection};
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
        is_default: false,
    };

    let role = conn
        .transaction::<Role, ApiErrResp, _>(|conn| {
            Box::pin(async move {
                let role = create_role(conn, new_role).await?;
                let root_role_id = get_role_id_by_name(conn, ROOT_ROLE).await?.unwrap();
                let new_relation = NewRelation {
                    user_id,
                    role_id: root_role_id,
                    object_id: role.id,
                };
                create_relation(conn, new_relation).await?;
                Ok(role)
            })
        })
        .scope_boxed()
        .await?;
    Ok(Json(role))
}
