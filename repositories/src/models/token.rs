use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Token {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub token_type: String,
    pub created_at: DateTime<Utc>,
}

pub struct TokenWhereInput {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub token_type: Option<String>,
}

pub struct CreateTokenInput {
    pub user_id: String,
    pub token: String,
    pub token_type: String,
}
