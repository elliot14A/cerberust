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
pub struct UserRole {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role_id: Uuid,
}
