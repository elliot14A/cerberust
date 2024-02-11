use std::sync::Arc;

use crate::{
    actions::{
        token::{delete::delete_user_tokens, details::get_token_by_token},
        user::update::update_email_verified,
    },
    error::{ApiErrResp, Result},
    models::{token::TokenType, user::User},
    utils::response::to_response,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};

pub async fn verify(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Path(token): Path<String>,
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;

    let token = get_token_by_token(&mut conn, token.clone())
        .await?
        .map(|token| {
            // if token is created more than an hour ago, return error
            let now = chrono::Utc::now();
            if now.signed_duration_since(token.created_at).num_hours() > 1 {
                return Err(ApiErrResp::unauthorized(Some("Token expired".to_string())));
            }
            return Ok(token);
        })
        .ok_or_else(|| ApiErrResp::unauthorized(Some(String::from("Invalid Token"))))??;

    // delete all verify email tokens
    let user_id = token.user_id.clone();
    delete_user_tokens(&mut conn, user_id, TokenType::VerifyEmail).await?;

    // update user to verified
    let user = update_email_verified(&mut conn, user_id).await?;

    let response = to_response::<User>("verfied".to_string(), user);
    Ok(Json(response))
}
