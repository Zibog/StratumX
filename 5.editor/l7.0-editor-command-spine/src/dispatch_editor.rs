//! Dispatch Editor — editor-level action dispatch.

use crate::dispatch_route_family::DispatchRouteFamily;
use crate::{ActionContext, ActionExecutor, ActionId, ActionRegistry, ActionResult, FocusTarget};
use std::sync::Arc;

pub struct EditorDispatch {
    registry: Arc<ActionRegistry>,
}

impl EditorDispatch {
    pub fn new(registry: Arc<ActionRegistry>) -> Self {
        Self { registry }
    }
    pub fn dispatch(&self, action_id: ActionId, context: ActionContext) -> ActionResult {
        let _definition = match self.registry.get_handler(&action_id) {
            Some(def) => def,
            None => return ActionExecutor::create_not_found_result(),
        };
        match DispatchRouteFamily::classify(&action_id) {
            DispatchRouteFamily::ProjectLifecycle => self.dispatch_project(action_id, context),
            DispatchRouteFamily::RuntimeControl => self.dispatch_runtime(action_id, context),
            DispatchRouteFamily::PanelChrome => self.dispatch_panel(action_id, context),
            DispatchRouteFamily::DiagnosticsProof => self.dispatch_diagnostics(action_id, context),
            DispatchRouteFamily::BuildRelease => self.dispatch_build(action_id, context),
            _ => ActionExecutor::create_not_found_result(),
        }
    }
    fn dispatch_project(&self, action_id: ActionId, _context: ActionContext) -> ActionResult {
        let _definition = match self.registry.get_handler(&action_id) {
            Some(def) => def,
            None => return ActionExecutor::create_not_found_result(),
        };
        ActionResult::success(FocusTarget::NoChange)
    }
    fn dispatch_runtime(&self, action_id: ActionId, _context: ActionContext) -> ActionResult {
        let _definition = match self.registry.get_handler(&action_id) {
            Some(def) => def,
            None => return ActionExecutor::create_not_found_result(),
        };
        ActionResult::success(FocusTarget::SpecificPanel("viewport".into()))
    }
    fn dispatch_panel(&self, action_id: ActionId, _context: ActionContext) -> ActionResult {
        let _definition = match self.registry.get_handler(&action_id) {
            Some(def) => def,
            None => return ActionExecutor::create_not_found_result(),
        };
        ActionResult::success(FocusTarget::NoChange)
    }
    fn dispatch_diagnostics(&self, action_id: ActionId, _context: ActionContext) -> ActionResult {
        let _definition = match self.registry.get_handler(&action_id) {
            Some(def) => def,
            None => return ActionExecutor::create_not_found_result(),
        };
        ActionResult::success(FocusTarget::SpecificPanel("diagnostics".into()))
    }
    fn dispatch_build(&self, action_id: ActionId, _context: ActionContext) -> ActionResult {
        let _definition = match self.registry.get_handler(&action_id) {
            Some(def) => def,
            None => return ActionExecutor::create_not_found_result(),
        };
        ActionResult::success(FocusTarget::SpecificPanel("build_release".into()))
    }
}
