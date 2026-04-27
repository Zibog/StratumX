use super::{
    AcousticProfile, AcousticProfileId, AudioSource, AudioSourceId, AudioZone, AudioZoneId,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Audio registry state container
///
/// Owns audio-level state including sources, zones, and acoustic profiles.
/// All audio data comes from Audio_Registry, never generated at runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRegistryState {
    /// Audio sources indexed by source ID
    pub audio_sources: HashMap<AudioSourceId, AudioSource>,

    /// Audio zones indexed by zone ID
    pub audio_zones: HashMap<AudioZoneId, AudioZone>,

    /// Acoustic profiles indexed by profile ID
    pub acoustic_profiles: HashMap<AcousticProfileId, AcousticProfile>,
}

impl AudioRegistryState {
    /// Creates a new empty audio registry state
    pub fn new() -> Self {
        Self {
            audio_sources: HashMap::new(),
            audio_zones: HashMap::new(),
            acoustic_profiles: HashMap::new(),
        }
    }

    /// Adds or updates an audio source
    pub fn add_source(&mut self, source: AudioSource) {
        self.audio_sources.insert(source.source_id.clone(), source);
    }

    /// Removes an audio source
    pub fn remove_source(&mut self, source_id: &AudioSourceId) {
        self.audio_sources.remove(source_id);
    }

    /// Gets an audio source by ID
    pub fn get_source(&self, source_id: &AudioSourceId) -> Option<&AudioSource> {
        self.audio_sources.get(source_id)
    }

    /// Adds or updates an audio zone
    pub fn add_zone(&mut self, zone: AudioZone) {
        self.audio_zones.insert(zone.zone_id.clone(), zone);
    }

    /// Removes an audio zone
    pub fn remove_zone(&mut self, zone_id: &AudioZoneId) {
        self.audio_zones.remove(zone_id);
    }

    /// Gets an audio zone by ID
    pub fn get_zone(&self, zone_id: &AudioZoneId) -> Option<&AudioZone> {
        self.audio_zones.get(zone_id)
    }

    /// Adds or updates an acoustic profile
    pub fn add_profile(&mut self, profile: AcousticProfile) {
        self.acoustic_profiles
            .insert(profile.profile_id.clone(), profile);
    }

    /// Gets an acoustic profile by ID
    pub fn get_profile(&self, profile_id: &AcousticProfileId) -> Option<&AcousticProfile> {
        self.acoustic_profiles.get(profile_id)
    }
}

impl Default for AudioRegistryState {
    fn default() -> Self {
        Self::new()
    }
}
