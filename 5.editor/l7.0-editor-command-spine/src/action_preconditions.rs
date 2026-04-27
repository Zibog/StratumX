//! Action Preconditions - Validation logic for action execution
//!
//! This module provides precondition checking to determine if an action
//! can be executed in the current context.

use crate::{ActionContext, DisabledReason};

// ============================================================================
// Precondition Validators
// ============================================================================

/// Checks if a project is required and available.
pub fn requires_project(context: &ActionContext) -> Result<(), DisabledReason> {
    if context.has_project() {
        Ok(())
    } else {
        Err(DisabledReason::NoLegalProject)
    }
}

/// Checks if an active world is required and available.
pub fn requires_active_world(context: &ActionContext) -> Result<(), DisabledReason> {
    if context.has_active_world() {
        Ok(())
    } else {
        Err(DisabledReason::NoActiveWorld)
    }
}

/// Checks if a selection is required and available.
pub fn requires_selection(context: &ActionContext) -> Result<(), DisabledReason> {
    if context.has_selection() {
        Ok(())
    } else {
        Err(DisabledReason::NoSelection)
    }
}

/// Checks if material registry is required and available.
pub fn requires_material_registry(context: &ActionContext) -> Result<(), DisabledReason> {
    if context.has_material_registry() {
        Ok(())
    } else {
        Err(DisabledReason::MaterialRegistryUnavailable)
    }
}

/// Checks if audio registry is required and available.
pub fn requires_audio_registry(context: &ActionContext) -> Result<(), DisabledReason> {
    if context.has_audio_registry() {
        Ok(())
    } else {
        Err(DisabledReason::AudioRegistryUnavailable)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_requires_project() {
        let empty_context = ActionContext::test_empty();
        assert!(requires_project(&empty_context).is_err());

        let project_context = ActionContext::test_with_project(Uuid::new_v4());
        assert!(requires_project(&project_context).is_ok());
    }

    #[test]
    fn test_requires_active_world() {
        let project_context = ActionContext::test_with_project(Uuid::new_v4());
        assert!(requires_active_world(&project_context).is_err());

        let world_context = ActionContext::test_with_world(Uuid::new_v4(), Uuid::new_v4());
        assert!(requires_active_world(&world_context).is_ok());
    }

    #[test]
    fn test_requires_selection() {
        let world_context = ActionContext::test_with_world(Uuid::new_v4(), Uuid::new_v4());
        assert!(requires_selection(&world_context).is_err());

        let selection_context = ActionContext::test_with_selection(
            Uuid::new_v4(),
            Uuid::new_v4(),
            vec![Uuid::new_v4()],
        );
        assert!(requires_selection(&selection_context).is_ok());
    }
}
