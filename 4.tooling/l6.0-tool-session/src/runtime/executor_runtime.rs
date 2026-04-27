use crate::{ToolingRuntime, ToolingError};
use crate::common::context::ToolSessionContext;
use crate::common::result::CommandResult;
use stratumx_tooling_l6_1_command_envelopes::{PromotedCommand, CanonicalCommandEnvelope, CommandPayload};
use serde_json;

/// Compatibility shim for CanonicalCommandEnvelope-based callers
pub fn execute(
    envelope: &CanonicalCommandEnvelope<CommandPayload>,
    ctx: &mut ToolSessionContext,
) -> CommandResult {
    CommandResult::from_route_metadata(&envelope.route_metadata, ctx)
}

pub struct RuntimeExecutor;

impl RuntimeExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(
        &mut self,
        command: PromotedCommand,
        _runtime: &mut ToolingRuntime,
    ) -> Result<Vec<u8>, ToolingError> {
        match command {
            PromotedCommand::RuntimePlay => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::RuntimePause => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::RuntimeStop => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::RuntimeSimulate => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            _ => Err(ToolingError::Message(format!("unknown runtime command: {:?}", command))),
        }
    }
}

impl Default for RuntimeExecutor {
    fn default() -> Self {
        Self::new()
    }
}
