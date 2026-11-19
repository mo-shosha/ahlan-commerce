use crate::{ImportJobId, Timestamp};

/// Allowed job statuses from the Chapter 11 import contract.
pub const STATUS_QUEUED: &str = "queued";
pub const STATUS_RUNNING: &str = "running";
pub const STATUS_SUCCEEDED: &str = "succeeded";
pub const STATUS_FAILED: &str = "failed";

/// A job may run at most this many worker attempts.
pub const MAX_IMPORT_ATTEMPTS: i32 = 3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportJob {
    pub id: ImportJobId,
    pub status: String,
    pub input_path: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl ImportJob {
    pub(crate) fn queued(id: ImportJobId, input_path: String, now: Timestamp) -> Self {
        Self {
            id,
            status: STATUS_QUEUED.to_string(),
            input_path,
            attempts: 0,
            last_error: None,
            created_at: now,
            updated_at: now,
        }
    }
}
