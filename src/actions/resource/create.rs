use diesel::insert_into;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    error::Result,
    models::resource::{NewResource, Resource},
    schema::resource,
};

pub async fn create_resource(
    conn: &mut AsyncPgConnection,
    new_resource: NewResource,
) -> Result<Resource> {
    Ok(insert_into(resource::table)
        .values(&new_resource)
        .get_result::<Resource>(conn)
        .await?)
}
