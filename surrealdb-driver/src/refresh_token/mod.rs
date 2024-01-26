use repositories::refresh_token::RefreshToken;
use serde::Deserialize;
use surrealdb::{opt::RecordId, sql::Datetime};

pub mod create;
pub mod delete;
pub mod details;

#[derive(Deserialize)]
pub(crate) struct SurrealRefreshToken {
    pub id: RecordId,
    pub session: RecordId,
    pub token: String,
    pub created_at: Datetime,
}

impl From<SurrealRefreshToken> for RefreshToken {
    fn from(value: SurrealRefreshToken) -> Self {
        Self {
            id: value.id.id.to_string(),
            session_id: value.session.id.to_string(),
            token: value.token,
            created_at: value.created_at.into(),
        }
    }
}
