use crate::common::context::ToolSessionContext;
use crate::common::result::CommandResult;
use crate::{ToolingError, ToolingRuntime};
use serde_json;
use stratumx_tooling_l6_1_command_envelopes::{
    CanonicalCommandEnvelope, CommandPayload, PromotedCommand,
};

/// Compatibility shim for CanonicalCommandEnvelope-based callers
pub fn execute(
    envelope: &CanonicalCommandEnvelope<CommandPayload>,
    ctx: &mut ToolSessionContext,
) -> CommandResult {
    CommandResult::from_route_metadata(&envelope.route_metadata, ctx)
}

pub struct WorldExecutor;

impl Default for WorldExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl WorldExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(
        &mut self,
        command: PromotedCommand,
        _runtime: &mut ToolingRuntime,
    ) -> Result<Vec<u8>, ToolingError> {
        match command {
            PromotedCommand::WorldOpen { world_path } => {
                Ok(serde_json::to_vec(&format!("world_open: {}", world_path)).unwrap_or_default())
            }
            PromotedCommand::WorldSave { world_path } => {
                Ok(serde_json::to_vec(&format!("world_save: {}", world_path)).unwrap_or_default())
            }
            PromotedCommand::WorldClose => {
                Ok(serde_json::to_vec(&"world_closed").unwrap_or_default())
            }
            _ => Err(ToolingError::Message(format!(
                "unknown world command: {:?}",
                command
            ))),
        }
    }
}
