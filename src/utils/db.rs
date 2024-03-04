use async_recursion::async_recursion;
use diesel_async::AsyncPgConnection;
use uuid::Uuid;

use crate::{
    actions::{
        relation::details::get_role_id_from_relation,
        resource::details::get_parent_resource_id,
        role::details::{get_privilegs_by_role_id, get_role_id_by_name},
    },
    error::Result,
    models::{
        role::{Privilege, PrivilegeVec},
        RESOURCE, ROLE, ROOT_ROLE,
    },
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
    callback: Option<fn(Vec<Privilege>, Vec<Privilege>) -> bool>,
    callback_args: Option<Vec<Privilege>>,
) -> Result<bool> {
    let role_id = get_role_id_from_relation(conn, user_id, resource_id).await?;

    if let Some(role_id) = role_id {
        // TODO: this query can be optimized
        let PrivilegeVec(privileges) = get_privilegs_by_role_id(conn, role_id).await?;

        let callback_bool = if let Some(callback) = callback {
            let callback_args = callback_args.unwrap();
            callback(privileges.clone(), callback_args)
        } else {
            true
        };

        return Ok(privileges.iter().any(|p| {
            p.entity == entity
                && (p.privileges.contains(&privilege.to_owned())
                    || p.privileges.contains(&"*".to_string()))
        }) && callback_bool);
    }

    let parent_resource_id = get_parent_resource_id(conn, resource_id).await?;

    if parent_resource_id.is_some() {
        return check_has_privilege(
            conn,
            user_id,
            parent_resource_id.unwrap(),
            privilege,
            entity,
            callback,
            callback_args,
        )
        .await;
    }

    Ok(false)
}

pub fn check_privileges_callback() -> fn(Vec<Privilege>, Vec<Privilege>) -> bool {
    let callback = |privileges: Vec<Privilege>, new_privileges: Vec<Privilege>| {
        let is_subset = privileges
            .iter()
            .all(|privilege| privilege.privileges.clone().pop() == Some("*".to_string()));

        if is_subset {
            return true;
        }

        let new_role_privileges = new_privileges.iter().find(|p| p.entity == ROLE);
        let user_role_privileges = privileges.iter().find(|p| p.entity == ROLE);
        let new_resource_privileges = new_privileges.iter().find(|p| p.entity == RESOURCE);
        let user_resource_privileges = privileges.iter().find(|p| p.entity == RESOURCE);

        // check "*" cases
        let is_role_privilege_subset = match (new_role_privileges, user_role_privileges) {
            (Some(new), Some(user)) => {
                user.privileges.clone().pop() == Some("*".to_string())
                    || new.privileges == user.privileges
            }
            (None, Some(_)) => true,
            _ => false,
        };

        let is_resource_privilege_subset = match (new_resource_privileges, user_resource_privileges)
        {
            (Some(new), Some(user)) => {
                user.privileges.clone().pop() == Some("*".to_string())
                    || new.privileges == user.privileges
            }
            (None, Some(_)) => true,
            _ => false,
        };

        is_resource_privilege_subset && is_role_privilege_subset
    };

    callback
}

pub async fn check_user_is_root(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
    object_id: Uuid,
) -> Result<bool> {
    let role = get_role_id_from_relation(conn, user_id, object_id).await?;
    if let Some(role) = role {
        let root = get_role_id_by_name(conn, ROOT_ROLE).await?.unwrap();
        return Ok(role == root);
    }
    Ok(false)
}
