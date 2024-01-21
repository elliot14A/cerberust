use repositories::{CreateEmailVerificationTokenInput, EmailVerificationToken, Error, Result};
use surrealdb::opt::RecordId;

use crate::{email_verification_token::SurrealEmailVerificationToken, DB};

pub async fn create(input: CreateEmailVerificationTokenInput) -> Result<EmailVerificationToken> {
    let CreateEmailVerificationTokenInput { user_id, token } = input;

    let surql = r#"
            create email_verification_token content {
                user: $user_id,
                token: $verification_token
            }
        "#;

    let user_id = RecordId::from(("user", user_id.as_str()));

    let mut response = DB
        .query(surql)
        .bind(("user_id", user_id))
        .bind(("verification_token", token))
        .await
        .map_err(|e| {
            println!("{:?}", e);
            Error::InternalError {
                message: e.to_string(),
            }
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
