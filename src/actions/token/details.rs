use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, OptionalExtension};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::error::{handle_diesel_error, Result};
use crate::models::token::Token;
use crate::schema::token;

pub async fn get_token_by_token(
    conn: &mut AsyncPgConnection,
    token_text: String,
) -> Result<Option<Token>> {
    Ok(token::table
        .filter(token::token_text.eq(token_text))
        .first::<Token>(conn)
        .await
        .optional()
        .map_err(handle_diesel_error)?)
}
