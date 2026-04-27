//! Dispatch Material - Material action dispatch
//!
//! Handles dispatch for material authoring actions.
//!
//! ## Responsibilities:
//! - Material authoring: author profile, bind surface, inspect

use crate::{ActionContext, ActionExecutor, ActionId, ActionRegistry, ActionResult, FocusTarget};
use std::sync::Arc;

use crate::dispatch_route_family::DispatchRouteFamily;

/// Dispatcher for material-related actions.
///
/// Handles:
/// - Material authoring (author profile, bind surface, inspect)
pub struct MaterialDispatch {
    registry: Arc<ActionRegistry>,
}

impl MaterialDispatch {
    /// Creates a new MaterialDispatch.
    pub fn new(registry: Arc<ActionRegistry>) -> Self {
        Self { registry }
    }

    /// Dispatches a material-related action.
    pub fn dispatch(&self, action_id: ActionId, context: ActionContext) -> ActionResult {
        let _definition = match self.registry.get_handler(&action_id) {
            Some(def) => def,
            None => return ActionExecutor::create_not_found_result(),
        };

        match DispatchRouteFamily::classify(&action_id) {
            DispatchRouteFamily::MaterialAuthoring => {
                self.dispatch_author_profile(action_id, context)
            }
            DispatchRouteFamily::MaterialBinding => self.dispatch_bind_surface(action_id, context),
            DispatchRouteFamily::MaterialInspection => self.dispatch_inspect(action_id, context),
            _ => ActionExecutor::create_not_found_result(),
        }
    }

    /// Dispatches material author profile action.
    fn dispatch_author_profile(
        &self,
        _action_id: ActionId,
        _context: ActionContext,
    ) -> ActionResult {
        // Return a focus-only success for the material surface at this routing seam.
        ActionResult::success(FocusTarget::SpecificPanel("material".to_string()))
    }

    /// Dispatches material bind surface action.
    fn dispatch_bind_surface(&self, _action_id: ActionId, _context: ActionContext) -> ActionResult {
        // Return a focus-only success for the material surface at this routing seam.
        ActionResult::success(FocusTarget::SpecificPanel("material".to_string()))
    }

    /// Dispatches material inspect action.
    fn dispatch_inspect(&self, _action_id: ActionId, _context: ActionContext) -> ActionResult {
        // Return a focus-only success for the material surface at this routing seam.
        ActionResult::success(FocusTarget::SpecificPanel("material".to_string()))
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ActionDefinition, ActionFamily, MutationClass};
    use uuid::Uuid;

    fn create_test_definition(action_id: &str) -> ActionDefinition {
        ActionDefinition {
            action_id: ActionId::new(action_id),
            display_label: format!("Test {}", action_id),
            action_family: ActionFamily::Material,
            tooling_route: format!("route.{}.v1", action_id),
            sdk_packet_family: "packet.test.*".to_string(),
            engine_truth_owner: "engine/test".to_string(),
            mutation_class: MutationClass::Mutate,
            possible_denial_families: vec![],
        }
    }

    fn create_test_context() -> ActionContext {
        ActionContext::test_with_world(Uuid::new_v4(), Uuid::new_v4())
    }

    #[test]
    fn test_dispatch_author_profile() {
        let mut registry = ActionRegistry::new();
        registry.register(create_test_definition("material.author_profile"));

        let dispatch = MaterialDispatch::new(Arc::new(registry));
        let result = dispatch.dispatch(
            ActionId::new("material.author_profile"),
            create_test_context(),
        );

        assert!(result.success);
        assert_eq!(
            result.focus_target,
            FocusTarget::SpecificPanel("material".to_string())
        );
    }

    #[test]
    fn test_dispatch_bind_surface() {
        let mut registry = ActionRegistry::new();
        registry.register(create_test_definition("material.bind_surface"));

        let dispatch = MaterialDispatch::new(Arc::new(registry));
        let result = dispatch.dispatch(
            ActionId::new("material.bind_surface"),
            create_test_context(),
        );

        assert!(result.success);
        assert_eq!(
            result.focus_target,
            FocusTarget::SpecificPanel("material".to_string())
        );
    }

    #[test]
    fn test_dispatch_inspect() {
        let mut registry = ActionRegistry::new();
        registry.register(create_test_definition("material.inspect"));

        let dispatch = MaterialDispatch::new(Arc::new(registry));
        let result = dispatch.dispatch(ActionId::new("material.inspect"), create_test_context());

        assert!(result.success);
        assert_eq!(
            result.focus_target,
            FocusTarget::SpecificPanel("material".to_string())
        );
    }
}
