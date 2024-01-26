use repositories::{Error, Result};
use surrealdb::opt::RecordId;

pub async fn invalidate_session(id: String) -> Result<()> {
    let id = RecordId::from(("session", id.as_str()));
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
