use repositories::{Account, AccountWhereInput, Error, Result};

use crate::{build_query, DB};

use super::SurrealAccount;

pub async fn get_account(input: AccountWhereInput) -> Result<Account> {
    let AccountWhereInput {
        id,
        user_id,
        account_type,
        provider_account_id,
        provider,
    } = input;
    let surql = r#" 
            select * from user 
    "#
    .to_string()
        + build_query(
            " where",
            vec![
                ("id", id),
                ("user", user_id),
                ("account_type", account_type),
                ("provider_account_id", provider_account_id),
                ("provider", provider),
            ],
            " AND",
        )?
        .as_str();

    let mut response = DB.query(surql).await.map_err(|e| Error::InternalError {
        message: e.to_string(),
    })?;
    let account: Option<SurrealAccount> = response
        .take::<Vec<SurrealAccount>>(0)
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?
        .pop();
    if account.is_none() {
        return Err(Error::AccountNotFound {
            message: String::from("Account not found for given query"),
        });
    }
    Ok(account.unwrap().into())
}
