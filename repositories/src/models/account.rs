use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// `CreateAccount` represents the data needed to create a new account.
#[derive(Debug, Deserialize)]
pub struct CreateAccountInput {
    pub user_id: String,
    pub account_type: String,
    pub provider_account_id: String,
    pub provider: String,
}

/// `Account` represents an account in the database.
#[derive(Debug, Clone, Serialize)]
pub struct Account {
    pub id: String,
    pub user_id: String,
    pub account_type: String,
    pub provider: String,
    pub provider_account_id: String,
    #[serde(skip_serializing)]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing)]
    pub access_token: Option<String>,
    #[serde(skip_serializing)]
    pub expires_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub scope: Option<String>,
    #[serde(skip_serializing)]
    pub id_token: Option<String>,
    #[serde(skip_serializing)]
    pub session_state: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
