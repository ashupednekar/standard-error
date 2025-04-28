#[cfg(feature = "dynerr")]
mod redis;

#[cfg(feature = "dynerr")]
use crate::{redis_settings, StandardError};

#[cfg(feature = "dynerr")]
pub trait DynErr {
    fn dyn_err(&mut self, prefix: &str) -> Self;
}

#[cfg(feature = "dynerr")]
impl DynErr for StandardError {
    fn dyn_err(&mut self, prefix: &str) -> Self {
        // TODO: retrieve from redis and update self.message
        self.clone()
    }
}
