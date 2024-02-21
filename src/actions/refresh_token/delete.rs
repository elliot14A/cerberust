use crate::{error::Result, schema::refresh_token};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

pub async fn delete_refresh_token_by_id(conn: &mut AsyncPgConnection, id: Uuid) -> Result<()> {
    diesel::delete(refresh_token::table.filter(refresh_token::id.eq(id)))
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn delete_refresh_token_by_token(
    conn: &mut AsyncPgConnection,
    token: &str,
) -> Result<()> {
    diesel::delete(refresh_token::table.filter(refresh_token::token.eq(token)))
        .execute(conn)
        .await?;
    Ok(())
}
