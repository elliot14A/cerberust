use crate::{
    error::{ApiErrResp, Result},
    utils::{response::to_response, smtp::SmtpService},
};
use axum::{response::IntoResponse, Extension, Json};
use hyper::StatusCode;
use repositories::{token::CreateTokenInput, DatabaseRepository, UserWhereInput};
use std::sync::Arc;

use super::VerifyOrResetRequestBody;

pub async fn resend_verification_email<H: DatabaseRepository>(
    Extension(ctx): Extension<Arc<H>>,
    Extension(smtp): Extension<Arc<SmtpService>>,
    Json(body): Json<VerifyOrResetRequestBody>,
) -> Result<impl IntoResponse> {
    let email = body.email;
    let user = ctx
        .get_user(UserWhereInput {
            id: None,
            email: Some(email),
            name: None,
        })
        .await?;
    if user.email_verified {
        return Err(ApiErrResp {
            code: StatusCode::CONFLICT,
            error: "CONFLICT".to_string(),
            message: "Email already verified".to_string(),
        });
    }
    let user_id = user.id.clone();
    let email = user.email.clone();

    tokio::spawn(async move {
        let token = uuid::Uuid::new_v4().to_string();
        // ignore the error for now
        ctx.create_token(CreateTokenInput {
            user_id,
            token: token.clone(),
            token_type: "email_verification".to_string(),
        })
        .await
        .unwrap();
        let _ = smtp.send_verification_email(email, token);
    });

    let response = to_response::<Option<String>>(
        "Verification email sent, please check your email".to_string(),
        None,
    );

    Ok(Json(response))
}
