use serde::Serialize;
use serde_json::{json, Value};

pub fn to_response<T: Serialize>(message: String, body: T) -> Value {
    json!({
        "message": message,
        "data": body
    })
}
