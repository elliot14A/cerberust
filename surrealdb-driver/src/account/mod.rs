use repositories::Account;
use serde::Deserialize;
use surrealdb::{opt::RecordId, sql::Datetime};

pub mod create;
pub mod delete;
pub mod details;
pub mod list;

/// Internal representation of an account.
#[derive(Deserialize)]
pub(crate) struct SurrealAccount {
    id: RecordId,
    user: RecordId,
    account_type: String,
    provider: String,
    provider_account_id: String,
    refresh_token: Option<String>,
    access_token: Option<String>,
    expires_at: Datetime,
    scope: Option<String>,
    id_token: Option<String>,
    session_state: Option<String>,
    created_at: Datetime,
    updated_at: Datetime,
}

impl From<SurrealAccount> for Account {
    fn from(value: SurrealAccount) -> Self {
        Self {
            id: value.id.id.to_string(),
            user_id: value.user.id.to_string(),
            account_type: value.account_type,
            provider: value.provider,
            provider_account_id: value.provider_account_id,
            refresh_token: value.refresh_token,
            access_token: value.access_token,
            expires_at: value.expires_at.into(),
            scope: value.scope,
            id_token: value.id_token,
            session_state: value.session_state,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}
