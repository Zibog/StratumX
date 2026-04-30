use super::state::CommandLifecycleState;

pub(crate) fn invalid_transition(from: CommandLifecycleState, to: CommandLifecycleState) -> String {
    format!("Invalid state transition from {:?} to {:?}", from, to)
}

pub(crate) fn command_not_found(command_id: u64) -> String {
    format!("Command {} not found", command_id)
}

pub(crate) fn command_not_retryable(command_id: u64, state: CommandLifecycleState) -> String {
    format!(
        "Command {} cannot be retried (state: {:?})",
        command_id, state
    )
}
