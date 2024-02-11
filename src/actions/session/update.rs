use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::error::handle_diesel_error;
use crate::error::Result;

pub async fn invalidate_session(conn: &mut AsyncPgConnection, session_id: Uuid) -> Result<()> {
    diesel::update(crate::schema::session::table.filter(crate::schema::session::id.eq(session_id)))
        .set(crate::schema::session::valid.eq(false))
        .execute(conn)
        .await
        .map_err(handle_diesel_error)?;
    Ok(())
}
