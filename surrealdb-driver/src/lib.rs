#![allow(dead_code)]
pub(crate) mod account;
pub(crate) mod user;

use once_cell::sync::Lazy;
use repositories::{
    AccountRepository, AccountWhereInput, CreateAccountInput, CreateUserInput, Error, Result,
    UpdateUserInput, UserRepository, UserWhereInput,
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
// #[async_trait]
// impl Database for DatabaseImpl {
//     async fn new() -> Self {
//         println!("surreadb impl");
//         Self
//     }
// }
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
