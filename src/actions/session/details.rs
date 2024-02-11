use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::error::Result;
use crate::{error::handle_diesel_error, models::session::Session};

pub async fn get_session_by_id(
    conn: &mut AsyncPgConnection,
    session_id: Uuid,
    user_id: Option<Uuid>,
) -> Result<Option<Session>> {
    if let Some(user_id) = user_id {
        Ok(crate::schema::session::table
            .filter(crate::schema::session::user_id.eq(user_id))
            .find(session_id)
            .first::<Session>(conn)
            .await
            .optional()
            .map_err(handle_diesel_error)?)
    } else {
        Ok(crate::schema::session::table
            .find(session_id)
            .first::<Session>(conn)
            .await
            .optional()
            .map_err(handle_diesel_error)?)
    }
}
