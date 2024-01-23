use repositories::CreateResetPasswordTokenInput;
use repositories::Error;
use repositories::ResetPasswordToken;
use repositories::Result;
use surrealdb::opt::RecordId;

use crate::reset_password_token::SurrealResetPasswordToken;
use crate::DB;

pub async fn create(input: CreateResetPasswordTokenInput) -> Result<ResetPasswordToken> {
    let CreateResetPasswordTokenInput { user_id, token } = input;
    let surql = r#"
        create reset_password_token content {
            user: $user_id,
            token: $token
        }
    "#;

    let user_id = RecordId::from(("user", user_id.as_str()));

    let mut response = DB
        .query(surql)
        .bind(("user_id", user_id))
        .bind(("token", token))
        .await
        .map_err(|e| {
            println!("{:?}", e);
            Error::InternalError {
                message: e.to_string(),
            }
        })?;

    let token: Option<ResetPasswordToken> = response
        .take::<Vec<SurrealResetPasswordToken>>(0)
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?
        .pop()
        .map(|d| d.into());

    Ok(token.unwrap())
}
