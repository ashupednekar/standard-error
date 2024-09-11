use serde::de::value::Error as SerdeError;
use serde_json::Error as SerdeJsonError;
use serde_yaml::Error as SerdeYamlError;

use crate::{StandardError, Interpolate};


impl From<SerdeError> for StandardError {
    fn from(error: SerdeError) -> Self {
        StandardError::new("ER-SERDE")
            .interpolate_err(format!("Serde error: {}", error))
    }
}

impl From<SerdeJsonError> for StandardError {
    fn from(error: SerdeJsonError) -> Self {
        StandardError::new("ER-SERDE-JSON")
            .interpolate_err(format!("Serde json error: {}", error))
    }
}

impl From<SerdeYamlError> for StandardError {
    fn from(error: SerdeYamlError) -> Self {
        StandardError::new("ER-SERDE-YAML")
            .interpolate_err(format!("Serde yaml error: {}", error))
    }
}
