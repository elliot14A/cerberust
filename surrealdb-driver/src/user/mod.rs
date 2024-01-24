use repositories::user::User;
use serde::Deserialize;
use surrealdb::{opt::RecordId, sql::Datetime};

pub mod create;
pub mod details;
pub mod update;

#[derive(Deserialize)]
/// Internal representation of a user.
struct SurrealUser {
    id: RecordId,
    email: String,
    password: String,
    name: String,
    created_at: Datetime,
    updated_at: Datetime,
    email_verified: bool,
}

impl From<SurrealUser> for User {
    fn from(value: SurrealUser) -> Self {
        let SurrealUser {
            id,
            email,
            password,
            name,
            created_at,
            updated_at,
            email_verified,
        } = value;
        Self {
            id: id.id.to_string(),
            email,
            password,
            name,
            created_at: created_at.into(),
            updated_at: updated_at.into(),
            email_verified,
        }
    }
}

// TODO: Improve the tests.
