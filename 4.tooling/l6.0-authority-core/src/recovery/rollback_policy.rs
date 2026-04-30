use super::errors::ErrorClass;
use super::evidence_recovery::RecoveryManager;

pub(crate) fn requires_rollback(error_class: ErrorClass) -> bool {
    matches!(
        error_class,
        ErrorClass::RegistryCorruption | ErrorClass::InvalidLifecycleTransition
    )
}

impl RecoveryManager {
    /// Rollback an operation to its previous consistent state.
    pub fn rollback(&mut self, operation: &str) -> Result<(), String> {
        let state = self
            .error_states
            .get_mut(operation)
            .ok_or_else(|| format!("No error state found for operation: {}", operation))?;

        state.requires_rollback = true;
        Ok(())
    }
}
