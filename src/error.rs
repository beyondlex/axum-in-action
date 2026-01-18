use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not Found ({0})")]
    NotFound(String),
    #[error("Bad Request ({0})")]
    BadRequest(String),
    #[error("Internal Error")]
    InternalError(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            Error::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Error::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Error::InternalError(ref err) => {
                println!("internal error {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
            }
        };
        let body = Json(json!({"error": msg}));
        (status, body).into_response()
    }
}
