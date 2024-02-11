use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::error::{handle_diesel_error, Result};
use crate::{models::refresh_token::RefreshToken, schema::refresh_token};

pub async fn get_refresh_token_by_token(
    conn: &mut AsyncPgConnection,
    token: &str,
) -> Result<Option<RefreshToken>> {
    Ok(refresh_token::table
        .filter(refresh_token::token.eq(token))
        .first(conn)
        .await
        .optional()
        .map_err(handle_diesel_error)?)
}
