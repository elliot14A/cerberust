use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use uuid::Uuid;

use crate::{
    error::ApiErrResp,
    models::session::Session,
    utils::jwt::{verify_token, TokenType},
};

pub struct Authenticated(pub Session);

#[async_trait]
impl FromRequestParts<Arc<Pool<AsyncPgConnection>>> for Authenticated {
    type Rejection = ApiErrResp;
    async fn from_request_parts(
        parts: &mut Parts,
        pool: &Arc<Pool<AsyncPgConnection>>,
    ) -> std::result::Result<Self, Self::Rejection> {
        let headers = parts.headers.clone();
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split_whitespace().nth(1))
            .ok_or(ApiErrResp::unauthorized(Some("missing token".to_string())))?
            .to_string();
        let claims = verify_token(token, TokenType::AccessToken)?;
        // validate session
        let mut conn = pool
            .get()
            .await
            .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;

        let session_id = Uuid::from_str(&claims.session_id)
            .map_err(|_| ApiErrResp::internal_server_error("invalid session_id".to_string()))?;
        let user_id = Uuid::from_str(&claims.user_id)
            .map_err(|_| ApiErrResp::internal_server_error("invalid user_id".to_string()))?;
        let session = crate::actions::session::details::get_session_by_id(
            &mut conn,
            session_id,
            Some(user_id),
        )
        .await?
        .ok_or(ApiErrResp::unauthorized(Some("unauthorized".to_string())))?;

        if !session.valid {
            return Err(ApiErrResp::unauthorized(Some("unauthorized".to_string())));
        }

        Ok(Authenticated(session))
    }
}
