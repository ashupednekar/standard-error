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
    values: HashMap<String, String>
}

trait Interpolate{
    fn interpolate(&mut self, values: HashMap<String, String>) -> String;
}

impl Interpolate for String{
    fn interpolate(&mut self, values: HashMap<String, String>) -> String {
        for (k, v) in values.into_iter(){
            *self = self.replace(
                &format!("[{}]", &k),
                &v
            ); 
        }
        self.to_string()
    }
}

impl StandardError {
   pub fn from(code: &str, status_code: Option<StatusCode>, values: Option<HashMap<String, String>>) -> Self {
        StandardError {
            code: code.to_string(),
            locale: locale::get_current_locale(),
            status_code: status_code.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            values: values.unwrap_or(HashMap::new())
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
            .interpolate(self.values.clone())
    }

    pub fn err_to_msg(e: String) -> Option<HashMap<String, String>>{
        let mut val: HashMap<String, String> = HashMap::new();
        val.insert("msg".to_string(), e.to_string());
        Some(val)
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
                StandardError::from("ER-0004", Some(StatusCode::BAD_REQUEST), None)
            })
        }

        foo("1").await?;

        Ok(())
    }

}
