use serde::de::value::Error as SerdeError;
use serde_json::Error as SerdeJsonError;
use serde_yaml::Error as SerdeYamlError;

use crate::{StandardError, Interpolate};


impl From<SerdeError> for StandardError {
    fn from(err: SerdeError) -> Self {
        StandardError::new("ER-SERDE")
            .interpolate_err(err.to_string())
    }
}

impl From<SerdeJsonError> for StandardError {
    fn from(err: SerdeJsonError) -> Self {
        StandardError::new("ER-SERDE-JSON")
            .interpolate_err(err.to_string())
    }
}

impl From<SerdeYamlError> for StandardError {
    fn from(err: SerdeYamlError) -> Self {
        StandardError::new("ER-SERDE-YAML")
            .interpolate_err(err.to_string())    }
}
