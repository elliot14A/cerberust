use core::fmt;

use axum::{
    body::Body,
    http,
    response::{IntoResponse, Response},
};

use diesel::result::Error;
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

impl From<Error> for ApiErrResp {
    fn from(e: Error) -> Self {
        match e {
            Error::NotFound => ApiErrResp {
                code: http::StatusCode::NOT_FOUND,
                error: "NOT_FOUND".to_string(),
                message: "Not Found".to_string(),
            },
            Error::DatabaseError(e, _e) => match e {
                diesel::result::DatabaseErrorKind::UniqueViolation => ApiErrResp {
                    code: http::StatusCode::CONFLICT,
                    error: "CONFLICT".to_string(),
                    message: _e.message().to_string(),
                },
                diesel::result::DatabaseErrorKind::ForeignKeyViolation => ApiErrResp {
                    code: http::StatusCode::BAD_REQUEST,
                    error: "BAD_REQUEST".to_string(),
                    message: _e.message().to_string(),
                },
                _ => ApiErrResp::internal_server_error(_e.message().to_string()),
            },
            _ => ApiErrResp::internal_server_error(e.to_string()),
        }
    }
}

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
