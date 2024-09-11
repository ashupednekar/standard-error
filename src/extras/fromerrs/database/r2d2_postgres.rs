use crate::{StandardError, Interpolate};
#[cfg(feature="diesel")]
use r2d2_postgres::r2d2::Error; 


#[cfg(feature="diesel")]
impl From<Error> for StandardError {
    fn from(error: Error) -> Self {
        StandardError::new("ER-DB-POOL").interpolate_err(error.to_string())
    }
}
