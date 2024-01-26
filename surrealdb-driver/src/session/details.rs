use repositories::{session::Session, Error, Result};
use surrealdb::opt::RecordId;

use super::SurrealSession;

pub async fn details(id: String, user_id: String) -> Result<Session> {
    let id = RecordId::from(("session", id.as_str()));
    let user_id = RecordId::from(("user", user_id.as_str()));
    let surql = r#"SELECT * FROM session WHERE id = $s_id AND user = $user_id"#;
    let mut response = crate::DB
        .query(surql)
        .bind(("s_id", id))
        .bind(("user_id", user_id))
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
