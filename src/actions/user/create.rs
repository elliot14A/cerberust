use diesel::insert_into;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::error::Result;
use crate::{
    models::user::{NewUser, User},
    schema::user,
};

pub async fn create_user(conn: &mut AsyncPgConnection, new_user: NewUser) -> Result<User> {
    Ok(insert_into(user::table)
        .values(&new_user)
        .get_result::<User>(conn)
        .await?)
}
