use super::{ActionContext, ActionDefinition, ActionValidator, DisabledReason, StateQueries};
use crate::MutationClass;

impl ActionValidator {
    pub(crate) fn validate_runtime_requirements(
        definition: &ActionDefinition,
        state_queries: &dyn StateQueries,
    ) -> Result<(), DisabledReason> {
        if Self::is_mutation(definition) && state_queries.is_transaction_active() {
            return Err(DisabledReason::ActiveTransactionForbidsMutation);
        }

        Ok(())
    }

    /// Validates preconditions for scene load operations specifically.
    pub fn validate_scene_load_preconditions(
        context: &ActionContext,
        _state_queries: &dyn StateQueries,
    ) -> Result<(), DisabledReason> {
        if let Some(session) = context.session() {
            if session.loading_in_progress {
                return Err(DisabledReason::ActiveTransactionForbidsMutation);
            }
        }

        if context.has_unsaved_changes() {
            return Err(DisabledReason::PlaceholderBlocksAction);
        }

        Ok(())
    }

    /// Determines if an action performs a mutation.
    pub(crate) fn is_mutation(definition: &ActionDefinition) -> bool {
        matches!(
            definition.mutation_class,
            MutationClass::Mutate | MutationClass::Simulate
        )
    }
}
