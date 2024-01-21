use std::sync::Arc;

use crate::{error::Result, utils::response::to_response};
use axum::{response::IntoResponse, Extension, Json};
use repositories::{CreateUserInput, DatabaseRepository, User};

use crate::{
    extractors::FromValidatedJson,
    utils::{hash::hash, smtp::SmtpService},
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
    let password = hash(password).await?;
    let user = db
        .create_user(CreateUserInput {
            name,
            email,
            password,
        })
        .await?;
    let email = user.email.clone();
    // send verification email
    // make sending email async as this might take some time
    tokio::spawn(async move {
        smtp.send_verification_email(email).unwrap();
    });
    let response = to_response::<User>("Verification email is sent".to_owned(), user);
    Ok(Json(response))
}
