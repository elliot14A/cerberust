use std::sync::Arc;

use crate::{error::Result, utils::response::to_response};
use crate::{
    extractors::FromValidatedJson,
    utils::{hash::hash_password, smtp::SmtpService},
};
use axum::{response::IntoResponse, Extension, Json};
use hyper::StatusCode;
use repositories::{
    token::CreateTokenInput,
    user::{CreateUserInput, User},
    DatabaseRepository,
};

pub async fn register<H>(
    Extension(db): Extension<Arc<H>>,
    Extension(smtp): Extension<Arc<SmtpService>>,
    FromValidatedJson(input): FromValidatedJson<CreateUserInput>,
) -> Result<impl IntoResponse>
where
    H: DatabaseRepository,
{
    let CreateUserInput {
        name,
        email,
        password,
    } = input;
    // TODO: implement faster hash function
    let password = hash_password(password).await?;
    let user = db
        .create_user(CreateUserInput {
            name,
            email,
            password,
        })
        .await?;
    let email = user.email.clone();
    let user_id = user.id.clone();
    // send verification email
    // make sending email async as this might take some time
    tokio::spawn(async move {
        let token = uuid::Uuid::new_v4().to_string();
        // ignore the error for now
        db.create_token(CreateTokenInput {
            user_id,
            token: token.clone(),
            token_type: "email_verification".to_string(),
        })
        .await
        .unwrap();
        smtp.send_verification_email(email, token).unwrap();
    });
    let response = to_response::<User>(
        "Your email is registered, please verify it now".to_owned(),
        user,
    );
    Ok((StatusCode::CREATED, Json(response)))
}
