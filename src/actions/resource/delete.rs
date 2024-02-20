use diesel::ExpressionMethods;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    error::{handle_diesel_error, Result},
    schema::resource,
};

pub async fn delete_resource(conn: &mut AsyncPgConnection, resource_id: Uuid) -> Result<()> {
    diesel::delete(resource::table)
        .filter(resource::id.eq(resource_id))
        .execute(conn)
        .await
        .map_err(handle_diesel_error)?;
    Ok(())
}
