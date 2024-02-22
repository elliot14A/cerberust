use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::error::Result;
use crate::models::relation::{NewRelation, Relation};
use crate::schema::relation;

pub async fn create_relation(
    conn: &mut AsyncPgConnection,
    new_relation: NewRelation,
) -> Result<Relation> {
    Ok(diesel::insert_into(relation::table)
        .values(&new_relation)
        .get_result(conn)
        .await?)
}
