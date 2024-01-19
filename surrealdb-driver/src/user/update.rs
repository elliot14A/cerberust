use crate::{
    build_query,
    user::{details, SurrealUser},
};
use repositories::{Error, Result, UpdateUserInput, User, UserWhereInput};

pub async fn update(input: UpdateUserInput) -> Result<User> {
    let UpdateUserInput {
        id,
        name,
        email,
        password,
    } = input;
    let query = format!("UPDATE user:{} set", id)
        + build_query(
            "",
            vec![("name", name), ("email", email), ("password", password)],
            " ,",
        )?
        .as_str();
    let _ = details::get_user(UserWhereInput {
        id: Some(id),
        email: None,
        name: None,
    })
    .await?;

    let mut response = crate::DB
        .query(&query)
        .await
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?;

    let user: Option<SurrealUser> = response
        .take::<Vec<SurrealUser>>(0)
        .map_err(|e| Error::UsernameOrEmailAlreadyExists {
            message: e.to_string(),
        })?
        .pop();

    Ok(user.unwrap().into())
}
