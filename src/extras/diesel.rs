use diesel::result::Error as DieselError;
use crate::{StandardError, Interpolate};

impl From<DieselError> for StandardError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => StandardError::new("ER-DB-NOTFOUND")
                .interpolate_err("Record not found".to_string()),
            DieselError::DatabaseError(_, info) => StandardError::new("ER-DB-DATABASE")
                .interpolate_err(format!("Database error: {}", info.message())),
            DieselError::QueryBuilderError(_) => StandardError::new("ER-DB-QUERYBUILDER")
                .interpolate_err("Query builder error".to_string()),
            DieselError::DeserializationError(_) => StandardError::new("ER-DB-DESERIALIZATION")
                .interpolate_err("Deserialization error".to_string()),
            DieselError::SerializationError(_) => StandardError::new("ER-DB-SERIALIZATION")
                .interpolate_err("Serialization error".to_string()),
            DieselError::RollbackTransaction => StandardError::new("ER-DB-ROLLBACK")
                .interpolate_err("Transaction was rolled back".to_string()),
            _ => StandardError::new("ER-DB-UNKNOWN")
                .interpolate_err("An unknown Diesel error occurred".to_string()),
        }
    }
}





