//! Action Validation - Context Validation Logic
//!
//! Validates that action context meets all preconditions for execution.

use super::action_queries::StateQueries;
use crate::{ActionContext, ActionDefinition, DisabledReason};

mod validation_audio;
mod validation_diagnostics;
mod validation_material;
mod validation_project;
mod validation_runtime;
mod validation_world;

/// Validates action context against preconditions.
pub struct ActionValidator;

impl ActionValidator {
    /// Validates that the action context meets all preconditions.
    pub fn validate(
        definition: &ActionDefinition,
        context: &ActionContext,
        state_queries: &dyn StateQueries,
    ) -> Result<(), DisabledReason> {
        Self::validate_project_requirements(definition, state_queries)?;
        Self::validate_world_requirements(definition, state_queries)?;
        Self::validate_selection_requirements(definition, state_queries)?;
        Self::validate_runtime_requirements(definition, state_queries)?;
        Self::validate_material_requirements(definition, context)?;
        Self::validate_audio_requirements(definition, context)?;
        Self::validate_diagnostics_requirements(definition, context)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
