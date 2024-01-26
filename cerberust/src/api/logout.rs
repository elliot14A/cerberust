use std::sync::Arc;

use crate::error::{ApiErrResp, Result};
use axum::response::IntoResponse;
use axum::Extension;
use hyper::StatusCode;
use repositories::refresh_token::RefreshTokenWhereInput;
use repositories::DatabaseRepository;
use tower_cookies::Cookies;

use crate::utils::jwt::Claims;

pub async fn logout<H: DatabaseRepository>(
    claims: Claims,
    cookies: Cookies,
    Extension(ctx): Extension<Arc<H>>,
) -> Result<impl IntoResponse> {
    let Claims {
        user_id,
        session_id,
        ..
    } = claims;
    let mut cookie = cookies
        .get("cerberust_session_cookie")
        .ok_or(ApiErrResp::unauthorized(Some("unauthorized".to_string())))?;
    cookie.make_removal();
    let cookie = cookie.to_string();
    let session = ctx.find_session(session_id.clone(), user_id).await?;
    if !session.valid {
        return Err(ApiErrResp::unauthorized(Some("unauthorized".to_string())));
    }
    let refresh_token = cookie.split("=").nth(1).map(|s| s.to_string());

    ctx.delete_refresh_token(RefreshTokenWhereInput {
        id: None,
        session_id: Some(session_id.clone()),
        token: refresh_token,
    })
    .await?;
    ctx.invalidate_session(session_id.clone()).await?;

    Ok(StatusCode::NO_CONTENT)
}
