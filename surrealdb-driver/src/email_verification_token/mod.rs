use repositories::EmailVerificationToken;
use serde::Deserialize;
use surrealdb::{opt::RecordId, sql::Datetime};

pub mod create;
pub mod delete;
pub mod details;

#[derive(Deserialize)]
pub struct SurrealEmailVerificationToken {
    id: RecordId,
    user: RecordId,
    token: String,
    created_at: Datetime,
}

impl From<SurrealEmailVerificationToken> for EmailVerificationToken {
    fn from(value: SurrealEmailVerificationToken) -> Self {
        Self {
            id: value.id.id.to_string(),
            user_id: value.user.id.to_string(),
            token: value.token,
            created_at: value.created_at.into(),
        }
    }
}
