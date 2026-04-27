// Actor Command Handlers

use super::session::EditorAuthoringSession;
use link_egress_observations::*;
use link_ingress_packets::ActorCommand;

pub fn handle(
    session: &mut EditorAuthoringSession,
    cmd: ActorCommand,
) -> Result<EditorAuthoringObservation, String> {
    match cmd {
        ActorCommand::SpawnPreset {
            preset_id,
            transform,
        } => {
            let actor_id = session.next_actor_id;
            session.next_actor_id += 1;

            let actor = ActorDto {
                actor_id,
                preset_id,
                transform: TransformDto {
                    position: transform.position,
                    rotation: transform.rotation,
                    scale: transform.scale,
                },
                attached_weapon: None,
            };

            session.actors.insert(actor_id, actor.clone());
            Ok(EditorAuthoringObservation::ActorSpawned { actor })
        }
        ActorCommand::ListPresets => {
            let presets = vec![
                ActorPresetDto {
                    preset_id: 1,
                    label: "First Person Character".to_string(),
                    actor_type: "FPS".to_string(),
                },
                ActorPresetDto {
                    preset_id: 2,
                    label: "Third Person Character".to_string(),
                    actor_type: "TPS".to_string(),
                },
            ];
            Ok(EditorAuthoringObservation::ActorPresetList { presets })
        }
        ActorCommand::AttachWeapon {
            actor_id,
            weapon_profile_id,
        } => {
            if let Some(actor) = session.actors.get_mut(&actor_id) {
                actor.attached_weapon = Some(weapon_profile_id);
                Ok(EditorAuthoringObservation::WeaponAttached {
                    actor_id,
                    weapon_profile_id,
                })
            } else {
                Err(format!("Actor {} not found", actor_id))
            }
        }
        ActorCommand::SetActive { actor_id } => {
            if session.actors.contains_key(&actor_id) {
                session.active_actor = Some(actor_id);
                Ok(EditorAuthoringObservation::ActiveActorSet { actor_id })
            } else {
                Err(format!("Actor {} not found", actor_id))
            }
        }
        ActorCommand::GetActive => {
            let actor = session
                .active_actor
                .and_then(|id| session.actors.get(&id).cloned());
            Ok(EditorAuthoringObservation::ActiveActorInfo { actor })
        }
    }
}
