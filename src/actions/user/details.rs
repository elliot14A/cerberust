use crate::error::Result;
use crate::{models::user::User, schema::user};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

pub async fn get_user_by_id(conn: &mut AsyncPgConnection, id: Uuid) -> Result<Option<User>> {
    Ok(user::table.find(id).first::<User>(conn).await.optional()?)
}

pub async fn get_user_by_email(
    conn: &mut AsyncPgConnection,
    email: String,
) -> Result<Option<User>> {
    Ok(user::table
        .filter(user::email.eq(email))
        .first::<User>(conn)
        .await
        .optional()?)
}
