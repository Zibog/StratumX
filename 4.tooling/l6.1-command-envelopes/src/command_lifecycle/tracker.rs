// Command envelope tracker
//
// Wraps command payload with lifecycle tracking metadata.

use super::stages::CommandLifecycleState;
use serde::{Deserialize, Serialize};

/// Tracks the full command lifecycle: reception -> validation -> execution -> result publication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandEnvelope {
    pub command_id: u64,
    pub route_id: String,
    pub lifecycle_state: CommandLifecycleState,
    pub payload: Vec<u8>,
    pub origin_timestamp: u64,
    pub last_update_timestamp: u64,
    pub error_message: Option<String>,
}

impl CommandEnvelope {
    pub fn new(command_id: u64, route_id: impl Into<String>, payload: Vec<u8>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        Self {
            command_id,
            route_id: route_id.into(),
            lifecycle_state: CommandLifecycleState::Accepted,
            payload,
            origin_timestamp: now,
            last_update_timestamp: now,
            error_message: None,
        }
    }

    pub fn transition_to(
        &mut self,
        state: CommandLifecycleState,
        error: Option<String>,
    ) -> Result<(), String> {
        // Validate transition
        if !self.lifecycle_state.can_transition_to(state) {
            return Err(format!(
                "Invalid state transition from {:?} to {:?}",
                self.lifecycle_state, state
            ));
        }

        self.lifecycle_state = state;
        self.error_message = error;
        self.last_update_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Ok(())
    }

    /// Check if the command is in a terminal state
    pub fn is_terminal(&self) -> bool {
        self.lifecycle_state.is_terminal()
    }

    /// Check if the command can be retried
    pub fn can_retry(&self) -> bool {
        self.lifecycle_state.can_retry()
    }

    /// Get the duration since command was created (in milliseconds)
    pub fn age_ms(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        now.saturating_sub(self.origin_timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_envelope_creation() {
        let envelope = CommandEnvelope::new(1, "test.route", vec![1, 2, 3]);
        assert_eq!(envelope.command_id, 1);
        assert_eq!(envelope.route_id, "test.route");
        assert_eq!(envelope.lifecycle_state, CommandLifecycleState::Accepted);
    }

    #[test]
    fn test_command_envelope_transition() {
        let mut envelope = CommandEnvelope::new(1, "test.route", vec![]);

        // Valid transition
        assert!(envelope
            .transition_to(CommandLifecycleState::Validated, None)
            .is_ok());
        assert_eq!(envelope.lifecycle_state, CommandLifecycleState::Validated);

        // Invalid transition
        assert!(envelope
            .transition_to(CommandLifecycleState::Success, None)
            .is_err());
    }

    #[test]
    fn test_command_envelope_terminal() {
        let mut envelope = CommandEnvelope::new(1, "test.route", vec![]);
        assert!(!envelope.is_terminal());

        envelope
            .transition_to(CommandLifecycleState::Validated, None)
            .unwrap();
        envelope
            .transition_to(CommandLifecycleState::Running, None)
            .unwrap();
        envelope
            .transition_to(CommandLifecycleState::Success, None)
            .unwrap();

        assert!(envelope.is_terminal());
    }

    #[test]
    fn test_command_envelope_age() {
        let envelope = CommandEnvelope::new(1, "test.route", vec![]);
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(envelope.age_ms() >= 10);
    }
}

/// Orchestrates the full command lifecycle pipeline
///
/// Manages the complete flow: reception -> validation -> execution -> result publication
pub struct CommandLifecycleTracker {
    envelopes: std::collections::HashMap<u64, CommandEnvelope>,
    next_command_id: u64,
}

impl Default for CommandLifecycleTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandLifecycleTracker {
    pub fn new() -> Self {
        Self {
            envelopes: std::collections::HashMap::new(),
            next_command_id: 1,
        }
    }

    /// Phase 1: Reception - accept a command and create an envelope
    pub fn receive_command(&mut self, route_id: impl Into<String>, payload: Vec<u8>) -> u64 {
        let command_id = self.next_command_id;
        self.next_command_id += 1;

        let envelope = CommandEnvelope::new(command_id, route_id, payload);
        self.envelopes.insert(command_id, envelope);
        command_id
    }

    /// Phase 2: Validation - validate the command and transition to Validated or TerminalFailure
    pub fn validate_command(
        &mut self,
        command_id: u64,
        validation_passed: bool,
    ) -> Result<(), String> {
        let envelope = self
            .envelopes
            .get_mut(&command_id)
            .ok_or_else(|| format!("Command {} not found", command_id))?;

        if validation_passed {
            envelope.transition_to(CommandLifecycleState::Validated, None)
        } else {
            envelope.transition_to(
                CommandLifecycleState::TerminalFailure,
                Some("Validation failed".to_string()),
            )
        }
    }

    /// Phase 3: Execution - start command execution
    pub fn start_execution(&mut self, command_id: u64) -> Result<(), String> {
        let envelope = self
            .envelopes
            .get_mut(&command_id)
            .ok_or_else(|| format!("Command {} not found", command_id))?;

        envelope.transition_to(CommandLifecycleState::Running, None)
    }

    /// Phase 4: Result Publication - publish success or failure result
    pub fn publish_result(
        &mut self,
        command_id: u64,
        success: bool,
        error: Option<String>,
    ) -> Result<(), String> {
        let envelope = self
            .envelopes
            .get_mut(&command_id)
            .ok_or_else(|| format!("Command {} not found", command_id))?;

        if success {
            envelope.transition_to(CommandLifecycleState::Success, None)
        } else {
            let state = match &error {
                Some(e) if e.contains("retry") => CommandLifecycleState::RetryableFailure,
                _ => CommandLifecycleState::TerminalFailure,
            };
            envelope.transition_to(state, error)
        }
    }

    /// Retry a command that is in RetryableFailure state
    pub fn retry_command(&mut self, command_id: u64) -> Result<(), String> {
        let envelope = self
            .envelopes
            .get_mut(&command_id)
            .ok_or_else(|| format!("Command {} not found", command_id))?;

        if !envelope.can_retry() {
            return Err(format!(
                "Command {} cannot be retried (state: {:?})",
                command_id, envelope.lifecycle_state
            ));
        }

        envelope.transition_to(CommandLifecycleState::Running, None)
    }

    /// Get the current state of a command
    pub fn get_state(&self, command_id: u64) -> Option<&CommandLifecycleState> {
        self.envelopes.get(&command_id).map(|e| &e.lifecycle_state)
    }

    /// Check if a command has completed (terminal state)
    pub fn is_complete(&self, command_id: u64) -> Option<bool> {
        self.envelopes.get(&command_id).map(|e| e.is_terminal())
    }

    /// Get the envelope for a command
    pub fn get_envelope(&self, command_id: u64) -> Option<&CommandEnvelope> {
        self.envelopes.get(&command_id)
    }
}

#[cfg(test)]
mod tracker_tests {
    use super::*;

    #[test]
    fn test_full_lifecycle_reception_to_success() {
        let mut tracker = CommandLifecycleTracker::new();

        // Reception
        let id = tracker.receive_command("test.route", vec![1, 2, 3]);
        assert_eq!(
            tracker.get_state(id),
            Some(&CommandLifecycleState::Accepted)
        );

        // Validation
        tracker.validate_command(id, true).unwrap();
        assert_eq!(
            tracker.get_state(id),
            Some(&CommandLifecycleState::Validated)
        );

        // Execution
        tracker.start_execution(id).unwrap();
        assert_eq!(tracker.get_state(id), Some(&CommandLifecycleState::Running));

        // Result publication
        tracker.publish_result(id, true, None).unwrap();
        assert_eq!(tracker.get_state(id), Some(&CommandLifecycleState::Success));
        assert!(tracker.is_complete(id).unwrap());
    }

    #[test]
    fn test_lifecycle_validation_failure() {
        let mut tracker = CommandLifecycleTracker::new();
        let id = tracker.receive_command("test.route", vec![]);

        tracker.validate_command(id, false).unwrap();
        assert_eq!(
            tracker.get_state(id),
            Some(&CommandLifecycleState::TerminalFailure)
        );
    }

    #[test]
    fn test_lifecycle_retry() {
        let mut tracker = CommandLifecycleTracker::new();
        let id = tracker.receive_command("test.route", vec![]);

        tracker.validate_command(id, true).unwrap();
        tracker.start_execution(id).unwrap();
        tracker
            .publish_result(id, false, Some("retry: transient error".to_string()))
            .unwrap();
        assert_eq!(
            tracker.get_state(id),
            Some(&CommandLifecycleState::RetryableFailure)
        );

        // Retry
        tracker.retry_command(id).unwrap();
        assert_eq!(tracker.get_state(id), Some(&CommandLifecycleState::Running));

        // Complete on retry
        tracker.publish_result(id, true, None).unwrap();
        assert_eq!(tracker.get_state(id), Some(&CommandLifecycleState::Success));
    }
}
