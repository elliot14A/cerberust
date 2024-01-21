use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash,
};

use crate::error::{ApiErrResp, Result};

pub async fn hash(password: String) -> Result<String> {
    Ok(tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        Ok(
            PasswordHash::generate(Argon2::default(), password, salt.as_salt())
                .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?
                .to_string(),
        )
    })
    .await
    .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))??)
}
