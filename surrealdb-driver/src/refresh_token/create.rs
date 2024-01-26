use repositories::refresh_token::{RefreshToken, RefreshTokenCreateInput};
use repositories::{Error, Result};
use surrealdb::opt::RecordId;

use crate::refresh_token::SurrealRefreshToken;
use crate::DB;

pub async fn create(input: RefreshTokenCreateInput) -> Result<RefreshToken> {
    let RefreshTokenCreateInput { session_id, token } = input;
    let surql = r#"
        create refresh_token content {
            session: $session_id
            token: $s_token
        }
        "#;
    let session_id = RecordId::from(("session", session_id.as_str()));
    let mut response = DB
        .query(surql)
        .bind(("session_id", session_id))
        .bind(("s_token", token))
        .await
        .map_err(|e| {
            println!("{:?}", e);
            Error::InternalError {
                message: e.to_string(),
            }
        })?;
    let token: Option<RefreshToken> = response
        .take::<Option<SurrealRefreshToken>>(0)
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?
        .map(|d| d.into());
    Ok(token.unwrap())
}
