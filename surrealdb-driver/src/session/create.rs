use repositories::{
    session::{CreateSessionInput, Session},
    Error, Result,
};

use super::SurrealSession;

pub async fn create(input: CreateSessionInput) -> Result<Session> {
    let CreateSessionInput { user_id } = input;
    let surql = r#"
        create session content {
            user: $user_id
        }
        "#;
    let mut response = crate::DB
        .query(surql)
        .bind(("user_id", user_id))
        .await
        .map_err(|e| {
            println!("{:?}", e);
            Error::InternalError {
                message: e.to_string(),
            }
        })?;
    let session: Option<Session> = response
        .take::<Option<SurrealSession>>(0)
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?
        .map(|d| d.into());

    Ok(session.unwrap())
}
