use diesel::{
    ExpressionMethods, JoinOnDsl, NullableExpressionMethods, OptionalExtension, QueryDsl,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{error::Result, models::resource::Resource, schema::resource};

pub async fn get_resource(
    conn: &mut AsyncPgConnection,
    resource_id: Uuid,
) -> Result<Option<Resource>> {
    Ok(resource::table
        .filter(resource::id.eq(resource_id))
        .first(conn)
        .await
        .optional()?)
}

/// get parent resource of a resource by id
pub async fn get_parent_resource(
    conn: &mut AsyncPgConnection,
    resource_id: Uuid,
) -> Result<Option<Resource>> {
    let (child, parent) = diesel::alias!(resource as child, resource as parent);
    let parent_resource: Vec<Resource> = child
        .filter(child.field(resource::id).eq(resource_id))
        .inner_join(
            parent.on(child
                .field(resource::parent_resource_id)
                .eq(parent.field(resource::id).nullable())),
        )
        .select(parent.fields(resource::all_columns))
        .load(conn)
        .await?;
    // return the first parent resource if it exists
    // else return None
    Ok(parent_resource.into_iter().next())
}

/// get parent resource id of a resource by id
pub async fn get_parent_resource_id(
    conn: &mut AsyncPgConnection,
    resource_id: Uuid,
) -> Result<Option<Uuid>> {
    let (child, parent) = diesel::alias!(resource as child, resource as parent);
    let parent_resource: Vec<Uuid> = child
        .filter(child.field(resource::id).eq(resource_id))
        .inner_join(
            parent.on(child
                .field(resource::parent_resource_id)
                .eq(parent.field(resource::id).nullable())),
        )
        .select(parent.fields(resource::id))
        .load(conn)
        .await?;
    // return the first parent resource if it exists
    // else return None
    Ok(parent_resource.into_iter().next())
}

pub async fn get_resource_by_id(
    conn: &mut AsyncPgConnection,
    resource_id: Uuid,
) -> Result<Option<Resource>> {
    Ok(resource::table
        .filter(resource::id.eq(resource_id))
        .first(conn)
        .await
        .optional()?)
}
