use diesel::ExpressionMethods;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    error::{handle_diesel_error, Result},
    models::token::TokenType,
    schema::token,
};

pub async fn delete_user_tokens(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
    token_type: TokenType,
) -> Result<()> {
    diesel::delete(token::table)
        .filter(token::user_id.eq(user_id))
        .filter(token::token_type.eq(token_type))
        .execute(conn)
        .await
        .map_err(handle_diesel_error)?;
    Ok(())
}
