use core::fmt;

use axum::{
    body::Body,
    http,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use serde::{Serialize, Serializer};

pub type Result<T> = std::result::Result<T, ApiErrResp>;

#[derive(Serialize, thiserror::Error, Debug)]
pub struct ApiErrResp {
    #[serde(serialize_with = "serialize_statuscode")]
    pub code: http::StatusCode,
    pub error: String,
    pub message: String,
}

fn serialize_statuscode<S>(x: &http::StatusCode, s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u16(x.as_u16())
}

impl fmt::Display for ApiErrResp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]({}): {}", self.code, self.error, self.message)
    }
}

impl ApiErrResp {
    pub fn init(
        message: impl Into<String>,
        error: impl Into<String>,
        code: http::StatusCode,
    ) -> Self {
        Self {
            message: message.into(),
            code,
            error: error.into(),
        }
    }

    pub fn unauthorized(message: Option<String>) -> Self {
        let message: String = message.unwrap_or("Not authorized to make this request".to_string());
        Self {
            code: http::StatusCode::UNAUTHORIZED,
            error: String::from("Not Authorized"),
            message,
        }
    }

    pub fn forbidden() -> Self {
        Self {
            code: http::StatusCode::FORBIDDEN,
            error: String::from("Forbidden"),
            message: String::from("Forbidden to make this request"),
        }
    }

    pub fn internal_server_error(message: impl Into<String>) -> Self {
        Self {
            code: http::StatusCode::INTERNAL_SERVER_ERROR,
            error: String::from("Internal Server Error"),
            message: message.into(),
        }
    }
}

// impl From<Error> for ApiErrResp {
//     fn from(value: Error) -> Self {
//         match value {
//             Error::UserNotFound { message } => ApiErrResp::unauthorized(Some(message)),
//             Error::UsernameOrEmailAlreadyExists { message } => ApiErrResp {
//                 code: StatusCode::CONFLICT,
//                 error: "CONFLICT".to_string(),
//                 message,
//             },
//             Error::AccountNotFound { message } => ApiErrResp::unauthorized(Some(message)),
//             Error::AccountAlreadyExists { message } => ApiErrResp {
//                 code: StatusCode::CONFLICT,
//                 error: "CONFLICT".to_string(),
//                 message,
//             },
//             Error::AccountNotOwnedByUser { id: _ } => todo!(),
//             Error::InvalidQuery { message } => ApiErrResp::internal_server_error(message),
//             Error::InternalError { message } => ApiErrResp::internal_server_error(message),
//             Error::TokenNotFound => ApiErrResp::unauthorized(Some("Invalid Token".into())),
//         }
//     }
// }

impl IntoResponse for ApiErrResp {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self).unwrap();
        let body = axum::body::Body::from(body);
        Response::builder()
            .status(self.code)
            .body::<Body>(body)
            .unwrap()
    }
}
