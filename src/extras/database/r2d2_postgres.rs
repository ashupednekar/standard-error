use crate::{StandardError, Interpolate};
use r2d2_postgres::r2d2::Error; 

impl From<Error> for StandardError {
    fn from(error: Error) -> Self {
        StandardError::new("ER-DB-POOL").interpolate_err(error.to_string())
    }
}
