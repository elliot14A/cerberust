use repositories::token::Token;
use serde::Deserialize;
use surrealdb::{opt::RecordId, sql::Datetime};

pub mod create;
pub mod delete;
pub mod details;

#[derive(Deserialize)]
pub struct SurrealToken {
    pub id: RecordId,
    pub user: RecordId,
    pub token: String,
    pub token_type: String,
    pub created_at: Datetime,
}

impl From<SurrealToken> for Token {
    fn from(value: SurrealToken) -> Self {
        Self {
            id: value.id.id.to_string(),
            user_id: value.user.id.to_string(),
            token: value.token,
            token_type: value.token_type,
            created_at: value.created_at.into(),
        }
    }
}
