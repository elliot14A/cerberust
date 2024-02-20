use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    error::{handle_diesel_error, Result},
    models::resource::Resource,
    schema::resource,
};

/// get all parent resources  
pub async fn get_all_parent_resources(conn: &mut AsyncPgConnection) -> Result<Vec<Resource>> {
    Ok(resource::table
        .filter(resource::parent_resource_id.is_null())
        .load(conn)
        .await
        .map_err(handle_diesel_error)?)
}

/// get all child resources of a parent resource
pub async fn get_child_resources(
    conn: &mut AsyncPgConnection,
    resource_id: Uuid,
) -> Result<Vec<Resource>> {
    Ok(resource::table
        .filter(resource::parent_resource_id.eq(resource_id))
        .load(conn)
        .await
        .map_err(handle_diesel_error)?)
}

/// get all resources
pub async fn get_all_resources(conn: &mut AsyncPgConnection) -> Result<Vec<Resource>> {
    Ok(resource::table
        .load(conn)
        .await
        .map_err(handle_diesel_error)?)
}
