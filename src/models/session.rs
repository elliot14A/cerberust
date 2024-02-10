use crate::schema::session;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = session)]
pub struct NewSession {
    pub user_id: Uuid,
}

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = session)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub valid: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
