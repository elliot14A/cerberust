use std::sync::Arc;

use crate::{
    error::{ApiErrResp, Result},
    utils::{hash::hash_password, response::to_response, smtp::SmtpService},
};
use axum::{extract::Path, response::IntoResponse, Extension, Json};

use super::VerifyOrResetRequestBody;

pub async fn forgot_password_send_email(
    // Extension(ctx): Extension<Arc<H>>,
    Extension(smtp): Extension<Arc<SmtpService>>,
    Json(VerifyOrResetRequestBody { email }): Json<VerifyOrResetRequestBody>,
) -> Result<impl IntoResponse> {
    // let user = ctx
    //     .get_user(UserWhereInput {
    //         id: None,
    //         email: Some(email),
    //         name: None,
    //     })
    //     .await?;
    // let user_id = user.id.clone();
    // let email = user.email.clone();
    //
    // tokio::spawn(async move {
    //     let token = uuid::Uuid::new_v4().to_string();
    //     ctx.create_token(CreateTokenInput {
    //         user_id,
    //         token: token.clone(),
    //         token_type: "reset_password".to_string(),
    //     })
    //     .await
    //     .unwrap();
    //     smtp.send_password_reset_email(email, token).unwrap();
    // });
    //
    // let response =
    //     to_response::<Option<String>>(format!("reset password email sent to {}", user.email), None);
    //
    // Ok(Json(response))
    Ok(())
}

#[derive(serde::Deserialize)]
pub struct ResetPasswordRequestBody {
    new_password: String,
}

pub async fn reset_password(
    // Extension(ctx): Extension<Arc<H>>,
    Path(token): Path<String>,
    Json(ResetPasswordRequestBody { new_password }): Json<ResetPasswordRequestBody>,
) -> Result<impl IntoResponse> {
    // let token = ctx.find_token(token).await?;
    //
    // let now = chrono::Utc::now();
    // if now.signed_duration_since(token.created_at).num_hours() > 1 {
    //     return Err(ApiErrResp::unauthorized(Some("Token expired".to_string())));
    // }
    //
    // // make hashing password async since it's cpu intensive
    // let new_password = hash_password(new_password).await.unwrap();
    //
    // let user = ctx
    //     .update_user(UpdateUserInput {
    //         id: token.user_id,
    //         password: Some(new_password),
    //         name: None,
    //         email: None,
    //         email_verified: None,
    //     })
    //     .await?;
    //
    // ctx.delete_token(TokenWhereInput {
    //     id: None,
    //     user_id: None,
    //     token_type: "reset_password".to_string(),
    // })
    // .await?;
    //
    // // delete all reset_password tokens for this user
    //
    // let response = to_response::<Option<String>>(
    //     format!("password reset successful for user:{}", user.name),
    //     None,
    // );
    //
    // Ok(Json(response))
    Ok(())
}
