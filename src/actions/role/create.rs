use diesel::insert_into;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    error::{handle_diesel_error, Result},
    models::role::{NewRole, Role},
    schema::role,
};

pub async fn create_role(conn: &mut AsyncPgConnection, new_role: NewRole) -> Result<Role> {
    Ok(insert_into(role::table)
        .values(&new_role)
        .get_result::<Role>(conn)
        .await
        .map_err(handle_diesel_error)?)
}
