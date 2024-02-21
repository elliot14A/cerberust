use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    error::Result,
    models::session::{NewSession, Session},
};
pub async fn create_session(
    conn: &mut AsyncPgConnection,
    new_session: NewSession,
) -> Result<Session> {
    Ok(diesel::insert_into(crate::schema::session::table)
        .values(&new_session)
        .get_result(conn)
        .await?)
}
