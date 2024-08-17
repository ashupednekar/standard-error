use axum::http::StatusCode;
use std::collections::HashMap;
use lazy_static::lazy_static;

mod locale;
mod loader;
mod response;

pub use locale::get_current_locale as get_current_locale;
pub use locale::set_current_locale as set_current_locale;

pub type StandardErrorMessages = HashMap<String, HashMap<String, String>>;

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
    pub static ref error_messages: StandardErrorMessages =
        StandardError::load_error_messages().expect("error loading error messages");
}

#[cfg(test)]
mod tests {
}
