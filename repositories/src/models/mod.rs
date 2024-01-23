mod account;
mod email_verification_token;
mod reset_password_token;
mod user;

pub use account::CreateAccountInput;
pub use reset_password_token::{
    CreateResetPasswordTokenInput, ResetPasswordToken, ResetPasswordTokenWhereInput,
};

pub use self::email_verification_token::{
    CreateEmailVerificationTokenInput, EmailVerificationToken, EmailVerificationTokenWhereInput,
};
pub use self::{
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
    EmailVerificationTokenNotFound { message: String },
    EmailVerificationTokenAlreadyExists,
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
pub trait EmailVerificationTokenRepository {
    async fn create_token(
        &self,
        input: CreateEmailVerificationTokenInput,
    ) -> Result<EmailVerificationToken>;
    async fn find_one_token(
        &self,
        input: EmailVerificationTokenWhereInput,
    ) -> Result<EmailVerificationToken>;
    async fn delete_token(&self, input: EmailVerificationTokenWhereInput) -> Result<()>;
}
