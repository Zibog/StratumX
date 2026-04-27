//! Action Dispatch Core - Central dispatch logic
//!
//! Per canonical boundary law: editor routes to tooling, tooling executes.
//! This dispatcher routes actions toward the tooling layer.

use crate::{ActionContext, ActionId, ActionRegistry, ActionResult, ErrorCode, FocusTarget};

// ============================================================================
// Action Dispatcher
// ============================================================================

/// Core dispatcher for routing actions to tooling layer.
///
/// Per canonical boundary (09_GLOBAL_BOUNDARY_PRESERVATION_MATRIX):
/// - Editor defines routes (ActionRegistry)
/// - Tooling executes transactions
/// - This dispatcher bridges editor -> tooling
pub struct ActionDispatcher {
    registry: ActionRegistry,
}

impl ActionDispatcher {
    pub fn new(registry: ActionRegistry) -> Self {
        Self { registry }
    }

    /// Dispatches an action to tooling layer.
    ///
    /// Routes to tooling command executor.
    /// Returns failure until tooling integration is wired.
    pub fn dispatch(&self, action_id: &ActionId, _context: &ActionContext) -> ActionResult {
        if let Some(_definition) = self.registry.get_handler(action_id) {
            // Tooling route exists, but this seam still reports failure until executor bridging is attached.
            ActionResult::failure(ErrorCode::UnknownAction, FocusTarget::Diagnostics)
        } else {
            ActionResult::failure(ErrorCode::UnknownAction, FocusTarget::Diagnostics)
        }
    }

    pub fn registry(&self) -> &ActionRegistry {
        &self.registry
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dispatcher_creation() {
        let registry = ActionRegistry::new();
        let dispatcher = ActionDispatcher::new(registry);
        assert!(dispatcher.registry().definitions().count() == 0);
    }

    #[test]
    fn test_dispatch_unknown_action() {
        let registry = ActionRegistry::new();
        let dispatcher = ActionDispatcher::new(registry);
        let context = ActionContext::test_empty();

        let result = dispatcher.dispatch(&ActionId::new("unknown.action"), &context);
        assert!(!result.success);
    }
}
