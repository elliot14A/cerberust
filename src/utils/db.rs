use async_recursion::async_recursion;
use diesel_async::AsyncPgConnection;
use uuid::Uuid;

use crate::{
    actions::{
        relation::details::get_role_id_from_relation, resource::details::get_parent_resource_id,
        role::details::get_privilegs_by_role_id,
    },
    error::Result,
    models::role::PrivilegeVec,
};

// firstly check relation table for user and resource and get the role_id
// if user and resource relation not found then
// check if resource has parent resource
// if parent resource found then check user and parent resource relation
// repeat this process until parent resource not found
// if user and resource relation not found then even after checking all parent resources
// return false
#[async_recursion]
/// recursively checks if user has privilege on resource or the parent resource
pub async fn check_has_privilege(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
    resource_id: Uuid,
    privilege: &str,
    entity: &str,
) -> Result<bool> {
    let role_id = get_role_id_from_relation(conn, user_id, resource_id).await?;

    if let Some(role_id) = role_id {
        // TODO: this query can be optimized
        let PrivilegeVec(privileges) = get_privilegs_by_role_id(conn, role_id).await?;

        return Ok(privileges.iter().any(|p| {
            p.entity == entity
                && (p.privileges.contains(&privilege.to_owned())
                    || p.privileges.contains(&"*".to_string()))
        }));
    }

    let parent_resource_id = get_parent_resource_id(conn, resource_id).await?;

    if parent_resource_id.is_some() {
        return check_has_privilege(
            conn,
            user_id,
            parent_resource_id.unwrap(),
            privilege,
            entity,
        )
        .await;
    }

    Ok(false)
}
