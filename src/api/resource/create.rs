use std::sync::Arc;

use axum::{extract::State, http, response::IntoResponse, Json};
use diesel_async::{
    pooled_connection::bb8::Pool, scoped_futures::ScopedFutureExt, AsyncConnection,
    AsyncPgConnection,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    actions::{
        relation::create::create_relation, resource::create::create_resource,
        role::details::get_role_id_by_name, user_role::create::create_user_role,
    },
    error::{ApiErrResp, Result},
    extractors::{authenticator::Authenticated, FromValidatedJson},
    models::{
        relation::NewRelation,
        resource::{NewResource, Resource},
        session::Session,
        user_role::NewUserRole,
        CREATE, RESOURCE, ROOT_ROLE,
    },
    utils::db::check_has_privilege,
};

#[derive(Deserialize, Validate)]
pub struct RequestBody {
    pub parent_resource_id: Option<Uuid>,
    #[validate(length(min = 3, max = 24))]
    pub name: String,
    #[validate(length(min = 3, max = 255))]
    pub description: Option<String>,
}

pub async fn create_resource_handler(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Authenticated(Session { user_id, .. }): Authenticated,
    FromValidatedJson(new_resource): FromValidatedJson<RequestBody>,
) -> Result<impl IntoResponse> {
    let new_resource = NewResource {
        parent_resource_id: None,
        name: new_resource.name,
        description: new_resource.description,
    };

    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;

    let resource = conn
        .transaction::<Resource, ApiErrResp, _>(|conn| {
            Box::pin(
                async move { create_resource_and_assign_role(conn, new_resource, user_id).await },
            )
        })
        .scope_boxed()
        .await?;

    Ok(Json(resource))
}

pub async fn create_child_resource_handler(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Authenticated(Session { user_id, .. }): Authenticated,
    FromValidatedJson(new_resource): FromValidatedJson<RequestBody>,
) -> Result<impl IntoResponse> {
    let new_resource = NewResource {
        parent_resource_id: new_resource.parent_resource_id,
        name: new_resource.name,
        description: new_resource.description,
    };

    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;

    let parent_resource_id = new_resource.parent_resource_id.unwrap();

    let has_privilege = check_has_privilege(
        &mut conn,
        user_id,
        parent_resource_id,
        CREATE,
        RESOURCE,
        None,
        None,
    )
    .await?;

    if !has_privilege {
        return Err(ApiErrResp {
            code: http::StatusCode::FORBIDDEN,
            error: "FORBIDDEN".to_string(),
            message: "You do not have permission to create a resource".to_string(),
        });
    }

    let resource = conn
        .transaction::<Resource, ApiErrResp, _>(|conn| {
            Box::pin(
                async move { create_resource_and_assign_role(conn, new_resource, user_id).await },
            )
        })
        .scope_boxed()
        .await?;

    Ok(Json(resource))
}

async fn create_resource_and_assign_role(
    conn: &mut AsyncPgConnection,
    new_resource: NewResource,
    user_id: Uuid,
) -> Result<Resource> {
    let resource = create_resource(conn, new_resource).await?;
    let role_id = get_role_id_by_name(conn, ROOT_ROLE).await?;
    if role_id.is_none() {
        return Err(ApiErrResp {
            code: http::StatusCode::INTERNAL_SERVER_ERROR,
            error: "INTERNAL_SERVER_ERROR".to_string(),
            message: "Role not found".to_string(),
        });
    }

    let role_id = role_id.unwrap();

    let new_user_role = NewUserRole { user_id, role_id };
    create_user_role(conn, new_user_role).await?;

    let new_relation = NewRelation {
        user_id,
        object_id: resource.id,
        role_id,
    };
    create_relation(conn, new_relation).await?;

    Ok(resource)
}
