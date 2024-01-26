use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RefreshToken {
    pub id: String,
    pub session_id: String,
    pub token: String,
    pub created_at: DateTime<Utc>,
}

pub struct RefreshTokenCreateInput {
    pub session_id: String,
    pub token: String,
}

pub struct RefreshTokenWhereInput {
    pub id: Option<String>,
    pub session_id: Option<String>,
}
