use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::schema::role_privilege;

#[derive(Debug, Insertable, Validate, Deserialize, Clone)]
#[diesel(table_name = role_privilege)]
pub struct NewRolePrivilege {
    pub role_id: Uuid,
    pub privilege_id: Uuid,
    pub resource_id: Uuid,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Clone)]
#[diesel(table_name = role_privilege)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(role_id, privilege_id, resource_id))]
pub struct RolePrivilege {
    pub role_id: Uuid,
    pub privilege_id: Uuid,
    pub resource_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
