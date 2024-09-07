use axum::http::StatusCode;

use crate::StandardError;



pub trait Status{
    fn code(&mut self, code: StatusCode) -> Self;
}

impl Status for StandardError{
    fn code(&mut self, code: StatusCode) -> Self{
        self.status_code = code;
        self.clone()
    }
}


