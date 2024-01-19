use repositories::{Account, Error, Result};

use crate::{account::SurrealAccount, DB};

pub async fn get_user_accounts(user_id: String) -> Result<Vec<Account>> {
    let surql = r#"
        select * from account where user = $user_id
        "#;

    let mut response = DB
        .query(surql)
        .bind(("user", "user:".to_string() + user_id.as_str()))
        .await
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?;

    let accounts: Vec<Account> = response
        .take::<Vec<SurrealAccount>>(0)
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?
        .into_iter()
        .map(|a| a.into())
        .collect();

    Ok(accounts)
}
