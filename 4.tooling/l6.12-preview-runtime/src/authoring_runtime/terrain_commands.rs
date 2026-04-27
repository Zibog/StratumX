// Terrain Command Handlers

use super::session::EditorAuthoringSession;
use link_egress_observations::*;
use link_ingress_packets::TerrainCommand;

pub fn handle(
    session: &mut EditorAuthoringSession,
    cmd: TerrainCommand,
) -> Result<EditorAuthoringObservation, String> {
    match cmd {
        TerrainCommand::CreatePatch { input: _ } => {
            Err("CreatePatch not implemented for proof-scene subset".into())
        }
        TerrainCommand::ListPatches => {
            if let Some(vs) = &session.vertical_slice_session {
                let scene = vs.authoring_get_scene_summary()?;
                let patches = vec![AuthoringTerrainPatchDto {
                    patch_id: scene.terrain.entity_id,
                    label: "Terrain".to_string(),
                    position: scene.terrain.origin,
                    size: scene.terrain.world_size,
                    surface_regions: vec![],
                }];
                Ok(EditorAuthoringObservation::TerrainPatchList { patches })
            } else {
                Err("Runtime session not initialized".into())
            }
        }
        TerrainCommand::PaintSurfaceStack {
            target_entity_id: _,
            paint: _,
        } => Err("PaintSurfaceStack not implemented for proof-scene subset".into()),
        TerrainCommand::GetPatchDetails { patch_id } => {
            if let Some(vs) = &session.vertical_slice_session {
                let scene = vs.authoring_get_scene_summary()?;
                if patch_id == scene.terrain.entity_id {
                    let patch = AuthoringTerrainPatchDto {
                        patch_id: scene.terrain.entity_id,
                        label: "Terrain".to_string(),
                        position: scene.terrain.origin,
                        size: scene.terrain.world_size,
                        surface_regions: vec![],
                    };
                    Ok(EditorAuthoringObservation::TerrainPatchDetails { patch })
                } else {
                    Err(format!(
                        "Patch {} not found in proof-scene subset",
                        patch_id
                    ))
                }
            } else {
                Err("Runtime session not initialized".into())
            }
        }
    }
}
