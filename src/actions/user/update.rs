use crate::{
    error::{handle_diesel_error, Result},
    models::user::User,
    schema::user,
};
use diesel::ExpressionMethods;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

pub async fn update_email_verified(conn: &mut AsyncPgConnection, user_id: Uuid) -> Result<User> {
    Ok(diesel::update(user::table)
        .filter(user::id.eq(user_id))
        .set(user::email_verified.eq(true))
        .get_result::<User>(conn)
        .await
        .map_err(handle_diesel_error)?)
}

pub async fn update_password(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
    password: String,
) -> Result<User> {
    Ok(diesel::update(user::table)
        .filter(user::id.eq(user_id))
        .set(user::password.eq(password))
        .get_result::<User>(conn)
        .await
        .map_err(handle_diesel_error)?)
}
