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

pub struct MaterialExecutor;

impl MaterialExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(
        &mut self,
        command: PromotedCommand,
        _runtime: &mut ToolingRuntime,
    ) -> Result<Vec<u8>, ToolingError> {
        match command {
            PromotedCommand::MaterialAuthorityInitialize => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::MaterialAuthorityDispose => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::MaterialCreate { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::MaterialDelete { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::MaterialDuplicateProfile { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::MaterialBindVisualResponse { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::MaterialBindAcousticProfile { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::MaterialBindLightResponse { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::MaterialBindMicrodetailProfile { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::MaterialBindWeatherModulation { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::MaterialPreviewBurn { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::MaterialSetCheapRuntimeRung { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::MaterialInspectBranchCoverage { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            _ => Err(ToolingError::Message(format!(
                "unknown material command: {:?}",
                command
            ))),
        }
    }
}

impl Default for MaterialExecutor {
    fn default() -> Self {
        Self::new()
    }
}
