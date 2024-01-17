use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// `CreateAccount` represents the data needed to create a new account.
#[derive(Debug, Deserialize)]
pub struct CreateAccount {
    pub user_id: String,
    pub account_type: String,
    pub provider_account_id: String,
    pub provider: String,
}

/// `Account` represents an account in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    id: String,
    user_id: String,
    account_type: String,
    provider: String,
    provider_account_id: String,
    #[serde(skip_serializing)]
    refresh_token: Option<String>,
    #[serde(skip_serializing)]
    access_token: Option<String>,
    #[serde(skip_serializing)]
    expires_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    scope: Option<String>,
    #[serde(skip_serializing)]
    id_token: Option<String>,
    #[serde(skip_serializing)]
    session_state: Option<String>,
    created_at: DateTime<Utc>,
    upated_at: DateTime<Utc>,
}

/// `AccountWhereInput` represents the data needed to query for an account.
/// At least one field must be present.
pub struct AccountWhereInput {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub account_type: Option<String>,
    pub provider: Option<String>,
    pub provider_account_id: Option<String>,
}
