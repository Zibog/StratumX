//! Profile operations for audio authority.

use super::{AcousticProfile, AudioAuthorityContainer};

impl AudioAuthorityContainer {
    /// Query acoustic profiles (read-only)
    pub fn query_profiles(&self) -> &[AcousticProfile] {
        &self.registry.profiles
    }

    /// Add an acoustic profile to the registry
    /// Called by executor_audio.rs after validation
    pub fn add_profile(&mut self, profile: AcousticProfile) -> Result<(), String> {
        if !self.is_initialized() {
            return Err("Authority not initialized".to_string());
        }
        self.registry.profiles.push(profile);
        Ok(())
    }

    /// Update an existing acoustic profile
    /// Called by executor_audio.rs after validation
    pub fn update_profile<F>(&mut self, profile_id: &str, update_fn: F) -> Result<(), String>
    where
        F: FnOnce(&mut AcousticProfile),
    {
        if !self.is_initialized() {
            return Err("Authority not initialized".to_string());
        }

        if let Some(profile) = self
            .registry
            .profiles
            .iter_mut()
            .find(|p| p.id == profile_id)
        {
            update_fn(profile);
            Ok(())
        } else {
            Err(format!("Profile not found: {}", profile_id))
        }
    }

    /// Remove an acoustic profile from the registry
    /// Called by executor_audio.rs after validation
    pub fn remove_profile(&mut self, profile_id: &str) -> Result<(), String> {
        if !self.is_initialized() {
            return Err("Authority not initialized".to_string());
        }

        let initial_len = self.registry.profiles.len();
        self.registry.profiles.retain(|p| p.id != profile_id);

        if self.registry.profiles.len() == initial_len {
            Err(format!("Profile not found: {}", profile_id))
        } else {
            Ok(())
        }
    }
}
