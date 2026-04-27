use std::path::PathBuf;

/// Error that occurs during waiver validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaiverError {
    pub path: PathBuf,
    pub rule: String,
    pub error: String,
}
