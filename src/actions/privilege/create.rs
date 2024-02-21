use diesel::insert_into;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::error::Result;
use crate::models::privilege::{NewPrivilege, Privilege};
use crate::schema::privilege;

pub async fn create_privilege(
    conn: &mut AsyncPgConnection,
    new_privilege: NewPrivilege,
) -> Result<Privilege> {
    Ok(insert_into(privilege::table)
        .values(&new_privilege)
        .get_result::<Privilege>(conn)
        .await?)
}
