//! Focus Routing - Panel focus management after action execution
//!
//! This module implements focus target routing that validates panel availability
//! before applying focus changes.
//!
//! **Requirements: 25.1, 25.2, 25.8**
//!
//! ## Canonical Architecture:
//! ```text
//! dispatch_core.rs → focus_routing.rs (this) → validates panel availability
//!                                            → applies focus to available panels
//! ```

use crate::{ActionResult, FocusTarget};

/// Focus routing manager that validates and applies focus targets.
///
/// **Requirements: 25.1, 25.2, 25.8**
pub struct FocusRouter;

impl FocusRouter {
    /// Validates and applies focus routing for an ActionResult.
    ///
    /// **Requirements:**
    /// - 25.1: Focus panel specified in ActionResult.focus_target on success
    /// - 25.2: Focus Diagnostics panel on failure
    /// - 25.8: Validate FocusTarget panel is available before focusing
    ///
    /// # Arguments
    /// * `result` - The action result containing the focus target
    /// * `panel_validator` - Function to validate if a panel ID is available
    ///
    /// # Returns
    /// The validated focus target that should be applied
    pub fn validate_and_route<F>(result: &ActionResult, panel_validator: F) -> FocusTarget
    where
        F: Fn(&str) -> bool,
    {
        // Requirement 25.2: Focus Diagnostics panel on failure
        if !result.success {
            return FocusTarget::Diagnostics;
        }

        // Requirement 25.1: Focus panel specified in ActionResult.focus_target on success
        // Requirement 25.8: Validate FocusTarget panel is available before focusing
        match &result.focus_target {
            FocusTarget::SpecificPanel(panel_id) => {
                // Validate panel is available
                if panel_validator(panel_id) {
                    FocusTarget::SpecificPanel(panel_id.clone())
                } else {
                    // Panel not available - don't change focus
                    FocusTarget::NoChange
                }
            }
            FocusTarget::Diagnostics => {
                // Diagnostics panel is always assumed to be available
                FocusTarget::Diagnostics
            }
            FocusTarget::OwnerLab => {
                // Owner lab focus is always valid
                FocusTarget::OwnerLab
            }
            FocusTarget::NoChange => {
                // No focus change requested
                FocusTarget::NoChange
            }
            FocusTarget::None => {
                // No focus target specified
                FocusTarget::NoChange
            }
        }
    }

    /// Applies focus routing without validation (for testing or when validation is external).
    ///
    /// **Requirements: 25.1, 25.2**
    ///
    /// # Arguments
    /// * `result` - The action result containing the focus target
    ///
    /// # Returns
    /// The focus target from the result, with Diagnostics override on failure
    pub fn route_without_validation(result: &ActionResult) -> FocusTarget {
        // Requirement 25.2: Focus Diagnostics panel on failure
        if !result.success {
            return FocusTarget::Diagnostics;
        }

        // Requirement 25.1: Focus panel specified in ActionResult.focus_target on success
        result.focus_target.clone()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DisabledReason, ErrorCode, FocusTarget};

    #[test]
    fn test_focus_routing_success_with_valid_panel() {
        let result = ActionResult::success(FocusTarget::SpecificPanel("viewport".to_string()));

        // Panel is available
        let focus = FocusRouter::validate_and_route(&result, |panel_id| panel_id == "viewport");

        assert_eq!(focus, FocusTarget::SpecificPanel("viewport".to_string()));
    }

    #[test]
    fn test_focus_routing_success_with_invalid_panel() {
        let result = ActionResult::success(FocusTarget::SpecificPanel("nonexistent".to_string()));

        // Panel is not available
        let focus = FocusRouter::validate_and_route(&result, |_panel_id| false);

        // Should fall back to NoChange when panel is not available
        assert_eq!(focus, FocusTarget::NoChange);
    }

    #[test]
    fn test_focus_routing_failure_always_diagnostics() {
        let result = ActionResult::failure(
            ErrorCode::DisabledReason(DisabledReason::NoLegalProject),
            FocusTarget::SpecificPanel("viewport".to_string()),
        );

        // Even though result specifies viewport, failure should focus Diagnostics
        let focus = FocusRouter::validate_and_route(&result, |_| true);

        assert_eq!(focus, FocusTarget::Diagnostics);
    }

    #[test]
    fn test_focus_routing_diagnostics_always_valid() {
        let result = ActionResult::success(FocusTarget::Diagnostics);

        // Diagnostics should always be valid regardless of validator
        let focus = FocusRouter::validate_and_route(&result, |_| false);

        assert_eq!(focus, FocusTarget::Diagnostics);
    }

    #[test]
    fn test_focus_routing_owner_lab_always_valid() {
        let result = ActionResult::success(FocusTarget::OwnerLab);

        // OwnerLab should always be valid regardless of validator
        let focus = FocusRouter::validate_and_route(&result, |_| false);

        assert_eq!(focus, FocusTarget::OwnerLab);
    }

    #[test]
    fn test_focus_routing_no_change() {
        let result = ActionResult::success(FocusTarget::NoChange);

        // NoChange should remain NoChange
        let focus = FocusRouter::validate_and_route(&result, |_| false);

        assert_eq!(focus, FocusTarget::NoChange);
    }

    #[test]
    fn test_route_without_validation_success() {
        let result = ActionResult::success(FocusTarget::SpecificPanel("viewport".to_string()));

        let focus = FocusRouter::route_without_validation(&result);

        assert_eq!(focus, FocusTarget::SpecificPanel("viewport".to_string()));
    }

    #[test]
    fn test_route_without_validation_failure() {
        let result = ActionResult::failure(
            ErrorCode::DisabledReason(DisabledReason::NoLegalProject),
            FocusTarget::SpecificPanel("viewport".to_string()),
        );

        let focus = FocusRouter::route_without_validation(&result);

        // Failure should always route to Diagnostics
        assert_eq!(focus, FocusTarget::Diagnostics);
    }
}
