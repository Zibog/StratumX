//! Focus Recovery - Focus restoration after action execution
//!
//! This module handles focus recovery and restoration logic after
//! actions complete, ensuring the UI returns to the appropriate state.

use crate::{ActionResult, FocusTarget};

// ============================================================================
// Focus Recovery
// ============================================================================

/// Determines the appropriate focus target after an action completes.
pub fn determine_focus_after_action(result: &ActionResult) -> FocusTarget {
    result.focus_target.clone()
}

/// Checks if a focus target requires panel activation.
pub fn requires_panel_activation(target: &FocusTarget) -> bool {
    matches!(target, FocusTarget::SpecificPanel(_))
}

/// Extracts panel ID from a focus target if it's a specific panel.
pub fn extract_panel_id(target: &FocusTarget) -> Option<String> {
    match target {
        FocusTarget::SpecificPanel(id) => Some(id.clone()),
        _ => None,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ErrorCode;

    #[test]
    fn test_determine_focus_after_success() {
        let result = ActionResult::success(FocusTarget::SpecificPanel("viewport".to_string()));
        let focus = determine_focus_after_action(&result);
        assert!(matches!(focus, FocusTarget::SpecificPanel(_)));
    }

    #[test]
    fn test_determine_focus_after_failure() {
        let result = ActionResult::failure(ErrorCode::UnknownAction, FocusTarget::Diagnostics);
        let focus = determine_focus_after_action(&result);
        assert!(matches!(focus, FocusTarget::Diagnostics));
    }

    #[test]
    fn test_requires_panel_activation() {
        assert!(requires_panel_activation(&FocusTarget::SpecificPanel(
            "viewport".to_string()
        )));
        assert!(!requires_panel_activation(&FocusTarget::Diagnostics));
        assert!(!requires_panel_activation(&FocusTarget::None));
    }

    #[test]
    fn test_extract_panel_id() {
        let panel_target = FocusTarget::SpecificPanel("inspector".to_string());
        assert_eq!(
            extract_panel_id(&panel_target),
            Some("inspector".to_string())
        );

        let diag_target = FocusTarget::Diagnostics;
        assert_eq!(extract_panel_id(&diag_target), None);
    }
}
