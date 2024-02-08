use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    serialize::ToSql,
    sql_types::{Timestamptz, Uuid},
};
use serde::{Deserialize, Serialize};

pub mod refresh_token;
pub mod session;
pub mod token;
pub mod user;

#[derive(Debug, PartialEq, Eq, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[diesel(sql_type = Uuid)]
pub struct UuidString(pub String);

#[derive(Debug, PartialEq, Eq, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[diesel(sql_type = Timestamptz)]
pub struct Timestamp(pub String);

impl ToSql<Uuid, Pg> for UuidString {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        todo!()
    }
}

impl FromSql<Uuid, Pg> for UuidString {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        todo!()
    }
}

impl ToSql<Timestamptz, Pg> for Timestamp {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        todo!()
    }
}

impl FromSql<Timestamptz, Pg> for Timestamp {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        todo!()
    }
}
