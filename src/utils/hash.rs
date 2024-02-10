use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash,
};

use crate::error::{ApiErrResp, Result};

pub async fn hash_password(password: String) -> Result<String> {
    Ok(tokio::task::spawn_blocking(move || -> Result<String> {
        // TODO: use a real salt
        let salt_string = SaltString::generate(&mut OsRng);
        Ok(
            PasswordHash::generate(Argon2::default(), password, &salt_string)
                .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?
                .to_string(),
        )
    })
    .await
    .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))??)
}
pub async fn verify_password(password: String, hash: String) -> Result<()> {
    Ok(tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(&hash)
            .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;
        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => {
                    ApiErrResp::unauthorized(Some("Invalid email or password".to_string()))
                }
                _ => ApiErrResp::internal_server_error(e.to_string()),
            })
    })
    .await
    .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))??)
}
