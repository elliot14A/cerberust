pub mod create;
pub mod details;
pub mod update;

use repositories::session::Session;
use serde::Deserialize;
use surrealdb::{opt::RecordId, sql::Datetime};

#[derive(Deserialize)]
struct SurrealSession {
    pub id: RecordId,
    pub user: RecordId,
    pub valid: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

impl From<SurrealSession> for Session {
    fn from(value: SurrealSession) -> Self {
        Self {
            id: value.id.id.to_string(),
            user_id: value.user.id.to_string(),
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
            valid: value.valid,
        }
    }
}
