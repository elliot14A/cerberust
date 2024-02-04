use std::sync::Arc;

use crate::{
    error::{ApiErrResp, Result},
    utils::response::to_response,
};
use axum::{extract::Path, response::IntoResponse, Extension, Json};
// use repositories::{token::TokenWhereInput, user::UpdateUserInput, DatabaseRepository};

pub async fn verify(
    // Extension(ctx): Extension<Arc<H>>,
    Path(token): Path<String>,
) -> Result<impl IntoResponse> {
    // let token = ctx.find_token(token.clone()).await?;
    // // if token is created more than an hour ago, return error
    // let now = chrono::Utc::now();
    // if now.signed_duration_since(token.created_at).num_hours() > 1 {
    //     return Err(ApiErrResp::unauthorized(Some("Token expired".to_string())));
    // }
    // let user_id = token.user_id.clone();
    //
    // ctx.delete_token(TokenWhereInput {
    //     id: None,
    //     user_id: Some(user_id),
    //     token_type: "email_verification".to_string(),
    // })
    // .await?;
    //
    // ctx.update_user(UpdateUserInput {
    //     id: token.user_id,
    //     email_verified: Some(true),
    //     name: None,
    //     email: None,
    //     password: None,
    // })
    // .await?;
    //
    // let response = to_response::<Option<String>>("verfied".to_string(), None);
    // Ok(Json(response))
    Ok(())
}
