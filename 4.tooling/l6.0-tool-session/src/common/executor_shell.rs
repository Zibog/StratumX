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

pub struct ShellExecutor;

impl ShellExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(
        &mut self,
        command: PromotedCommand,
        _runtime: &mut ToolingRuntime,
    ) -> Result<Vec<u8>, ToolingError> {
        match command {
            PromotedCommand::ProjectBootstrap { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::ProjectCreate { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::ProjectSave { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::ProjectBuild { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::ProjectExport { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::ProjectLaunch { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::ProjectVerifyFirstResult => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::ShellActivateViewport => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::ShellActivateOutliner => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::ShellActivateInspector => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::ShellActivateContentBrowser => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::ShellActivateMaterialLab => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::ShellActivateTerrainLab => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::ShellActivateSkyLab => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            _ => Err(ToolingError::Message(format!(
                "unknown shell command: {:?}",
                command
            ))),
        }
    }
}

impl Default for ShellExecutor {
    fn default() -> Self {
        Self::new()
    }
}
