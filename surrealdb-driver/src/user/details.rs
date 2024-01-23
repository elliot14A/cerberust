use repositories::user::{User, UserWhereInput};
use repositories::{Error, Result};

use crate::{user::SurrealUser, DB};

use crate::build_query;

pub async fn get_user(query: UserWhereInput) -> Result<User> {
    let UserWhereInput { id, email, name } = query;
    // check if atleast one of the fields is present
    if id.is_none() && email.is_none() && name.is_none() {
        return Err(Error::InvalidQuery {
            message: "Atleast one of the fields must be present".into(),
        }
        .into());
    }
    let id = id.map(|id| "user:".to_string() + id.as_str());

    let query = "SELECT * FROM user".to_string()
        + build_query(
            " WHERE",
            vec![("id", id), ("email", email), ("name", name)],
            " AND",
        )?
        .as_str();

    let mut response = DB.query(&query).await.map_err(|e| Error::InternalError {
        message: e.to_string(),
    })?;
    let user: Option<SurrealUser> = response.take::<Vec<SurrealUser>>(0).unwrap().pop();
    if user.is_none() {
        let message = format!("User not found with query: {}", query);
        return Err(Error::UserNotFound { message });
    }
    Ok(user.unwrap().into())
}
