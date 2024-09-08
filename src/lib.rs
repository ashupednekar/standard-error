use axum::http::StatusCode;
use lazy_static::lazy_static;
use std::collections::HashMap;
use thiserror::Error;

mod conf;
mod loader;
mod locale;
pub mod extras;

pub use locale::get_current_locale;
pub use locale::set_current_locale;


pub type StandardErrorMessages = HashMap<String, HashMap<String, String>>;
pub use extras::status::Status;
pub use extras::interpolate::Interpolate;

#[derive(Debug, Clone, Error)]
#[error("Error {err_code} with status {status_code}")]
pub struct StandardError {
    pub err_code: String,
    pub status_code: StatusCode,
    values: HashMap<String, String>,
    pub message: String,
}

impl StandardError {
    pub fn new(code: &str) -> Self {
        StandardError {
            err_code: code.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            values: HashMap::new(),
            message: error_messages
                .get(code)
                .and_then(|locale_message| locale_message.get(&locale::get_current_locale()))
                .map_or_else(|| "unknown error".to_string(), |msg| msg.to_string()),
        }
    }
}


lazy_static! {
    pub static ref settings: conf::Settings = conf::Settings::new().expect("improperly configured");
    pub static ref error_messages: StandardErrorMessages =
        StandardError::load_error_messages().expect("error loading error messages");
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, num::ParseIntError};
    use crate::extras::{status::Status, interpolate::Interpolate};
    use axum::http::StatusCode;

    use crate::StandardError;

    #[tokio::test]
    async fn test_question_mark() -> Result<(), StandardError> {
        async fn foo(a: &str) -> Result<i32, StandardError> {
            a.parse().map_err(|_: ParseIntError| {
                StandardError::new("ER-0004")
            })
        }

        let res = foo("a").await;

        if let Err(e) = res{
            assert_eq!(e.status_code, StatusCode::INTERNAL_SERVER_ERROR);
            assert_eq!(e.message, "Should be an integer".to_string())
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_status_code() -> Result<(), StandardError> {
        async fn foo(a: &str) -> Result<i32, StandardError> {
            a.parse().map_err(|_: ParseIntError| {
                StandardError::new("ER-0004").code(StatusCode::BAD_REQUEST)
            })
        }

        let res = foo("a").await;

        if let Err(e) = res{
            assert_eq!(e.status_code, StatusCode::BAD_REQUEST);
            assert_eq!(e.message, "Should be an integer".to_string())
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_interpolate_err() -> Result<(), StandardError> {
        async fn foo(a: &str) -> Result<i32, StandardError> {
            a.parse().map_err(|e: ParseIntError| {
                StandardError::new("ER-0005").interpolate_err(e.to_string())
            })
        }

        let res = foo("a").await;

        if let Err(e) = res{
            assert_eq!(e.status_code, StatusCode::INTERNAL_SERVER_ERROR);
            assert_eq!(e.message, "Should be an integer: invalid digit found in string".to_string())
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_interpolate_values() -> Result<(), StandardError> {
        async fn foo(a: &str) -> Result<i32, StandardError> {
            a.parse().map_err(|_: ParseIntError| {
                let mut values: HashMap<String, String> = HashMap::new();
                values.insert("fname".to_string(), "ashu".to_string());
                values.insert("lname".to_string(), "pednekar".to_string());
                StandardError::new("ER-0006").interpolate_values(values)
            })
        }

        let res = foo("a").await;

        if let Err(e) = res{
            assert_eq!(e.status_code, StatusCode::INTERNAL_SERVER_ERROR);
            assert_eq!(e.message, "Should be an integer - fname: ashu | lname: pednekar".to_string())
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_chain() -> Result<(), StandardError> {
        async fn foo(a: &str) -> Result<i32, StandardError> {
            a.parse().map_err(|e: ParseIntError| {
                let mut values: HashMap<String, String> = HashMap::new();
                values.insert("fname".to_string(), "ashu".to_string());
                values.insert("lname".to_string(), "pednekar".to_string());
                StandardError::new("ER-0007")
                    .code(StatusCode::IM_A_TEAPOT)
                    .interpolate_values(values)
                    .interpolate_err(e.to_string())
            })
        }

        let res = foo("a").await;

        if let Err(e) = res{
            assert_eq!(e.status_code, StatusCode::IM_A_TEAPOT);
            assert_eq!(e.message, "Should be an integer - fname: ashu | lname: pednekar - invalid digit found in string".to_string())
        }

        Ok(())
    }



}
