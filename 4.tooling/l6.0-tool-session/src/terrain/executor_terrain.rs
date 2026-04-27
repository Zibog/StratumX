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

pub struct TerrainExecutor;

impl TerrainExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(
        &mut self,
        command: PromotedCommand,
        _runtime: &mut ToolingRuntime,
    ) -> Result<Vec<u8>, ToolingError> {
        match command {
            PromotedCommand::TerrainImport { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::TerrainRebuild => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::TerrainSculptRaise { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::TerrainSculptLower { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::TerrainSculptSmooth { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::TerrainSculptFlatten { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::TerrainPaintMaterial { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::TerrainSetLayerMaterial { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::TerrainAddHole { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::TerrainRemoveHole { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            _ => Err(ToolingError::Message(format!(
                "unknown terrain command: {:?}",
                command
            ))),
        }
    }
}

impl Default for TerrainExecutor {
    fn default() -> Self {
        Self::new()
    }
}
