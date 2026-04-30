use super::state::CommandLifecycleState;
use super::tracker::now_ms;
use super::{CommandEnvelope, CommandLifecycleTracker};

impl CommandEnvelope {
    /// Check if the command is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        self.lifecycle_state.is_terminal()
    }

    /// Check if the command can be retried.
    pub fn can_retry(&self) -> bool {
        self.lifecycle_state.can_retry()
    }

    /// Get the duration since the command was created in milliseconds.
    pub fn age_ms(&self) -> u64 {
        now_ms().saturating_sub(self.origin_timestamp)
    }
}

impl CommandLifecycleTracker {
    /// Get the current state of a command.
    pub fn get_state(&self, command_id: u64) -> Option<&CommandLifecycleState> {
        self.envelopes
            .get(&command_id)
            .map(|envelope| &envelope.lifecycle_state)
    }

    /// Check if a command has completed (terminal state).
    pub fn is_complete(&self, command_id: u64) -> Option<bool> {
        self.envelopes
            .get(&command_id)
            .map(|envelope| envelope.is_terminal())
    }

    /// Get the envelope for a command.
    pub fn get_envelope(&self, command_id: u64) -> Option<&CommandEnvelope> {
        self.envelopes.get(&command_id)
    }
}
