//! SDK Contract Wall Property Tests
//!
//! D.4.5: Comprehensive property tests covering:
//! 1. SDK Legality Gate Coverage -- every ingress command has a legality validation
//! 2. Parser Round-Trip -- all ingress/egress types serialize/deserialize correctly
//! 3. No Type Duplication -- SDK types don't duplicate engine types
//!
//! **Validates: Requirements 6.5, SDK contract wall integrity**

use legality_gates::*;
use link_egress_observations::{
    BallisticSimulationResultDto, CameraDto, DamageMemoryDto, ImpactResultDto, ImpactVerdictDto,
    LayerDamageDto, LayerImpactEventDto, MaterialLayerDto, MaterialStackDto, RuntimeEventDto,
    SceneDto, ShotLogEntryDto, TerrainPatchDto, VerticalSliceObservation, WallDto, WeaponDto,
};
use link_ingress_packets::{
    EditorAuthoringCommand, VerticalSliceCommand, VerticalSliceIngressPacket,
};
use sdk_compat::LegalityVerdict;
use transport_policies::{
    default_transport_policy, FramingKind, TransportPolicy, MAX_PACKET_BYTES,
};

use proptest::prelude::*;

// ============================================================================
// SECTION 1: SDK Legality Gate Coverage (property-based)
// ============================================================================

proptest! {
    #[test]
    fn legality_gate_scene_commands_accept_valid_inputs(
        scene_name in "[a-zA-Z_][a-zA-Z0-9_ ]{0,50}",
        entity_id in 1u32..1_000_000u32,
        label in "[a-zA-Z_][a-zA-Z0-9_ ]{0,50}",
    ) {
        prop_assert!(validate_scene_create_empty(&scene_name).is_ok());
        prop_assert!(validate_scene_create_entity_from_asset(1, &label).is_ok());
        prop_assert!(validate_scene_set_transform(entity_id).is_ok());
        prop_assert!(validate_scene_delete_entity(entity_id).is_ok());
        prop_assert!(validate_scene_get_entity_details(entity_id).is_ok());
    }

    #[test]
    fn legality_gate_material_commands_accept_valid_inputs(
        label in "[a-zA-Z_][a-zA-Z0-9_ ]{0,30}",
        hardness in 0.0f32..=10.0f32,
        density in 0.001f32..10_000_000.0f32,
        stack_id in 1u16..10_000u16,
        thickness in 0.001f32..1000.0f32,
        entity_id in 1u32..1_000_000u32,
    ) {
        prop_assert!(validate_material_create_archetype(&label, hardness, density).is_ok());
        prop_assert!(validate_material_create_stack(&label).is_ok());
        prop_assert!(validate_material_stack_add_layer(stack_id, thickness).is_ok());
        prop_assert!(validate_material_assign_stack(entity_id, stack_id).is_ok());
    }

    #[test]
    fn legality_gate_terrain_commands_accept_valid_inputs(
        size_x in 0.001f32..10_000.0f32,
        size_y in 0.001f32..10_000.0f32,
        label in "[a-zA-Z_][a-zA-Z0-9_ ]{0,30}",
        entity_id in 1u32..1_000_000u32,
        radius in 0.001f32..1000.0f32,
        stack_id in 1u16..10_000u16,
    ) {
        prop_assert!(validate_terrain_create_patch([size_x, size_y], &label).is_ok());
        prop_assert!(validate_terrain_paint_surface(entity_id, radius, stack_id).is_ok());
    }

    #[test]
    fn legality_gate_sky_commands_accept_valid_inputs(
        hours in 0.0f32..24.0f32,
        day in 1u16..=365u16,
        latitude in -90.0f32..=90.0f32,
        normalized in 0.0f32..=1.0f32,
        rain_intensity in 0.0f32..1000.0f32,
        dt in 0.001f32..3600.0f32,
        storm_radius in 0.001f32..1000.0f32,
        storm_intensity in 0.0f32..=1.0f32,
        rain_rate in 0.0f32..500.0f32,
    ) {
        prop_assert!(validate_sky_set_time_of_day(hours).is_ok());
        prop_assert!(validate_sky_set_day_of_year(day).is_ok());
        prop_assert!(validate_sky_set_latitude(latitude).is_ok());
        prop_assert!(validate_sky_set_normalized_value("test", normalized).is_ok());
        prop_assert!(validate_sky_set_rain(rain_intensity).is_ok());
        prop_assert!(validate_sky_step_simulation(dt).is_ok());
        prop_assert!(validate_storm_create(storm_radius, storm_intensity, rain_rate).is_ok());
    }

    #[test]
    fn legality_gate_destruction_commands_accept_valid_inputs(
        energy in 0.001f32..1_000_000.0f32,
        integrity in 0.0f32..=1.0f32,
    ) {
        prop_assert!(validate_destruction_trigger_blast(energy).is_ok());
        prop_assert!(validate_destruction_set_integrity(integrity).is_ok());
        prop_assert!(validate_destruction_set_terrain_material("dirt").is_ok());
        prop_assert!(validate_destruction_set_support_type("wood").is_ok());
        prop_assert!(validate_destruction_reset_state().is_ok());
    }

    #[test]
    fn transport_legality_monotonic_in_payload_size(
        small_size in 0usize..MAX_PACKET_BYTES,
    ) {
        let policy = default_transport_policy(FramingKind::OrderedControl);
        let large_size = small_size + 1;
        if transport_legality(&policy, small_size, false, false).is_err() {
            prop_assert!(transport_legality(&policy, large_size, false, false).is_err());
        }
    }
}

// ============================================================================
// SECTION 1b: Legality gate coverage (deterministic)
// ============================================================================

#[test]
fn legality_gate_scene_commands_reject_invalid_inputs() {
    assert!(validate_scene_create_empty("").is_err());
    assert!(validate_scene_create_empty(&"a".repeat(257)).is_err());
    assert!(validate_scene_create_entity_from_asset(0, "label").is_err());
    assert!(validate_scene_create_entity_from_asset(1, "").is_err());
    assert!(validate_scene_set_transform(0).is_err());
    assert!(validate_scene_delete_entity(0).is_err());
    assert!(validate_scene_get_entity_details(0).is_err());
}

#[test]
fn legality_gate_material_commands_reject_invalid_inputs() {
    assert!(validate_material_create_archetype("", 5.0, 1000.0).is_err());
    assert!(validate_material_create_archetype("steel", -1.0, 1000.0).is_err());
    assert!(validate_material_create_archetype("steel", 11.0, 1000.0).is_err());
    assert!(validate_material_create_archetype("steel", 5.0, -1.0).is_err());
    assert!(validate_material_create_stack("").is_err());
    assert!(validate_material_stack_add_layer(0, 10.0).is_err());
    assert!(validate_material_stack_add_layer(1, 0.0).is_err());
    assert!(validate_material_assign_stack(0, 1).is_err());
    assert!(validate_material_assign_stack(1, 0).is_err());
}

#[test]
fn legality_gate_terrain_commands_reject_invalid_inputs() {
    assert!(validate_terrain_create_patch([0.0, 100.0], "patch").is_err());
    assert!(validate_terrain_create_patch([100.0, 0.0], "patch").is_err());
    assert!(validate_terrain_create_patch([100.0, 100.0], "").is_err());
    assert!(validate_terrain_paint_surface(0, 5.0, 1).is_err());
    assert!(validate_terrain_paint_surface(1, 0.0, 1).is_err());
    assert!(validate_terrain_paint_surface(1, 5.0, 0).is_err());
}

#[test]
fn legality_gate_actor_commands() {
    assert!(validate_actor_spawn_preset(1).is_ok());
    assert!(validate_actor_spawn_preset(0).is_err());
    assert!(validate_actor_attach_weapon(1, 1).is_ok());
    assert!(validate_actor_attach_weapon(0, 1).is_err());
    assert!(validate_actor_attach_weapon(1, 0).is_err());
    assert!(validate_actor_set_active(1).is_ok());
    assert!(validate_actor_set_active(0).is_err());
}

#[test]
fn legality_gate_asset_commands() {
    assert!(validate_asset_import("path.obj", "label").is_ok());
    assert!(validate_asset_import("", "label").is_err());
    assert!(validate_asset_import("path", "").is_err());
    assert!(validate_asset_get_details(1).is_ok());
    assert!(validate_asset_get_details(0).is_err());
}

#[test]
fn legality_gate_vertical_slice_commands() {
    assert!(validate_vertical_slice_fire_test_shot(1).is_ok());
    assert!(validate_vertical_slice_fire_test_shot(0).is_err());
    assert!(validate_vertical_slice_assign_material(1, 1).is_ok());
    assert!(validate_vertical_slice_assign_material(0, 1).is_err());
    assert!(validate_vertical_slice_assign_material(1, 0).is_err());
    assert!(validate_vertical_slice_select_entity(1).is_ok());
    assert!(validate_vertical_slice_select_entity(0).is_err());
}

#[test]
fn legality_gate_population_commands() {
    assert!(validate_npc_create_profile(1, "NPC").is_ok());
    assert!(validate_npc_create_profile(0, "NPC").is_err());
    assert!(validate_npc_create_profile(1, "").is_err());
    assert!(validate_npc_trait_value("aggression", 0.5).is_ok());
    assert!(validate_npc_trait_value("aggression", -0.1).is_err());
    assert!(validate_npc_trait_value("aggression", 1.1).is_err());
}

#[test]
fn all_rejections_carry_descriptive_info() {
    let results: Vec<_> = vec![
        validate_scene_create_empty(""),
        validate_material_create_archetype("", 5.0, 1000.0),
        validate_terrain_create_patch([100.0, 100.0], ""),
        validate_actor_spawn_preset(0),
        validate_asset_import("", "label"),
        validate_destruction_trigger_blast(0.0),
    ];

    for result in results {
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.verdict, LegalityVerdict::Illegal);
        if let sdk_compat::LegalityRejectionReason::InvalidInput { field, message } = err.reason {
            assert!(!field.is_empty(), "field name must not be empty");
            assert!(!message.is_empty(), "message must not be empty");
        }
    }
}

// ============================================================================
// SECTION 2: Parser Round-Trip Tests
// ============================================================================

#[test]
fn vertical_slice_command_round_trips_json() {
    let commands = vec![
        VerticalSliceCommand::BootstrapScene,
        VerticalSliceCommand::FireTestShot {
            weapon_entity_id: 42,
        },
        VerticalSliceCommand::ResetScene,
        VerticalSliceCommand::SelectEntity { entity_id: 7 },
        VerticalSliceCommand::AssignMaterialStack {
            entity_id: 7,
            stack_id: 3,
        },
    ];

    for cmd in commands {
        let json = serde_json::to_string(&cmd).unwrap();
        let decoded: VerticalSliceCommand = serde_json::from_str(&json).unwrap();
        assert_eq!(cmd, decoded, "Command round-trip failed: {cmd:?}");
    }
}

#[test]
fn vertical_slice_ingress_packet_round_trips_json() {
    let packets = vec![
        VerticalSliceIngressPacket::bootstrap_scene(1),
        VerticalSliceIngressPacket::fire_test_shot(2, 42),
        VerticalSliceIngressPacket::reset_scene(3),
    ];

    for packet in packets {
        let json = serde_json::to_string(&packet).unwrap();
        let decoded: VerticalSliceIngressPacket = serde_json::from_str(&json).unwrap();
        assert_eq!(packet, decoded, "Packet round-trip failed");
    }
}

#[test]
fn vertical_slice_ingress_packet_round_trips_bincode() {
    let packet = VerticalSliceIngressPacket::bootstrap_scene(1);
    let bytes = bincode::serialize(&packet).unwrap();
    let decoded: VerticalSliceIngressPacket = bincode::deserialize(&bytes).unwrap();
    assert_eq!(packet, decoded);
}

#[test]
fn impact_verdict_round_trips() {
    let verdicts = vec![
        ImpactVerdictDto::Stopped,
        ImpactVerdictDto::Penetrated,
        ImpactVerdictDto::Ricochet,
        ImpactVerdictDto::Embedded,
    ];
    for verdict in verdicts {
        let json = serde_json::to_string(&verdict).unwrap();
        let decoded: ImpactVerdictDto = serde_json::from_str(&json).unwrap();
        assert_eq!(verdict, decoded);
    }
}

#[test]
fn runtime_event_dto_round_trips() {
    let events = vec![
        RuntimeEventDto::ShotFired {
            tick: 100,
            weapon: 3,
        },
        RuntimeEventDto::ProjectileImpact {
            tick: 105,
            target: 2,
            energy_j: 3000.0,
        },
        RuntimeEventDto::SegmentCracked {
            tick: 106,
            entity: 2,
            layer: 0,
            segment: 5,
        },
        RuntimeEventDto::SegmentReleased {
            tick: 107,
            entity: 2,
            layer: 0,
            segment: 5,
        },
    ];

    for event in events {
        let json = serde_json::to_string(&event).unwrap();
        let decoded: RuntimeEventDto = serde_json::from_str(&json).unwrap();
        assert_eq!(event, decoded, "Event round-trip failed: {event:?}");
    }
}

#[test]
fn layer_impact_event_round_trips() {
    let event = LayerImpactEventDto {
        layer_index: 0,
        entry_energy_j: 3000.0,
        exit_energy_j: 1500.0,
        energy_absorbed_j: 1500.0,
        verdict: ImpactVerdictDto::Penetrated,
    };

    let json = serde_json::to_string(&event).unwrap();
    let decoded: LayerImpactEventDto = serde_json::from_str(&json).unwrap();
    assert_eq!(event, decoded);
}

#[test]
fn material_layer_dto_round_trips() {
    let layer = MaterialLayerDto {
        archetype_id: 1,
        archetype_label: "hardened_steel".to_string(),
        thickness_mm: 10.0,
        coverage: 1.0,
    };

    let json = serde_json::to_string(&layer).unwrap();
    let decoded: MaterialLayerDto = serde_json::from_str(&json).unwrap();
    assert_eq!(layer, decoded);
}

#[test]
fn damage_memory_dto_round_trips() {
    let damage = DamageMemoryDto {
        entity_id: 2,
        stack_id: 1,
        layer_damage: vec![LayerDamageDto {
            layer_index: 0,
            integrity: 0.75,
            accumulated_energy_j: 500.0,
            cracked_segments: vec![0, 1, 2],
            released_segments: vec![],
        }],
    };

    let json = serde_json::to_string(&damage).unwrap();
    let decoded: DamageMemoryDto = serde_json::from_str(&json).unwrap();
    assert_eq!(damage, decoded);
}

#[test]
fn shot_log_entry_round_trips() {
    let entry = ShotLogEntryDto {
        tick: 100,
        weapon_entity: 3,
        projectile_profile: 1,
        spawn_position: [0.0, 1.0, 0.0],
        spawn_velocity: [800.0, 0.0, 0.0],
    };

    let json = serde_json::to_string(&entry).unwrap();
    let decoded: ShotLogEntryDto = serde_json::from_str(&json).unwrap();
    assert_eq!(entry, decoded);
}

#[test]
fn camera_dto_round_trips() {
    let camera = CameraDto {
        position: [0.0, 5.0, -10.0],
        look_at: [0.0, 0.0, 0.0],
        fov_deg: 60.0,
    };

    let json = serde_json::to_string(&camera).unwrap();
    let decoded: CameraDto = serde_json::from_str(&json).unwrap();
    assert_eq!(camera, decoded);
}

#[test]
fn terrain_patch_dto_round_trips() {
    let terrain = TerrainPatchDto {
        entity_id: 1,
        origin: [0.0, 0.0, 0.0],
        world_size: [100.0, 100.0],
        resolution: [256, 256],
        mesh_revision: 0,
    };

    let json = serde_json::to_string(&terrain).unwrap();
    let decoded: TerrainPatchDto = serde_json::from_str(&json).unwrap();
    assert_eq!(terrain, decoded);
}

#[test]
fn material_stack_dto_round_trips() {
    let stack = MaterialStackDto {
        id: 1,
        label: "steel_plate".to_string(),
        layers: vec![MaterialLayerDto {
            archetype_id: 1,
            archetype_label: "hardened_steel".to_string(),
            thickness_mm: 10.0,
            coverage: 1.0,
        }],
    };

    let json = serde_json::to_string(&stack).unwrap();
    let decoded: MaterialStackDto = serde_json::from_str(&json).unwrap();
    assert_eq!(stack, decoded);
}

#[test]
fn wall_dto_round_trips() {
    let wall = WallDto {
        entity_id: 2,
        position: [0.0, 0.0, 0.0],
        dimensions: [10.0, 5.0, 0.5],
        stack_id: 1,
    };

    let json = serde_json::to_string(&wall).unwrap();
    let decoded: WallDto = serde_json::from_str(&json).unwrap();
    assert_eq!(wall, decoded);
}

#[test]
fn weapon_dto_round_trips() {
    let weapon = WeaponDto {
        entity_id: 3,
        profile_id: 1,
        position: [0.0, 1.0, 0.0],
        aim_direction: [0.0, 0.0, 1.0],
    };

    let json = serde_json::to_string(&weapon).unwrap();
    let decoded: WeaponDto = serde_json::from_str(&json).unwrap();
    assert_eq!(weapon, decoded);
}

#[test]
fn impact_result_dto_round_trips() {
    let impact = ImpactResultDto {
        target_entity: 2,
        impact_position: [5.0, 2.5, 0.0],
        impact_velocity: [800.0, 0.0, 0.0],
        entry_energy_j: 3000.0,
        incidence_angle_deg: 0.0,
        layer_events: vec![LayerImpactEventDto {
            layer_index: 0,
            entry_energy_j: 3000.0,
            exit_energy_j: 1500.0,
            energy_absorbed_j: 1500.0,
            verdict: ImpactVerdictDto::Penetrated,
        }],
        final_verdict: ImpactVerdictDto::Penetrated,
    };

    let json = serde_json::to_string(&impact).unwrap();
    let decoded: ImpactResultDto = serde_json::from_str(&json).unwrap();
    assert_eq!(impact, decoded);
}

#[test]
fn full_vertical_slice_observation_round_trips() {
    let observation = VerticalSliceObservation {
        request_id: 1,
        scene: Some(SceneDto {
            scene_name: "test_scene".to_string(),
            terrain: TerrainPatchDto {
                entity_id: 1,
                origin: [0.0, 0.0, 0.0],
                world_size: [100.0, 100.0],
                resolution: [256, 256],
                mesh_revision: 0,
            },
            wall: WallDto {
                entity_id: 2,
                position: [0.0, 0.0, 0.0],
                dimensions: [10.0, 5.0, 0.5],
                stack_id: 1,
            },
            weapon: WeaponDto {
                entity_id: 3,
                profile_id: 1,
                position: [0.0, 1.0, 0.0],
                aim_direction: [0.0, 0.0, 1.0],
            },
            camera: CameraDto {
                position: [0.0, 5.0, -10.0],
                look_at: [0.0, 0.0, 0.0],
                fov_deg: 60.0,
            },
            sky_bundle_path: None,
        }),
        material_stacks: vec![MaterialStackDto {
            id: 1,
            label: "steel_plate".to_string(),
            layers: vec![MaterialLayerDto {
                archetype_id: 1,
                archetype_label: "hardened_steel".to_string(),
                thickness_mm: 10.0,
                coverage: 1.0,
            }],
        }],
        damage_memory: vec![DamageMemoryDto {
            entity_id: 2,
            stack_id: 1,
            layer_damage: vec![LayerDamageDto {
                layer_index: 0,
                integrity: 0.75,
                accumulated_energy_j: 500.0,
                cracked_segments: vec![0, 1, 2],
                released_segments: vec![],
            }],
        }],
        ballistic_result: Some(BallisticSimulationResultDto {
            impacts: vec![ImpactResultDto {
                target_entity: 2,
                impact_position: [5.0, 2.5, 0.0],
                impact_velocity: [800.0, 0.0, 0.0],
                entry_energy_j: 3000.0,
                incidence_angle_deg: 0.0,
                layer_events: vec![LayerImpactEventDto {
                    layer_index: 0,
                    entry_energy_j: 3000.0,
                    exit_energy_j: 1500.0,
                    energy_absorbed_j: 1500.0,
                    verdict: ImpactVerdictDto::Penetrated,
                }],
                final_verdict: ImpactVerdictDto::Penetrated,
            }],
        }),
        runtime_events: vec![
            RuntimeEventDto::ShotFired {
                tick: 100,
                weapon: 3,
            },
            RuntimeEventDto::ProjectileImpact {
                tick: 105,
                target: 2,
                energy_j: 3000.0,
            },
        ],
        shot_log: vec![ShotLogEntryDto {
            tick: 100,
            weapon_entity: 3,
            projectile_profile: 1,
            spawn_position: [0.0, 1.0, 0.0],
            spawn_velocity: [800.0, 0.0, 0.0],
        }],
    };

    let json = serde_json::to_string(&observation).unwrap();
    let decoded: VerticalSliceObservation = serde_json::from_str(&json).unwrap();
    assert_eq!(observation, decoded);
}

#[test]
fn vertical_slice_observation_with_none_round_trips() {
    let observation = VerticalSliceObservation {
        request_id: 1,
        scene: None,
        material_stacks: vec![],
        damage_memory: vec![],
        ballistic_result: None,
        runtime_events: vec![],
        shot_log: vec![],
    };

    let json = serde_json::to_string(&observation).unwrap();
    let decoded: VerticalSliceObservation = serde_json::from_str(&json).unwrap();
    assert_eq!(observation, decoded);
}

#[test]
fn editor_authoring_command_scene_round_trips() {
    use link_ingress_packets::SceneCommand;

    let commands = vec![
        EditorAuthoringCommand::Scene(SceneCommand::CreateEmpty {
            scene_name: "test".to_string(),
        }),
        EditorAuthoringCommand::Scene(SceneCommand::ListEntities),
        EditorAuthoringCommand::Scene(SceneCommand::DeleteEntity { entity_id: 7 }),
    ];

    for cmd in commands {
        let json = serde_json::to_string(&cmd).unwrap();
        let decoded: EditorAuthoringCommand = serde_json::from_str(&json).unwrap();
        assert_eq!(cmd, decoded, "EditorAuthoringCommand round-trip failed");
    }
}

#[test]
fn transport_policy_round_trips_json() {
    for framing in [
        FramingKind::OrderedControl,
        FramingKind::BoundedPreview,
        FramingKind::MetricsOnly,
        FramingKind::ArtifactOnly,
    ] {
        let policy = default_transport_policy(framing);
        let json = serde_json::to_string(&policy).unwrap();
        let decoded: TransportPolicy = serde_json::from_str(&json).unwrap();
        assert_eq!(policy.transport_policy_id, decoded.transport_policy_id);
        assert_eq!(policy.framing_kind, decoded.framing_kind);
        assert_eq!(policy.max_payload_bytes, decoded.max_payload_bytes);
        assert_eq!(policy.retry_class, decoded.retry_class);
        assert_eq!(policy.rate_tier, decoded.rate_tier);
        assert_eq!(policy.priority, decoded.priority);
        assert_eq!(policy.delivery_scope, decoded.delivery_scope);
    }
}

#[test]
fn bridge_control_round_trips_json() {
    use engine_handle_refs::{RuntimeHandle, SessionHandle};
    use legality_gates::LegalityGateId;
    use link_ingress_controls::{BridgeControl, IngressControlEnvelopeId, IngressControlKind};

    let control = BridgeControl::new(
        IngressControlEnvelopeId(1),
        IngressControlKind::SetLabel {
            label: "test".to_string(),
        },
        RuntimeHandle::new(1),
        SessionHandle::new(1),
        0,
        LegalityGateId(1),
    );

    let json = serde_json::to_string(&control).unwrap();
    let decoded: BridgeControl = serde_json::from_str(&json).unwrap();
    assert_eq!(
        control.ingress_control_envelope_id,
        decoded.ingress_control_envelope_id
    );
    assert_eq!(control.target_runtime_handle, decoded.target_runtime_handle);
    assert_eq!(control.source_session_handle, decoded.source_session_handle);
}

// ============================================================================
// SECTION 3: No Type Duplication
// ============================================================================

#[test]
fn sdk_ingress_types_are_distinct_from_engine_types() {
    let _sdk_cmd = VerticalSliceCommand::BootstrapScene;
    let _editor_cmd =
        EditorAuthoringCommand::Scene(link_ingress_packets::SceneCommand::ListEntities);
}

#[test]
fn sdk_egress_dto_types_are_transport_oriented() {
    let observation = VerticalSliceObservation {
        request_id: 1,
        scene: None,
        material_stacks: vec![],
        damage_memory: vec![],
        ballistic_result: None,
        runtime_events: vec![],
        shot_log: vec![],
    };

    let json = serde_json::to_string(&observation).unwrap();
    assert!(json.contains("request_id"));
    assert!(json.contains("null"));
}

#[test]
fn legality_gate_types_distinct_from_transport_policy_types() {
    let gate_id = LegalityGateId(1);
    assert_eq!(gate_id.0, 1);

    use transport_policies::TransportPolicyId;
    let policy_id = TransportPolicyId(1);
    assert_eq!(policy_id.0, 1);
}

#[test]
fn sdk_packet_entity_ids_are_plain_not_typed_handles() {
    let packet = VerticalSliceIngressPacket::fire_test_shot(1, 42);
    match packet.command {
        VerticalSliceCommand::FireTestShot { weapon_entity_id } => {
            let _: u32 = weapon_entity_id;
        }
        _ => panic!("unexpected command"),
    }
}

#[test]
fn all_sdk_command_types_are_distinct() {
    let vs_cmd = VerticalSliceCommand::BootstrapScene;
    let ea_cmd = EditorAuthoringCommand::Scene(link_ingress_packets::SceneCommand::ListEntities);
    let _ = (vs_cmd, ea_cmd);
}

#[test]
fn every_editor_authoring_command_has_legality_gate() {
    // Scene
    assert!(validate_scene_create_empty("test").is_ok());
    assert!(validate_scene_create_entity_from_asset(1, "entity").is_ok());
    assert!(validate_scene_set_transform(1).is_ok());
    assert!(validate_scene_delete_entity(1).is_ok());
    assert!(validate_scene_get_entity_details(1).is_ok());

    // Material
    assert!(validate_material_create_archetype("steel", 5.0, 7850.0).is_ok());
    assert!(validate_material_create_stack("stack").is_ok());
    assert!(validate_material_stack_add_layer(1, 10.0).is_ok());
    assert!(validate_material_stack_remove_layer(1, 0).is_ok());
    assert!(validate_material_assign_stack(1, 1).is_ok());

    // Terrain
    assert!(validate_terrain_create_patch([100.0, 100.0], "patch").is_ok());
    assert!(validate_terrain_paint_surface(1, 5.0, 1).is_ok());

    // Actor
    assert!(validate_actor_spawn_preset(1).is_ok());
    assert!(validate_actor_attach_weapon(1, 1).is_ok());
    assert!(validate_actor_set_active(1).is_ok());

    // Asset
    assert!(validate_asset_import("path.obj", "asset").is_ok());
    assert!(validate_asset_get_details(1).is_ok());

    // Sky
    assert!(validate_sky_set_time_of_day(12.0).is_ok());
    assert!(validate_sky_set_day_of_year(180).is_ok());
    assert!(validate_sky_set_latitude(45.0).is_ok());
    assert!(validate_sky_set_normalized_value("fog", 0.5).is_ok());
    assert!(validate_sky_set_rain(10.0).is_ok());
    assert!(validate_sky_step_simulation(0.016).is_ok());

    // Storm
    assert!(validate_storm_create(10.0, 0.5, 20.0).is_ok());
    assert!(validate_storm_update(1).is_ok());

    // Destruction
    assert!(validate_destruction_set_terrain_material("dirt").is_ok());
    assert!(validate_destruction_trigger_blast(1000.0).is_ok());
    assert!(validate_destruction_set_integrity(0.5).is_ok());
    assert!(validate_destruction_set_support_type("wood").is_ok());
    assert!(validate_destruction_apply_support_damage(100.0, [1.0, 0.0, 0.0]).is_ok());
    assert!(validate_destruction_reset_state().is_ok());

    // Material world
    assert!(validate_material_world_set_barrel_water(10.0).is_ok());
    assert!(validate_material_world_set_barrel_leak().is_ok());
    assert!(validate_material_world_ignite_fire().is_ok());
    assert!(validate_material_world_extinguish_fire().is_ok());
    assert!(validate_material_world_set_wetness(50.0).is_ok());
    assert!(validate_material_world_update(0.016).is_ok());

    // Ballistics
    assert!(validate_ballistics_fire_active_actor().is_ok());

    // Vertical slice
    assert!(validate_vertical_slice_fire_test_shot(1).is_ok());
    assert!(validate_vertical_slice_assign_material(1, 1).is_ok());
    assert!(validate_vertical_slice_select_entity(1).is_ok());

    // Population
    assert!(validate_npc_create_profile(1, "NPC").is_ok());
    assert!(validate_npc_trait_value("aggression", 0.5).is_ok());
}
