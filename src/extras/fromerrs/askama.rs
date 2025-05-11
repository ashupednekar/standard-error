#[cfg(feature = "askama")]
use askama::Error as AskamaError;

#[cfg(feature = "askama")]
impl From<AskamaError> for StandardError {
    fn from(error: AskamaError) -> Self {
        log::error!("Askama template error: {}", &error.to_string());
        match error {
            AskamaError::Fmt(fmt_err) => StandardError::new("ER-ASKAMA-FMT")
                .interpolate_err(format!("Formatting error in template: {}", fmt_err)),
            AskamaError::Io(io_err) => StandardError::new("ER-ASKAMA-IO")
                .interpolate_err(format!("IO error during template rendering: {}", io_err)),
            AskamaError::Custom(msg) => StandardError::new("ER-ASKAMA-CUSTOM")
                .interpolate_err(format!("Custom Askama error: {}", msg)),
        }
    }
}
