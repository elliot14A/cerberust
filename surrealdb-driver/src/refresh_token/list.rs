use repositories::{refresh_token::RefreshToken, Error, Result};

use crate::refresh_token::SurrealRefreshToken;
pub async fn list(session_id: String) -> Result<Vec<RefreshToken>> {
    let surql = r#"
        SELECT * FROM refresh_token WHERE session_id = $s_session_id
        "#;
    let mut response = crate::DB
        .query(surql)
        .bind(("s_session_id", session_id))
        .await
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?;

    let refresh_tokens = response
        .take::<Vec<SurrealRefreshToken>>(0)
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?
        .into_iter()
        .map(|d| d.into())
        .collect::<Vec<RefreshToken>>();

    Ok(refresh_tokens)
}
