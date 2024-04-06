use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::relation;

#[derive(Debug, Insertable, Deserialize, Clone)]
#[diesel(table_name = relation)]
pub struct NewRelation {
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub resource_id: Uuid,
}

#[derive(Debug, Queryable, Selectable, Serialize, Clone)]
#[diesel(table_name = relation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(user_id, resource_id))]
pub struct Relation {
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub resource_id: Uuid,
}
