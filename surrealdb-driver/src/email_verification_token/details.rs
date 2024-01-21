use repositories::{EmailVerificationToken, EmailVerificationTokenWhereInput, Error, Result};

use crate::email_verification_token::SurrealEmailVerificationToken;

// TODO: fix token issue and use all fields
pub async fn details(input: EmailVerificationTokenWhereInput) -> Result<EmailVerificationToken> {
    let EmailVerificationTokenWhereInput {
        id: _,
        user_id: _,
        token,
    } = input;

    let surql = r#"SELECT * FROM email_verification_token WHERE token = $verify_token"#.to_string();

    let mut response = crate::DB
        .query(&surql)
        .bind(("verify_token", token))
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
    if token.is_none() {
        return Err(Error::EmailVerificationTokenNotFound {
            message: format!("Token not found for given query: {}", surql),
        });
    }
    Ok(token.unwrap())
}
