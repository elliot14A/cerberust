use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::resource;

use super::role::PrivilegeVec;

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = resource)]
pub struct NewResource {
    pub parent_resource_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Queryable, Selectable, Associations, Serialize, Clone)]
#[diesel(table_name = resource)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Resource, foreign_key = parent_resource_id))]
pub struct Resource {
    pub id: Uuid,
    pub parent_resource_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct RoleResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub role: RoleFlattenned,
}

#[derive(Debug, Serialize)]
pub struct RoleFlattenned {
    pub name: String,
    pub description: Option<String>,
    pub privileges: PrivilegeVec,
}
