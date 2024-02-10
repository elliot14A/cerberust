use crate::error::{handle_diesel_error, Result};
use crate::{models::user::User, schema::user};
use diesel::{OptionalExtension, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

pub async fn get_user_by_id(conn: &mut AsyncPgConnection, id: Uuid) -> Result<Option<User>> {
    let user = user::table
        .find(id)
        .first::<User>(conn)
        .await
        .optional()
        .map_err(handle_diesel_error)?;
    Ok(user)
}
