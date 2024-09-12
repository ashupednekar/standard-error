use std::collections::HashMap;

use config::{Config, ConfigError, Environment};
use lazy_static::lazy_static;
use serde::Deserialize;

use crate::{StandardError, StandardErrorMessages};

fn default_locale() -> String {
    "en_US".to_string()
}
fn default_yaml_path() -> String {
    "errors.yaml".to_string()
}
pub fn default_error_messages() -> StandardErrorMessages{
    [
        ("ER-DB-NOTFOUND", "Record not found: [err]"),
        ("ER-DB-DATABASE", "Database error: [err]"),
        ("ER-DB-QUERYBUILDER", "Query builder error: [err]"),
        ("ER-DB-DESERIALIZATION", "Deserialization error: [err]"),
        ("ER-DB-SERIALIZATION", "Serialization error: [err]"),
        ("ER-DB-ROLLBACK", "Transaction was rolled back: [err]"),
        ("ER-DB-UNKNOWN", "An unknown Diesel error occurred: [err]"),
        ("ER-IO-NOTFOUND", "File not found: [err]"),
        ("ER-IO-PERMISSION", "Permission denied: [err]"),
        ("ER-IO-CONNECTION", "Connection refused: [err]"),
        ("ER-IO-RESET", "Connection reset: [err]"),
        ("ER-IO-ABORTED", "Connection aborted: [err]"),
        ("ER-IO-NOTCONNECTED", "Not connected: [err]"),
        ("ER-IO-ADDRINUSE", "Address in use: [err]"),
        ("ER-IO-ADDRNOTAVAILABLE", "Address not available: [err]"),
        ("ER-IO-BROKENPIPE", "Broken pipe: [err]"),
        ("ER-IO-ALREADYEXISTS", "Already exists: [err]"),
        ("ER-IO-WOULDBLOCK", "Operation would block: [err]"),
        ("ER-IO-TIMEDOUT", "Operation timed out: [err]"),
        ("ER-IO-INTERRUPTED", "Operation interrupted: [err]"),
        ("ER-IO-UNEXPECTEDEOF", "Unexpected end of file: [err]"),
        ("ER-IO-UNKNOWN", "An unknown I/O error occurred: [err]"),
        ("ERR-GIT-GENERIC", "A generic error occurred in the Git operation: [err]"),
        ("ERR-GIT-NOTFOUND", "The requested resource was not found in the Git repository: [err]"),
        ("ERR-GIT-EXISTS", "The resource already exists in the Git repository: [err]"),
        ("ERR-GIT-AMBIGUOUS", "The Git reference is ambiguous: [err]"),
        ("ERR-GIT-BUFSIZE", "Buffer size is insufficient for the Git operation: [err]"),
        ("ERR-GIT-USER", "User-defined error encountered in the Git operation: [err]"),
        ("ERR-GIT-BARE-REPO", "Operation cannot be performed on a bare Git repository: [err]"),
        ("ERR-GIT-UNBORN-BRANCH", "The branch has not been created yet: [err]"),
        ("ERR-GIT-UNMERGED", "There are unmerged changes in the Git repository: [err]"),
        ("ERR-GIT-NOT-FAST-FORWARD", "The branch is not fast-forwardable: [err]"),
        ("ERR-GIT-INVALID-SPEC", "The Git specification provided is invalid: [err]"),
        ("ERR-GIT-CONFLICT", "A conflict occurred during the Git operation: [err]"),
        ("ERR-GIT-LOCKED", "The Git resource is locked: [err]"),
        ("ERR-GIT-MODIFIED", "The file has been modified: [err]"),
        ("ERR-GIT-AUTH", "Authentication failed during the Git operation: [err]"),
        ("ERR-GIT-CERTIFICATE", "Certificate validation failed during the Git operation: [err]"),
        ("ERR-GIT-APPLIED", "The patch has already been applied: [err]"),
        ("ERR-GIT-PEEL", "Peeling operation failed: [err]"),
        ("ERR-GIT-EOF", "Unexpected end of file encountered: [err]"),
        ("ERR-GIT-INVALID", "An invalid operation was attempted: [err]"),
        ("ERR-GIT-UNCOMMITTED", "There are uncommitted changes: [err]"),
        ("ERR-GIT-DIRECTORY", "The directory is invalid or not found: [err]"),
        ("ERR-GIT-MERGE-CONFLICT", "A merge conflict occurred: [err]"),
        ("ERR-GIT-HASHSUM-MISMATCH", "Hashsum mismatch detected: [err]"),
        ("ERR-GIT-INDEX-DIRTY", "The Git index is dirty: [err]"),
        ("ERR-GIT-APPLY-FAIL", "Failed to apply the patch: [err]"),
        ("ERR-GIT-OWNER", "Invalid owner in the Git operation: [err]"),
        ("ERR-SERDE", "Error from serde: [err]"),
        ("ERR-SERDE-JSON", "Error from serde-json: [err]"),
        ("ERR-SERDE-YAML", "Error from serde-yaml: [err]")
    ]
    .iter()
    .map(|(code, en_us)| {
        let mut locales = HashMap::new();
        locales.insert("en_US".to_string(), en_us.to_string());
        (code.to_string(), locales)
    })
    .collect()
}

#[derive(Deserialize)]
pub struct Settings {
    #[serde(default = "default_locale")]
    pub default_locale: String,
    #[serde(default = "default_yaml_path")]
    pub error_yaml_file_path: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let conf = Config::builder()
            .add_source(Environment::default())
            .build()?;
        conf.try_deserialize()
    }
}

lazy_static! {
    pub static ref error_messages: StandardErrorMessages =
        StandardError::load_error_messages().expect("error loading error csv");
}
