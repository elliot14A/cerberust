use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::schema::resource;

use super::user::User;

#[derive(Debug, Insertable, Validate, Deserialize, Clone)]
#[diesel(table_name = resource)]
pub struct NewResource {
    pub parent_resource_id: Option<Uuid>,
    #[validate(length(min = 3, max = 24))]
    pub name: String,
    #[validate(length(min = 8))]
    pub description: Option<String>,
    pub created_by_id: Uuid,
}

#[derive(Debug, Queryable, Selectable, Associations, Serialize, Clone)]
#[diesel(table_name = resource)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User, foreign_key = created_by_id))]
#[diesel(belongs_to(Resource, foreign_key = parent_resource_id))]
pub struct Resource {
    pub id: Uuid,
    pub parent_resource_id: Option<Uuid>,
    pub created_by_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
