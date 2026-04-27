//! ActionDispatch — main dispatcher facade delegating to DispatchCore.

pub use crate::action_execution::ActionExecutor;
pub use crate::action_focus::ActionFocus;
pub use crate::action_queries::StateQueries;
pub use crate::action_validation::ActionValidator;
use crate::dispatch_core::DispatchCore;
use crate::{ActionContext, ActionId, ActionRegistry, ActionResult};
use std::sync::Arc;

pub struct ActionDispatch {
    dispatch_core: DispatchCore,
}

impl ActionDispatch {
    pub fn new(registry: Arc<ActionRegistry>, state_queries: Arc<dyn StateQueries>) -> Self {
        Self {
            dispatch_core: DispatchCore::new(registry, state_queries),
        }
    }
    pub fn with_panel_validator<F>(
        registry: Arc<ActionRegistry>,
        state_queries: Arc<dyn StateQueries>,
        panel_validator: F,
    ) -> Self
    where
        F: Fn(&str) -> bool + Send + Sync + 'static,
    {
        Self {
            dispatch_core: DispatchCore::new(registry, state_queries)
                .with_panel_validator(panel_validator),
        }
    }
    pub fn dispatch(&self, action_id: ActionId, context: ActionContext) -> ActionResult {
        self.dispatch_core.dispatch(action_id, context)
    }
}
