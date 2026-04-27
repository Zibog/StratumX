//! Dispatch World — world/terrain/environment action dispatch.

use crate::dispatch_route_family::DispatchRouteFamily;
use crate::{ActionContext, ActionExecutor, ActionId, ActionRegistry, ActionResult, FocusTarget};
use std::sync::Arc;

pub struct WorldDispatch {
    registry: Arc<ActionRegistry>,
}

impl WorldDispatch {
    pub fn new(registry: Arc<ActionRegistry>) -> Self {
        Self { registry }
    }
    pub fn dispatch(&self, action_id: ActionId, context: ActionContext) -> ActionResult {
        let _definition = match self.registry.get_handler(&action_id) {
            Some(def) => def,
            None => return ActionExecutor::create_not_found_result(),
        };
        match DispatchRouteFamily::classify(&action_id) {
            DispatchRouteFamily::WorldLifecycle => {
                self.dispatch_world_lifecycle(action_id, context)
            }
            DispatchRouteFamily::TerrainAuthoring => self.dispatch_terrain(action_id, context),
            DispatchRouteFamily::EnvironmentAuthoring => {
                self.dispatch_environment(action_id, context)
            }
            _ => ActionExecutor::create_not_found_result(),
        }
    }
    fn dispatch_world_lifecycle(
        &self,
        action_id: ActionId,
        _context: ActionContext,
    ) -> ActionResult {
        let _definition = match self.registry.get_handler(&action_id) {
            Some(def) => def,
            None => return ActionExecutor::create_not_found_result(),
        };
        ActionResult::success(FocusTarget::SpecificPanel("viewport".into()))
    }
    fn dispatch_terrain(&self, action_id: ActionId, _context: ActionContext) -> ActionResult {
        let _definition = match self.registry.get_handler(&action_id) {
            Some(def) => def,
            None => return ActionExecutor::create_not_found_result(),
        };
        ActionResult::success(FocusTarget::SpecificPanel("terrain".into()))
    }
    fn dispatch_environment(&self, action_id: ActionId, _context: ActionContext) -> ActionResult {
        let _definition = match self.registry.get_handler(&action_id) {
            Some(def) => def,
            None => return ActionExecutor::create_not_found_result(),
        };
        ActionResult::success(FocusTarget::SpecificPanel("sky_lighting".into()))
    }
}
