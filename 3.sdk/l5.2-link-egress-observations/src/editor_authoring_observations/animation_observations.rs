use serde::{Deserialize, Serialize};

/// Observations flowing from the engine animation system back to the editor authoring tools.
/// Each variant represents a distinct animation-related fact emitted by the engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AnimationObservation {
    /// An animation clip has been successfully loaded into the engine.
    AnimationClipLoaded {
        clip_id: u32,
        duration: f32,
        frame_count: u32,
    },
    /// An animation clip is currently playing on an entity.
    AnimationClipPlaying {
        entity_id: u32,
        clip_id: u32,
        current_time: f32,
        loop_count: u32,
    },
    /// An animation clip has stopped playing on an entity.
    AnimationClipStopped {
        entity_id: u32,
        clip_id: u32,
    },
    /// The blend weight for an animation layer on an entity has been updated.
    AnimationBlendUpdated {
        entity_id: u32,
        layer: u8,
        weight: f32,
    },
    /// An IK target has been updated for a specific IK chain on an entity.
    IKTargetUpdated {
        entity_id: u32,
        ik_chain_id: u16,
        position: [f32; 3],
        weight: f32,
    },
    /// An animation event has been triggered during playback.
    AnimationEventTriggered {
        entity_id: u32,
        event_name: String,
        timestamp: f32,
    },
    /// An animation state machine has been created on an entity.
    AnimationStateMachineCreated {
        entity_id: u32,
        state_count: u32,
    },
    /// An error occurred within the animation system for an entity.
    AnimationError {
        entity_id: u32,
        error_message: String,
    },
}
