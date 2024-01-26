use crate::error::{ApiErrResp, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub session_id: String,
    #[serde(with = "jwt_numeric_date")]
    iat: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    exp: OffsetDateTime,
}

impl Claims {
    pub fn new(
        user_id: String,
        session_id: String,
        iat: OffsetDateTime,
        exp: OffsetDateTime,
    ) -> Self {
        let iat = iat
            .date()
            .with_hms_milli(iat.hour(), iat.minute(), iat.second(), 0)
            .unwrap()
            .assume_utc();
        let exp = exp
            .date()
            .with_hms_milli(exp.hour(), exp.minute(), exp.second(), 0)
            .unwrap()
            .assume_utc();

        Self {
            user_id,
            iat,
            exp,
            session_id,
        }
    }
}

mod jwt_numeric_date {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.unix_timestamp();
        serializer.serialize_i64(timestamp)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        OffsetDateTime::from_unix_timestamp(i64::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}

pub enum TokenType {
    AccessToken,
    RefreshToken,
}

pub async fn create_token(
    user_id: String,
    session_id: String,
    token_type: TokenType,
) -> Result<String> {
    // set iat to now
    let iat = OffsetDateTime::now_utc();
    // get expiration from env for access and refresh tokens
    let access_exp = std::env::var("ACCESS_TOKEN_EXP")
        .unwrap_or_else(|_| "15".to_string())
        .parse::<i64>()
        .expect("ACCESS_TOKEN_EXP must be a number");
    let refresh_exp = std::env::var("REFRESH_TOKEN_EXP")
        .unwrap_or_else(|_| "7".to_string())
        .parse::<i64>()
        .expect("REFRESH_TOKEN_EXP must be a number");
    // set exp based on token type
    let exp = match token_type {
        TokenType::AccessToken => iat + time::Duration::minutes(access_exp),
        TokenType::RefreshToken => iat + time::Duration::days(refresh_exp),
    };
    // create claims
    let claims = Claims::new(user_id, session_id, iat, exp);
    let header = Header::new(Algorithm::RS512);
    // read private key from file
    let private_key_path = match token_type {
        TokenType::AccessToken => "./keys/access_private_key.pem",
        TokenType::RefreshToken => "./keys/refresh_private_key.pem",
    };
    let private_key = std::fs::read_to_string(private_key_path).map_err(|_| {
        ApiErrResp::internal_server_error(format!("cannot find private_key in keys dir"))
    })?;
    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap(),
    )
    .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;
    Ok(token)
}

pub fn verify_token(token: String, token_type: TokenType) -> Result<Claims> {
    // read public key from file
    let public_key_path = match token_type {
        TokenType::AccessToken => "./keys/access_public_key.pem",
        TokenType::RefreshToken => "./keys/refresh_public_key.pem",
    };
    let public_key = std::fs::read_to_string(public_key_path)
        .map_err(|_| ApiErrResp::internal_server_error(format!("cannot find public_key.pem")))?;
    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap(),
        &Validation::new(Algorithm::RS512),
    )
    .map_err(|e| ApiErrResp::unauthorized(Some(e.to_string())))?;
    Ok(token.claims)
}
