//! Action Execution - Handler Execution Logic
//!
//! Executes action handlers and constructs results.
//!
//! ## Canonical Architecture:
//! ```text
//! action_execution.rs (this) - handler execution
//!         ↓
//! action_dispatch.rs - orchestrates validation + execution
//! ```

use crate::{ActionContext, ActionDefinition, ActionResult, FocusTarget};

/// Executes action handlers and returns results.
pub struct ActionExecutor;

impl ActionExecutor {
    /// Executes the action handler.
    ///
    /// Current behavior acknowledges a validated action without invoking a concrete handler.
    pub fn execute(_definition: &ActionDefinition, _context: &ActionContext) -> ActionResult {
        // Return a success envelope while handler invocation is delegated elsewhere in the spine.
        ActionResult::success(FocusTarget::NoChange)
    }

    /// Creates a failure result for unregistered actions.
    pub fn create_not_found_result() -> ActionResult {
        ActionResult::failure(
            crate::ErrorCode::DisabledReason(crate::DisabledReason::NoLegalRecoveryAvailable),
            FocusTarget::Diagnostics,
        )
    }

    /// Creates a failure result from a disabled reason.
    pub fn create_disabled_result(reason: crate::DisabledReason) -> ActionResult {
        ActionResult::failure(
            crate::ErrorCode::DisabledReason(reason),
            FocusTarget::Diagnostics,
        )
    }

    /// Creates a success result with focus target.
    pub fn create_success_result(focus_target: FocusTarget) -> ActionResult {
        ActionResult::success(focus_target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ActionFamily, ActionId, MutationClass};

    fn make_def() -> ActionDefinition {
        ActionDefinition {
            action_id: ActionId::new("test"),
            display_label: "Test".to_string(),
            action_family: ActionFamily::World,
            tooling_route: "route.test.v1".to_string(),
            sdk_packet_family: "packet.*".to_string(),
            engine_truth_owner: "engine/test".to_string(),
            mutation_class: MutationClass::Mutate,
            possible_denial_families: vec![],
        }
    }

    #[test]
    fn test_execute_returns_success() {
        let def = make_def();
        let ctx = ActionContext::test_empty();

        let result = ActionExecutor::execute(&def, &ctx);
        assert!(result.success);
    }

    #[test]
    fn test_create_not_found_result() {
        let result = ActionExecutor::create_not_found_result();
        assert!(!result.success);
    }

    #[test]
    fn test_create_success_with_focus() {
        let result = ActionExecutor::create_success_result(FocusTarget::SpecificPanel(
            "viewport".to_string(),
        ));
        assert!(result.success);
        assert_eq!(
            result.focus_target,
            FocusTarget::SpecificPanel("viewport".to_string())
        );
    }
}
