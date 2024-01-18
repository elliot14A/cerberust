#![allow(dead_code)]
pub(crate) mod account;
pub(crate) mod user;

use once_cell::sync::Lazy;
use repositories::{
    CreateUserInput, Error, Result, UpdateUserInput, UserRepository, UserWhereInput,
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
