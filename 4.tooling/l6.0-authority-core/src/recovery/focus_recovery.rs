use super::evidence_recovery::RecoveryManager;

impl RecoveryManager {
    /// Clear the error state for an operation.
    pub fn clear_error(&mut self, operation: &str) -> Result<(), String> {
        let state = self
            .error_states
            .get_mut(operation)
            .ok_or_else(|| format!("No error state found for operation: {}", operation))?;

        state.cleared = true;
        Ok(())
    }

    /// Check if an operation's error has been cleared.
    pub fn is_error_cleared(&self, operation: &str) -> bool {
        self.error_states
            .get(operation)
            .map(|state| state.cleared)
            .unwrap_or(true)
    }

    /// Remove a cleared error state.
    pub fn purge_cleared(&mut self, operation: &str) {
        self.error_states.remove(operation);
    }
}
