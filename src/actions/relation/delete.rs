use crate::{
    error::{ApiErrResp, Result},
    schema::relation,
};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use hyper::StatusCode;
use uuid::Uuid;

pub async fn delete_relation(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
    object_id: Uuid,
) -> Result<()> {
    let role_id = super::details::get_role_id_from_relation(conn, user_id, object_id).await?;

    if let Some(_) = role_id {
        diesel::delete(
            relation::table.filter(
                relation::object_id
                    .eq(object_id)
                    .and(relation::user_id.eq(user_id)),
            ),
        )
        .execute(conn)
        .await?;

        return Ok(());
    }
    Err(ApiErrResp {
        code: StatusCode::NOT_FOUND,
        error: "NOT_FOUND".to_string(),
        message: "Relation not found".to_string(),
    })
}
