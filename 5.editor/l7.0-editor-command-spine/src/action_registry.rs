//! Action Registry - Centralized registry of editor actions
//!
//! Per canonical boundary law (09_GLOBAL_BOUNDARY_PRESERVATION_MATRIX):
//! - Editor defines routes and metadata (ActionDefinition)
//! - Tooling owns transaction execution (actual handlers)
//! - Stub handlers here are for testing only

use crate::{ActionContext, ActionResult};
use crate::{ActionDefinition, ActionId};
use std::collections::HashMap;

/// Type alias for stub action handler functions (testing only).
///
/// Real handlers live in tooling layer per canonical boundary law.
pub type StubHandler = fn(&ActionContext) -> ActionResult;

/// Centralized registry of editor action routes and metadata.
///
/// Stores ActionDefinitions (routes/metadata) and optional stub handlers for testing.
pub struct ActionRegistry {
    actions: HashMap<ActionId, ActionDefinition>,
    stub_handlers: HashMap<ActionId, StubHandler>,
}

impl ActionRegistry {
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
            stub_handlers: HashMap::new(),
        }
    }

    /// Registers an action definition (route metadata).
    pub fn register(&mut self, definition: ActionDefinition) {
        self.actions
            .insert(definition.action_id.clone(), definition);
    }

    /// Registers a stub handler for testing (not used in production).
    pub fn register_stub_handler(&mut self, action_id: ActionId, handler: StubHandler) {
        self.stub_handlers.insert(action_id, handler);
    }

    /// Looks up an action definition by ID.
    pub fn get_handler(&self, action_id: &ActionId) -> Option<&ActionDefinition> {
        self.actions.get(action_id)
    }

    /// Invokes a stub handler for testing (not used in production).
    pub fn invoke_stub(
        &self,
        action_id: &ActionId,
        context: &ActionContext,
    ) -> Option<ActionResult> {
        self.stub_handlers.get(action_id).map(|h| h(context))
    }

    pub fn len(&self) -> usize {
        self.actions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    pub fn action_ids(&self) -> impl Iterator<Item = &ActionId> {
        self.actions.keys()
    }

    pub fn definitions(&self) -> impl Iterator<Item = &ActionDefinition> {
        self.actions.values()
    }
}

impl Default for ActionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Stub Handlers (Testing Only)
// ============================================================================

pub mod stub_handlers {
    use super::*;
    use crate::FocusTarget;

    pub fn _stub_success(_context: &ActionContext) -> ActionResult {
        ActionResult::success(FocusTarget::NoChange)
    }

    pub fn stub_viewport_focus(_context: &ActionContext) -> ActionResult {
        ActionResult::success(FocusTarget::SpecificPanel("viewport".to_string()))
    }
}
