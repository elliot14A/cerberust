use repositories::{refresh_token::RefreshToken, Error, Result};

use crate::DB;

use super::SurrealRefreshToken;

pub async fn get_refresh_token(token: String) -> Result<RefreshToken> {
    let surql = "SELECT * FROM refresh_token WHERE token = $s_token";
    let mut response = DB
        .query(surql)
        .bind(("s_token", token))
        .await
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?;
    let token: Option<RefreshToken> = response
        .take::<Option<SurrealRefreshToken>>(0)
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?
        .map(|d| d.into());
    if token.is_none() {
        return Err(Error::TokenNotFound);
    }
    Ok(token.unwrap())
}
