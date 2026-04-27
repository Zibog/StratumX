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

pub struct EnvironmentExecutor;

impl EnvironmentExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(
        &mut self,
        command: PromotedCommand,
        _runtime: &mut ToolingRuntime,
    ) -> Result<Vec<u8>, ToolingError> {
        match command {
            PromotedCommand::EnvironmentSetTime { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::EnvironmentSetWeather { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::EnvironmentSetCloudCoverage { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::EnvironmentSetFogDensity { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::SkyBindProfile { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::SkySetTimeOfDay { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::SkySetWeatherRegime { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            PromotedCommand::SkyBindCloudProfile { .. } => {
                Ok(serde_json::to_vec(&"executed").unwrap_or_default())
            }
            _ => Err(ToolingError::Message(format!(
                "unknown environment command: {:?}",
                command
            ))),
        }
    }
}

impl Default for EnvironmentExecutor {
    fn default() -> Self {
        Self::new()
    }
}
