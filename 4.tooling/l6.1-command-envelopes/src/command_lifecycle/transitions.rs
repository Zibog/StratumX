use super::errors::{command_not_found, command_not_retryable, invalid_transition};
use super::state::CommandLifecycleState;
use super::tracker::now_ms;
use super::{CommandEnvelope, CommandLifecycleTracker};

impl CommandLifecycleState {
    /// Check if the state is terminal (no further transitions possible).
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Success | Self::TerminalFailure)
    }

    /// Check if the state allows retry.
    pub fn can_retry(&self) -> bool {
        matches!(self, Self::RetryableFailure)
    }

    /// Check if the command is currently executing.
    pub fn is_executing(&self) -> bool {
        matches!(self, Self::Running | Self::Partial)
    }

    /// Get valid next states for this state.
    pub fn valid_transitions(&self) -> Vec<CommandLifecycleState> {
        match self {
            Self::Accepted => vec![Self::Validated, Self::TerminalFailure],
            Self::Validated => vec![Self::Running, Self::TerminalFailure],
            Self::Running => vec![
                Self::Partial,
                Self::Success,
                Self::RetryableFailure,
                Self::TerminalFailure,
            ],
            Self::Partial => vec![
                Self::Running,
                Self::Success,
                Self::RetryableFailure,
                Self::TerminalFailure,
            ],
            Self::RetryableFailure => vec![Self::Running, Self::TerminalFailure],
            Self::Success | Self::TerminalFailure => vec![],
        }
    }

    /// Check if transition to target state is valid.
    pub fn can_transition_to(&self, target: CommandLifecycleState) -> bool {
        self.valid_transitions().contains(&target)
    }
}

impl CommandEnvelope {
    pub fn transition_to(
        &mut self,
        state: CommandLifecycleState,
        error: Option<String>,
    ) -> Result<(), String> {
        if !self.lifecycle_state.can_transition_to(state) {
            return Err(invalid_transition(self.lifecycle_state, state));
        }

        self.lifecycle_state = state;
        self.error_message = error;
        self.last_update_timestamp = now_ms();
        Ok(())
    }
}

impl CommandLifecycleTracker {
    /// Phase 2: Validation - validate the command and transition to Validated or TerminalFailure.
    pub fn validate_command(
        &mut self,
        command_id: u64,
        validation_passed: bool,
    ) -> Result<(), String> {
        let envelope = self
            .envelopes
            .get_mut(&command_id)
            .ok_or_else(|| command_not_found(command_id))?;

        if validation_passed {
            envelope.transition_to(CommandLifecycleState::Validated, None)
        } else {
            envelope.transition_to(
                CommandLifecycleState::TerminalFailure,
                Some("Validation failed".to_string()),
            )
        }
    }

    /// Phase 3: Execution - start command execution.
    pub fn start_execution(&mut self, command_id: u64) -> Result<(), String> {
        let envelope = self
            .envelopes
            .get_mut(&command_id)
            .ok_or_else(|| command_not_found(command_id))?;

        envelope.transition_to(CommandLifecycleState::Running, None)
    }

    /// Phase 4: Result Publication - publish success or failure result.
    pub fn publish_result(
        &mut self,
        command_id: u64,
        success: bool,
        error: Option<String>,
    ) -> Result<(), String> {
        let envelope = self
            .envelopes
            .get_mut(&command_id)
            .ok_or_else(|| command_not_found(command_id))?;

        if success {
            envelope.transition_to(CommandLifecycleState::Success, None)
        } else {
            let state = match &error {
                Some(message) if message.contains("retry") => {
                    CommandLifecycleState::RetryableFailure
                }
                _ => CommandLifecycleState::TerminalFailure,
            };
            envelope.transition_to(state, error)
        }
    }

    /// Retry a command that is in RetryableFailure state.
    pub fn retry_command(&mut self, command_id: u64) -> Result<(), String> {
        let envelope = self
            .envelopes
            .get_mut(&command_id)
            .ok_or_else(|| command_not_found(command_id))?;

        if !envelope.can_retry() {
            return Err(command_not_retryable(command_id, envelope.lifecycle_state));
        }

        envelope.transition_to(CommandLifecycleState::Running, None)
    }
}
