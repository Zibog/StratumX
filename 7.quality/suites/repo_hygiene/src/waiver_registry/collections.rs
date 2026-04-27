use serde::{Deserialize, Serialize};

/// A single waiver entry with path and justification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WaiverEntry {
    pub path: String,
    pub justification: String,
}
