use async_trait::async_trait;
use axum::{
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    response::IntoResponse,
    response::Response,
    Json,
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

// #[derive(Debug, Serialize)]
// struct ErrorResponse {
//     message: String,
//     errors: Vec<ErrorResponseBody>,
// }
//
// #[derive(Debug, Serialize)]
// struct ErrorResponseBody {
//     source: String,
//     body: String,
// }

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumJsonRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}

// fn build_error_response(map: &HashMap<&str, ValidationErrorsKind>) -> Vec<ErrorResponseBody> {
//     let mut messages = vec![];
//     for (source, error) in map {
//         match error {
//             ValidationErrorsKind::Field(e) => {
//                 for error in e {
//                     match error.code.to_string().as_str() {
//                         "email" => messages.push(ErrorResponseBody {
//                             source: String::from(*source),
//                             body: String::from("Invalid Email"),
//                         }),
//                         "length" => {
//                             if let Some(Value::Number(val)) = error.params.get("min") {
//                                 let min = val.as_u64().unwrap();
//                                 messages.push(ErrorResponseBody {
//                                     source: String::from(*source),
//                                     body: format!("Must be at least {} characters", min),
//                                 })
//                             }
//                         }
//                         _ => messages.push(ErrorResponseBody {
//                             source: String::from(*source),
//                             body: String::from("Unknown Validation Error"),
//                         }),
//                     }
//                 }
//             }
//             _ => {}
//         }
//     }
//     messages
// }
