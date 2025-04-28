use crate::StandardError;

pub trait DynErr{
    fn dyn_err(&mut self, prefix: &str) -> Self;
}

impl DynErr for StandardError{
    fn dyn_err(&mut self, prefix: &str) -> Self {
        // TODO: retrieve from redis and update self.message
        self.clone()
    }
}
