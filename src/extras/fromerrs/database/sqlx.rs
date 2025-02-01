#[cfg(feature = "sqlx")]
use sqlx::error::Error as SqlxError;
use crate::{StandardError, Interpolate};

#[cfg(feature = "sqlx")]
impl From<SqlxError> for StandardError {
    fn from(error: SqlxError) -> Self {
        log::error!("db error: {}", &error.to_string());
        match error {
            SqlxError::RowNotFound => StandardError::new("ER-DB-NOTFOUND")
                .interpolate_err("Record not found".to_string()),
            SqlxError::Database(db_err) => StandardError::new("ER-DB-DATABASE")
                .interpolate_err(format!("Database error: {}", db_err.message())),
            SqlxError::PoolTimedOut => StandardError::new("ER-DB-POOLTIMEOUT")
                .interpolate_err("Database connection pool timeout".to_string()),
            SqlxError::Io(_) => StandardError::new("ER-DB-IO")
                .interpolate_err("IO error while accessing the database".to_string()),
            SqlxError::Tls(_) => StandardError::new("ER-DB-TLS")
                .interpolate_err("TLS error while connecting to the database".to_string()),
            SqlxError::Protocol(_) => StandardError::new("ER-DB-PROTOCOL")
                .interpolate_err("Protocol error in database communication".to_string()),
            SqlxError::TypeNotFound { type_name } => StandardError::new("ER-DB-TYPENOTFOUND")
                .interpolate_err(format!("Type '{}' not found in database", type_name)),
            SqlxError::ColumnNotFound(col) => StandardError::new("ER-DB-COLUMNNOTFOUND")
                .interpolate_err(format!("Column '{}' not found", col)),
            SqlxError::Decode(_) => StandardError::new("ER-DB-DECODE")
                .interpolate_err("Error decoding database response".to_string()),
            SqlxError::Migrate(_) => StandardError::new("ER-DB-MIGRATION")
                .interpolate_err("Database migration error".to_string()),
            _ => StandardError::new("ER-DB-UNKNOWN")
                .interpolate_err("An unknown SQLx error occurred".to_string()),
        }
    }
}
