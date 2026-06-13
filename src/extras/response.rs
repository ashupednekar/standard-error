use crate::StandardError;
#[cfg(feature = "askama")]
use axum::response::Html;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

impl IntoResponse for StandardError {
    fn into_response(self) -> Response {
        #[cfg(feature = "askama")]
        if let Some(html) = self.html {
            return (self.status_code, Html(html)).into_response();
        }
        (self.status_code, Json(json!({"detail": self.message}))).into_response()
    }
}
