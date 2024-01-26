use repositories::{session::Session, Error, Result};

use super::SurrealSession;

pub async fn details(id: String) -> Result<Session> {
    let id = format!("session:{}", id);
    let surql = r#"
        SELECT * FROM session WHERE id = $s_id FETCH refresh_tokens
        "#;
    let mut response = crate::DB
        .query(surql)
        .bind(("s_id", id))
        .await
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?;
    let token: Option<Session> = response
        .take::<Option<SurrealSession>>(0)
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?
        .map(|d| d.into());

    if token.is_none() {
        return Err(Error::TokenNotFound);
    }

    Ok(token.unwrap())
}
