use super::{ActionContext, ActionDefinition, ActionValidator, DisabledReason};
use crate::ActionFamily;

impl ActionValidator {
    pub(crate) fn validate_audio_requirements(
        definition: &ActionDefinition,
        context: &ActionContext,
    ) -> Result<(), DisabledReason> {
        if definition.action_family == ActionFamily::Audio && !context.has_audio_registry() {
            return Err(DisabledReason::RouteUnsupportedByBuildProfile);
        }

        Ok(())
    }
}
