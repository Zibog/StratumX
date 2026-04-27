//! Binding operations for audio authoring service.
//!
//! Contains methods for binding sound profiles, assigning emitter classes,
//! binding zone reverb profiles, and creating ducking policies.

use crate::model::{AudioRegistry, ObjectHandle};
use std::collections::HashMap;

/// Bind a sound profile to an audio source.
pub fn bind_sound_profile(
    sound_profile_bindings: &mut HashMap<ObjectHandle, uuid::Uuid>,
    registry: &AudioRegistry,
    source_id: ObjectHandle,
    profile_id: uuid::Uuid,
) -> Result<(), String> {
    if registry.get_source(source_id).is_none() {
        return Err("Audio source not found".to_string());
    }

    sound_profile_bindings.insert(source_id, profile_id);
    Ok(())
}

/// Assign an emitter class to an audio source.
pub fn assign_emitter_class(
    registry: &mut AudioRegistry,
    source_id: ObjectHandle,
    emitter_class: String,
) -> Result<(), String> {
    registry.assign_emitter_class(source_id, emitter_class)
}

/// Bind a reverb profile to a zone.
pub fn bind_zone_reverb_profile(
    registry: &mut AudioRegistry,
    zone_id: ObjectHandle,
    reverb_profile: String,
) -> Result<(), String> {
    registry.bind_zone_reverb_profile(zone_id, reverb_profile)
}

/// Create a ducking policy.
pub fn create_ducking_policy(
    registry: &mut AudioRegistry,
    name: String,
    priority_levels: Vec<u32>,
    duck_amount: u32,
) -> ObjectHandle {
    registry.create_ducking_policy(name, priority_levels, duck_amount)
}
