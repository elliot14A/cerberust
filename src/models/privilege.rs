use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::schema::privilege;

#[derive(Debug, Insertable, Validate, Deserialize, Clone)]
#[diesel(table_name = privilege)]
pub struct NewPrivilege {
    #[validate(length(min = 3, max = 24))]
    pub name: String,
    #[validate(length(min = 8))]
    pub description: Option<String>,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Clone)]
#[diesel(table_name = privilege)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Privilege {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
