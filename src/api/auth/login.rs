use std::sync::Arc;

use crate::actions::refresh_token::create::create_refresh_token;
use crate::actions::session::create::create_session;
use crate::actions::user::details::get_user_by_email;
use crate::error::ApiErrResp;
use crate::models::refresh_token::NewRefreshToken;
use crate::models::session::NewSession;
use crate::utils::jwt::{self, TokenType};
use crate::{error::Result, utils::hash::verify_password};
use axum::extract::State;
use axum::{response::IntoResponse, Json};
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::AsyncPgConnection;
use serde_json::json;
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies};

#[derive(serde::Deserialize)]
pub struct LoginRequestBody {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    cookies: Cookies,
    Json(LoginRequestBody { email, password }): Json<LoginRequestBody>,
) -> Result<impl IntoResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;
    let user = get_user_by_email(&mut conn, email)
        .await?
        .ok_or_else(|| ApiErrResp::unauthorized(Some("Invalid email or password".to_string())))?;
    //

    if !user.email_verified {
        return Err(ApiErrResp::unauthorized(Some(
            "Email not verified".to_string(),
        )));
    }
    //
    verify_password(password, user.password).await?;
    //
    let session = create_session(
        &mut conn,
        NewSession {
            user_id: user.id.clone(),
        },
    )
    .await?;
    //
    let user_id = user.id.to_string().clone();
    let session_id = session.id.to_string().clone();
    let access_token =
        jwt::create_token(user_id.clone(), session_id.clone(), TokenType::AccessToken).await?;
    let refresh_token =
        jwt::create_token(user_id.clone(), session_id.clone(), TokenType::RefreshToken).await?;
    //
    // // save refresh token to db
    // // ctx.create_token(CreateTokenInput {
    // //     user_id: user.id,
    // //     token: refresh_token.clone(),
    // //     token_type: "refresh_token".to_string(),
    // // })
    // // .await?;
    let mut cookie = Cookie::new("cerberust_session_cookie", refresh_token.clone());
    // cookie.set_secure(true);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Strict);
    cookies.add(cookie);
    //
    create_refresh_token(
        &mut conn,
        NewRefreshToken {
            session_id: session.id,
            token: refresh_token,
        },
    )
    .await?;
    //
    let json = json!({
        "access_token": access_token,
    });

    Ok(Json(json))
}
