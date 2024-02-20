use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    error::{handle_diesel_error, Result},
    models::role::Role,
    schema::role,
};

pub async fn get_role_by_id(conn: &mut AsyncPgConnection, role_id: Uuid) -> Result<Option<Role>> {
    Ok(role::table
        .filter(role::id.eq(role_id))
        .first(conn)
        .await
        .optional()
        .map_err(handle_diesel_error)?)
}
