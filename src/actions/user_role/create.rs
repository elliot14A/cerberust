use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::error::Result;
use crate::models::user_role::{NewUserRole, UserRole};
use crate::schema::user_role;

pub async fn create_user_role(
    conn: &mut AsyncPgConnection,
    new_user_role: NewUserRole,
) -> Result<UserRole> {
    Ok(diesel::insert_into(user_role::table)
        .values(&new_user_role)
        .get_result(conn)
        .await?)
}
