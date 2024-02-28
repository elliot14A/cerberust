use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    error::Result,
    models::{
        resource::{Resource, RoleFlattenned, RoleResource},
        role::Role,
    },
    schema::{relation, resource, role},
};

/// get all parent resources  
pub async fn get_all_parent_resources(conn: &mut AsyncPgConnection) -> Result<Vec<Resource>> {
    Ok(resource::table
        .filter(resource::parent_resource_id.is_null())
        .load(conn)
        .await?)
}

/// get all child resources of a parent resource
pub async fn get_child_resources(
    conn: &mut AsyncPgConnection,
    resource_id: Uuid,
) -> Result<Vec<Resource>> {
    Ok(resource::table
        .filter(resource::parent_resource_id.eq(resource_id))
        .load(conn)
        .await?)
}

/// get all resources
pub async fn get_all_resources(conn: &mut AsyncPgConnection) -> Result<Vec<Resource>> {
    Ok(resource::table.load(conn).await?)
}

pub async fn get_user_resources(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
) -> Result<Vec<RoleResource>> {
    let result: Vec<(Resource, Role)> = relation::table
        .filter(relation::user_id.eq(user_id))
        .inner_join(resource::table.on(relation::object_id.eq(resource::id)))
        .inner_join(role::table.on(relation::role_id.eq(role::id)))
        .select((resource::all_columns, role::all_columns))
        .load(conn)
        .await?;
    println!("{:?}", result);
    let role_resources = result
        .into_iter()
        .map(|(resource, role)| RoleResource {
            resource,
            role: RoleFlattenned {
                name: role.name,
                description: role.description,
                privileges: role.privileges,
            },
        })
        .collect();
    Ok(role_resources)
}
