use repositories::{Error, ResetPasswordTokenWhereInput, Result};

use crate::{build_query, DB};

pub async fn delete(input: ResetPasswordTokenWhereInput) -> Result<()> {
    let ResetPasswordTokenWhereInput { id, user_id, token } = input;
    let user_id = user_id.map(|id| format!("user:{}", id));
    let surql = r#"DELETE reset_password_token"#.to_string()
        + build_query(
            " WHERE",
            vec![("id", id), ("user", user_id), ("token", token)],
            " AND",
        )?
        .as_ref();
    DB.query(&surql).await.map_err(|e| Error::InternalError {
        message: e.to_string(),
    })?;

    Ok(())
}
