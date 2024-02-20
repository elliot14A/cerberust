use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::schema::user_role;

#[derive(Debug, Insertable, Validate, Deserialize, Clone)]
#[diesel(table_name = user_role)]
pub struct NewUserRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Clone)]
#[diesel(table_name = user_role)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(user_id, role_id))]
pub struct UserRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
