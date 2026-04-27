// Vertical Slice End-to-End Tests
//
// Property 24: Vertical Slice Completeness - each active slice has all 4 layers
// Property 25: Incomplete Slice Marking - incomplete slices are marked "future"
// Property 26: Vertical Slice End-to-End Testing - each active slice works end-to-end

use link_ingress_packets::{
    DestructionCommand, EditorAuthoringCommand, EditorAuthoringPacket, MaterialWorldCommand,
    PopulationCommand, SkyCommand, TacticsCommand, EcologyCommand, NavDoorInventoryCommand,
    StormCommand, VerticalSliceIngressPacket, VerticalSliceSessionHandle,
};
use link_egress_observations::EditorAuthoringObservation;
use stratumx_tooling_l6_12_preview_runtime::VerticalSliceSession;
use vertical_slice_tests::{get_active_slices, get_future_slices, load_audit_report, slice_has_all_layers};

// ============================================================================
// Property 24: Vertical Slice Completeness
// Each active slice has all 4 layers (engine, SDK, tooling, editor)
// ============================================================================

#[test]
fn property_24_active_slices_have_all_four_layers() {
    let active_slices = get_active_slices();
    assert!(
        !active_slices.is_empty(),
        "There should be at least one active slice"
    );

    for slice in &active_slices {
        let name = slice
            .get("slice_name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        assert!(
            slice_has_all_layers(slice),
            "Active slice '{}' must have all four layers (engine, sdk, tooling, editor). Missing: {:?}",
            name,
            slice.get("missing_components").unwrap_or(&serde_json::json!([]))
        );
    }
}

#[test]
fn property_24_world_scale_has_all_layers() {
    let report = load_audit_report();
    let slices = report.get("slices").and_then(|s| s.as_array()).unwrap();
    let world_scale = slices
        .iter()
        .find(|s| s.get("slice_name").and_then(|v| v.as_str()) == Some("world-scale"))
        .expect("world-scale slice should exist");

    assert_eq!(
        world_scale.get("status").and_then(|v| v.as_str()),
        Some("active")
    );
    assert!(
        world_scale.get("engine_present").and_then(|v| v.as_bool()) == Some(true),
        "world-scale must have engine truth component"
    );
    assert!(
        world_scale.get("sdk_present").and_then(|v| v.as_bool()) == Some(true),
        "world-scale must have SDK packets component"
    );
    assert!(
        world_scale.get("tooling_present").and_then(|v| v.as_bool()) == Some(true),
        "world-scale must have tooling routes component"
    );
    assert!(
        world_scale.get("editor_present").and_then(|v| v.as_bool()) == Some(true),
        "world-scale must have editor surface component"
    );
}

#[test]
fn property_24_physical_substrate_has_all_layers() {
    let report = load_audit_report();
    let slices = report.get("slices").and_then(|s| s.as_array()).unwrap();
    let physical = slices
        .iter()
        .find(|s| s.get("slice_name").and_then(|v| v.as_str()) == Some("physical-substrate"))
        .expect("physical-substrate slice should exist");

    assert_eq!(
        physical.get("status").and_then(|v| v.as_str()),
        Some("active")
    );
    assert!(physical.get("engine_present").and_then(|v| v.as_bool()) == Some(true));
    assert!(physical.get("sdk_present").and_then(|v| v.as_bool()) == Some(true));
    assert!(physical.get("tooling_present").and_then(|v| v.as_bool()) == Some(true));
    assert!(physical.get("editor_present").and_then(|v| v.as_bool()) == Some(true));
}

#[test]
fn property_24_photoreal_proof_has_all_layers() {
    let report = load_audit_report();
    let slices = report.get("slices").and_then(|s| s.as_array()).unwrap();
    let photoreal = slices
        .iter()
        .find(|s| s.get("slice_name").and_then(|v| v.as_str()) == Some("photoreal-proof"))
        .expect("photoreal-proof slice should exist");

    assert_eq!(
        photoreal.get("status").and_then(|v| v.as_str()),
        Some("active")
    );
    assert!(photoreal.get("engine_present").and_then(|v| v.as_bool()) == Some(true));
    assert!(photoreal.get("sdk_present").and_then(|v| v.as_bool()) == Some(true));
    assert!(photoreal.get("tooling_present").and_then(|v| v.as_bool()) == Some(true));
    assert!(photoreal.get("editor_present").and_then(|v| v.as_bool()) == Some(true));
}

#[test]
fn property_24_living_runtime_has_all_layers() {
    let report = load_audit_report();
    let slices = report.get("slices").and_then(|s| s.as_array()).unwrap();
    let living = slices
        .iter()
        .find(|s| s.get("slice_name").and_then(|v| v.as_str()) == Some("living-runtime"))
        .expect("living-runtime slice should exist");

    assert_eq!(
        living.get("status").and_then(|v| v.as_str()),
        Some("active")
    );
    assert!(living.get("engine_present").and_then(|v| v.as_bool()) == Some(true));
    assert!(living.get("sdk_present").and_then(|v| v.as_bool()) == Some(true));
    assert!(living.get("tooling_present").and_then(|v| v.as_bool()) == Some(true));
    assert!(living.get("editor_present").and_then(|v| v.as_bool()) == Some(true));
}

#[test]
fn property_24_audio_reality_has_all_layers() {
    let report = load_audit_report();
    let slices = report.get("slices").and_then(|s| s.as_array()).unwrap();
    let audio = slices
        .iter()
        .find(|s| s.get("slice_name").and_then(|v| v.as_str()) == Some("audio-reality"))
        .expect("audio-reality slice should exist");

    assert_eq!(
        audio.get("status").and_then(|v| v.as_str()),
        Some("active")
    );
    assert!(audio.get("engine_present").and_then(|v| v.as_bool()) == Some(true));
    assert!(audio.get("sdk_present").and_then(|v| v.as_bool()) == Some(true));
    assert!(audio.get("tooling_present").and_then(|v| v.as_bool()) == Some(true));
    assert!(audio.get("editor_present").and_then(|v| v.as_bool()) == Some(true));
}

// ============================================================================
// Property 25: Incomplete Slice Marking
// Incomplete slices are marked as "future" and excluded from active spine
// ============================================================================

#[test]
fn property_25_incomplete_slices_marked_as_future() {
    let future_slices = get_future_slices();
    assert!(
        !future_slices.is_empty(),
        "There should be at least one future slice (animation and release-capture-freeze)"
    );

    for slice in &future_slices {
        let name = slice
            .get("slice_name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        assert_eq!(
            slice.get("status").and_then(|v| v.as_str()),
            Some("future"),
            "Slice '{}' should be marked as 'future'",
            name
        );

        // Future slices should either have missing components, not all layers,
        // or be complete but intentionally excluded from the active spine
        let missing = slice
            .get("missing_components")
            .and_then(|v| v.as_array())
            .map(|arr| arr.len())
            .unwrap_or(0);

        let has_missing = !slice_has_all_layers(slice);
        let excluded_from_spine = slice
            .get("excluded_from_active_spine")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        assert!(
            has_missing || missing > 0 || excluded_from_spine,
            "Future slice '{}' should have missing components, not all layers, or be intentionally excluded from active spine",
            name
        );
    }
}

#[test]
fn property_25_animation_slice_is_future() {
    let report = load_audit_report();
    let slices = report.get("slices").and_then(|s| s.as_array()).unwrap();
    let animation = slices
        .iter()
        .find(|s| s.get("slice_name").and_then(|v| v.as_str()) == Some("animation"))
        .expect("animation slice should exist");

    assert_eq!(
        animation.get("status").and_then(|v| v.as_str()),
        Some("future"),
        "animation slice should be marked as 'future' (missing SDK packets and tooling commands)"
    );
    assert!(
        !slice_has_all_layers(animation),
        "animation slice should be missing SDK/toolting layers"
    );
}

#[test]
fn property_25_release_capture_freeze_is_future() {
    let report = load_audit_report();
    let slices = report.get("slices").and_then(|s| s.as_array()).unwrap();
    let release = slices
        .iter()
        .find(|s| {
            s.get("slice_name").and_then(|v| v.as_str())
                == Some("release-capture-freeze")
        })
        .expect("release-capture-freeze slice should exist");

    assert_eq!(
        release.get("status").and_then(|v| v.as_str()),
        Some("future"),
        "release-capture-freeze should be marked as 'future' (post-production, not in active spine)"
    );
}

#[test]
fn property_25_future_slices_excluded_from_active_list() {
    let active_slices = get_active_slices();
    for slice in &active_slices {
        assert_eq!(
            slice.get("status").and_then(|v| v.as_str()),
            Some("active"),
            "Active slice list should not contain future slices"
        );
    }
}

#[test]
fn property_25_audit_report_has_valid_summary() {
    let report = load_audit_report();
    let summary = report.get("summary").expect("Audit report should have summary");

    let total = summary.get("total_slices").and_then(|v| v.as_u64()).unwrap_or(0);
    let active = summary.get("active").and_then(|v| v.as_u64()).unwrap_or(0);
    let future = summary.get("future").and_then(|v| v.as_u64()).unwrap_or(0);

    assert_eq!(
        total,
        active + future,
        "Total slices should equal active + future"
    );
    assert!(total > 0, "Should have audited slices");
}

// ============================================================================
// Property 26: Vertical Slice End-to-End Testing
// Each active slice works end-to-end through all 4 layers
// ============================================================================

#[test]
fn property_26_world_scale_create_and_query_end_to_end() {
    // Test: Create scene -> Query entities -> Verify observation flow
    let mut session = stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession::new();

    // Initialize the vertical slice session (engine layer)
    session
        .initialize_vertical_slice_session()
        .expect("Session should initialize");

    // Create empty scene (editor -> SDK packet -> tooling route -> engine)
    let create_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Scene(link_ingress_packets::scene::SceneCommand::CreateEmpty {
            scene_name: "test_world".to_string(),
        }),
        request_id: 1,
    };
    let observation = session
        .handle_command(create_packet)
        .expect("CreateEmpty should succeed");

    match observation {
        EditorAuthoringObservation::SceneCreated { scene } => {
            assert_eq!(scene.scene_name, "test_world");
        }
        _ => panic!("Expected SceneCreated observation"),
    }

    // Query world summary (engine -> SDK observation -> tooling -> editor)
    let summary = session.get_world_summary();
    assert!(summary.active_scene.is_some(), "Should have active scene");
}

#[test]
fn property_26_terrain_list_and_query_end_to_end() {
    // Test: List terrain patches -> Get patch details
    let mut session = stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession::new();
    session
        .initialize_vertical_slice_session()
        .expect("Session should initialize");

    // List patches
    let list_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Terrain(
            link_ingress_packets::terrain::TerrainCommand::ListPatches,
        ),
        request_id: 1,
    };
    let observation = session
        .handle_command(list_packet)
        .expect("ListPatches should succeed");

    match observation {
        EditorAuthoringObservation::TerrainPatchList { ref patches } => {
            assert!(!patches.is_empty(), "Should have at least one terrain patch");
        }
        _ => panic!("Expected TerrainPatchList observation"),
    }

    // Get patch details
    if let EditorAuthoringObservation::TerrainPatchList { patches } = observation {
        let patch_id = patches[0].patch_id;
        let details_packet = EditorAuthoringPacket {
            command: EditorAuthoringCommand::Terrain(
                link_ingress_packets::terrain::TerrainCommand::GetPatchDetails { patch_id },
            ),
            request_id: 2,
        };
        let details_obs = session
            .handle_command(details_packet)
            .expect("GetPatchDetails should succeed");

        match details_obs {
            EditorAuthoringObservation::TerrainPatchDetails { patch } => {
                assert_eq!(patch.patch_id, patch_id);
            }
            _ => panic!("Expected TerrainPatchDetails observation"),
        }
    }
}

#[test]
fn property_26_destruction_commands_flow_end_to_end() {
    // Test: Destruction commands flow through SDK -> tooling -> observation
    let mut session = stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession::new();
    session
        .initialize_vertical_slice_session()
        .expect("Session should initialize");

    // Set terrain material
    let set_material_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::SetTerrainMaterial {
            material_type: "concrete".to_string(),
        }),
        request_id: 1,
    };
    let obs = session
        .handle_command(set_material_packet)
        .expect("SetTerrainMaterial should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::TerrainMaterialSet { .. }),
        "Should return TerrainMaterialSet observation"
    );

    // Trigger blast
    let blast_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::TriggerBlast {
            position: [0.0, 0.0, 0.0],
            energy_j: 1000.0,
        }),
        request_id: 2,
    };
    let obs = session
        .handle_command(blast_packet)
        .expect("TriggerBlast should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::BlastTriggered { .. }),
        "Should return BlastTriggered observation"
    );

    // Get destruction summary
    let summary_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetDestructionSummary),
        request_id: 3,
    };
    let obs = session
        .handle_command(summary_packet)
        .expect("GetDestructionSummary should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::DestructionSummaryInfo { .. }),
        "Should return DestructionSummaryInfo observation"
    );

    // Reset destruction state
    let reset_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::ResetDestructionState),
        request_id: 4,
    };
    let obs = session
        .handle_command(reset_packet)
        .expect("ResetDestructionState should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::DestructionStateReset),
        "Should return DestructionStateReset observation"
    );
}

#[test]
fn property_26_sky_weather_commands_flow_end_to_end() {
    // Test: Sky/weather commands flow through SDK -> tooling -> observation
    let mut session = stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession::new();
    session
        .initialize_vertical_slice_session()
        .expect("Session should initialize");

    // Get sky summary
    let sky_summary_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::GetSkySummary),
        request_id: 1,
    };
    let obs = session
        .handle_command(sky_summary_packet)
        .expect("GetSkySummary should succeed");
    match &obs {
        EditorAuthoringObservation::SkySummaryRead {
            time_of_day_hours,
            cloud_coverage,
            ..
        } => {
            assert!(*time_of_day_hours >= 0.0 && *time_of_day_hours <= 24.0);
            assert!(*cloud_coverage >= 0.0 && *cloud_coverage <= 1.0);
        }
        _ => panic!("Expected SkySummaryRead observation"),
    }

    // Set time of day
    let set_time_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::SetTimeOfDay { hours: 18.0 }),
        request_id: 2,
    };
    let obs = session
        .handle_command(set_time_packet)
        .expect("SetTimeOfDay should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::SkyValueUpdated),
        "Should return SkyValueUpdated observation"
    );

    // Set rain
    let set_rain_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::SetRain {
            enabled: true,
            intensity_mm_per_hour: 5.0,
        }),
        request_id: 3,
    };
    let obs = session
        .handle_command(set_rain_packet)
        .expect("SetRain should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::SkyValueUpdated),
        "Should return SkyValueUpdated observation"
    );

    // Create storm front
    let storm_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Storm(StormCommand::CreateStormFront {
            position: [100.0, 0.0, 50.0],
            velocity: [1.0, 0.0, 0.0],
            radius_km: 10.0,
            intensity: 0.8,
            rain_intensity_mm_per_hour: 20.0,
        }),
        request_id: 4,
    };
    let obs = session
        .handle_command(storm_packet)
        .expect("CreateStormFront should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::StormFrontCreated { .. }),
        "Should return StormFrontCreated observation"
    );

    // List storm fronts
    let list_storms_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Storm(StormCommand::ListStormFronts),
        request_id: 5,
    };
    let obs = session
        .handle_command(list_storms_packet)
        .expect("ListStormFronts should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::StormFrontList { .. }),
        "Should return StormFrontList observation"
    );
}

#[test]
fn property_26_material_world_commands_flow_end_to_end() {
    // Test: Material world commands flow through SDK -> tooling -> observation
    let mut session = stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession::new();
    session
        .initialize_vertical_slice_session()
        .expect("Session should initialize");

    // Set barrel water
    let water_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::SetBarrelWater {
            liters: 50.0,
        }),
        request_id: 1,
    };
    let obs = session
        .handle_command(water_packet)
        .expect("SetBarrelWater should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::BarrelWaterSet { .. }),
        "Should return BarrelWaterSet observation"
    );

    // Ignite fire object
    let ignite_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::IgniteFireObject),
        request_id: 2,
    };
    let obs = session
        .handle_command(ignite_packet)
        .expect("IgniteFireObject should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::FireObjectIgnited { success: true }),
        "Should return FireObjectIgnited with success=true"
    );

    // Get fire object state
    let fire_state_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::GetFireObjectState),
        request_id: 3,
    };
    let obs = session
        .handle_command(fire_state_packet)
        .expect("GetFireObjectState should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::FireObjectState { burning: true, .. }),
        "Should return FireObjectState with burning=true"
    );

    // Update material world
    let update_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::UpdateMaterialWorld {
            delta_time: 0.016,
        }),
        request_id: 4,
    };
    let obs = session
        .handle_command(update_packet)
        .expect("UpdateMaterialWorld should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::MaterialWorldUpdated { .. }),
        "Should return MaterialWorldUpdated observation"
    );
}

#[test]
fn property_26_living_runtime_population_commands_flow_end_to_end() {
    // Test: Population/agent commands flow end-to-end
    let mut session = stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession::new();
    session
        .initialize_vertical_slice_session()
        .expect("Session should initialize");

    // Create NPC profile
    let create_npc_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Population(PopulationCommand::CreateNpcProfile {
            npc_id: 100,
            name: "TestNPC".to_string(),
            position: [0.0, 0.0, 0.0],
        }),
        request_id: 1,
    };
    let obs = session
        .handle_command(create_npc_packet)
        .expect("CreateNpcProfile should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::NpcProfileCreated { npc_id: 100, .. }),
        "Should return NpcProfileCreated observation"
    );

    // Set NPC traits
    let traits_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Population(PopulationCommand::SetNpcTraits {
            aggression: 0.7,
            greed: 0.3,
            loyalty: 0.8,
            courage: 0.5,
            discipline: 0.6,
            sociability: 0.4,
        }),
        request_id: 2,
    };
    let obs = session
        .handle_command(traits_packet)
        .expect("SetNpcTraits should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::NpcTraitsSet),
        "Should return NpcTraitsSet observation"
    );
}

#[test]
fn property_26_living_runtime_tactics_commands_flow_end_to_end() {
    // Test: Tactics commands flow end-to-end
    let mut session = stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession::new();
    session
        .initialize_vertical_slice_session()
        .expect("Session should initialize");

    // Create squad
    let create_squad_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Tactics(TacticsCommand::CreateSquad {
            squad_id: 1,
            member_ids: vec![100, 101, 102],
            roles: vec!["leader".to_string(), "rifleman".to_string(), "medic".to_string()],
        }),
        request_id: 1,
    };
    let obs = session
        .handle_command(create_squad_packet)
        .expect("CreateSquad should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::SquadCreated { member_count: 3, .. }),
        "Should return SquadCreated with correct member count"
    );

    // Get squad tactic state
    let get_tactic_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Tactics(TacticsCommand::GetSquadTacticState),
        request_id: 2,
    };
    let obs = session
        .handle_command(get_tactic_packet)
        .expect("GetSquadTacticState should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::SquadTacticStateInfo { .. }),
        "Should return SquadTacticStateInfo observation"
    );
}

#[test]
fn property_26_living_runtime_ecology_commands_flow_end_to_end() {
    // Test: Ecology commands flow end-to-end
    let mut session = stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession::new();
    session
        .initialize_vertical_slice_session()
        .expect("Session should initialize");

    // Create creature ecology
    let create_creature_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Ecology(EcologyCommand::CreateCreatureEcology {
            creature_id: 200,
            species: "wolf".to_string(),
            position: [50.0, 0.0, 30.0],
        }),
        request_id: 1,
    };
    let obs = session
        .handle_command(create_creature_packet)
        .expect("CreateCreatureEcology should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::CreatureEcologyCreated { creature_id: 200, .. }),
        "Should return CreatureEcologyCreated observation"
    );

    // Get creature state
    let get_state_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Ecology(EcologyCommand::GetCreatureState),
        request_id: 2,
    };
    let obs = session
        .handle_command(get_state_packet)
        .expect("GetCreatureState should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::CreatureStateInfo { .. }),
        "Should return CreatureStateInfo observation"
    );
}

#[test]
fn property_26_nav_door_inventory_commands_flow_end_to_end() {
    // Test: Nav/Door/Inventory commands flow end-to-end
    let mut session = stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession::new();
    session
        .initialize_vertical_slice_session()
        .expect("Session should initialize");

    // Open door
    let open_door_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(NavDoorInventoryCommand::OpenDoor),
        request_id: 1,
    };
    let obs = session
        .handle_command(open_door_packet)
        .expect("OpenDoor should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::DoorOpened),
        "Should return DoorOpened observation"
    );

    // Add item to inventory
    let add_item_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::AddItemToInventory {
                item_id: 1,
                item_name: "TestItem".to_string(),
                item_type: "weapon".to_string(),
            },
        ),
        request_id: 2,
    };
    let obs = session
        .handle_command(add_item_packet)
        .expect("AddItemToInventory should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::ItemAddedToInventory { .. }),
        "Should return ItemAddedToInventory observation"
    );

    // Get inventory state
    let get_inv_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::GetInventoryState,
        ),
        request_id: 3,
    };
    let obs = session
        .handle_command(get_inv_packet)
        .expect("GetInventoryState should succeed");
    assert!(
        matches!(obs, EditorAuthoringObservation::InventoryStateInfo { .. }),
        "Should return InventoryStateInfo observation"
    );
}

#[test]
fn property_26_vertical_slice_session_bootstrap_and_reset() {
    // Test: Vertical slice session bootstraps and resets correctly
    let mut session = VerticalSliceSession::new().expect("Session should create");

    // Bootstrap scene
    let bootstrap_packet = VerticalSliceIngressPacket::bootstrap_scene(1);
    let observation = session
        .handle_command(bootstrap_packet)
        .expect("BootstrapScene should succeed");

    assert!(observation.scene.is_some(), "Scene DTO should exist");
    let scene = observation.scene.unwrap();
    assert!(!scene.scene_name.is_empty(), "Scene name should not be empty");
    assert!(scene.terrain.entity_id > 0, "Terrain entity should exist");

    // Reset scene
    let reset_packet = VerticalSliceIngressPacket::reset_scene(2);
    let reset_obs = session
        .handle_command(reset_packet)
        .expect("ResetScene should succeed");

    assert!(
        reset_obs.scene.is_some(),
        "Scene should exist after reset"
    );
}

#[test]
fn property_26_all_command_handlers_return_observations_not_errors() {
    // Test: Every command handler in the authoring session returns observations
    // rather than errors, ensuring end-to-end flow for all active slices
    let mut session = stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession::new();
    session
        .initialize_vertical_slice_session()
        .expect("Session should initialize");

    // Test sky commands
    let sky_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::GetSkyDiagnostics),
        request_id: 1,
    };
    assert!(
        session.handle_command(sky_packet).is_ok(),
        "Sky commands should not return errors"
    );

    // Test storm commands
    let storm_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Storm(StormCommand::ListStormFronts),
        request_id: 2,
    };
    assert!(
        session.handle_command(storm_packet).is_ok(),
        "Storm commands should not return errors"
    );

    // Test destruction commands
    let destruction_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetDestructionSummary),
        request_id: 3,
    };
    assert!(
        session.handle_command(destruction_packet).is_ok(),
        "Destruction commands should not return errors"
    );

    // Test material world commands
    let material_world_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::GetBarrelWater),
        request_id: 4,
    };
    assert!(
        session.handle_command(material_world_packet).is_ok(),
        "MaterialWorld commands should not return errors"
    );

    // Test population commands
    let population_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Population(PopulationCommand::GetCrimePressure),
        request_id: 5,
    };
    assert!(
        session.handle_command(population_packet).is_ok(),
        "Population commands should not return errors"
    );

    // Test tactics commands
    let tactics_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Tactics(TacticsCommand::CheckCoverValidity {
            cover_position: [0.0, 0.0, 0.0],
        }),
        request_id: 6,
    };
    assert!(
        session.handle_command(tactics_packet).is_ok(),
        "Tactics commands should not return errors"
    );

    // Test ecology commands
    let ecology_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Ecology(EcologyCommand::GetCreatureState),
        request_id: 7,
    };
    assert!(
        session.handle_command(ecology_packet).is_ok(),
        "Ecology commands should not return errors"
    );

    // Test NavDoorInventory commands
    let nav_packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(NavDoorInventoryCommand::GetDoorState),
        request_id: 8,
    };
    assert!(
        session.handle_command(nav_packet).is_ok(),
        "NavDoorInventory commands should not return errors"
    );
}
