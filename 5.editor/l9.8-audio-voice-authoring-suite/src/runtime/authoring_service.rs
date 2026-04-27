//! Audio Authoring Service - owns audio truth.
//!
//! Provides the operational interface for managing audio sources, zones, and policies.

use crate::model::{AudioRegistry, AudioSource, AudioZone, DuckingPolicy, ObjectHandle};
use std::collections::HashMap;

use super::binding_ops;

/// Source identifier (UUID-based).
pub type SourceId = ObjectHandle;

/// Audio Authoring Service - owns audio truth.
///
/// **Requirements: 6.2, 6.5**
/// - Owns audio source registry (not UI)
/// - Owns sound profile bindings (not UI)
pub struct AudioAuthoringService {
    /// Audio source registry - authoritative audio configuration.
    source_registry: AudioRegistry,

    /// Sound profile bindings - which sources use which sound profiles.
    sound_profile_bindings: HashMap<SourceId, uuid::Uuid>,
}

impl AudioAuthoringService {
    /// Create a new audio authoring service.
    pub fn new() -> Self {
        Self {
            source_registry: AudioRegistry::new(),
            sound_profile_bindings: HashMap::new(),
        }
    }

    /// Register a new audio source.
    ///
    /// **Requirement 6.5**: Service owns audio source registry, not UI.
    pub fn register_audio_source(&mut self, name: String, position: [f32; 3]) -> SourceId {
        self.source_registry.create_source(name, position)
    }

    /// Bind a sound profile to an audio source.
    ///
    /// **Requirement 6.5**: Service owns sound profile bindings, not UI.
    pub fn bind_sound_profile(
        &mut self,
        source_id: SourceId,
        profile_id: uuid::Uuid,
    ) -> Result<(), String> {
        binding_ops::bind_sound_profile(
            &mut self.sound_profile_bindings,
            &self.source_registry,
            source_id,
            profile_id,
        )
    }

    /// Get a reference to the audio source registry.
    ///
    /// **Requirement 6.6**: UI reads from service.
    pub fn get_registry(&self) -> &AudioRegistry {
        &self.source_registry
    }

    /// Get a reference to the sound profile bindings.
    pub fn get_sound_profile_bindings(&self) -> &HashMap<SourceId, uuid::Uuid> {
        &self.sound_profile_bindings
    }

    /// Get the sound profile bound to a specific source.
    pub fn get_source_sound_profile(&self, source_id: SourceId) -> Option<uuid::Uuid> {
        self.sound_profile_bindings.get(&source_id).copied()
    }

    /// Unbind a sound profile from a source.
    pub fn unbind_sound_profile(&mut self, source_id: SourceId) -> Option<uuid::Uuid> {
        self.sound_profile_bindings.remove(&source_id)
    }

    /// Assign an emitter class to an audio source.
    pub fn assign_emitter_class(
        &mut self,
        source_id: SourceId,
        emitter_class: String,
    ) -> Result<(), String> {
        binding_ops::assign_emitter_class(&mut self.source_registry, source_id, emitter_class)
    }

    /// Create a new audio zone.
    pub fn create_zone(&mut self, name: String, indoor: bool) -> ObjectHandle {
        self.source_registry.create_zone(name, indoor)
    }

    /// Bind a reverb profile to a zone.
    pub fn bind_zone_reverb_profile(
        &mut self,
        zone_id: ObjectHandle,
        reverb_profile: String,
    ) -> Result<(), String> {
        binding_ops::bind_zone_reverb_profile(&mut self.source_registry, zone_id, reverb_profile)
    }

    /// Create a ducking policy.
    pub fn create_ducking_policy(
        &mut self,
        name: String,
        priority_levels: Vec<u32>,
        duck_amount: u32,
    ) -> ObjectHandle {
        binding_ops::create_ducking_policy(
            &mut self.source_registry,
            name,
            priority_levels,
            duck_amount,
        )
    }

    /// Get an audio source by ID.
    pub fn get_source(&self, source_id: SourceId) -> Option<&AudioSource> {
        self.source_registry.get_source(source_id)
    }

    /// Get an audio zone by ID.
    pub fn get_zone(&self, zone_id: ObjectHandle) -> Option<&AudioZone> {
        self.source_registry.get_zone(zone_id)
    }

    /// Get a ducking policy by ID.
    pub fn get_policy(&self, policy_id: ObjectHandle) -> Option<&DuckingPolicy> {
        self.source_registry.get_policy(policy_id)
    }

    /// Get all audio sources.
    pub fn get_all_sources(&self) -> Vec<&AudioSource> {
        self.source_registry.sources.values().collect()
    }

    /// Get all audio zones.
    pub fn get_all_zones(&self) -> Vec<&AudioZone> {
        self.source_registry.zones.values().collect()
    }

    /// Get all ducking policies.
    pub fn get_all_policies(&self) -> Vec<&DuckingPolicy> {
        self.source_registry.policies.values().collect()
    }

    /// Check if the service is initialized (has any sources or zones).
    pub fn is_initialized(&self) -> bool {
        !self.source_registry.sources.is_empty() || !self.source_registry.zones.is_empty()
    }
}

impl Default for AudioAuthoringService {
    fn default() -> Self {
        Self::new()
    }
}
