use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    error::Result,
    models::role::{PrivilegeVec, Role},
    schema::role,
};

pub async fn get_role_by_id(conn: &mut AsyncPgConnection, role_id: Uuid) -> Result<Option<Role>> {
    Ok(role::table
        .filter(role::id.eq(role_id))
        .first(conn)
        .await
        .optional()?)
}

pub async fn get_role_id_by_name(conn: &mut AsyncPgConnection, name: &str) -> Result<Option<Uuid>> {
    Ok(role::table
        .filter(role::name.eq(name))
        .select(role::id)
        .first(conn)
        .await
        .optional()?)
}

pub async fn get_privilegs_by_role_id(
    conn: &mut AsyncPgConnection,
    role_id: Uuid,
) -> Result<PrivilegeVec> {
    Ok(role::table
        .filter(role::id.eq(role_id))
        .select(role::privileges)
        .first(conn)
        .await?)
}
