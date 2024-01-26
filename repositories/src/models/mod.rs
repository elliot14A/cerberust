pub mod account;
pub mod refresh_token;
pub mod session;
pub mod token;
pub mod user;

use account::CreateAccountInput;

use self::refresh_token::{RefreshToken, RefreshTokenCreateInput, RefreshTokenWhereInput};
use self::session::{CreateSessionInput, Session};
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

#[async_trait::async_trait]
pub trait RefreshTokenRepository {
    async fn delete_refresh_token(&self, input: RefreshTokenWhereInput) -> Result<()>;
    async fn find_refresh_token(&self, token: String) -> Result<RefreshToken>;
    async fn create_refresh_token(&self, input: RefreshTokenCreateInput) -> Result<RefreshToken>;
}

#[async_trait::async_trait]
pub trait SessionRepository {
    async fn create_session(&self, input: CreateSessionInput) -> Result<Session>;
    async fn invalidate_session(&self, session_id: String) -> Result<()>;
    async fn find_session(&self, session_id: String) -> Result<Session>;
}
