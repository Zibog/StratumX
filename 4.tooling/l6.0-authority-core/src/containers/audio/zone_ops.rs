//! Zone operations for audio authority.

use super::{AudioAuthorityContainer, AudioZone};

impl AudioAuthorityContainer {
    /// Query audio zones (read-only)
    pub fn query_zones(&self) -> &[AudioZone] {
        &self.registry.zones
    }

    /// Add an audio zone to the registry
    /// Called by executor_audio.rs after validation
    pub fn add_zone(&mut self, zone: AudioZone) -> Result<(), String> {
        if !self.is_initialized() {
            return Err("Authority not initialized".to_string());
        }
        self.registry.zones.push(zone);
        Ok(())
    }

    /// Update an existing audio zone
    /// Called by executor_audio.rs after validation
    pub fn update_zone<F>(&mut self, zone_id: &str, update_fn: F) -> Result<(), String>
    where
        F: FnOnce(&mut AudioZone),
    {
        if !self.is_initialized() {
            return Err("Authority not initialized".to_string());
        }

        if let Some(zone) = self.registry.zones.iter_mut().find(|z| z.id == zone_id) {
            update_fn(zone);
            Ok(())
        } else {
            Err(format!("Zone not found: {}", zone_id))
        }
    }
}
