#![allow(dead_code)]
pub(crate) mod account;
pub(crate) mod user;

use once_cell::sync::Lazy;
use repositories::{
    AccountRepository, AccountWhereInput, CreateAccountInput, CreateUserInput, Database, Error,
    Result, UpdateUserInput, UserRepository, UserWhereInput,
};
use surrealdb::{
    engine::remote::http::{Client, Http},
    Surreal,
};
use user::{create::create, details::get_user, update::update};

pub(crate) static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
pub struct SurrealDriver {
    db_url: String,
    ns: String,
    db: String,
}

#[async_trait::async_trait]
impl UserRepository for SurrealDriver {
    async fn create_user(
        &self,
        input: CreateUserInput,
    ) -> repositories::Result<repositories::User> {
        create(input).await
    }

    async fn update_user(&self, input: UpdateUserInput) -> Result<repositories::User> {
        update(input).await
    }

    async fn get_user(&self, query: UserWhereInput) -> Result<repositories::User> {
        get_user(query).await
    }
}

#[async_trait::async_trait]
impl AccountRepository for SurrealDriver {
    async fn create_account(&self, input: CreateAccountInput) -> Result<repositories::Account> {
        account::create::create(input).await
    }
    async fn get_account(&self, query: AccountWhereInput) -> Result<repositories::Account> {
        account::details::get_account(query).await
    }

    async fn get_user_accounts(&self, user_id: String) -> Result<Vec<repositories::Account>> {
        account::list::get_user_accounts(user_id).await
    }
    async fn delete_account(&self, query: AccountWhereInput) -> Result<()> {
        account::delete::delete(query).await
    }
}

impl SurrealDriver {
    /// Creates a new instance of the SurrealDriver with given db_url, ns and db.
    pub fn new(db_url: String, ns: String, db: String) -> Self {
        Self { db_url, ns, db }
    }

    /// Initializes the surrealdb connection.
    pub async fn init(&self) -> Result<()> {
        DB.connect::<Http>(&self.db_url)
            .await
            .map_err(|e| Error::InternalError {
                message: format!("Unable to connect to surrealdb: {}", e.to_string()),
            })?;

        // use the auth namespace and auth database
        DB.use_ns(&self.ns)
            .use_db(&self.db)
            .await
            .map_err(|e| Error::InternalError {
                message: format!("Unable to connect to surrealdb: {}", e.to_string()),
            })?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Database for SurrealDriver {
    async fn new(db_url: &str, ns: &str, db: &str) -> Self {
        let surrealdb = SurrealDriver::new(db_url.to_string(), ns.to_string(), db.to_string());
        let s = surrealdb.init().await;
        if s.is_err() {
            panic!("Unable to connect to surrealdb: {:?}", s.unwrap_err());
        }
        surrealdb
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

#[cfg(test)]
mod test {
    use repositories::{AccountRepository, UpdateUserInput, UserRepository};

    use crate::SurrealDriver;

    #[tokio::test]
    async fn test() {
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

        let account = surrealdb
            .create_account(repositories::CreateAccountInput {
                user_id: user.id.clone(),
                account_type: "github".into(),
                provider_account_id: "123".into(),
                provider: "github".into(),
            })
            .await;
        assert_eq!(account.is_ok(), true);
        let account = account.unwrap();
        assert_eq!(account.user_id, user.id.clone());

        let account = surrealdb
            .get_account(repositories::AccountWhereInput {
                id: Some(account.id),
                user_id: None,
                account_type: None,
                provider_account_id: None,
                provider: None,
            })
            .await;

        assert_eq!(account.is_ok(), true);
        let account = account.unwrap();
        assert_eq!(account.user_id, user.id.clone());

        let accounts = surrealdb.get_user_accounts(user.id).await;

        assert_eq!(accounts.is_ok(), true);
        let accounts = accounts.unwrap();
        assert_eq!(accounts.len(), 1);
    }
}
