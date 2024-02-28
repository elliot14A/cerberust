use chrono::{DateTime, Utc};
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    prelude::*,
    sql_types::Jsonb,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::schema::role;

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Privilege {
    pub entity: String,
    pub privileges: Vec<String>,
}

#[derive(FromSqlRow, AsExpression, serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[diesel(sql_type = Jsonb)]
pub struct PrivilegeVec(pub Vec<Privilege>);

impl FromSql<Jsonb, Pg> for PrivilegeVec {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        Ok(serde_json::from_value(value)?)
    }
}

#[derive(Debug, Insertable, Validate, Deserialize, Clone)]
#[diesel(table_name = role)]
pub struct NewRole {
    #[validate(length(min = 3, max = 24))]
    pub name: String,
    #[validate(length(min = 8))]
    pub description: Option<String>,
    pub privileges: PrivilegeVec,
    pub is_default: bool,
}

#[derive(Debug, Queryable, Selectable, Serialize, Clone)]
#[diesel(table_name = role)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub privileges: PrivilegeVec,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
