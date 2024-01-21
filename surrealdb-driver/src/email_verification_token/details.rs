use repositories::{EmailVerificationToken, EmailVerificationTokenWhereInput, Error, Result};

use crate::{build_query, email_verification_token::SurrealEmailVerificationToken};

pub async fn details(input: EmailVerificationTokenWhereInput) -> Result<EmailVerificationToken> {
    let EmailVerificationTokenWhereInput { id, user_id, token } = input;

    let surql = r#"SELECT * FROM email_verification_token"#.to_string()
        + build_query(
            " WHERE",
            vec![("id", id), ("user", user_id), ("token", token)],
            " AND",
        )?
        .as_str();

    let mut response = crate::DB
        .query(&surql)
        .await
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?;

    let token: Option<EmailVerificationToken> = response
        .take::<Vec<SurrealEmailVerificationToken>>(0)
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?
        .pop()
        .map(|d| d.into());
    Ok(token.unwrap())
}
