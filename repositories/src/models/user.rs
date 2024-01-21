use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::account::Account;

/// `CreateUser` represents the data needed to create a new user.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserInput {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

/// `UpdateUser` represents the data needed to update a user.
/// Only the fields that are present will be updated.
/// Only name, email, and password are updatable.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserInput {
    pub id: String,
    #[validate(length(min = 3))]
    pub name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 8))]
    pub password: Option<String>,
    pub email_verified: Option<bool>,
}

/// `User` represents a user in the database.
#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub accounts: Vec<Account>,
}

/// `UserWhereInput` represents the data needed to query for a user.
/// At least one field must be present.
pub struct UserWhereInput {
    pub id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
}
