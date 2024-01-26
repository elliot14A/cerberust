use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::refresh_token::RefreshToken;

#[derive(Debug, Serialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub valid: bool,
    pub refresh_tokens: Vec<RefreshToken>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CreateSessionInput {
    pub user_id: String,
}

pub struct SessionWhereInput {
    pub id: Option<String>,
    pub user_id: Option<String>,
}
