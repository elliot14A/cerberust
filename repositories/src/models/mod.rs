mod account;
mod user;

pub use account::CreateAccount;

use self::{
    account::{Account, AccountWhereInput},
    user::{CreateUserInput, UpdateUserInput, User, UserWhereInput},
};

pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create_user(&self, input: CreateUserInput) -> Result<User>;
    async fn update_user(&self, input: UpdateUserInput) -> Result<User>;
    async fn get_user(&self, query: UserWhereInput) -> Result<User>;
}

#[async_trait::async_trait]
pub trait AccountRepository {
    async fn create_account(&self, input: CreateAccount) -> Result<Account>;
    async fn get_account(&self, query: AccountWhereInput) -> Result<Account>;
    async fn get_user_accounts(&self, query: UserWhereInput) -> Result<Vec<Account>>;
    async fn delete_account(&self, query: AccountWhereInput) -> Result<Account>;
}
