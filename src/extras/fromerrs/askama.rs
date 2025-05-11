use crate::StandardError;
use askama::Error as AskamaError;

impl From<AskamaError> for StandardError {
    fn from(error: AskamaError) -> Self {
        log::error!("Askama template error: {:?}", &error);
        match error {
            AskamaError::Fmt => StandardError::new("ER-TEMPLATE-FMT"),

            AskamaError::ValueMissing => StandardError::new("ER-TEMPLATE-MISSING"),

            AskamaError::ValueType => StandardError::new("ER-TEMPLATE-TYPE"),

            _ => StandardError::new("ER-TEMPLATE-UNKNOWN"),
        }
    }
}
