use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use hyper::StatusCode;
use uuid::Uuid;

use crate::{
    error::{ApiErrResp, Result},
    schema::role,
};

use super::details::get_role_by_id;

pub async fn delete_role(conn: &mut AsyncPgConnection, role_id: Uuid) -> Result<()> {
    let role = get_role_by_id(conn, role_id).await?;

    if let Some(role) = role {
        if role.is_default {
            return Err(ApiErrResp {
                code: StatusCode::FORBIDDEN,
                error: String::from("FORBIDDEN"),
                message: String::from("You cannot delete a default role"),
            });
        }
        diesel::delete(role::table.filter(role::id.eq(role_id)))
            .execute(conn)
            .await?;
        return Ok(());
    }

    Err(ApiErrResp {
        code: StatusCode::NOT_FOUND,
        error: String::from("NOT_FOUND"),
        message: format!("Cannot find the role by id: {}", role_id),
    })
}
