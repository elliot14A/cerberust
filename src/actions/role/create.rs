use diesel::{insert_into, ExpressionMethods};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    error::Result,
    models::role::{NewRole, Role},
    schema::role,
};

pub async fn create_role(conn: &mut AsyncPgConnection, new_role: NewRole) -> Result<Role> {
    let privileges = serde_json::to_value(&new_role.privileges).unwrap();
    Ok(insert_into(role::table)
        .values((
            role::name.eq(new_role.name),
            role::description.eq(new_role.description),
            role::privileges.eq(privileges),
            role::is_default.eq(new_role.is_default),
            role::resource_id.eq(new_role.resource_id),
        ))
        .get_result::<Role>(conn)
        .await?)
}
