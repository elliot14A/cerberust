use crate::account::SurrealAccount;
use repositories::{Error, Result, User};
use serde::Deserialize;
use surrealdb::{opt::RecordId, sql::Datetime};

mod create;
mod details;
mod update;

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
pub fn build_query(params: Vec<(&str, Option<String>)>, separator: &str) -> Result<String> {
    let mut query = " WHERE".to_string();
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
