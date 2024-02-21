use diesel::ExpressionMethods;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{error::Result, schema::resource};

pub async fn delete_resource(conn: &mut AsyncPgConnection, resource_id: Uuid) -> Result<()> {
    diesel::delete(resource::table)
        .filter(resource::id.eq(resource_id))
        .execute(conn)
        .await?;
    Ok(())
}
