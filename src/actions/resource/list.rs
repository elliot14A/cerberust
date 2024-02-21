use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{error::Result, models::resource::Resource, schema::resource};

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

pub async fn list_user_resources(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
) -> Result<Vec<Resource>> {
    Ok(resource::table
        .filter(resource::created_by_id.eq(user_id))
        .load(conn)
        .await?)
}
