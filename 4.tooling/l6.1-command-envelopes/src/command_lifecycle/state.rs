use serde::{Deserialize, Serialize};

/// Represents the current stage of command execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandLifecycleState {
    /// Command has been accepted but not yet validated.
    Accepted,
    /// Command has passed validation and is ready for execution.
    Validated,
    /// Command is currently executing.
    Running,
    /// Command execution is partially complete (for multi-stage commands).
    Partial,
    /// Command completed successfully.
    Success,
    /// Command failed but can be retried.
    RetryableFailure,
    /// Command failed and cannot be retried.
    TerminalFailure,
}
