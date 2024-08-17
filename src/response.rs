use crate::StandardError;
use axum::response::IntoResponse;
use axum::Json;
use axum::response::Response;
use serde_json::json;


impl IntoResponse for StandardError {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(json!({"detail": self.message()}))
        )
            .into_response()
    }
}
