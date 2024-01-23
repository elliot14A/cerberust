use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ResetPasswordToken {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub created_at: DateTime<Utc>,
}

pub struct ResetPasswordTokenWhereInput {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub token: Option<String>,
}

pub struct CreateResetPasswordTokenInput {
    pub user_id: String,
    pub token: String,
}
