pub mod account;
pub mod token;
pub mod user;
pub mod refresh_token;
pub mod session;

use account::CreateAccountInput;

use self::token::{CreateTokenInput, Token, TokenWhereInput};
use self::{
    account::{Account, AccountWhereInput},
    user::{CreateUserInput, UpdateUserInput, User, UserWhereInput},
};

/// Error type for db operations.
#[derive(Debug)]
pub enum Error {
    UserNotFound { message: String },
    UsernameOrEmailAlreadyExists { message: String },
    AccountNotFound { message: String },
    AccountAlreadyExists { message: String },
    AccountNotOwnedByUser { id: String },
    InvalidQuery { message: String },
    InternalError { message: String },
    TokenNotFound,
}

pub type Result<T> = std::result::Result<T, Error>;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create_user(&self, input: CreateUserInput) -> Result<User>;
    async fn update_user(&self, input: UpdateUserInput) -> Result<User>;
    async fn get_user(&self, query: UserWhereInput) -> Result<User>;
}

#[async_trait::async_trait]
pub trait AccountRepository {
    async fn create_account(&self, input: CreateAccountInput) -> Result<Account>;
    async fn get_account(&self, query: AccountWhereInput) -> Result<Account>;
    async fn get_user_accounts(&self, user_id: String) -> Result<Vec<Account>>;
    async fn delete_account(&self, query: AccountWhereInput) -> Result<()>;
}

#[async_trait::async_trait]
pub trait TokenRepository {
    async fn create_token(&self, input: CreateTokenInput) -> Result<Token>;
    async fn delete_token(&self, token: TokenWhereInput) -> Result<()>;
    async fn find_token(&self, token: String) -> Result<Token>;
}
