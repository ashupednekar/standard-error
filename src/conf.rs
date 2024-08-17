use config::{Config, ConfigError, Environment};
use lazy_static::lazy_static;
use serde::Deserialize;

use crate::{StandardError, StandardErrorMessages};

fn default_locale() -> String {"en_US".to_string()}
fn default_yaml_path() -> String {"errors.yaml".to_string()}

#[derive(Deserialize)]
pub struct Settings {
    #[serde(default = "default_locale")]
    pub default_locale: String,
    #[serde(default = "default_yaml_path")]
    pub error_yaml_file_path: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let conf = Config::builder()
            .add_source(Environment::default())
            .build()?;
        conf.try_deserialize()
    }
}

lazy_static! {
    pub static ref error_messages: StandardErrorMessages =
        StandardError::load_error_messages().expect("error loading error csv");
}
