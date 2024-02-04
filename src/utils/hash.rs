use argon2::{password_hash::Salt, Argon2, PasswordHash};

use crate::error::{ApiErrResp, Result};

pub async fn hash_password(password: String) -> Result<String> {
    Ok(tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = Salt::from_b64("").unwrap();
        Ok(PasswordHash::generate(Argon2::default(), password, salt)
            .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?
            .to_string())
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
