use repositories::{Error, Result};
use surrealdb::opt::RecordId;

pub async fn invalidate_session(id: String) -> Result<()> {
    let surql = r#"
        UPDATE session SET valid = false WHERE id = $s_id
        "#;
    crate::DB
        .query(surql)
        .bind(("s_id", id))
        .await
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?;
    Ok(())
}

pub async fn add_refresh_token(id: String, token: String) -> Result<()> {
    let id = RecordId::from(("session", id.as_str()));
    let token = RecordId::from(("refresh_token", token.as_str()));
    let surql = r#"
        UPDATE session SET refresh_tokens += $s_token WHERE id = $id 
        "#;
    crate::DB
        .query(surql)
        .bind(("s_token", token))
        .bind(("id", id))
        .await
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?;
    Ok(())
}
