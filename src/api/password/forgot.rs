use std::sync::Arc;

use crate::{
    actions::token::create::create_token,
    api::VerifyOrResetRequestBody,
    error::{ApiErrResp, Result},
    extractors::FromValidatedJson,
    models::token::{NewToken, TokenType},
    utils::{hash::hash_password, response::to_response, smtp::SmtpService},
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use hyper::StatusCode;
use validator::Validate;

pub async fn forgot_password_send_email(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Extension(smtp): Extension<Arc<SmtpService>>,
    Json(VerifyOrResetRequestBody { email }): Json<VerifyOrResetRequestBody>,
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;

    let user = crate::actions::user::details::get_user_by_email(&mut conn, email)
        .await?
        .ok_or_else(|| ApiErrResp {
            code: StatusCode::NOT_FOUND,
            error: "NOT_FOUND".to_string(),
            message: "User not found".to_string(),
        })?;
    let user_id = user.id.clone();
    let email = user.email.clone();
    //
    let token = uuid::Uuid::new_v4().to_string();
    create_token(
        &mut conn,
        NewToken {
            user_id,
            token_text: &token,
            token_type: TokenType::VerifyEmail,
        },
    )
    .await?;
    tokio::spawn(async move {
        smtp.send_password_reset_email(email, token).unwrap();
    });
    //
    let response =
        to_response::<Option<String>>(format!("reset password email sent to {}", user.email), None);

    Ok(Json(response))
}

#[derive(serde::Deserialize, Validate)]
pub struct ResetPasswordRequestBody {
    #[validate(length(min = 8))]
    new_password: String,
}

pub async fn reset_password(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Path(token): Path<String>,
    FromValidatedJson(ResetPasswordRequestBody { new_password }): FromValidatedJson<
        ResetPasswordRequestBody,
    >,
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;

    let token = crate::actions::token::details::get_token_by_token(&mut conn, token.clone())
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
    //
    //
    let new_password = hash_password(new_password).await.map_err(|e| {
        ApiErrResp::internal_server_error(format!("error hashing password:{}", e.to_string()))
    })?;
    //
    let user =
        crate::actions::user::update::update_password(&mut conn, token.user_id, new_password)
            .await?;

    // delete all reset_password tokens for this user
    crate::actions::token::delete::delete_user_tokens(
        &mut conn,
        token.user_id,
        TokenType::ResetPassword,
    )
    .await?;
    //
    let response = to_response::<Option<String>>(
        format!("password reset successful for user:{}", user.username),
        None,
    );

    Ok(Json(response))
}
