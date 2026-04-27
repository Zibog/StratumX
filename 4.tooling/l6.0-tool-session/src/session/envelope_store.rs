//! Active command envelope storage.
//!
//! Tracks lifecycle: Accepted → Running → Success / RetryableFailure.

use stratumx_tooling_l6_1_command_envelopes::{CommandEnvelope, CommandLifecycleState};

pub struct EnvelopeStore {
    active: Vec<CommandEnvelope>,
}

impl Default for EnvelopeStore {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvelopeStore {
    pub fn new() -> Self {
        Self { active: Vec::new() }
    }

    pub fn insert(&mut self, envelope: CommandEnvelope) {
        self.active.push(envelope);
    }

    pub fn find_mut(&mut self, command_id: u64) -> Option<&mut CommandEnvelope> {
        self.active.iter_mut().find(|e| e.command_id == command_id)
    }

    pub fn find(&self, command_id: u64) -> Option<&CommandEnvelope> {
        self.active.iter().find(|e| e.command_id == command_id)
    }

    pub fn mark_running(&mut self, command_id: u64) -> Result<(), String> {
        let envelope = self
            .find_mut(command_id)
            .ok_or_else(|| format!("envelope {} not found", command_id))?;
        let _ = envelope.transition_to(CommandLifecycleState::Running, None);
        Ok(())
    }

    pub fn mark_success(&mut self, command_id: u64) -> Result<(), String> {
        let envelope = self
            .find_mut(command_id)
            .ok_or_else(|| format!("envelope {} not found", command_id))?;
        let _ = envelope.transition_to(CommandLifecycleState::Success, None);
        Ok(())
    }

    pub fn mark_failure(&mut self, command_id: u64, error: &str) -> Result<(), String> {
        let envelope = self
            .find_mut(command_id)
            .ok_or_else(|| format!("envelope {} not found", command_id))?;
        let _ = envelope.transition_to(
            CommandLifecycleState::RetryableFailure,
            Some(error.to_string()),
        );
        Ok(())
    }

    pub fn active_envelopes(&self) -> &[CommandEnvelope] {
        &self.active
    }
}
