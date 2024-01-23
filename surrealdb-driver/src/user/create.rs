use repositories::user::{CreateUserInput, User};
use repositories::{Error, Result};

use crate::{user::SurrealUser, DB};

pub async fn create(input: CreateUserInput) -> Result<User> {
    let CreateUserInput {
        name,
        email,
        password,
    } = input;

    let surql = r#"
            create user content {
                email: $email,
                password: $password,
                name: $name
            }
        "#;
    let mut response = DB
        .query(surql)
        .bind(("email", email))
        .bind(("password", password))
        .bind(("name", name))
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
