use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::error::Result;
use crate::models::refresh_token::{NewRefreshToken, RefreshToken};
use crate::schema::refresh_token;

pub async fn create_refresh_token(
    conn: &mut AsyncPgConnection,
    new_refresh_token: NewRefreshToken,
) -> Result<RefreshToken> {
    Ok(diesel::insert_into(refresh_token::table)
        .values(&new_refresh_token)
        .get_result(conn)
        .await?)
}
