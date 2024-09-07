#[cfg(feature="git")]
use git2::{Error, ErrorCode};
use crate::{StandardError, Interpolate};

#[cfg(feature="git")]
impl From<git2::Error> for StandardError {
    fn from(error: git2::Error) -> Self {
        let code = match error.code() {
            git2::ErrorCode::NotFound => "ER-GIT-NOTFOUND",
            git2::ErrorCode::InvalidSpec => "ER-GIT-INVALIDSPEC",
            git2::ErrorCode::Authentication => "ER-GIT-AUTHENTICATION",
            git2::ErrorCode::Auth => "ER-GIT-AUTH",
            git2::ErrorCode::Config => "ER-GIT-CONFIG",
            git2::ErrorCode::Reference => "ER-GIT-REFERENCE",
            git2::ErrorCode::Object => "ER-GIT-OBJECT",
            git2::ErrorCode::Index => "ER-GIT-INDEX",
            git2::ErrorCode::Worktree => "ER-GIT-WORKTREE",
            git2::ErrorCode::Merge => "ER-GIT-MERGE",
            git2::ErrorCode::Tree => "ER-GIT-TREE",
            git2::ErrorCode::IndexNotFound => "ER-GIT-INDEXNOTFOUND",
            git2::ErrorCode::Branch => "ER-GIT-BRANCH",
            git2::ErrorCode::Tag => "ER-GIT-TAG",
            git2::ErrorCode::Commit => "ER-GIT-COMMIT",
            git2::ErrorCode::Checkout => "ER-GIT-CHECKOUT",
            git2::ErrorCode::Repo => "ER-GIT-REPO",
            _ => "ER-GIT-UNKNOWN",
        };

        StandardError::new(code).interpolate_err(error.to_string())
    }
}
