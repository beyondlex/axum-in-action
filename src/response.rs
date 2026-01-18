use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
pub struct Resp<T> {
    pub code: i32,
    pub data: Option<T>,
    pub msg: String,
}

impl<T> Resp<T> where T: Serialize {
    pub fn ok(data: T) -> Self {
        Resp { code: 0, data: Some(data), msg: "".to_string() }
    }
}

impl<T> IntoResponse for Resp<T> where T: Serialize {
    fn into_response(self) -> axum::response::Response {
        Json(json!(self)).into_response()
    }
}
