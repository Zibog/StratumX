// Scene Command Handlers - Query Operations

use crate::authoring_runtime::session::EditorAuthoringSession;
use link_egress_observations::*;

pub fn handle_list_entities(
    session: &mut EditorAuthoringSession,
) -> Result<EditorAuthoringObservation, String> {
    if let Some(vs) = &session.vertical_slice_session {
        let scene = vs.authoring_get_scene_summary()?;
        let entities = vec![
            EntityDto {
                entity_id: scene.terrain.entity_id,
                label: "Terrain".to_string(),
                entity_type: "TerrainPatch".to_string(),
                transform: TransformDto {
                    position: scene.terrain.origin,
                    rotation: [0.0, 0.0, 0.0, 1.0],
                    scale: [
                        scene.terrain.world_size[0],
                        1.0,
                        scene.terrain.world_size[1],
                    ],
                },
                asset_id: None,
                material_slots: vec![],
            },
            EntityDto {
                entity_id: scene.wall.entity_id,
                label: "Wall".to_string(),
                entity_type: "Wall".to_string(),
                transform: TransformDto {
                    position: scene.wall.position,
                    rotation: [0.0, 0.0, 0.0, 1.0],
                    scale: scene.wall.dimensions,
                },
                asset_id: None,
                material_slots: vec![MaterialSlotDto {
                    slot_id: 0,
                    slot_name: "Material".to_string(),
                    stack_id: Some(scene.wall.stack_id),
                }],
            },
            EntityDto {
                entity_id: scene.weapon.entity_id,
                label: "Weapon".to_string(),
                entity_type: "Weapon".to_string(),
                transform: TransformDto {
                    position: scene.weapon.position,
                    rotation: [0.0, 0.0, 0.0, 1.0],
                    scale: [1.0, 1.0, 1.0],
                },
                asset_id: None,
                material_slots: vec![],
            },
        ];
        Ok(EditorAuthoringObservation::EntityList { entities })
    } else {
        Err("Runtime session not initialized".into())
    }
}

pub fn handle_get_entity_details(
    session: &mut EditorAuthoringSession,
    entity_id: u32,
) -> Result<EditorAuthoringObservation, String> {
    if let Some(vs) = &session.vertical_slice_session {
        let scene = vs.authoring_get_scene_summary()?;
        let details = match entity_id {
            1 => EntityDetailsDto {
                entity_id: scene.terrain.entity_id,
                label: "Terrain".to_string(),
                entity_type: "TerrainPatch".to_string(),
                transform: TransformDto {
                    position: scene.terrain.origin,
                    rotation: [0.0, 0.0, 0.0, 1.0],
                    scale: [
                        scene.terrain.world_size[0],
                        1.0,
                        scene.terrain.world_size[1],
                    ],
                },
                asset_id: None,
                material_slots: vec![],
                components: vec!["Transform".to_string(), "TerrainPatch".to_string()],
            },
            2 => EntityDetailsDto {
                entity_id: scene.wall.entity_id,
                label: "Wall".to_string(),
                entity_type: "Wall".to_string(),
                transform: TransformDto {
                    position: scene.wall.position,
                    rotation: [0.0, 0.0, 0.0, 1.0],
                    scale: scene.wall.dimensions,
                },
                asset_id: None,
                material_slots: vec![MaterialSlotDto {
                    slot_id: 0,
                    slot_name: "Material".to_string(),
                    stack_id: Some(scene.wall.stack_id),
                }],
                components: vec![
                    "Transform".to_string(),
                    "Wall".to_string(),
                    "MaterialBinding".to_string(),
                ],
            },
            3 => EntityDetailsDto {
                entity_id: scene.weapon.entity_id,
                label: "Weapon".to_string(),
                entity_type: "Weapon".to_string(),
                transform: TransformDto {
                    position: scene.weapon.position,
                    rotation: [0.0, 0.0, 0.0, 1.0],
                    scale: [1.0, 1.0, 1.0],
                },
                asset_id: None,
                material_slots: vec![],
                components: vec!["Transform".to_string(), "Weapon".to_string()],
            },
            _ => {
                return Err(format!(
                    "Entity {} not found in proof-scene subset",
                    entity_id
                ))
            }
        };
        Ok(EditorAuthoringObservation::EntityDetails { details })
    } else {
        Err("Runtime session not initialized".into())
    }
}
