use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{error::Result, schema::privilege};

pub async fn delete_privilege(conn: &mut AsyncPgConnection, privilege_id: Uuid) -> Result<()> {
    diesel::delete(privilege::table.filter(privilege::id.eq(privilege_id)))
        .execute(conn)
        .await?;
    Ok(())
}
