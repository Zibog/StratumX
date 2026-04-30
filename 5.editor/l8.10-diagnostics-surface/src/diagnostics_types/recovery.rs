//! Recovery anchors captured from diagnostics workflows.

use crate::diagnostics_types::panels::RecoverySnapshot;

#[derive(Debug, Clone)]
pub struct RecoveryAnchor {
    pub id: String,
    pub label: String,
    pub timestamp: String,
    pub snapshot: RecoverySnapshot,
}
