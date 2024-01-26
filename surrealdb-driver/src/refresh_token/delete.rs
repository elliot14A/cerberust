use repositories::{refresh_token::RefreshTokenWhereInput, Error, Result};

use crate::build_query;

pub async fn delete_refresh_token(input: RefreshTokenWhereInput) -> Result<()> {
    let RefreshTokenWhereInput {
        id,
        session_id,
        token,
    } = input;
    let id = id.map(|id| format!("refresh_token:{}", id));
    let session_id = session_id.map(|id| format!("session:{}", id));
    let surql = r#"DELETE refresh_token"#.to_string()
        + build_query(
            " WHERE",
            vec![("id", id), ("session", session_id), ("token", token)],
            " AND",
        )?
        .as_ref();
    crate::DB
        .query(&surql)
        .await
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?;
    Ok(())
}
