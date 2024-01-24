use std::sync::Arc;

use crate::error::ApiErrResp;
use crate::utils::jwt::{self, TokenType};
use crate::utils::response::to_response;
use crate::{error::Result, utils::hash::verify_password};
use axum::{response::IntoResponse, Extension, Json};
use repositories::token::CreateTokenInput;
use repositories::{user::UserWhereInput, DatabaseRepository};
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

    let access_token = jwt::create_token(user.id.clone(), TokenType::AccessToken).await?;
    let refresh_token = jwt::create_token(user.id.clone(), TokenType::RefreshToken).await?;

    // save refresh token to db
    ctx.create_token(CreateTokenInput {
        user_id: user.id,
        token: refresh_token.clone(),
        token_type: "refresh_token".to_string(),
    })
    .await?;
    let mut cookie = Cookie::new("refresh_token", refresh_token);
    // cookie.set_secure(true);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Strict);
    cookies.add(cookie);

    let response = to_response::<String>(format!("Login successful"), access_token);

    Ok(Json(response))
}
