use std::sync::Arc;

use crate::error::ApiErrResp;
use crate::utils::jwt::{self, TokenType};
use crate::utils::response::to_response;
use crate::{error::Result, utils::hash::verify_password};
use axum::{response::IntoResponse, Extension, Json};
use repositories::refresh_token::RefreshTokenCreateInput;
use repositories::session::CreateSessionInput;
use repositories::{user::UserWhereInput, DatabaseRepository};
use serde_json::{json, Value};
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies};

#[derive(serde::Deserialize)]
pub struct LoginRequestBody {
    pub email: String,
    pub password: String,
}

pub async fn login<H: DatabaseRepository>(
    Extension(ctx): Extension<Arc<H>>,
    cookies: Cookies,
    Json(LoginRequestBody { email, password }): Json<LoginRequestBody>,
) -> Result<impl IntoResponse> {
    let user = ctx
        .get_user(UserWhereInput {
            id: None,
            email: Some(email),
            name: None,
        })
        .await?;

    if !user.email_verified {
        return Err(ApiErrResp::unauthorized(Some(
            "Email not verified".to_string(),
        )));
    }

    verify_password(password, user.password).await?;

    let session = ctx
        .create_session(CreateSessionInput {
            user_id: user.id.clone(),
        })
        .await?;

    let access_token =
        jwt::create_token(user.id.clone(), session.id.clone(), TokenType::AccessToken).await?;
    let refresh_token =
        jwt::create_token(user.id.clone(), session.id.clone(), TokenType::RefreshToken).await?;

    // save refresh token to db
    // ctx.create_token(CreateTokenInput {
    //     user_id: user.id,
    //     token: refresh_token.clone(),
    //     token_type: "refresh_token".to_string(),
    // })
    // .await?;
    let mut cookie = Cookie::new("cerberust_session_cookie", refresh_token.clone());
    // cookie.set_secure(true);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Strict);
    cookies.add(cookie);

    ctx.create_refresh_token(RefreshTokenCreateInput {
        session_id: session.id.clone(),
        token: refresh_token.clone(),
    })
    .await?;

    let json = json!({
        "access_token": access_token,
    });
    let response = to_response::<Value>(format!("Login successful"), json);

    Ok(Json(response))
}
