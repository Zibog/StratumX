//! Action Registry - Centralized registry of editor actions
//!
//! Per canonical boundary law (09_GLOBAL_BOUNDARY_PRESERVATION_MATRIX):
//! - Editor defines routes and metadata (ActionDefinition)
//! - Tooling owns transaction execution (actual handlers)
//! - Stub handlers here are for testing only

#[cfg(test)]
use crate::{ActionContext, ActionResult};
use crate::{ActionDefinition, ActionId};
use std::collections::HashMap;

/// Type alias for stub action handler functions (testing only).
///
/// Real handlers live in tooling layer per canonical boundary law.
#[cfg(test)]
pub type StubHandler = fn(&ActionContext) -> ActionResult;

/// Centralized registry of editor action routes and metadata.
///
/// Stores ActionDefinitions (routes/metadata) and optional stub handlers for testing.
pub struct ActionRegistry {
    actions: HashMap<ActionId, ActionDefinition>,
    #[cfg(test)]
    stub_handlers: HashMap<ActionId, StubHandler>,
}

impl ActionRegistry {
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
            #[cfg(test)]
            stub_handlers: HashMap::new(),
        }
    }

    /// Registers an action definition (route metadata).
    pub fn register(&mut self, definition: ActionDefinition) {
        self.actions
            .insert(definition.action_id.clone(), definition);
    }

    /// Registers a stub handler for testing (not used in production).
    #[cfg(test)]
    pub fn register_stub_handler(&mut self, action_id: ActionId, handler: StubHandler) {
        self.stub_handlers.insert(action_id, handler);
    }

    /// Looks up an action definition by ID.
    pub fn get_handler(&self, action_id: &ActionId) -> Option<&ActionDefinition> {
        self.actions.get(action_id)
    }

    /// Invokes a stub handler for testing (not used in production).
    #[cfg(test)]
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

#[cfg(test)]
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

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ActionFamily, MutationClass};

    fn create_test_definition(action_id: &str, family: ActionFamily) -> ActionDefinition {
        ActionDefinition {
            action_id: ActionId::new(action_id),
            display_label: format!("Test {}", action_id),
            action_family: family,
            tooling_route: format!("route.{}.v1", action_id),
            sdk_packet_family: "packet.test.*".to_string(),
            engine_truth_owner: "engine/test".to_string(),
            mutation_class: MutationClass::Read,
            possible_denial_families: vec![],
        }
    }

    #[test]
    fn test_new_registry_is_empty() {
        let registry = ActionRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_register_action() {
        let mut registry = ActionRegistry::new();
        let definition = create_test_definition("world.open", ActionFamily::World);

        registry.register(definition);

        assert_eq!(registry.len(), 1);
        assert!(!registry.is_empty());
    }

    #[test]
    fn test_get_handler_found() {
        let mut registry = ActionRegistry::new();
        registry.register(create_test_definition("world.open", ActionFamily::World));

        let result = registry.get_handler(&ActionId::new("world.open"));
        assert!(result.is_some());
        assert_eq!(result.unwrap().action_id.as_str(), "world.open");
    }

    #[test]
    fn test_stub_handler_execution() {
        let mut registry = ActionRegistry::new();
        registry.register(create_test_definition("world.open", ActionFamily::World));
        registry.register_stub_handler(
            ActionId::new("world.open"),
            stub_handlers::stub_viewport_focus,
        );

        let context = ActionContext::test_empty();
        let result = registry.invoke_stub(&ActionId::new("world.open"), &context);

        assert!(result.is_some());
        assert!(result.unwrap().success);
    }
}
