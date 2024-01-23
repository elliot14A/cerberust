use repositories::{token::Token, Error, Result};

use crate::DB;

use super::SurrealToken;
pub async fn get_token(token: String) -> Result<Token> {
    let surql = "SELECT * FROM token WHERE token = $s_token";

    let mut response = DB
        .query(surql)
        .bind(("s_token", token))
        .await
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?;

    let token: Option<Token> = response
        .take::<Option<SurrealToken>>(0)
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?
        .map(|d| d.into());

    if token.is_none() {
        return Err(Error::TokenNotFound);
    }

    Ok(token.unwrap())
}
