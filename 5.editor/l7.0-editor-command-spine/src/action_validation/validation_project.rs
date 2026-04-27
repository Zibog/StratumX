use super::{ActionDefinition, ActionValidator, DisabledReason, StateQueries};
use crate::ActionFamily;

impl ActionValidator {
    pub(crate) fn validate_project_requirements(
        definition: &ActionDefinition,
        state_queries: &dyn StateQueries,
    ) -> Result<(), DisabledReason> {
        if Self::requires_project(definition) && !state_queries.has_project() {
            return Err(DisabledReason::NoLegalProject);
        }

        Ok(())
    }

    /// Determines if an action requires a project to be open.
    ///
    /// Most actions require a project except:
    /// - ViewPanel actions (always available)
    /// - ProjectFile "new" actions (create new project)
    pub(crate) fn requires_project(definition: &ActionDefinition) -> bool {
        match definition.action_family {
            ActionFamily::ViewPanel => false,
            ActionFamily::ProjectFile => !definition.action_id.as_str().contains("new"),
            _ => true,
        }
    }
}
