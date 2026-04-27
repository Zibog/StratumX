// Command lifecycle state machine
//
// Represents the current stage of command execution.
// Transitions follow a strict state machine:
// - Accepted -> Validated -> Running -> (Success | RetryableFailure | TerminalFailure)
// - RetryableFailure can transition back to Running

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandLifecycleState {
    /// Command has been accepted but not yet validated
    Accepted,
    /// Command has passed validation and is ready for execution
    Validated,
    /// Command is currently executing
    Running,
    /// Command execution is partially complete (for multi-stage commands)
    Partial,
    /// Command completed successfully
    Success,
    /// Command failed but can be retried
    RetryableFailure,
    /// Command failed and cannot be retried
    TerminalFailure,
}

impl CommandLifecycleState {
    /// Check if the state is terminal (no further transitions possible)
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Success | Self::TerminalFailure)
    }

    /// Check if the state allows retry
    pub fn can_retry(&self) -> bool {
        matches!(self, Self::RetryableFailure)
    }

    /// Check if the command is currently executing
    pub fn is_executing(&self) -> bool {
        matches!(self, Self::Running | Self::Partial)
    }

    /// Get valid next states for this state
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

    /// Check if transition to target state is valid
    pub fn can_transition_to(&self, target: CommandLifecycleState) -> bool {
        self.valid_transitions().contains(&target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifecycle_state_transitions() {
        assert!(CommandLifecycleState::Accepted.can_transition_to(CommandLifecycleState::Validated));
        assert!(CommandLifecycleState::Validated.can_transition_to(CommandLifecycleState::Running));
        assert!(CommandLifecycleState::Running.can_transition_to(CommandLifecycleState::Success));
        assert!(!CommandLifecycleState::Success.can_transition_to(CommandLifecycleState::Running));
    }

    #[test]
    fn test_lifecycle_state_terminal() {
        assert!(CommandLifecycleState::Success.is_terminal());
        assert!(CommandLifecycleState::TerminalFailure.is_terminal());
        assert!(!CommandLifecycleState::Running.is_terminal());
    }

    #[test]
    fn test_lifecycle_state_retry() {
        assert!(CommandLifecycleState::RetryableFailure.can_retry());
        assert!(!CommandLifecycleState::TerminalFailure.can_retry());
    }
}
