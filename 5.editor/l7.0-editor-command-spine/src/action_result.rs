//! Action Result - Result types for action execution
//!
//! Defines the result structure returned by action handlers.

use crate::{ActionId, DenialFamily, DisabledReason};
use serde::{Deserialize, Serialize};

// ============================================================================
// Focus Target
// ============================================================================

/// Specifies where focus should be directed after an action completes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FocusTarget {
    SpecificPanel(String),
    OwnerLab,
    Diagnostics,
    NoChange,
    None,
}

// ============================================================================
// Action Result
// ============================================================================

/// Result of action execution.
#[derive(Debug, Clone)]
pub struct ActionResult {
    pub success: bool,
    pub error_code: Option<ErrorCode>,
    pub artifact_ref: Option<String>,
    pub focus_target: FocusTarget,
    pub next_legal_recovery_action: Option<ActionId>,
}

impl ActionResult {
    pub fn success(focus_target: FocusTarget) -> Self {
        Self {
            success: true,
            error_code: None,
            artifact_ref: None,
            focus_target,
            next_legal_recovery_action: None,
        }
    }

    pub fn success_with_artifact(artifact_ref: String, focus_target: FocusTarget) -> Self {
        Self {
            success: true,
            error_code: None,
            artifact_ref: Some(artifact_ref),
            focus_target,
            next_legal_recovery_action: None,
        }
    }

    pub fn failure(error_code: ErrorCode, focus_target: FocusTarget) -> Self {
        Self {
            success: false,
            error_code: Some(error_code),
            artifact_ref: None,
            focus_target,
            next_legal_recovery_action: None,
        }
    }

    pub fn failure_with_recovery(
        error_code: ErrorCode,
        recovery_action: ActionId,
        focus_target: FocusTarget,
    ) -> Self {
        Self {
            success: false,
            error_code: Some(error_code),
            artifact_ref: None,
            focus_target,
            next_legal_recovery_action: Some(recovery_action),
        }
    }
}

// ============================================================================
// Error Code
// ============================================================================

/// Error code for failed actions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorCode {
    DisabledReason(DisabledReason),
    DenialFamily(DenialFamily),
    UnknownAction,
}
