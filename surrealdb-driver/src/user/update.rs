use crate::{
    build_query,
    user::{details, SurrealUser},
};
use repositories::{
    user::{UpdateUserInput, User, UserWhereInput},
    Error, Result,
};

pub async fn update(input: UpdateUserInput) -> Result<User> {
    let UpdateUserInput {
        id,
        name,
        email,
        password,
        email_verified,
    } = input;
    let mut query = format!("UPDATE user:{} set", id)
        + build_query(
            "",
            vec![("name", name), ("email", email), ("password", password)],
            " ,",
        )
        .unwrap_or("".to_string())
        .as_str();
    let _ = details::get_user(UserWhereInput {
        id: Some(id),
        email: None,
        name: None,
    })
    .await?;
    let mut response;
    if email_verified.is_some() {
        let email_verified = email_verified.unwrap();
        query = query + format!(" email_verified = $email_verified").as_str();
        response = crate::DB
            .query(&query)
            .bind(("email_verified", email_verified))
            .await
            .map_err(|e| Error::InternalError {
                message: e.to_string(),
            })?;
    } else {
        response = crate::DB
            .query(&query)
            .await
            .map_err(|e| Error::InternalError {
                message: e.to_string(),
            })?;
    }

    let user: Option<SurrealUser> = response
        .take::<Vec<SurrealUser>>(0)
        .map_err(|e| Error::UsernameOrEmailAlreadyExists {
            message: e.to_string(),
        })?
        .pop();

    Ok(user.unwrap().into())
}
