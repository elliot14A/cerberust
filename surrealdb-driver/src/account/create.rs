use repositories::{Account, AccountWhereInput, CreateAccountInput, Error, Result, UserWhereInput};

use crate::{account::SurrealAccount, user::details::get_user, DB};

use super::details::get_account;

// TODO: Reduce the number of queries
async fn create(input: CreateAccountInput) -> Result<Account> {
    let CreateAccountInput {
        user_id,
        account_type,
        provider_account_id,
        provider,
    } = input;
    // check if user exists
    let _ = get_user(UserWhereInput {
        id: Some(user_id.clone()),
        email: None,
        name: None,
    })
    .await?;

    // check if user exists with given provider account
    let account = get_account(AccountWhereInput {
        id: None,
        user_id: Some(user_id.clone()),
        account_type: Some(account_type.clone()),
        provider_account_id: Some(provider_account_id.clone()),
        provider: Some(provider.clone()),
    })
    .await;

    if account.is_ok() {
        return Err(Error::AccountAlreadyExists {
            message: format!(
                "Account already exists for user {} with provider: {}",
                user_id, provider
            ),
        });
    }

    let surql = r#" 
            create account content {
                account_type: $account_type,
                provider_account_id: $provider_account_id,
                provider: $provider
            }
    "#;

    let mut response = DB
        .query(surql)
        .bind(("account_type", account_type))
        .bind(("provider_account_id", provider_account_id))
        .bind(("provider", provider))
        .await
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?;

    let account: Option<SurrealAccount> = response
        .take::<Vec<SurrealAccount>>(0)
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?
        .pop();

    Ok(account.unwrap().into())
}
