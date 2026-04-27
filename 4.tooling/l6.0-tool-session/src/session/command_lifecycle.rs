//! Command lifecycle transition rules.
//!
//! Accepted → Running → Success
//! Accepted → Running → RetryableFailure

use stratumx_tooling_l6_1_command_envelopes::CommandLifecycleState;

pub struct CommandLifecycle;

impl CommandLifecycle {
    pub fn is_terminal(state: &CommandLifecycleState) -> bool {
        matches!(
            state,
            CommandLifecycleState::Success | CommandLifecycleState::RetryableFailure
        )
    }

    pub fn is_failure(state: &CommandLifecycleState) -> bool {
        matches!(state, CommandLifecycleState::RetryableFailure)
    }
}
