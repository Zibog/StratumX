// Animation Command Handlers
// Routes AnimationCommand variants from the SDK to the engine animation runtime.

use super::session::EditorAuthoringSession;
use link_egress_observations::EditorAuthoringObservation;
use link_ingress_packets::AnimationCommand;

pub fn handle(
    _session: &mut EditorAuthoringSession,
    cmd: AnimationCommand,
) -> Result<EditorAuthoringObservation, String> {
    match cmd {
        AnimationCommand::PlayClip {
            entity_id,
            clip_id,
            blend_time,
        } => {
            // Deferred: keep the current observation stub until the animation runtime bridge is restored.
            let _ = (clip_id, blend_time);
            Ok(EditorAuthoringObservation::AnimationClipPlaying {
                entity_id,
                clip_id,
                current_time: 0.0,
                loop_count: 0,
            })
        }
        AnimationCommand::StopClip { entity_id } => {
            // Deferred: keep the current observation stub until the animation runtime bridge is restored.
            Ok(EditorAuthoringObservation::AnimationClipStopped {
                entity_id,
                clip_id: 0,
            })
        }
        AnimationCommand::SetAnimationSpeed { entity_id, speed } => {
            // Deferred: keep the current observation stub until the animation runtime bridge is restored.
            let _ = speed;
            Ok(EditorAuthoringObservation::AnimationClipPlaying {
                entity_id,
                clip_id: 0,
                current_time: 0.0,
                loop_count: 0,
            })
        }
        AnimationCommand::SetAnimationWeight {
            entity_id,
            layer,
            weight,
        } => {
            // Deferred: keep the current observation stub until the animation runtime bridge is restored.
            Ok(EditorAuthoringObservation::AnimationBlendUpdated {
                entity_id,
                layer,
                weight,
            })
        }
        AnimationCommand::SetIKTarget {
            entity_id,
            ik_chain_id,
            position,
            rotation: _rotation,
            weight,
        } => {
            // Deferred: keep the current observation stub until the animation runtime bridge is restored.
            let _ = _rotation;
            Ok(EditorAuthoringObservation::IKTargetUpdated {
                entity_id,
                ik_chain_id,
                position,
                weight,
            })
        }
        AnimationCommand::LoadAnimationClip { clip_id, clip_data } => {
            // Deferred: keep the current observation stub until the animation runtime bridge is restored.
            let _ = clip_data;
            Ok(EditorAuthoringObservation::AnimationClipLoaded {
                clip_id,
                duration: 0.0,
                frame_count: 0,
            })
        }
        AnimationCommand::CreateAnimationStateMachine {
            entity_id,
            states,
            transitions: _transitions,
        } => {
            // Deferred: keep the current observation stub until the animation runtime bridge is restored.
            let _ = _transitions;
            Ok(EditorAuthoringObservation::AnimationStateMachineCreated {
                entity_id,
                state_count: states.len() as u32,
            })
        }
        AnimationCommand::TriggerAnimationEvent {
            entity_id,
            event_name,
        } => {
            // Deferred: keep the current observation stub until the animation runtime bridge is restored.
            Ok(EditorAuthoringObservation::AnimationEventTriggered {
                entity_id,
                event_name,
                timestamp: 0.0,
            })
        }
    }
}
