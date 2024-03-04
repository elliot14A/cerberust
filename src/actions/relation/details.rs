use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::error::Result;
use crate::schema::relation;

pub async fn get_role_id_from_relation(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
    object_id: Uuid,
) -> Result<Option<Uuid>> {
    Ok(relation::table
        .filter(relation::user_id.eq(user_id))
        .filter(relation::object_id.eq(object_id))
        .select(relation::role_id)
        .first(conn)
        .await
        .optional()?)
}
