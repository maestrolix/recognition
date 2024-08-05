use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;

pub struct Error {
    pub message: String,
    pub status_code: StatusCode,
}

impl Error {
    pub fn new(message: &str, status_code: StatusCode) -> Self {
        Error {
            message: message.to_string(),
            status_code,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        let body = Json(json!({
            "error": self.message
        }));

        (self.status_code, body).into_response()
    }
}
