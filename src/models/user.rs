use crate::schema::user;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Insertable, Validate, Deserialize)]
#[diesel(table_name = user)]
pub struct NewUser<'a> {
    #[validate(length(min = 3, max = 24))]
    pub username: &'a str,
    #[validate(email)]
    pub email: &'a str,
    #[validate(length(min = 8))]
    pub password: &'a str,
}

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
