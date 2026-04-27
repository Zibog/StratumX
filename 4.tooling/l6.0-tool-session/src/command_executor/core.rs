// Command Executor Core - struct definition and basic operations

use crate::audio::executor_audio::AudioExecutor;
use crate::build::executor_build::BuildExecutor;
use crate::common::executor_shell::ShellExecutor;
use crate::environment::executor_environment::EnvironmentExecutor;
use crate::material::executor_material::MaterialExecutor;
use crate::runtime::executor_runtime::RuntimeExecutor;
use crate::terrain::executor_terrain::TerrainExecutor;
use crate::world::executor_world::WorldExecutor;
use crate::{ToolingError, ToolingRuntime};
use link_ingress_packets::PacketExecutor;
use serde_json;
use stratumx_tooling_l6_1_command_envelopes::{
    CommandEnvelope, CommandLifecycleState, PromotedCommand,
};

// REAL RUNTIME PATH: CommandExecutor now owns PacketExecutor
// Scene commands go through real execution, not just serialization
// Domain-specific executors handle command routing through tooling L6 planes

pub struct CommandExecutor {
    next_command_id: u64,
    #[allow(dead_code)]
    pub(crate) next_request_id: u64,
    active_envelopes: Vec<CommandEnvelope>,
    pub(crate) packet_executor: PacketExecutor,
    // Domain-specific executors
    pub(crate) world_executor: WorldExecutor,
    pub(crate) runtime_executor: RuntimeExecutor,
    pub(crate) terrain_executor: TerrainExecutor,
    pub(crate) environment_executor: EnvironmentExecutor,
    pub(crate) material_executor: MaterialExecutor,
    pub(crate) audio_executor: AudioExecutor,
    pub(crate) build_executor: BuildExecutor,
    pub(crate) shell_executor: ShellExecutor,
}

impl Default for CommandExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandExecutor {
    pub fn new() -> Self {
        Self {
            next_command_id: 1,
            next_request_id: 1,
            active_envelopes: Vec::new(),
            packet_executor: PacketExecutor::new(),
            world_executor: WorldExecutor::new(),
            runtime_executor: RuntimeExecutor::new(),
            terrain_executor: TerrainExecutor::new(),
            environment_executor: EnvironmentExecutor::new(),
            material_executor: MaterialExecutor::new(),
            audio_executor: AudioExecutor::new(),
            build_executor: BuildExecutor::new(),
            shell_executor: ShellExecutor::new(),
        }
    }

    pub fn packet_executor_mut(&mut self) -> &mut PacketExecutor {
        &mut self.packet_executor
    }

    pub fn has_vertical_slice_session(&self) -> bool {
        self.packet_executor.has_session()
    }

    pub fn initialize_vertical_slice_session(&mut self) -> Result<(), ToolingError> {
        if !self.has_vertical_slice_session() {
            return Err(ToolingError::Message(
                "Vertical slice session must be initialized from tooling layer".into(),
            ));
        }
        Ok(())
    }

    pub fn submit_command(&mut self, command: PromotedCommand) -> Result<u64, ToolingError> {
        let command_id = self.next_command_id;
        self.next_command_id += 1;

        let payload = serde_json::to_vec(&command)
            .map_err(|e| ToolingError::Message(format!("command serialization failed: {}", e)))?;

        let mut envelope = CommandEnvelope::new(command_id, command.route_id(), payload);
        let _ = envelope.transition_to(CommandLifecycleState::Accepted, None);

        self.active_envelopes.push(envelope);
        Ok(command_id)
    }

    pub fn dispatch_command(
        &mut self,
        command_id: u64,
        runtime: &mut ToolingRuntime,
    ) -> Result<Vec<u8>, ToolingError> {
        use super::routing::route_promoted_command;

        let envelope = self
            .active_envelopes
            .iter_mut()
            .find(|e| e.command_id == command_id)
            .ok_or(ToolingError::Message("command not found".into()))?;

        let _ = envelope.transition_to(CommandLifecycleState::Running, None);

        let command: PromotedCommand = serde_json::from_slice(&envelope.payload)
            .map_err(|e| ToolingError::Message(format!("command deserialization failed: {}", e)))?;

        let result = route_promoted_command(self, command, runtime);

        let envelope = self
            .active_envelopes
            .iter_mut()
            .find(|e| e.command_id == command_id)
            .ok_or(ToolingError::Message("command not found".into()))?;

        match &result {
            Ok(_) => {
                let _ = envelope.transition_to(CommandLifecycleState::Success, None);
            }
            Err(e) => {
                let _ = envelope
                    .transition_to(CommandLifecycleState::RetryableFailure, Some(e.to_string()));
            }
        }

        result
    }

    pub fn get_envelope(&self, command_id: u64) -> Option<&CommandEnvelope> {
        self.active_envelopes
            .iter()
            .find(|e| e.command_id == command_id)
    }

    pub fn active_envelopes(&self) -> &[CommandEnvelope] {
        &self.active_envelopes
    }
}
