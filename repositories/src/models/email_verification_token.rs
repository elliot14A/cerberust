use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct EmailVerificationToken {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub created_at: DateTime<Utc>,
}

pub struct EmailVerificationTokenWhereInput {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEmailVerificationTokenInput {
    pub user_id: String,
    pub token: String,
}
