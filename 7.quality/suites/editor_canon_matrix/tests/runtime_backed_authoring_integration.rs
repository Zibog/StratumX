// RUNTIME-BACKED AUTHORING INTEGRATION TEST (Pack 2)
// Verifies that editor authoring operations mutate real runtime truth,
// not just local HashMap state

use link_ingress_packets::{
    EditorAuthoringCommand, EditorAuthoringPacket, MaterialCommand, SceneCommand, TerrainCommand,
    Transform,
};
use stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession;

#[test]
fn authoring_scene_initialization_creates_runtime_session() {
    let mut session = EditorAuthoringSession::new();
    assert!(!session.has_runtime_session());

    // Initialize scene - should create runtime-backed session
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Scene(SceneCommand::CreateEmpty {
            scene_name: "test_scene".to_string(),
        }),
        request_id: 1,
    };

    let result = session.handle_command(packet);
    assert!(
        result.is_ok(),
        "Scene initialization should succeed: {:?}",
        result
    );
    assert!(
        session.has_runtime_session(),
        "Runtime session should be initialized"
    );
}

#[test]
fn authoring_list_entities_returns_runtime_state() {
    let mut session = EditorAuthoringSession::new();

    // Initialize scene
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Scene(SceneCommand::CreateEmpty {
                scene_name: "test_scene".to_string(),
            }),
            request_id: 1,
        })
        .expect("init");

    // List entities - should return real runtime entities
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Scene(SceneCommand::ListEntities),
        request_id: 2,
    };

    let result = session.handle_command(packet).expect("list entities");

    // Should have 3 entities from runtime: terrain, wall, weapon
    if let link_egress_observations::EditorAuthoringObservation::EntityList { entities } = result {
        assert_eq!(entities.len(), 3, "Should have 3 entities from runtime");
        assert!(entities.iter().any(|e| e.label == "Terrain"));
        assert!(entities.iter().any(|e| e.label == "Wall"));
        assert!(entities.iter().any(|e| e.label == "Weapon"));
    } else {
        panic!("Expected EntityList observation");
    }
}

#[test]
fn authoring_update_wall_position_mutates_runtime() {
    let mut session = EditorAuthoringSession::new();

    // Initialize scene
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Scene(SceneCommand::CreateEmpty {
                scene_name: "test_scene".to_string(),
            }),
            request_id: 1,
        })
        .expect("init");

    // Get initial wall position
    let initial_entities = session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Scene(SceneCommand::ListEntities),
            request_id: 2,
        })
        .expect("list");

    let initial_wall_pos =
        if let link_egress_observations::EditorAuthoringObservation::EntityList { entities } =
            initial_entities
        {
            entities
                .iter()
                .find(|e| e.label == "Wall")
                .unwrap()
                .transform
                .position
        } else {
            panic!("Expected EntityList");
        };

    // Update wall position
    let new_position = [100.0, 50.0, 200.0];
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Scene(SceneCommand::SetTransform {
                entity_id: 2, // Wall
                transform: Transform {
                    position: new_position,
                    rotation: [0.0, 0.0, 0.0, 1.0],
                    scale: [1.0, 1.0, 1.0],
                },
            }),
            request_id: 3,
        })
        .expect("update position");

    // Verify position changed in runtime
    let updated_entities = session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Scene(SceneCommand::ListEntities),
            request_id: 4,
        })
        .expect("list after update");

    if let link_egress_observations::EditorAuthoringObservation::EntityList { entities } =
        updated_entities
    {
        let updated_wall_pos = entities
            .iter()
            .find(|e| e.label == "Wall")
            .unwrap()
            .transform
            .position;
        assert_eq!(
            updated_wall_pos, new_position,
            "Wall position should be updated in runtime"
        );
        assert_ne!(
            updated_wall_pos, initial_wall_pos,
            "Position should have changed"
        );
    } else {
        panic!("Expected EntityList");
    }
}

#[test]
fn authoring_update_weapon_position_mutates_runtime() {
    let mut session = EditorAuthoringSession::new();

    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Scene(SceneCommand::CreateEmpty {
                scene_name: "test_scene".to_string(),
            }),
            request_id: 1,
        })
        .expect("init");

    // Update weapon position
    let new_position = [50.0, 25.0, 75.0];
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Scene(SceneCommand::SetTransform {
                entity_id: 3, // Weapon
                transform: Transform {
                    position: new_position,
                    rotation: [0.0, 0.0, 0.0, 1.0],
                    scale: [1.0, 1.0, 1.0],
                },
            }),
            request_id: 2,
        })
        .expect("update weapon");

    // Verify in runtime
    let entities = session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Scene(SceneCommand::ListEntities),
            request_id: 3,
        })
        .expect("list");

    if let link_egress_observations::EditorAuthoringObservation::EntityList { entities } = entities
    {
        let weapon_pos = entities
            .iter()
            .find(|e| e.label == "Weapon")
            .unwrap()
            .transform
            .position;
        assert_eq!(
            weapon_pos, new_position,
            "Weapon position should be updated in runtime"
        );
    } else {
        panic!("Expected EntityList");
    }
}

#[test]
fn authoring_assign_material_mutates_runtime() {
    let mut session = EditorAuthoringSession::new();

    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Scene(SceneCommand::CreateEmpty {
                scene_name: "test_scene".to_string(),
            }),
            request_id: 1,
        })
        .expect("init");

    // Get initial material
    let initial_entities = session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Scene(SceneCommand::ListEntities),
            request_id: 2,
        })
        .expect("list");

    let initial_stack_id =
        if let link_egress_observations::EditorAuthoringObservation::EntityList { entities } =
            initial_entities
        {
            entities
                .iter()
                .find(|e| e.label == "Wall")
                .unwrap()
                .material_slots[0]
                .stack_id
                .unwrap()
        } else {
            panic!("Expected EntityList");
        };

    // Assign new material stack
    let new_stack_id = 99;
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Material(MaterialCommand::AssignStackToEntitySlot {
                entity_id: 2, // Wall
                slot_id: 0,
                stack_id: new_stack_id,
            }),
            request_id: 3,
        })
        .expect("assign material");

    // Verify material changed in runtime
    let updated_entities = session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Scene(SceneCommand::ListEntities),
            request_id: 4,
        })
        .expect("list after material");

    if let link_egress_observations::EditorAuthoringObservation::EntityList { entities } =
        updated_entities
    {
        let updated_stack_id = entities
            .iter()
            .find(|e| e.label == "Wall")
            .unwrap()
            .material_slots[0]
            .stack_id
            .unwrap();
        assert_eq!(
            updated_stack_id, new_stack_id,
            "Material stack should be updated in runtime"
        );
        assert_ne!(
            updated_stack_id, initial_stack_id,
            "Material should have changed"
        );
    } else {
        panic!("Expected EntityList");
    }
}

#[test]
fn authoring_terrain_list_returns_runtime_state() {
    let mut session = EditorAuthoringSession::new();

    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Scene(SceneCommand::CreateEmpty {
                scene_name: "test_scene".to_string(),
            }),
            request_id: 1,
        })
        .expect("init");

    // List terrain patches - should return real runtime terrain
    let result = session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Terrain(TerrainCommand::ListPatches),
            request_id: 2,
        })
        .expect("list patches");

    if let link_egress_observations::EditorAuthoringObservation::TerrainPatchList { patches } =
        result
    {
        assert_eq!(patches.len(), 1, "Should have 1 terrain patch from runtime");
        assert_eq!(patches[0].label, "Terrain");
    } else {
        panic!("Expected TerrainPatchList observation");
    }
}

#[test]
fn authoring_unsupported_operations_fail_honestly() {
    let mut session = EditorAuthoringSession::new();

    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Scene(SceneCommand::CreateEmpty {
                scene_name: "test_scene".to_string(),
            }),
            request_id: 1,
        })
        .expect("init");

    // CreateEntityFromAsset should fail honestly
    let result = session.handle_command(EditorAuthoringPacket {
        command: EditorAuthoringCommand::Scene(SceneCommand::CreateEntityFromAsset {
            asset_id: 1,
            transform: Transform::default(),
            label: "test".to_string(),
        }),
        request_id: 2,
    });
    assert!(
        result.is_err(),
        "CreateEntityFromAsset should fail for proof-scene subset"
    );
    assert!(result.unwrap_err().contains("not implemented"));

    // DeleteEntity should fail honestly
    let result = session.handle_command(EditorAuthoringPacket {
        command: EditorAuthoringCommand::Scene(SceneCommand::DeleteEntity { entity_id: 1 }),
        request_id: 3,
    });
    assert!(
        result.is_err(),
        "DeleteEntity should fail for proof-scene subset"
    );
    assert!(result.unwrap_err().contains("not implemented"));

    // CreatePatch should fail honestly
    let result = session.handle_command(EditorAuthoringPacket {
        command: EditorAuthoringCommand::Terrain(TerrainCommand::CreatePatch {
            input: link_ingress_packets::TerrainPatchInput {
                position: [0.0, 0.0, 0.0],
                size: [10.0, 10.0],
                label: "test".to_string(),
            },
        }),
        request_id: 4,
    });
    assert!(
        result.is_err(),
        "CreatePatch should fail for proof-scene subset"
    );
    assert!(result.unwrap_err().contains("not implemented"));
}

#[test]
fn authoring_world_summary_reflects_runtime_state() {
    let mut session = EditorAuthoringSession::new();

    // Before initialization
    let summary = session.get_world_summary();
    assert_eq!(
        summary.entity_count, 0,
        "Should have 0 entities before init"
    );

    // After initialization
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Scene(SceneCommand::CreateEmpty {
                scene_name: "test_scene".to_string(),
            }),
            request_id: 1,
        })
        .expect("init");

    let summary = session.get_world_summary();
    assert_eq!(
        summary.entity_count, 3,
        "Should have 3 entities from runtime after init"
    );
    assert!(summary.active_scene.is_some(), "Should have active scene");
    // Scene name comes from runtime, not from CreateEmpty command
    assert_eq!(
        summary.active_scene.unwrap(),
        "vertical_slice.wall_terrain.ak_demo"
    );
}
