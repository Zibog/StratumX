// Scene Command Handlers - Entity Operations

use crate::authoring_runtime::session::EditorAuthoringSession;
use link_egress_observations::*;

pub fn handle_set_transform(
    session: &mut EditorAuthoringSession,
    entity_id: u32,
    transform: TransformDto,
) -> Result<EditorAuthoringObservation, String> {
    if let Some(vs) = &mut session.vertical_slice_session {
        match entity_id {
            2 => {
                vs.authoring_update_wall_position(transform.position)?;
                Ok(EditorAuthoringObservation::EntityTransformUpdated { entity_id })
            }
            3 => {
                vs.authoring_update_weapon_position(transform.position)?;
                Ok(EditorAuthoringObservation::EntityTransformUpdated { entity_id })
            }
            1 => {
                vs.authoring_update_terrain_position(transform.position)?;
                Ok(EditorAuthoringObservation::EntityTransformUpdated { entity_id })
            }
            _ => Err(format!(
                "Entity {} not found in proof-scene subset",
                entity_id
            )),
        }
    } else {
        Err("Runtime session not initialized".into())
    }
}

pub fn handle_delete_entity(
    _session: &mut EditorAuthoringSession,
    _entity_id: u32,
) -> Result<EditorAuthoringObservation, String> {
    Err("DeleteEntity not implemented for proof-scene subset".into())
}
