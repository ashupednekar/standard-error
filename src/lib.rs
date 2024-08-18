use axum::http::StatusCode;
use std::collections::HashMap;
use lazy_static::lazy_static;
use thiserror::Error;

mod locale;
mod loader;
mod response;
mod conf;

pub use locale::get_current_locale as get_current_locale;
pub use locale::set_current_locale as set_current_locale;

pub type StandardErrorMessages = HashMap<String, HashMap<String, String>>;

#[derive(Debug, Error)]
#[error("Error {code} with status {status_code} for locale {locale}")]
pub struct StandardError {
    code: String,
    locale: String,
    status_code: StatusCode,
}

impl StandardError {
   pub fn from(code: &str, status_code: StatusCode) -> Self {
        StandardError {
            code: code.to_string(),
            locale: locale::get_current_locale(),
            status_code,
        }
    }

    pub fn message(&self) -> String {
        error_messages
            .get(&self.code)
            .and_then(|locale_message| locale_message.get(&self.locale))
            .map_or_else(
                || "unknown error".to_string(),
                |msg| msg.to_string(),
            )
    }

}


lazy_static! {
    pub static ref settings: conf::Settings = conf::Settings::new().expect("improperly configured");
    pub static ref error_messages: StandardErrorMessages =
        StandardError::load_error_messages().expect("error loading error messages");
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use axum::http::StatusCode;

    use crate::StandardError;


    #[tokio::test]
    async fn test_question_mark() -> Result<(), StandardError>{

        async fn foo(a: &str) -> Result<i32, StandardError>{
            a.parse().map_err(|_: ParseIntError|{
                StandardError::from("ER-0004", StatusCode::BAD_REQUEST)
            })
        }

        foo("1").await?;

        Ok(())
    }

}
