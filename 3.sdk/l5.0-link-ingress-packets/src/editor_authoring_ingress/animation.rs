use serde::{Deserialize, Serialize};

// ============================================================================
// ANIMATION COMMANDS
// ============================================================================

/// Commands for controlling animation playback, IK, state machines, and clip loading.
/// Flows from editor authoring tools to the engine animation system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimationCommand {
    /// Start playing an animation clip on an entity with optional blending.
    PlayClip {
        entity_id: u32,
        clip_id: u32,
        blend_time: f32,
    },
    /// Stop the currently playing animation clip on an entity.
    StopClip {
        entity_id: u32,
    },
    /// Set the playback speed multiplier for an entity's animation.
    SetAnimationSpeed {
        entity_id: u32,
        speed: f32,
    },
    /// Set the blend weight for an animation layer on an entity.
    SetAnimationWeight {
        entity_id: u32,
        layer: u8,
        weight: f32,
    },
    /// Update an IK target position, rotation, and weight for an IK chain.
    SetIKTarget {
        entity_id: u32,
        ik_chain_id: u16,
        position: [f32; 3],
        rotation: [f32; 4],
        weight: f32,
    },
    /// Load an animation clip into the engine from serialized clip data.
    LoadAnimationClip {
        clip_id: u32,
        clip_data: Vec<u8>,
    },
    /// Create an animation state machine on an entity with states and transitions.
    CreateAnimationStateMachine {
        entity_id: u32,
        states: Vec<AnimationStateDef>,
        transitions: Vec<AnimationTransitionDef>,
    },
    /// Trigger a named event in an entity's animation state machine.
    TriggerAnimationEvent {
        entity_id: u32,
        event_name: String,
    },
}

/// Definition of a single state within an animation state machine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationStateDef {
    pub state_id: u16,
    pub label: String,
    pub clip_id: u32,
    pub loop_enabled: bool,
}

/// Definition of a transition between two animation states.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationTransitionDef {
    pub from_state_id: u16,
    pub to_state_id: u16,
    pub trigger_event: String,
    pub blend_duration: f32,
}

/// Envelope packet carrying an animation command through the SDK ingress link.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationIngressPacket {
    pub command: AnimationCommand,
    pub request_id: u64,
}

impl AnimationIngressPacket {
    /// Construct a PlayClip packet.
    pub fn play_clip(request_id: u64, entity_id: u32, clip_id: u32, blend_time: f32) -> Self {
        Self {
            command: AnimationCommand::PlayClip {
                entity_id,
                clip_id,
                blend_time,
            },
            request_id,
        }
    }

    /// Construct a StopClip packet.
    pub fn stop_clip(request_id: u64, entity_id: u32) -> Self {
        Self {
            command: AnimationCommand::StopClip { entity_id },
            request_id,
        }
    }

    /// Construct a SetAnimationSpeed packet.
    pub fn set_animation_speed(request_id: u64, entity_id: u32, speed: f32) -> Self {
        Self {
            command: AnimationCommand::SetAnimationSpeed { entity_id, speed },
            request_id,
        }
    }

    /// Construct a SetAnimationWeight packet.
    pub fn set_animation_weight(
        request_id: u64,
        entity_id: u32,
        layer: u8,
        weight: f32,
    ) -> Self {
        Self {
            command: AnimationCommand::SetAnimationWeight {
                entity_id,
                layer,
                weight,
            },
            request_id,
        }
    }

    /// Construct a SetIKTarget packet.
    pub fn set_ik_target(
        request_id: u64,
        entity_id: u32,
        ik_chain_id: u16,
        position: [f32; 3],
        rotation: [f32; 4],
        weight: f32,
    ) -> Self {
        Self {
            command: AnimationCommand::SetIKTarget {
                entity_id,
                ik_chain_id,
                position,
                rotation,
                weight,
            },
            request_id,
        }
    }

    /// Construct a LoadAnimationClip packet.
    pub fn load_animation_clip(request_id: u64, clip_id: u32, clip_data: Vec<u8>) -> Self {
        Self {
            command: AnimationCommand::LoadAnimationClip { clip_id, clip_data },
            request_id,
        }
    }

    /// Construct a CreateAnimationStateMachine packet.
    pub fn create_animation_state_machine(
        request_id: u64,
        entity_id: u32,
        states: Vec<AnimationStateDef>,
        transitions: Vec<AnimationTransitionDef>,
    ) -> Self {
        Self {
            command: AnimationCommand::CreateAnimationStateMachine {
                entity_id,
                states,
                transitions,
            },
            request_id,
        }
    }

    /// Construct a TriggerAnimationEvent packet.
    pub fn trigger_animation_event(
        request_id: u64,
        entity_id: u32,
        event_name: String,
    ) -> Self {
        Self {
            command: AnimationCommand::TriggerAnimationEvent {
                entity_id,
                event_name,
            },
            request_id,
        }
    }
}
