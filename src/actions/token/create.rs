use diesel::insert_into;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::error::{handle_diesel_error, Result};
use crate::models::token::{NewToken, Token};
use crate::schema::token;

pub async fn create_token<'a>(
    conn: &mut AsyncPgConnection,
    new_token: NewToken<'a>,
) -> Result<Token> {
    Ok(insert_into(token::table)
        .values(&new_token)
        .get_result(conn)
        .await
        .map_err(handle_diesel_error)?)
}
