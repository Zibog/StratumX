use super::{ActionContext, ActionDefinition, ActionValidator, DisabledReason};
use crate::ActionFamily;

impl ActionValidator {
    pub(crate) fn validate_material_requirements(
        definition: &ActionDefinition,
        context: &ActionContext,
    ) -> Result<(), DisabledReason> {
        if definition.action_family == ActionFamily::Material && !context.has_material_registry() {
            return Err(DisabledReason::RouteUnsupportedByBuildProfile);
        }

        Ok(())
    }
}
