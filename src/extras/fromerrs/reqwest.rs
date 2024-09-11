#[cfg(feature = "reqwest")]
use reqwest::Error as ReqwestError;

use crate::{StandardError, Interpolate};

#[cfg(feature = "reqwest")]
impl From<ReqwestError> for StandardError {
    fn from(error: ReqwestError) -> Self {
        if error.is_timeout() {
            StandardError::new("ER-REQWEST-TIMEOUT")
                .interpolate_err("Request timed out".to_string())
        } else if error.is_status() {
            StandardError::new("ER-REQWEST-STATUS")
                .interpolate_err(format!("Bad HTTP status: {}", error.status().unwrap()))
        } else if error.is_connect() {
            StandardError::new("ER-REQWEST-CONNECT")
                .interpolate_err("Failed to connect".to_string())
        } else if error.is_body() {
            StandardError::new("ER-REQWEST-BODY")
                .interpolate_err("Error with response body".to_string())
        } else if error.is_decode() {
            StandardError::new("ER-REQWEST-DECODE")
                .interpolate_err("Error decoding the response".to_string())
        } else {
            StandardError::new("ER-REQWEST-UNKNOWN")
                .interpolate_err(format!("An unknown Reqwest error occurred: {}", error))
        }
    }
}
