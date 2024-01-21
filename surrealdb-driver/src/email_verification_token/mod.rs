use repositories::EmailVerificationToken;
use serde::Deserialize;
use surrealdb::{opt::RecordId, sql::Datetime};

pub mod create;
pub mod delete;
pub mod details;

#[derive(Deserialize)]
pub struct SurrealEmailVerificationToken {
    id: RecordId,
    user: RecordId,
    token: String,
    created_at: Datetime,
}

impl From<SurrealEmailVerificationToken> for EmailVerificationToken {
    fn from(value: SurrealEmailVerificationToken) -> Self {
        Self {
            id: value.id.id.to_string(),
            user_id: value.user.id.to_string(),
            token: value.token,
            created_at: value.created_at.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use repositories::{CreateEmailVerificationTokenInput, EmailVerificationTokenRepository};
    use surrealdb::sql::Uuid;

    use crate::SurrealDriver;

    #[tokio::test]
    async fn test() {
        let surrealdb = SurrealDriver {
            db_url: "localhost:8000".to_string(),
            ns: "auth".to_string(),
            db: "auth".into(),
        };

        surrealdb.init().await.unwrap();
        let token_string = Uuid::new_v4().to_string();
        let token = surrealdb
            .create_token(CreateEmailVerificationTokenInput {
                user_id: "dasdadf".to_string(),
                token: token_string.clone(),
            })
            .await;
        assert_eq!(token.is_ok(), true);
        let token = token.unwrap();
        assert_eq!(token.token, token_string);

        let token = surrealdb
            .find_one_token(repositories::EmailVerificationTokenWhereInput {
                user_id: None,
                token: Some(token.token),
                id: None,
            })
            .await;

        println!("{:?}", token);

        assert_eq!(token.is_ok(), true);
        let token = token.unwrap();
        assert_eq!(token.token, token_string);
    }
}
