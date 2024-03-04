use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::error::Result;
use crate::models::role::Role;
use crate::schema::{relation, role};

pub async fn get_all_default_roles(conn: &mut AsyncPgConnection) -> Result<Vec<Role>> {
    Ok(role::table
        .filter(role::is_default.eq(true))
        .load(conn)
        .await?)
}

pub async fn get_custom_roles(conn: &mut AsyncPgConnection, user_id: Uuid) -> Result<Vec<Role>> {
    Ok(relation::table
        .inner_join(role::table.on(relation::object_id.eq(role::id)))
        .select(role::all_columns)
        .filter(relation::user_id.eq(user_id))
        .load(conn)
        .await?)
}
