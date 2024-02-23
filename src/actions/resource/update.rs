use crate::{
    error::{ApiErrResp, Result},
    models::resource::Resource,
    schema::resource,
};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use hyper::StatusCode;
use uuid::Uuid;

pub async fn update_resource(
    conn: &mut AsyncPgConnection,
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
) -> Result<Option<Resource>> {
    if name.is_none() && description.is_none() {
        return Err(ApiErrResp {
            code: StatusCode::BAD_REQUEST,
            error: "Bad Request".to_string(),
            message: "At least one of name or description should be provided".to_string(),
        });
    }

    let name = name.unwrap_or_else(|| "".to_string());
    let description = description.unwrap_or_else(|| "".to_string());

    let updated_resource = diesel::update(resource::table.filter(resource::id.eq(id)))
        .set((
            resource::name.eq(name),
            resource::description.eq(description),
        ))
        .get_result(conn)
        .await
        .optional()?;

    Ok(updated_resource)
}
