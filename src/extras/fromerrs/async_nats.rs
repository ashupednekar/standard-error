use crate::StandardError;
use async_nats::Error as NatsError;

impl From<NatsError> for StandardError {
    fn from(error: NatsError) -> Self {
        log::error!("Nats template error: {:?}", &error);
        match error {
            _ => StandardError::new("ER-NATS-UNKNOWN"),
        }
    }
}
