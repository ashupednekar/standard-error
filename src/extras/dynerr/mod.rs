#[cfg(feature = "dynerr")]
mod redis;

use redis::RedisBackend;

#[cfg(feature = "dynerr")]
use crate::StandardError;

#[cfg(feature = "dynerr")]
pub trait DynErr {
    fn dyn_err(&mut self, prefix: &str) -> Self;
}

#[cfg(feature = "dynerr")]
impl DynErr for StandardError {
    fn dyn_err(&mut self, key: &str) -> Self {
        match RedisBackend::new(){
            Ok(client) => {
                if let Ok(Some(msg)) = client.get::<String>(&key){
                    self.message = msg;
                    log::info!("using dynamic message: {}", &self.message);
                }
            },
            Err(e) => {return e;}
        }
        self.clone()
    }
}
