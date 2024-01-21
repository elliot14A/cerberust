use repositories::{EmailVerificationTokenWhereInput, Error, Result};

use crate::{build_query, DB};

pub async fn delete(input: EmailVerificationTokenWhereInput) -> Result<()> {
    let EmailVerificationTokenWhereInput { id, user_id, token } = input;
    let surql = r#"DELETE email_verification_token"#.to_string()
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
