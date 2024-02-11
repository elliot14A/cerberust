use std::sync::Arc;

use crate::{
    actions::{
        refresh_token::delete::delete_refresh_token_by_token, session::update::invalidate_session,
    },
    error::{ApiErrResp, Result},
    extractors::authenticator::Authenticated,
};
use axum::{extract::State, response::IntoResponse};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use hyper::StatusCode;
use tower_cookies::{Cookie, Cookies};

pub async fn logout(
    State(pool): State<Arc<Pool<AsyncPgConnection>>>,
    Authenticated(session): Authenticated,
    cookies: Cookies,
) -> Result<impl IntoResponse> {
    let cookie = cookies
        .get("cerberust_session_cookie")
        .ok_or(ApiErrResp::unauthorized(Some("unauthorized".to_string())))?;
    //
    //
    let cookie = cookie.to_string();
    let refresh_token = cookie
        .split("=")
        .nth(1)
        .map(|s| s.to_string())
        .ok_or(ApiErrResp::unauthorized(Some("unauthorized".to_string())))?;
    cookies.remove(Cookie::new("cerberust_session_cookie", ""));
    //
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;
    delete_refresh_token_by_token(&mut conn, &refresh_token).await?;
    invalidate_session(&mut conn, session.id).await?;
    //
    Ok(StatusCode::NO_CONTENT)
}
