use super::{ActionContext, ActionDefinition, ActionValidator, DisabledReason};
use crate::ActionFamily;

impl ActionValidator {
    pub(crate) fn validate_diagnostics_requirements(
        definition: &ActionDefinition,
        context: &ActionContext,
    ) -> Result<(), DisabledReason> {
        if definition.action_family == ActionFamily::DiagnosticsProof
            && !context.has_diagnostics_registry()
        {
            return Err(DisabledReason::DiagnosticsSourceUnavailable);
        }

        Ok(())
    }
}
