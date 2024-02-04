use std::sync::Arc;

use crate::{
    error::{ApiErrResp, Result},
    utils::{
        jwt::{create_token, verify_token, Claims, TokenType},
        response::to_response,
    },
};
use axum::{response::IntoResponse, Extension, Json};
use serde_json::Value;
use tower_cookies::{cookie::SameSite, Cookie, Cookies};

pub async fn refesh(
    cookies: Cookies,
    // Extension(ctx): Extension<Arc<H>>,
) -> Result<impl IntoResponse> {
    // let cookie = cookies
    //     .get("cerberust_session_cookie")
    //     .ok_or(ApiErrResp::unauthorized(Some("unauthorized".to_string())))?
    //     .to_string();
    //
    // let refresh_token = cookie.split("=").nth(1).map(|s| s.to_string());
    // if refresh_token.is_none() {
    //     return Err(ApiErrResp::unauthorized(Some("unauthorized".to_string())));
    // }
    // let refresh_token = refresh_token.unwrap();
    // // verify refresh token
    // let claims = verify_token(refresh_token.clone(), TokenType::RefreshToken)?;
    // let Claims {
    //     session_id,
    //     user_id,
    //     ..
    // } = claims;
    //
    // let session = ctx
    //     .find_session(session_id.clone(), user_id.clone())
    //     .await?;
    // if !session.valid {
    //     return Err(ApiErrResp::unauthorized(Some("unauthorized".to_string())));
    // }
    // cookies.remove(Cookie::new("cerberust_session_cookie", ""));
    //
    // let refresh_token = ctx.find_refresh_token(refresh_token).await;
    //
    // if refresh_token.is_err() {
    //     let err = refresh_token.unwrap_err();
    //     match err {
    //         Error::TokenNotFound => {
    //             // detected reuse of refresh token
    //             // invalidate session the user is trying to refresh
    //             // delete refresh token of the session
    //             // and return unauthorized
    //             ctx.invalidate_session(session_id.clone()).await?;
    //             ctx.delete_refresh_token(RefreshTokenWhereInput {
    //                 id: None,
    //                 session_id: Some(session_id.clone()),
    //                 token: None,
    //             })
    //             .await?;
    //             return Err(ApiErrResp::unauthorized(Some("unauthorized".to_string())));
    //         }
    //         _ => {
    //             return Err(ApiErrResp::internal_server_error(
    //                 "internal server error".to_string(),
    //             ))
    //         }
    //     }
    // }
    // // delete refresh token
    // let refresh_token = refresh_token.unwrap();
    // ctx.delete_refresh_token(RefreshTokenWhereInput {
    //     id: Some(refresh_token.id),
    //     session_id: None,
    //     token: None,
    // })
    // .await?;
    //
    // let access_token =
    //     create_token(user_id.clone(), session_id.clone(), TokenType::AccessToken).await?;
    // let refresh_token =
    //     create_token(user_id.clone(), session_id.clone(), TokenType::RefreshToken).await?;
    // ctx.create_refresh_token(RefreshTokenCreateInput {
    //     session_id,
    //     token: refresh_token.clone(),
    // })
    // .await?;
    //
    // let mut cookie = Cookie::new("cerberust_session_cookie", refresh_token.clone());
    // // cookie.set_secure(true);
    // cookie.set_http_only(true);
    // cookie.set_same_site(SameSite::Strict);
    // cookies.add(cookie);
    //
    // let json = serde_json::json!({
    //     "access_token": access_token,
    // });
    //
    // let response = to_response::<Value>("refresh successful".to_string(), json);
    //
    // Ok(Json(response))
    Ok(())
}
