use repositories::{token::TokenWhereInput, Error, Result};

use crate::build_query;

pub async fn delete_token(input: TokenWhereInput) -> Result<()> {
    let TokenWhereInput {
        user_id,
        token_type,
        id,
    } = input;
    let user_id = user_id.map(|id| format!("user:{}", id));
    let id = id.map(|id| format!("token:{}", id));

    let surql = r#"DELETE token"#.to_string()
        + build_query(
            " WHERE",
            vec![
                ("id", id),
                ("user", user_id),
                ("token_type", Some(token_type)),
            ],
            " AND",
        )?
        .as_ref();

    crate::DB
        .query(&surql)
        .await
        .map_err(|e| Error::InternalError {
            message: e.to_string(),
        })?;

    Ok(())
}
