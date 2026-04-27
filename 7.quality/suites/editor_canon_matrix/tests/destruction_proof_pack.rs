// ============================================================================
// PACK 4: DESTRUCTION / COLLAPSE / DEBRIS PROOF-PACK INTEGRATION TESTS
// ============================================================================
//
// These tests prove that editor can runtime-backed author and inspect
// minimal destruction/collapse/debris subset through real runtime truth:
// - Terrain material affects blast outcome (dirt vs asphalt)
// - Wall/structure integrity and destruction state
// - Support object failure progression
// - Debris/aftermath state in runtime truth
// - Inspection returns runtime truth, not local DTO fiction

use link_ingress_packets::*;
use stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession;

// ============================================================================
// TERRAIN DESTRUCTION TESTS
// ============================================================================

#[test]
fn test_terrain_material_state_lives_in_runtime_truth() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set terrain material to Dirt
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::SetTerrainMaterial {
            material_type: "Dirt".to_string(),
        }),
        request_id: 1,
    };
    let result = session.handle_command(packet).unwrap();
    assert!(matches!(
        result,
        link_egress_observations::EditorAuthoringObservation::TerrainMaterialSet { .. }
    ));

    // Get terrain material from runtime
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetTerrainMaterial),
        request_id: 2,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::TerrainMaterialInfo {
        material_type,
    } = result
    {
        assert_eq!(material_type, "Dirt");
    } else {
        panic!("Expected TerrainMaterialInfo");
    }
}

#[test]
fn test_dirt_vs_asphalt_blast_gives_different_runtime_outcome() {
    // Test dirt blast
    let mut dirt_session = EditorAuthoringSession::new();
    dirt_session.initialize_vertical_slice_session().unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::SetTerrainMaterial {
            material_type: "Dirt".to_string(),
        }),
        request_id: 1,
    };
    dirt_session.handle_command(packet).unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::TriggerBlast {
            position: [5.0, 0.0, 5.0],
            energy_j: 50000.0,
        }),
        request_id: 2,
    };
    dirt_session.handle_command(packet).unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetTerrainBlastResponse),
        request_id: 3,
    };
    let dirt_result = dirt_session.handle_command(packet).unwrap();

    // Test asphalt blast
    let mut asphalt_session = EditorAuthoringSession::new();
    asphalt_session.initialize_vertical_slice_session().unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::SetTerrainMaterial {
            material_type: "Asphalt".to_string(),
        }),
        request_id: 1,
    };
    asphalt_session.handle_command(packet).unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::TriggerBlast {
            position: [5.0, 0.0, 5.0],
            energy_j: 50000.0,
        }),
        request_id: 2,
    };
    asphalt_session.handle_command(packet).unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetTerrainBlastResponse),
        request_id: 3,
    };
    let asphalt_result = asphalt_session.handle_command(packet).unwrap();

    // Compare results - dirt should have deeper crater
    if let (
        link_egress_observations::EditorAuthoringObservation::TerrainBlastResponseInfo {
            crater_radius_m: dirt_radius,
            crater_depth_m: dirt_depth,
            ..
        },
        link_egress_observations::EditorAuthoringObservation::TerrainBlastResponseInfo {
            crater_radius_m: asphalt_radius,
            crater_depth_m: asphalt_depth,
            ..
        },
    ) = (dirt_result, asphalt_result)
    {
        assert!(
            dirt_depth > asphalt_depth,
            "Dirt crater should be deeper than asphalt"
        );
        assert!(
            dirt_radius > asphalt_radius,
            "Dirt crater should be wider than asphalt"
        );
    } else {
        panic!("Expected TerrainBlastResponseInfo for both");
    }
}

#[test]
fn test_blast_creates_inspectable_crater_in_runtime() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set terrain material
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::SetTerrainMaterial {
            material_type: "Dirt".to_string(),
        }),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    // Trigger blast
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::TriggerBlast {
            position: [5.0, 0.0, 5.0],
            energy_j: 50000.0,
        }),
        request_id: 2,
    };
    session.handle_command(packet).unwrap();

    // Inspect blast response
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetTerrainBlastResponse),
        request_id: 3,
    };
    let result = session.handle_command(packet).unwrap();

    if let link_egress_observations::EditorAuthoringObservation::TerrainBlastResponseInfo {
        crater_radius_m,
        crater_depth_m,
        debris_count,
        ejecta_volume_m3,
    } = result
    {
        assert!(crater_radius_m > 0.0, "Crater should have positive radius");
        assert!(crater_depth_m > 0.0, "Crater should have positive depth");
        assert!(debris_count > 0, "Blast should create debris");
        assert!(ejecta_volume_m3 > 0.0, "Blast should eject material");
    } else {
        panic!("Expected TerrainBlastResponseInfo");
    }
}

// ============================================================================
// WALL DESTRUCTION TESTS
// ============================================================================

#[test]
fn test_wall_integrity_state_lives_in_runtime_truth() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set wall integrity
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::SetWallIntegrity {
            integrity: 0.5,
        }),
        request_id: 1,
    };
    let result = session.handle_command(packet).unwrap();
    assert!(
        matches!(result, link_egress_observations::EditorAuthoringObservation::WallIntegritySet { integrity } if integrity == 0.5)
    );

    // Get wall integrity from runtime
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetWallIntegrity),
        request_id: 2,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::WallIntegrityInfo { integrity } =
        result
    {
        assert_eq!(integrity, 0.5);
    } else {
        panic!("Expected WallIntegrityInfo");
    }
}

#[test]
fn test_wall_enters_destroyed_state_when_integrity_zero() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set wall integrity to zero
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::SetWallIntegrity {
            integrity: 0.0,
        }),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    // Check destroyed state
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetWallDestroyedState),
        request_id: 2,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::WallDestroyedStateInfo {
        destroyed,
    } = result
    {
        assert!(destroyed, "Wall should be destroyed when integrity is zero");
    } else {
        panic!("Expected WallDestroyedStateInfo");
    }
}

#[test]
fn test_blast_damages_wall_in_runtime() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Get initial wall integrity
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetWallIntegrity),
        request_id: 1,
    };
    let result = session.handle_command(packet).unwrap();
    let initial_integrity =
        if let link_egress_observations::EditorAuthoringObservation::WallIntegrityInfo {
            integrity,
        } = result
        {
            integrity
        } else {
            panic!("Expected WallIntegrityInfo");
        };

    // Trigger blast near wall (wall is at [0.0, 1.5, 10.0])
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::TriggerBlast {
            position: [0.0, 1.5, 8.0], // 2 meters in front of wall
            energy_j: 100000.0,
        }),
        request_id: 2,
    };
    session.handle_command(packet).unwrap();

    // Check wall integrity decreased
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetWallIntegrity),
        request_id: 3,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::WallIntegrityInfo { integrity } =
        result
    {
        assert!(integrity < initial_integrity, "Blast should damage wall");
    } else {
        panic!("Expected WallIntegrityInfo");
    }
}

// ============================================================================
// SUPPORT OBJECT DESTRUCTION TESTS
// ============================================================================

#[test]
fn test_support_object_state_lives_in_runtime_truth() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set support object type
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::SetSupportObjectType {
            structure_type: "WoodenWall".to_string(),
        }),
        request_id: 1,
    };
    let result = session.handle_command(packet).unwrap();
    assert!(matches!(
        result,
        link_egress_observations::EditorAuthoringObservation::SupportObjectTypeSet { .. }
    ));

    // Get support object state from runtime
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetSupportObjectState),
        request_id: 2,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::SupportObjectStateInfo {
        structure_type,
        ..
    } = result
    {
        assert_eq!(structure_type, "WoodenWall");
    } else {
        panic!("Expected SupportObjectStateInfo");
    }
}

#[test]
fn test_support_object_enters_failure_progression() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set support object to wooden wall (low failure threshold)
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::SetSupportObjectType {
            structure_type: "WoodenWall".to_string(),
        }),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    // Apply moderate damage
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::ApplySupportDamage {
            energy_j: 2000.0,
            impact_direction: [1.0, 0.0, 0.0],
        }),
        request_id: 2,
    };
    session.handle_command(packet).unwrap();

    // Check state - should be cracked but not destroyed
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetSupportObjectState),
        request_id: 3,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::SupportObjectStateInfo {
        integrity,
        destroyed,
        failure_mode,
        ..
    } = result
    {
        assert!(integrity < 1.0, "Support should be damaged");
        assert!(!destroyed, "Support should not be destroyed yet");
        assert_eq!(failure_mode, "Crack", "Support should be cracked");
    } else {
        panic!("Expected SupportObjectStateInfo");
    }

    // Apply more damage to destroy
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::ApplySupportDamage {
            energy_j: 5000.0,
            impact_direction: [1.0, 0.0, 0.0],
        }),
        request_id: 4,
    };
    session.handle_command(packet).unwrap();

    // Check state - should be destroyed
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetSupportObjectState),
        request_id: 5,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::SupportObjectStateInfo {
        destroyed,
        failure_mode,
        fragment_count,
        ..
    } = result
    {
        assert!(destroyed, "Support should be destroyed");
        assert_eq!(failure_mode, "Fracture", "Wooden wall should fracture");
        assert!(
            fragment_count > 0,
            "Destroyed support should have fragments"
        );
    } else {
        panic!("Expected SupportObjectStateInfo");
    }
}

#[test]
fn test_blast_damages_support_object() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Get initial support state
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetSupportObjectState),
        request_id: 1,
    };
    let result = session.handle_command(packet).unwrap();
    let initial_integrity =
        if let link_egress_observations::EditorAuthoringObservation::SupportObjectStateInfo {
            integrity,
            ..
        } = result
        {
            integrity
        } else {
            panic!("Expected SupportObjectStateInfo");
        };

    // Trigger blast near support object (support is at [3.0, 0.0, 8.0])
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::TriggerBlast {
            position: [3.0, 0.0, 6.0], // 2 meters away
            energy_j: 100000.0,
        }),
        request_id: 2,
    };
    session.handle_command(packet).unwrap();

    // Check support integrity decreased
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetSupportObjectState),
        request_id: 3,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::SupportObjectStateInfo {
        integrity,
        ..
    } = result
    {
        assert!(
            integrity < initial_integrity,
            "Blast should damage support object"
        );
    } else {
        panic!("Expected SupportObjectStateInfo");
    }
}

// ============================================================================
// DESTRUCTION SUMMARY TESTS
// ============================================================================

#[test]
fn test_destruction_summary_reads_runtime_truth() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set terrain material
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::SetTerrainMaterial {
            material_type: "Asphalt".to_string(),
        }),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    // Trigger blast near wall (wall is at [0.0, 1.5, 10.0])
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::TriggerBlast {
            position: [0.0, 1.5, 8.0], // 2 meters in front of wall
            energy_j: 100000.0,
        }),
        request_id: 2,
    };
    session.handle_command(packet).unwrap();

    // Damage support object
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::ApplySupportDamage {
            energy_j: 10000.0,
            impact_direction: [1.0, 0.0, 0.0],
        }),
        request_id: 3,
    };
    session.handle_command(packet).unwrap();

    // Get destruction summary
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetDestructionSummary),
        request_id: 4,
    };
    let result = session.handle_command(packet).unwrap();

    if let link_egress_observations::EditorAuthoringObservation::DestructionSummaryInfo {
        terrain_material,
        crater_radius_m,
        crater_depth_m,
        wall_integrity,
        support_integrity,
        ..
    } = result
    {
        assert_eq!(terrain_material, "Asphalt");
        assert!(
            crater_radius_m.is_some(),
            "Summary should include crater info"
        );
        assert!(
            crater_depth_m.is_some(),
            "Summary should include crater depth"
        );
        assert!(wall_integrity < 1.0, "Wall should be damaged by blast");
        assert!(
            support_integrity.is_some(),
            "Summary should include support info"
        );
        assert!(
            support_integrity.unwrap() < 1.0,
            "Support should be damaged"
        );
    } else {
        panic!("Expected DestructionSummaryInfo");
    }
}

#[test]
fn test_reset_destruction_state() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Trigger blast and damage
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::TriggerBlast {
            position: [5.0, 0.0, 5.0],
            energy_j: 50000.0,
        }),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    // Reset destruction state
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::ResetDestructionState),
        request_id: 2,
    };
    let result = session.handle_command(packet).unwrap();
    assert!(matches!(
        result,
        link_egress_observations::EditorAuthoringObservation::DestructionStateReset
    ));

    // Check wall integrity restored
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetWallIntegrity),
        request_id: 3,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::WallIntegrityInfo { integrity } =
        result
    {
        assert_eq!(integrity, 1.0, "Wall integrity should be restored");
    } else {
        panic!("Expected WallIntegrityInfo");
    }

    // Check support integrity restored
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetSupportObjectState),
        request_id: 4,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::SupportObjectStateInfo {
        integrity,
        destroyed,
        ..
    } = result
    {
        assert_eq!(integrity, 1.0, "Support integrity should be restored");
        assert!(!destroyed, "Support should not be destroyed");
    } else {
        panic!("Expected SupportObjectStateInfo");
    }
}

// ============================================================================
// INTEGRATION CYCLE TESTS
// ============================================================================

#[test]
fn test_full_destruction_cycle() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // 1. Set terrain material to Dirt
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::SetTerrainMaterial {
            material_type: "Dirt".to_string(),
        }),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    // 2. Set support object to Tree
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::SetSupportObjectType {
            structure_type: "Tree".to_string(),
        }),
        request_id: 2,
    };
    session.handle_command(packet).unwrap();

    // 3. Trigger large blast
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::TriggerBlast {
            position: [2.0, 0.0, 8.0], // Near support object
            energy_j: 150000.0,
        }),
        request_id: 3,
    };
    session.handle_command(packet).unwrap();

    // 4. Verify terrain blast response
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetTerrainBlastResponse),
        request_id: 4,
    };
    let result = session.handle_command(packet).unwrap();
    assert!(matches!(
        result,
        link_egress_observations::EditorAuthoringObservation::TerrainBlastResponseInfo { .. }
    ));

    // 5. Verify wall damaged
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetWallIntegrity),
        request_id: 5,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::WallIntegrityInfo { integrity } =
        result
    {
        assert!(integrity < 1.0, "Wall should be damaged by blast");
    } else {
        panic!("Expected WallIntegrityInfo");
    }

    // 6. Verify support object destroyed (tree toppled)
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetSupportObjectState),
        request_id: 6,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::SupportObjectStateInfo {
        destroyed,
        failure_mode,
        fragment_count,
        ..
    } = result
    {
        assert!(destroyed, "Tree should be destroyed by blast");
        assert_eq!(failure_mode, "Topple", "Tree should topple");
        assert!(fragment_count > 0, "Destroyed tree should have fragments");
    } else {
        panic!("Expected SupportObjectStateInfo");
    }

    // 7. Get comprehensive destruction summary
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetDestructionSummary),
        request_id: 7,
    };
    let result = session.handle_command(packet).unwrap();
    assert!(matches!(
        result,
        link_egress_observations::EditorAuthoringObservation::DestructionSummaryInfo { .. }
    ));
}

#[test]
fn test_destruction_without_runtime_session_fails() {
    let mut session = EditorAuthoringSession::new();
    // Don't initialize runtime session

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetTerrainMaterial),
        request_id: 1,
    };
    let result = session.handle_command(packet);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Runtime session not initialized"));
}
