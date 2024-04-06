use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::error::Result;
use crate::models::role::Role;
use crate::schema::role;

pub async fn get_all_default_roles(conn: &mut AsyncPgConnection) -> Result<Vec<Role>> {
    Ok(role::table
        .filter(role::is_default.eq(true))
        .load(conn)
        .await?)
}

pub async fn get_custom_roles(
    conn: &mut AsyncPgConnection,
    resource_id: Uuid,
) -> Result<Vec<Role>> {
    Ok(role::table
        .filter(role::resource_id.eq(resource_id))
        .load(conn)
        .await?)
}
