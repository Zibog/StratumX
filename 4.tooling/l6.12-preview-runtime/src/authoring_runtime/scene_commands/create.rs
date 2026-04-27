// Scene Command Handlers - Creation Operations

use crate::authoring_runtime::session::EditorAuthoringSession;
use link_egress_observations::*;

pub fn handle_create_empty(
    session: &mut EditorAuthoringSession,
    scene_name: String,
) -> Result<EditorAuthoringObservation, String> {
    // Set the active scene to the requested name before initializing
    session.active_scene = Some(scene_name.clone());
    session.initialize_vertical_slice_session()?;

    let _scene_dto = if let Some(vs) = &session.vertical_slice_session {
        vs.authoring_get_scene_summary()?
    } else {
        return Err("Runtime session not initialized".into());
    };

    Ok(EditorAuthoringObservation::SceneCreated {
        scene: AuthoringSceneDto {
            scene_name: session.active_scene.clone().unwrap_or(scene_name),
            entities: vec![],
        },
    })
}

pub fn handle_create_entity_from_asset(
    _session: &mut EditorAuthoringSession,
    _asset_id: String,
    _transform: TransformDto,
    _label: String,
) -> Result<EditorAuthoringObservation, String> {
    Err("CreateEntityFromAsset not implemented for proof-scene subset".into())
}
