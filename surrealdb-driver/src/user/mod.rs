use crate::account::SurrealAccount;
use repositories::{Error, Result, User};
use serde::Deserialize;
use surrealdb::{opt::RecordId, sql::Datetime};

pub mod create;
pub mod details;
pub mod update;

#[derive(Deserialize)]
/// Internal representation of a user.
struct SurrealUser {
    id: RecordId,
    email: String,
    password: String,
    name: String,
    created_at: Datetime,
    updated_at: Datetime,
    email_verified: bool,
    accounts: Vec<SurrealAccount>,
}

impl From<SurrealUser> for User {
    fn from(value: SurrealUser) -> Self {
        let SurrealUser {
            id,
            email,
            password,
            name,
            created_at,
            updated_at,
            email_verified,
            accounts,
        } = value;
        Self {
            id: id.id.to_string(),
            email,
            password,
            name,
            created_at: created_at.into(),
            updated_at: updated_at.into(),
            email_verified,
            accounts: accounts.into_iter().map(|a| a.into()).collect(),
        }
    }
}

/// Builds the query string for the given parameters.
/// params is a vector of tuples of the form (key, value).
/// key is the name of the field and value is the value of the field.
/// separator is the string used to separate the conditions.
pub fn build_query(
    prefix: &str,
    params: Vec<(&str, Option<String>)>,
    separator: &str,
) -> Result<String> {
    let mut query = prefix.to_string();
    let conditions: Vec<String> = params
        .iter()
        .filter_map(|(key, value)| value.clone().map(|value| format!(" {} = '{}'", key, value)))
        .collect();

    if conditions.is_empty() {
        return Err(Error::InvalidQuery {
            message: "Atleast one of the fields must be present".into(),
        });
    }

    query.push_str(&conditions.join(separator));

    Ok(query)
}

// TODO: Improve the tests.
#[cfg(test)]
mod test {
    use repositories::{UpdateUserInput, UserRepository};

    use crate::SurrealDriver;

    #[tokio::test]
    async fn success_test() {
        let surrealdb = SurrealDriver::new(
            "localhost:8000".to_string(),
            "auth".to_string(),
            "auth".into(),
        );
        surrealdb.init().await.unwrap();
        let user = surrealdb
            .create_user(repositories::CreateUserInput {
                name: "John Doe".into(),
                email: "john@email.com".into(),
                password: "123456".into(),
            })
            .await;
        assert_eq!(user.is_ok(), true);
        let user = user.unwrap();
        assert_eq!(user.name, "John Doe".to_string());
        assert_eq!(user.email, "john@email.com".to_string());
        assert_eq!(user.password, "123456".to_string());
        assert_eq!(user.email_verified, false);
        assert_eq!(user.accounts.len(), 0);

        let user = surrealdb
            .get_user(repositories::UserWhereInput {
                id: Some(user.id),
                email: None,
                name: None,
            })
            .await;

        assert_eq!(user.is_ok(), true);
        let user = user.unwrap();
        assert_eq!(user.name, "John Doe".to_string());

        let user = surrealdb
            .update_user(UpdateUserInput {
                id: user.id,
                name: Some("Jane Doe".into()),
                email: None,
                password: None,
            })
            .await;

        println!("{:?}", user);
        assert_eq!(user.is_ok(), true);
        let user = user.unwrap();

        assert_eq!(user.name, "Jane Doe".to_string());

        let not_user = surrealdb
            .create_user(repositories::CreateUserInput {
                name: "John Doe".into(),
                email: "john@email.com".into(),
                password: "123456".into(),
            })
            .await;
        assert_eq!(not_user.is_ok(), false);

        let not_user = surrealdb
            .get_user(repositories::UserWhereInput {
                id: Some("123".into()),
                email: None,
                name: None,
            })
            .await;
        assert_eq!(not_user.is_ok(), false);

        let not_user = surrealdb
            .update_user(UpdateUserInput {
                id: "123".to_string(),
                name: None,
                email: None,
                password: None,
            })
            .await;
        assert_eq!(not_user.is_ok(), false);
    }
}
