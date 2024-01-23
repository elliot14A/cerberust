use std::sync::Arc;

use crate::{
    error::{ApiErrResp, Result},
    utils::smtp::SmtpService,
};
use axum::{response::IntoResponse, Extension, Json};
use repositories::{DatabaseRepository, UserWhereInput};

use super::VerifyOrResetRequestBody;

async fn forgot_password_send_email<H: DatabaseRepository>(
    Extension(ctx): Extension<Arc<H>>,
    Extension(smtp): Extension<Arc<SmtpService>>,
    Json(VerifyOrResetRequestBody { email }): Json<VerifyOrResetRequestBody>,
) -> Result<impl IntoResponse> {
    let user = ctx
        .get_user(UserWhereInput {
            id: None,
            email: Some(email),
            name: None,
        })
        .await?;
    let user_id = user.id.clone();
    let email = user.email.clone();

    Ok(())
}
