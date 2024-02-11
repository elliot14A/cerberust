use crate::{
    actions::{token::create::create_token, user::details::get_user_by_email},
    error::{ApiErrResp, Result},
    extractors::FromValidatedJson,
    models::token::{NewToken, TokenType},
    utils::{response::to_response, smtp::SmtpService},
};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use hyper::StatusCode;
use std::sync::Arc;

use crate::api::VerifyOrResetRequestBody;

pub async fn resend_verification_email(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Extension(smtp): Extension<Arc<SmtpService>>,
    FromValidatedJson(VerifyOrResetRequestBody { email }): FromValidatedJson<
        VerifyOrResetRequestBody,
    >,
) -> Result<impl IntoResponse> {
    // get a connection from the pool
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;

    let user = get_user_by_email(&mut conn, email)
        .await?
        .map(|user| {
            if user.email_verified {
                return Err(ApiErrResp {
                    code: StatusCode::BAD_REQUEST,
                    error: "BAD_REQUEST".to_string(),
                    message: "Email already verified".to_string(),
                });
            }
            Ok(user)
        })
        .ok_or_else(|| ApiErrResp {
            code: StatusCode::NOT_FOUND,
            message: "User not found".to_string(),
            error: "NOT_FOUND".to_string(),
        })??;

    let user_id = user.id.clone();
    let email = user.email.clone();

    let token = uuid::Uuid::new_v4().to_string();
    // create a new verify email token
    create_token(
        &mut conn,
        NewToken {
            user_id,
            token_text: &token,
            token_type: TokenType::VerifyEmail,
        },
    )
    .await?;

    // send verification email
    tokio::spawn(async move {
        let _ = smtp.send_verification_email(email, token);
    });

    // return response
    let response = to_response::<Option<String>>(
        "Verification email sent, please check your email".to_string(),
        None,
    );

    Ok(Json(response))
}
