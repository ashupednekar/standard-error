#[cfg(feature = "axum")]
use axum::response::IntoResponse;
use axum::http::{StatusCode, HeaderValue, Error as HttpError};
use crate::{StandardError, Interpolate};
use std::error::Error as StdError;

#[cfg(feature = "axum")]
impl From<axum::Error> for StandardError {
    fn from(error: axum::Error) -> Self {
        log::error!("axum error: {}", &error.to_string());

        if let Some(io_err) = error.source().and_then(|e| e.downcast_ref::<std::io::Error>()) {
            StandardError::new("ER-AXUM-IO")
                .interpolate_err(format!("IO error occurred: {}", io_err))
        } else if let Some(hyper_err) = error.source().and_then(|e| e.downcast_ref::<hyper::Error>()) {
            StandardError::new("ER-AXUM-HYPER")
                .interpolate_err(format!("Hyper error occurred: {}", hyper_err))
        } else if let Some(http_err) = error.source().and_then(|e| e.downcast_ref::<HttpError>()) {
            StandardError::new("ER-AXUM-HTTP")
                .interpolate_err(format!("HTTP error occurred: {}", http_err))
        } else {
            StandardError::new("ER-AXUM-UNKNOWN")
                .interpolate_err("An unknown Axum error occurred".to_string())
        }
    }
}

#[cfg(feature = "axum")]
impl From<axum::http::StatusCode> for StandardError {
    fn from(status: StatusCode) -> Self {
        log::error!("HTTP error: {}", status.as_u16());

        match status {
            StatusCode::NOT_FOUND => StandardError::new("ER-AXUM-NOTFOUND")
                .interpolate_err("Resource not found".to_string()),
            StatusCode::INTERNAL_SERVER_ERROR => StandardError::new("ER-AXUM-INTERNAL")
                .interpolate_err("Internal server error".to_string()),
            StatusCode::BAD_REQUEST => StandardError::new("ER-AXUM-BADREQUEST")
                .interpolate_err("Bad request".to_string()),
            StatusCode::FORBIDDEN => StandardError::new("ER-AXUM-FORBIDDEN")
                .interpolate_err("Forbidden request".to_string()),
            StatusCode::UNAUTHORIZED => StandardError::new("ER-AXUM-UNAUTHORIZED")
                .interpolate_err("Unauthorized request".to_string()),
            _ => StandardError::new("ER-AXUM-UNKNOWN")
                .interpolate_err(format!("Unknown error: {}", status)),
        }
    }
}

#[cfg(feature = "axum")]
impl From<axum::http::header::InvalidHeaderValue> for StandardError {
    fn from(error: axum::http::header::InvalidHeaderValue) -> Self {
        log::error!("Invalid header value error: {}", &error.to_string());
        StandardError::new("ER-AXUM-INVALIDHEADERVALUE")
            .interpolate_err("Invalid header value".to_string())
    }
}

#[cfg(feature = "axum")]
impl From<axum::http::header::InvalidHeaderName> for StandardError {
    fn from(error: axum::http::header::InvalidHeaderName) -> Self {
        log::error!("Invalid header name error: {}", &error.to_string());
        StandardError::new("ER-AXUM-INVALIDHEADERNAME")
            .interpolate_err("Invalid header name".to_string())
    }
}

#[cfg(feature = "axum")]
impl From<axum::http::Error> for StandardError {
    fn from(error: axum::http::Error) -> Self {
        log::error!("HTTP error: {}", &error.to_string());
        StandardError::new("ER-AXUM-HTTPERROR")
            .interpolate_err("General HTTP error".to_string())
    }
}
