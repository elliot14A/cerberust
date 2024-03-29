use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::refresh_token;

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = refresh_token)]
pub struct NewRefreshToken {
    pub session_id: Uuid,
    pub token: String,
}

#[derive(Debug, Queryable, Serialize, Selectable)]
#[diesel(table_name = refresh_token)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RefreshToken {
    pub id: Uuid,
    pub token: String,
    pub session_id: Uuid,
    pub created_at: DateTime<Utc>,
}
