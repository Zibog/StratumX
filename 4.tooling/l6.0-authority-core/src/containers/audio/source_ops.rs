//! Source operations for audio authority.

use super::{AudioAuthorityContainer, AudioSource};

impl AudioAuthorityContainer {
    /// Query audio sources (read-only)
    pub fn query_sources(&self) -> &[AudioSource] {
        &self.registry.sources
    }

    /// Add an audio source to the registry
    /// Called by executor_audio.rs after validation
    ///
    /// **Canonical Route:** AudioExecutor -> ToolingRuntime -> AudioAuthorityContainer::add_source
    pub fn add_source(&mut self, source: AudioSource) -> Result<(), String> {
        if !self.is_initialized() {
            return Err("Authority not initialized".to_string());
        }
        self.registry.sources.push(source);
        Ok(())
    }
}
