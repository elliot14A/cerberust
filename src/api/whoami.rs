use std::sync::Arc;

use crate::error::Result;
use axum::{response::IntoResponse, Extension, Json};
use serde_json::{json, Value};

use crate::{
    error::ApiErrResp,
    utils::{jwt::Claims, response::to_response},
};

pub async fn whoami(
    claims: Claims,
    // Extension(ctx): Extension<Arc<H>>,
) -> Result<impl IntoResponse> {
    // let Claims {
    //     user_id,
    //     session_id,
    //     ..
    // } = claims;
    //
    // let session = ctx.find_session(session_id, user_id).await?;
    //
    // if !session.valid {
    //     return Err(ApiErrResp::forbidden());
    // }
    //
    // let user = ctx
    //     .get_user(UserWhereInput {
    //         id: Some(session.user_id),
    //         name: None,
    //         email: None,
    //     })
    //     .await?;
    //
    // let json = json!({
    //     "user": user
    // });
    //
    // let response = to_response::<Value>("User fetched".to_owned(), json);
    //
    // Ok(Json(response))
    Ok(())
}
