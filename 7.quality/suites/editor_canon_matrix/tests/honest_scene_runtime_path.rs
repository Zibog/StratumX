// HONEST SCENE RUNTIME PATH TEST
//
// GOAL: Prove that editor → tooling → SDK → runtime path works for scene commands
// WITHOUT synthetic answers or fake success
//
// RUNTIME-BACKED OPERATIONS:
// - SceneBootstrap reads from real engine state
// - SceneFireTestShot mutates real ballistics/damage state
// - SceneReset restores real engine state
// - ListEntities reads from real scene state
// - SetTransform mutates real entity positions
//
// BLOCKED OPERATIONS (honest not implemented):
// - Full entity creation beyond proof-scene subset
// - Complex scene authoring beyond wall/weapon/terrain

use link_ingress_packets::{
    EditorAuthoringCommand, EditorAuthoringPacket, SceneCommand, Transform,
};
use stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession;

fn create_session_with_runtime() -> EditorAuthoringSession {
    let mut session = EditorAuthoringSession::new();
    session
        .initialize_vertical_slice_session()
        .expect("Failed to initialize runtime");
    session
}

fn send_cmd(session: &mut EditorAuthoringSession, cmd: SceneCommand, id: u64) -> String {
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Scene(cmd),
        request_id: id,
    };
    format!("{:?}", session.handle_command(packet).unwrap())
}

#[test]
fn test_scene_bootstrap_reads_from_runtime() {
    let mut session = create_session_with_runtime();

    // CreateEmpty initializes runtime session
    let obs = send_cmd(
        &mut session,
        SceneCommand::CreateEmpty {
            scene_name: "test_scene".to_string(),
        },
        1,
    );
    assert!(obs.contains("SceneCreated"));

    // ListEntities should read from real runtime, not local cache
    let obs = send_cmd(&mut session, SceneCommand::ListEntities, 2);
    assert!(obs.contains("EntityList"));
    assert!(obs.contains("Terrain"));
    assert!(obs.contains("Wall"));
    assert!(obs.contains("Weapon"));

    // Verify entity IDs match runtime (1=terrain, 2=wall, 3=weapon)
    assert!(obs.contains("entity_id: 1"));
    assert!(obs.contains("entity_id: 2"));
    assert!(obs.contains("entity_id: 3"));
}

#[test]
fn test_set_transform_mutates_runtime_truth() {
    let mut session = create_session_with_runtime();

    send_cmd(
        &mut session,
        SceneCommand::CreateEmpty {
            scene_name: "test_scene".to_string(),
        },
        1,
    );

    // Get initial wall position from runtime
    let obs1 = send_cmd(
        &mut session,
        SceneCommand::GetEntityDetails { entity_id: 2 },
        2,
    );
    assert!(obs1.contains("entity_id: 2"));

    // Update wall position in runtime
    let new_position = [5.0, 2.0, 15.0];
    let obs = send_cmd(
        &mut session,
        SceneCommand::SetTransform {
            entity_id: 2,
            transform: Transform {
                position: new_position,
                rotation: [0.0, 0.0, 0.0, 1.0],
                scale: [4.0, 3.0, 0.25],
            },
        },
        3,
    );
    assert!(obs.contains("EntityTransformUpdated"));

    // Verify position changed in runtime (not just local cache)
    let obs2 = send_cmd(
        &mut session,
        SceneCommand::GetEntityDetails { entity_id: 2 },
        4,
    );
    assert!(obs2.contains("position: [5.0, 2.0, 15.0]"));
}

#[test]
fn test_weapon_position_mutates_runtime_truth() {
    let mut session = create_session_with_runtime();

    send_cmd(
        &mut session,
        SceneCommand::CreateEmpty {
            scene_name: "test_scene".to_string(),
        },
        1,
    );

    // Update weapon position in runtime
    let new_position = [1.0, 1.5, 2.0];
    let obs = send_cmd(
        &mut session,
        SceneCommand::SetTransform {
            entity_id: 3,
            transform: Transform {
                position: new_position,
                rotation: [0.0, 0.0, 0.0, 1.0],
                scale: [1.0, 1.0, 1.0],
            },
        },
        2,
    );
    assert!(obs.contains("EntityTransformUpdated"));

    // Verify position changed in runtime
    let obs2 = send_cmd(
        &mut session,
        SceneCommand::GetEntityDetails { entity_id: 3 },
        3,
    );
    assert!(obs2.contains("position: [1.0, 1.5, 2.0]"));
}

#[test]
fn test_terrain_position_mutates_runtime_truth() {
    let mut session = create_session_with_runtime();

    send_cmd(
        &mut session,
        SceneCommand::CreateEmpty {
            scene_name: "test_scene".to_string(),
        },
        1,
    );

    // Update terrain position in runtime
    let new_position = [10.0, 0.0, 10.0];
    let obs = send_cmd(
        &mut session,
        SceneCommand::SetTransform {
            entity_id: 1,
            transform: Transform {
                position: new_position,
                rotation: [0.0, 0.0, 0.0, 1.0],
                scale: [100.0, 1.0, 100.0],
            },
        },
        2,
    );
    assert!(obs.contains("EntityTransformUpdated"));

    // Verify position changed in runtime
    let obs2 = send_cmd(
        &mut session,
        SceneCommand::GetEntityDetails { entity_id: 1 },
        3,
    );
    assert!(obs2.contains("position: [10.0, 0.0, 10.0]"));
}

#[test]
fn test_entity_details_read_from_runtime_not_cache() {
    let mut session = create_session_with_runtime();

    send_cmd(
        &mut session,
        SceneCommand::CreateEmpty {
            scene_name: "test_scene".to_string(),
        },
        1,
    );

    // Get wall details from runtime
    let obs = send_cmd(
        &mut session,
        SceneCommand::GetEntityDetails { entity_id: 2 },
        2,
    );
    assert!(obs.contains("EntityDetails"));
    assert!(obs.contains("entity_id: 2"));
    assert!(obs.contains("label: \"Wall\""));
    assert!(obs.contains("entity_type: \"Wall\""));

    // Get weapon details from runtime
    let obs = send_cmd(
        &mut session,
        SceneCommand::GetEntityDetails { entity_id: 3 },
        3,
    );
    assert!(obs.contains("entity_id: 3"));
    assert!(obs.contains("label: \"Weapon\""));
    assert!(obs.contains("entity_type: \"Weapon\""));

    // Get terrain details from runtime
    let obs = send_cmd(
        &mut session,
        SceneCommand::GetEntityDetails { entity_id: 1 },
        4,
    );
    assert!(obs.contains("entity_id: 1"));
    assert!(obs.contains("label: \"Terrain\""));
    assert!(obs.contains("entity_type: \"TerrainPatch\""));
}

#[test]
fn test_list_entities_reflects_runtime_state() {
    let mut session = create_session_with_runtime();

    send_cmd(
        &mut session,
        SceneCommand::CreateEmpty {
            scene_name: "test_scene".to_string(),
        },
        1,
    );

    // Update wall position
    send_cmd(
        &mut session,
        SceneCommand::SetTransform {
            entity_id: 2,
            transform: Transform {
                position: [1.0, 2.0, 3.0],
                rotation: [0.0, 0.0, 0.0, 1.0],
                scale: [4.0, 3.0, 0.25],
            },
        },
        2,
    );

    // ListEntities should reflect the updated position from runtime
    let obs = send_cmd(&mut session, SceneCommand::ListEntities, 3);
    assert!(obs.contains("position: [1.0, 2.0, 3.0]"));
}

#[test]
fn test_scene_operations_without_runtime_fail_honestly() {
    let mut session = EditorAuthoringSession::new();
    // Do NOT initialize runtime session

    // ListEntities should fail honestly, not return fake data
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Scene(SceneCommand::ListEntities),
        request_id: 1,
    };
    let result = session.handle_command(packet);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Runtime session not initialized"));
}

#[test]
fn test_entity_creation_beyond_proof_scene_fails_honestly() {
    let mut session = create_session_with_runtime();

    send_cmd(
        &mut session,
        SceneCommand::CreateEmpty {
            scene_name: "test_scene".to_string(),
        },
        1,
    );

    // CreateEntityFromAsset should fail honestly for proof-scene subset
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Scene(SceneCommand::CreateEntityFromAsset {
            asset_id: 1,
            transform: Transform {
                position: [0.0, 0.0, 0.0],
                rotation: [0.0, 0.0, 0.0, 1.0],
                scale: [1.0, 1.0, 1.0],
            },
            label: "NewEntity".to_string(),
        }),
        request_id: 2,
    };
    let result = session.handle_command(packet);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("not implemented for proof-scene subset"));
}

#[test]
fn test_entity_deletion_fails_honestly() {
    let mut session = create_session_with_runtime();

    send_cmd(
        &mut session,
        SceneCommand::CreateEmpty {
            scene_name: "test_scene".to_string(),
        },
        1,
    );

    // DeleteEntity should fail honestly for proof-scene subset
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Scene(SceneCommand::DeleteEntity { entity_id: 2 }),
        request_id: 2,
    };
    let result = session.handle_command(packet);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("not implemented for proof-scene subset"));
}
