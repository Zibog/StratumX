//! Dispatch Audio - Audio action dispatch
//!
//! Handles dispatch for audio authoring actions.
//!
//! ## Responsibilities:
//! - Audio authoring: author source, configure zone, preview

use crate::{ActionContext, ActionExecutor, ActionId, ActionRegistry, ActionResult, FocusTarget};
use std::sync::Arc;

use crate::dispatch_route_family::DispatchRouteFamily;

/// Dispatcher for audio-related actions.
///
/// Handles:
/// - Audio authoring (author source, configure zone, preview)
pub struct AudioDispatch {
    registry: Arc<ActionRegistry>,
}

impl AudioDispatch {
    /// Creates a new AudioDispatch.
    pub fn new(registry: Arc<ActionRegistry>) -> Self {
        Self { registry }
    }

    /// Dispatches an audio-related action.
    pub fn dispatch(&self, action_id: ActionId, context: ActionContext) -> ActionResult {
        let _definition = match self.registry.get_handler(&action_id) {
            Some(def) => def,
            None => return ActionExecutor::create_not_found_result(),
        };

        match DispatchRouteFamily::classify(&action_id) {
            DispatchRouteFamily::AudioAuthoring => self.dispatch_author_source(action_id, context),
            DispatchRouteFamily::AudioZone => self.dispatch_configure_zone(action_id, context),
            DispatchRouteFamily::AudioPreview => self.dispatch_preview(action_id, context),
            _ => ActionExecutor::create_not_found_result(),
        }
    }

    /// Dispatches audio author source action.
    fn dispatch_author_source(
        &self,
        _action_id: ActionId,
        _context: ActionContext,
    ) -> ActionResult {
        // Return a focus-only success for the audio surface at this routing seam.
        ActionResult::success(FocusTarget::SpecificPanel("audio".to_string()))
    }

    /// Dispatches audio configure zone action.
    fn dispatch_configure_zone(
        &self,
        _action_id: ActionId,
        _context: ActionContext,
    ) -> ActionResult {
        // Return a focus-only success for the audio surface at this routing seam.
        ActionResult::success(FocusTarget::SpecificPanel("audio".to_string()))
    }

    /// Dispatches audio preview action.
    fn dispatch_preview(&self, _action_id: ActionId, _context: ActionContext) -> ActionResult {
        // Return a focus-only success for the audio surface at this routing seam.
        ActionResult::success(FocusTarget::SpecificPanel("audio".to_string()))
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
            action_family: ActionFamily::Audio,
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
    fn test_dispatch_author_source() {
        let mut registry = ActionRegistry::new();
        registry.register(create_test_definition("audio.author_source"));

        let dispatch = AudioDispatch::new(Arc::new(registry));
        let result = dispatch.dispatch(ActionId::new("audio.author_source"), create_test_context());

        assert!(result.success);
        assert_eq!(
            result.focus_target,
            FocusTarget::SpecificPanel("audio".to_string())
        );
    }

    #[test]
    fn test_dispatch_configure_zone() {
        let mut registry = ActionRegistry::new();
        registry.register(create_test_definition("audio.configure_zone"));

        let dispatch = AudioDispatch::new(Arc::new(registry));
        let result =
            dispatch.dispatch(ActionId::new("audio.configure_zone"), create_test_context());

        assert!(result.success);
        assert_eq!(
            result.focus_target,
            FocusTarget::SpecificPanel("audio".to_string())
        );
    }

    #[test]
    fn test_dispatch_preview() {
        let mut registry = ActionRegistry::new();
        registry.register(create_test_definition("audio.preview"));

        let dispatch = AudioDispatch::new(Arc::new(registry));
        let result = dispatch.dispatch(ActionId::new("audio.preview"), create_test_context());

        assert!(result.success);
        assert_eq!(
            result.focus_target,
            FocusTarget::SpecificPanel("audio".to_string())
        );
    }
}
