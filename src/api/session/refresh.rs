use std::{str::FromStr, sync::Arc};

use crate::{
    actions::{
        refresh_token::{
            create::create_refresh_token, delete::delete_refresh_token_by_token,
            details::get_refresh_token_by_token,
        },
        session::{details::get_session_by_id, update::invalidate_session},
    },
    error::{ApiErrResp, Result},
    models::refresh_token::NewRefreshToken,
    utils::{
        jwt::{create_token, verify_token, Claims, TokenType},
        response::to_response,
    },
};
use axum::{extract::State, response::IntoResponse, Json};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use serde_json::Value;
use tower_cookies::{cookie::SameSite, Cookie, Cookies};
use uuid::Uuid;

pub async fn refresh(
    cookies: Cookies,
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
) -> Result<impl IntoResponse> {
    let cookie = cookies
        .get("cerberust_session_cookie")
        .ok_or(ApiErrResp::unauthorized(Some(
            "cookie not found".to_string(),
        )))?
        .to_string();
    //
    let refresh_token = cookie.split("=").nth(1).map(|s| s.to_string());
    if refresh_token.is_none() {
        return Err(ApiErrResp::unauthorized(Some("invalid cookie".to_string())));
    }
    let refresh_token = refresh_token.unwrap();
    // // verify refresh token
    let claims = verify_token(refresh_token.clone(), TokenType::RefreshToken)?;
    let Claims {
        session_id,
        user_id,
        ..
    } = claims;
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;
    //
    let session_id = Uuid::from_str(&session_id)
        .map_err(|_| ApiErrResp::internal_server_error("invalid session_id".to_string()))?;
    let user_id = Uuid::from_str(&user_id)
        .map_err(|_| ApiErrResp::internal_server_error("invalid user_id".to_string()))?;
    let session = get_session_by_id(&mut conn, session_id, Some(user_id))
        .await?
        .ok_or_else(|| ApiErrResp::unauthorized(Some("session not found".to_string())))?;

    // check if session is valid
    if !session.valid {
        return Err(ApiErrResp::unauthorized(Some(
            "invalid session".to_string(),
        )));
    }

    cookies.remove(Cookie::new("cerberust_session_cookie", ""));
    //
    let refresh_token_string = refresh_token.clone();
    let refresh_token = get_refresh_token_by_token(&mut conn, &refresh_token).await?;
    //
    // detected reuse of refresh token
    // invalidate session the user is trying to refresh
    // delete refresh token of the session
    // and return unauthorized
    if refresh_token.is_none() {
        invalidate_session(&mut conn, session_id).await?;
        delete_refresh_token_by_token(&mut conn, &refresh_token_string).await?;
        return Err(ApiErrResp::unauthorized(Some(
            "refresh token reuse".to_string(),
        )));
    }

    let refresh_token = refresh_token.unwrap();
    delete_refresh_token_by_token(&mut conn, &refresh_token.token).await?;
    //
    let user_id_string = user_id.to_string();
    let session_id_string = session_id.to_string();
    let access_token = create_token(
        user_id_string.clone(),
        session_id_string.clone(),
        TokenType::AccessToken,
    )
    .await?;
    let new_refresh_token = create_token(
        user_id_string.clone(),
        session_id_string.clone(),
        TokenType::RefreshToken,
    )
    .await?;
    create_refresh_token(
        &mut conn,
        NewRefreshToken {
            session_id,
            token: new_refresh_token.clone(),
        },
    )
    .await?;
    let mut cookie = Cookie::new("cerberust_session_cookie", new_refresh_token.clone());
    // cookie.set_secure(true);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Strict);
    cookies.add(cookie);
    //
    let json = serde_json::json!({
        "access_token": access_token,
    });

    let response = to_response::<Value>("refresh successful".to_string(), json);

    Ok(Json(response))
}
