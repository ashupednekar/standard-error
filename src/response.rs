use crate::StandardError;
use axum::response::IntoResponse;
use axum::Json;
use axum::response::Response;
use serde_json::json;


impl IntoResponse for StandardError {
    fn into_response(self) -> Response {
        let message = error_messages
            .get(&self.code)
            .and_then(|locale_message| locale_message.get(&self.locale))
            .map_or_else(
                || "unknown error".to_string(),
                |msg| msg.to_string(),
            );
        (
            self.status_code,
            Json(json!({"detail": message}))
        )
            .into_response()
    }
}
