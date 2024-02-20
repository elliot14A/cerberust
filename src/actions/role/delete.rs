use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    error::{handle_diesel_error, Result},
    schema::role,
};

pub async fn delete_role(conn: &mut AsyncPgConnection, role_id: Uuid) -> Result<()> {
    diesel::delete(role::table.filter(role::id.eq(role_id)))
        .execute(conn)
        .await
        .map_err(handle_diesel_error)?;
    Ok(())
}
