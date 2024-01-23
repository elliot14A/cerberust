use crate::account::SurrealAccount;
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
    accounts: Vec<SurrealAccount>,
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
            accounts,
        } = value;
        Self {
            id: id.id.to_string(),
            email,
            password,
            name,
            created_at: created_at.into(),
            updated_at: updated_at.into(),
            email_verified,
            accounts: accounts.into_iter().map(|a| a.into()).collect(),
        }
    }
}

// TODO: Improve the tests.
