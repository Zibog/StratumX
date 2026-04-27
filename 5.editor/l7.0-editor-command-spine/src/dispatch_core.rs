//! Dispatch Core — routes actions to domain dispatchers.

use super::{
    dispatch_editor::EditorDispatch, dispatch_material::MaterialDispatch,
    dispatch_world::WorldDispatch,
};
use crate::dispatch_audio::AudioDispatch;
use crate::{ActionContext, ActionFamily, ActionId, ActionRegistry, ActionResult, FocusRouter};
use crate::{ActionExecutor, ActionFocus, ActionValidator, StateQueries};
use std::sync::Arc;

type PanelValidator = Arc<dyn Fn(&str) -> bool + Send + Sync>;

pub struct DispatchCore {
    registry: Arc<ActionRegistry>,
    state_queries: Arc<dyn StateQueries>,
    world_dispatch: WorldDispatch,
    editor_dispatch: EditorDispatch,
    material_dispatch: MaterialDispatch,
    audio_dispatch: AudioDispatch,
    panel_validator: Option<PanelValidator>,
}

impl DispatchCore {
    pub fn new(registry: Arc<ActionRegistry>, state_queries: Arc<dyn StateQueries>) -> Self {
        Self {
            registry: registry.clone(),
            state_queries,
            world_dispatch: WorldDispatch::new(registry.clone()),
            editor_dispatch: EditorDispatch::new(registry.clone()),
            material_dispatch: MaterialDispatch::new(registry.clone()),
            audio_dispatch: AudioDispatch::new(registry.clone()),
            panel_validator: None,
        }
    }
    pub fn with_panel_validator<F>(mut self, validator: F) -> Self
    where
        F: Fn(&str) -> bool + Send + Sync + 'static,
    {
        self.panel_validator = Some(Arc::new(validator));
        self
    }

    pub fn dispatch(&self, action_id: ActionId, context: ActionContext) -> ActionResult {
        let definition = match self.registry.get_handler(&action_id) {
            Some(def) => def,
            None => return ActionExecutor::create_not_found_result(),
        };
        if let Err(disabled_reason) =
            ActionValidator::validate(definition, &context, self.state_queries.as_ref())
        {
            return ActionExecutor::create_disabled_result(disabled_reason);
        }
        let mut result = match definition.action_family {
            ActionFamily::World | ActionFamily::Terrain | ActionFamily::SkyEnvironment => {
                self.world_dispatch.dispatch(action_id, context)
            }
            ActionFamily::ProjectFile
            | ActionFamily::ViewPanel
            | ActionFamily::Runtime
            | ActionFamily::DiagnosticsProof
            | ActionFamily::BuildRelease => self.editor_dispatch.dispatch(action_id, context),
            ActionFamily::Material => self.material_dispatch.dispatch(action_id, context),
            ActionFamily::Audio => self.audio_dispatch.dispatch(action_id, context),
        };
        result.focus_target = if result.success {
            ActionFocus::determine(definition)
        } else {
            crate::FocusTarget::Diagnostics
        };
        if let Some(ref validator) = self.panel_validator {
            result.focus_target =
                FocusRouter::validate_and_route(&result, |panel_id| validator(panel_id));
        } else {
            result.focus_target = FocusRouter::route_without_validation(&result);
        }
        result
    }
}
