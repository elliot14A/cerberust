use std::sync::Arc;

use crate::actions::token::create::create_token;
use crate::actions::user::create::create_user;
use crate::error::ApiErrResp;
use crate::error::Result;
use crate::models::token::TokenType;
use crate::models::user::NewUser;
use crate::{
    extractors::FromValidatedJson,
    utils::{hash::hash_password, smtp::SmtpService},
};
use axum::extract::State;
use axum::{response::IntoResponse, Extension, Json};
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::AsyncPgConnection;
use hyper::StatusCode;

pub async fn register(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Extension(smtp): Extension<Arc<SmtpService>>,
    FromValidatedJson(input): FromValidatedJson<NewUser>,
) -> Result<impl IntoResponse> {
    let NewUser {
        username,
        email,
        password,
    } = input;
    // TODO: implement faster hash function
    let password = hash_password(password).await?;
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;
    let user = create_user(
        &mut conn,
        NewUser {
            username,
            email,
            password,
        },
    )
    .await?;
    let email = user.email.clone();
    let user_id = user.id.clone();
    let token = uuid::Uuid::new_v4().to_string();
    create_token(
        &mut conn,
        crate::models::token::NewToken {
            user_id,
            token_text: &token,
            token_type: TokenType::VerifyEmail,
        },
    )
    .await
    .unwrap();
    // send verification email
    // make sending email async as this might take some time
    tokio::spawn(async move {
        smtp.send_verification_email(email, token).unwrap();
    });
    Ok((StatusCode::CREATED, Json(user)))
}
