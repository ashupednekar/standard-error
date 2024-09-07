use crate::{settings, StandardError, StandardErrorMessages};
use serde_json::to_string_pretty;
use serde_yaml::Value;
use std::{collections::HashMap, fs::File, io::BufReader};

impl StandardError {
    pub fn load_error_messages() -> Result<StandardErrorMessages, Box<dyn std::error::Error>> {
        let file = File::open(&settings.error_yaml_file_path)?;
        let reader = BufReader::new(file);
        let yaml: Value = serde_yaml::from_reader(reader)?;
        let mut messages = HashMap::new();
        if let Some(errors) = yaml.get("errors").and_then(|v| v.as_sequence()) {
            for error in errors {
                if let Some(code) = error.get("code").and_then(|v| v.as_str()) {
                    let mut locale_messages = HashMap::new();

                    for (key, value) in error.as_mapping().unwrap() {
                        if let Some(key_str) = key.as_str() {
                            if key_str.starts_with("detail_") {
                                if let Some(detail) = value.as_str() {
                                    locale_messages.insert(
                                        key_str.replace("detail_", "").to_string(),
                                        detail.to_string(),
                                    );
                                }
                            }
                        }
                    }
                    messages.insert(code.to_string(), locale_messages);
                }
            }
        }
        log::debug!("Loaded error messages: {}", to_string_pretty(&messages)?);
        Ok(messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_error_csv_loader() {
        env_logger::init();
        StandardError::load_error_messages().unwrap();
    }
}
