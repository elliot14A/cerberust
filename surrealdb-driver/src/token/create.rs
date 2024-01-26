use repositories::{
    token::{CreateTokenInput, Token},
    Error, Result,
};
use surrealdb::opt::RecordId;

use crate::{token::SurrealToken, DB};

pub async fn create(input: CreateTokenInput) -> Result<Token> {
    let CreateTokenInput {
        user_id,
        token,
        token_type,
    } = input;
    let surql = r#"
        create token content {
            user: $user_id,
            token: $s_token,
            token_type: $token_type
        }
        "#;

    let user_id = RecordId::from(("user", user_id.as_str()));
    let mut response = DB
        .query(surql)
        .bind(("user_id", user_id))
        .bind(("s_token", token))
        .bind(("token_type", token_type))
        .await
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?;
    let token: Option<Token> = response
        .take::<Option<SurrealToken>>(0)
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?
        .map(|d| d.into());

    Ok(token.unwrap())
}
