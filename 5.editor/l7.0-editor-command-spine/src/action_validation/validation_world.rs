use super::{ActionDefinition, ActionValidator, DisabledReason, StateQueries};
use crate::ActionFamily;

impl ActionValidator {
    pub(crate) fn validate_world_requirements(
        definition: &ActionDefinition,
        state_queries: &dyn StateQueries,
    ) -> Result<(), DisabledReason> {
        if Self::requires_world(definition) && !state_queries.has_active_world() {
            return Err(DisabledReason::NoLegalProject);
        }

        Ok(())
    }

    pub(crate) fn validate_selection_requirements(
        definition: &ActionDefinition,
        state_queries: &dyn StateQueries,
    ) -> Result<(), DisabledReason> {
        if Self::requires_selection(definition) && !state_queries.has_selection() {
            return Err(DisabledReason::NoLegalTargetSelected);
        }

        Ok(())
    }

    /// Determines if an action requires a world to be open.
    pub(crate) fn requires_world(definition: &ActionDefinition) -> bool {
        match definition.action_family {
            ActionFamily::World => !definition.action_id.as_str().contains("open"),
            ActionFamily::Terrain
            | ActionFamily::Material
            | ActionFamily::Audio
            | ActionFamily::SkyEnvironment
            | ActionFamily::Runtime => true,
            _ => false,
        }
    }

    /// Determines if an action requires a selection.
    pub(crate) fn requires_selection(definition: &ActionDefinition) -> bool {
        let id = definition.action_id.as_str();
        id.contains("selected") || id.contains("inspect")
    }
}
