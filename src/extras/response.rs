use crate::StandardError;
use axum::response::IntoResponse;
use axum::Json;
use axum::response::Response;
use serde_json::json;

impl IntoResponse for StandardError {
    fn into_response(self) -> Response {
        #[cfg(feature = "askama")]
        if let Some(html) = self.html{
            return html.into_response()
        }
        (
            self.status_code,
            Json(json!({"detail": self.message}))
        )
            .into_response()
    }
}
