#[cfg(feature="git")]
use crate::{StandardError, Interpolate};


#[cfg(feature="git")]
impl From<git2::Error> for StandardError {
    fn from(error: git2::Error) -> Self {
        let code = match error.code() {
            git2::ErrorCode::GenericError => "ERR-GIT-GENERIC",
            git2::ErrorCode::NotFound => "ERR-GIT-NOTFOUND",
            git2::ErrorCode::Exists => "ERR-GIT-EXISTS",
            git2::ErrorCode::Ambiguous => "ERR-GIT-AMBIGUOUS",
            git2::ErrorCode::BufSize => "ERR-GIT-BUFSIZE",
            git2::ErrorCode::User => "ERR-GIT-USER",
            git2::ErrorCode::BareRepo => "ERR-GIT-BARE-REPO",
            git2::ErrorCode::UnbornBranch => "ERR-GIT-UNBORN-BRANCH",
            git2::ErrorCode::Unmerged => "ERR-GIT-UNMERGED",
            git2::ErrorCode::NotFastForward => "ERR-GIT-NOT-FAST-FORWARD",
            git2::ErrorCode::InvalidSpec => "ERR-GIT-INVALID-SPEC",
            git2::ErrorCode::Conflict => "ERR-GIT-CONFLICT",
            git2::ErrorCode::Locked => "ERR-GIT-LOCKED",
            git2::ErrorCode::Modified => "ERR-GIT-MODIFIED",
            git2::ErrorCode::Auth => "ERR-GIT-AUTH",
            git2::ErrorCode::Certificate => "ERR-GIT-CERTIFICATE",
            git2::ErrorCode::Applied => "ERR-GIT-APPLIED",
            git2::ErrorCode::Peel => "ERR-GIT-PEEL",
            git2::ErrorCode::Eof => "ERR-GIT-EOF",
            git2::ErrorCode::Invalid => "ERR-GIT-INVALID",
            git2::ErrorCode::Uncommitted => "ERR-GIT-UNCOMMITTED",
            git2::ErrorCode::Directory => "ERR-GIT-DIRECTORY",
            git2::ErrorCode::MergeConflict => "ERR-GIT-MERGE-CONFLICT",
            git2::ErrorCode::HashsumMismatch => "ERR-GIT-HASHSUM-MISMATCH",
            git2::ErrorCode::IndexDirty => "ERR-GIT-INDEX-DIRTY",
            git2::ErrorCode::ApplyFail => "ERR-GIT-APPLY-FAIL",
            git2::ErrorCode::Owner => "ERR-GIT-OWNER",
        };

        StandardError::new(code).interpolate_err(error.to_string())
    }
}
