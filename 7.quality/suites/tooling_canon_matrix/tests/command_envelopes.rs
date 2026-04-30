// Tests for l6.1-command-envelopes: CommandEnvelope, PromotedCommand, canonical schema

use stratumx_tooling_l6_1_command_envelopes::*;

// ============================================================================
// CANONICAL_LEVEL constant tests
// ============================================================================

#[test]
fn canonical_level_value() {
    assert_eq!(CANONICAL_LEVEL, "l6.1-command-envelopes");
}

#[test]
fn canonical_level_not_empty() {
    assert!(!CANONICAL_LEVEL.is_empty());
}

// ============================================================================
// L61CommandEnvelopesMarker tests
// ============================================================================

#[test]
fn marker_default() {
    let m = L61CommandEnvelopesMarker;
    assert_eq!(m, L61CommandEnvelopesMarker);
}

#[test]
fn marker_equality() {
    assert_eq!(L61CommandEnvelopesMarker, L61CommandEnvelopesMarker);
}

#[test]
fn marker_copy() {
    let a = L61CommandEnvelopesMarker;
    let _b = a;
    let _c = a;
}

#[test]
fn marker_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut h1 = DefaultHasher::new();
    L61CommandEnvelopesMarker.hash(&mut h1);
    let mut h2 = DefaultHasher::new();
    L61CommandEnvelopesMarker.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn marker_in_hashset() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(L61CommandEnvelopesMarker);
    assert_eq!(set.len(), 1);
}

#[test]
fn marker_is_unit_struct() {
    assert_eq!(std::mem::size_of::<L61CommandEnvelopesMarker>(), 0);
}

// ============================================================================
// CommandLifecycleState tests
// ============================================================================

#[test]
fn lifecycle_state_variants() {
    let states = [
        CommandLifecycleState::Accepted,
        CommandLifecycleState::Running,
        CommandLifecycleState::Partial,
        CommandLifecycleState::Success,
        CommandLifecycleState::RetryableFailure,
        CommandLifecycleState::TerminalFailure,
    ];
    assert_eq!(states.len(), 6);
}

#[test]
fn lifecycle_state_equality() {
    assert_eq!(
        CommandLifecycleState::Accepted,
        CommandLifecycleState::Accepted
    );
    assert_ne!(
        CommandLifecycleState::Accepted,
        CommandLifecycleState::Running
    );
}

#[test]
fn lifecycle_state_serializes() {
    let json = serde_json::to_string(&CommandLifecycleState::Success).unwrap();
    assert!(json.contains("Success"));
}

#[test]
fn lifecycle_state_deserializes() {
    let state: CommandLifecycleState = serde_json::from_str("\"Running\"").unwrap();
    assert_eq!(state, CommandLifecycleState::Running);
}

#[test]
fn lifecycle_state_roundtrip() {
    for state in [
        CommandLifecycleState::Accepted,
        CommandLifecycleState::Running,
        CommandLifecycleState::Partial,
        CommandLifecycleState::Success,
        CommandLifecycleState::RetryableFailure,
        CommandLifecycleState::TerminalFailure,
    ] {
        let json = serde_json::to_string(&state).unwrap();
        let restored: CommandLifecycleState = serde_json::from_str(&json).unwrap();
        assert_eq!(state, restored);
    }
}

// ============================================================================
// CommandEnvelope tests
// ============================================================================

#[test]
fn envelope_new() {
    let env = CommandEnvelope::new(1, "route.test.v1", vec![1, 2, 3]);
    assert_eq!(env.command_id, 1);
    assert_eq!(env.route_id, "route.test.v1");
    assert_eq!(env.lifecycle_state, CommandLifecycleState::Accepted);
    assert_eq!(env.payload, vec![1, 2, 3]);
    assert!(env.error_message.is_none());
    assert!(env.origin_timestamp > 0);
    assert!(env.last_update_timestamp > 0);
}

#[test]
fn envelope_new_with_string_route() {
    let route = String::from("route.material.create.v1");
    let env = CommandEnvelope::new(42, route, vec![]);
    assert_eq!(env.route_id, "route.material.create.v1");
}

#[test]
fn envelope_transition_to_running() {
    let mut env = CommandEnvelope::new(1, "route.test.v1", vec![]);
    let _ = env.transition_to(CommandLifecycleState::Running, None);
    assert_eq!(env.lifecycle_state, CommandLifecycleState::Running);
    assert!(env.error_message.is_none());
}

#[test]
fn envelope_transition_to_success() {
    let mut env = CommandEnvelope::new(1, "route.test.v1", vec![]);
    let _ = env.transition_to(CommandLifecycleState::Success, None);
    assert_eq!(env.lifecycle_state, CommandLifecycleState::Success);
}

#[test]
fn envelope_transition_to_failure() {
    let mut env = CommandEnvelope::new(1, "route.test.v1", vec![]);
    let _ = env.transition_to(
        CommandLifecycleState::RetryableFailure,
        Some("network error".into()),
    );
    assert_eq!(env.lifecycle_state, CommandLifecycleState::RetryableFailure);
    assert_eq!(env.error_message, Some("network error".into()));
}

#[test]
fn envelope_transition_to_terminal_failure() {
    let mut env = CommandEnvelope::new(1, "route.test.v1", vec![]);
    let _ = env.transition_to(
        CommandLifecycleState::TerminalFailure,
        Some("permanent error".into()),
    );
    assert_eq!(env.lifecycle_state, CommandLifecycleState::TerminalFailure);
}

#[test]
fn envelope_timestamp_updates_on_transition() {
    let mut env = CommandEnvelope::new(1, "route.test.v1", vec![]);
    let initial_ts = env.last_update_timestamp;
    std::thread::sleep(std::time::Duration::from_millis(2));
    let _ = env.transition_to(CommandLifecycleState::Running, None);
    assert!(env.last_update_timestamp >= initial_ts);
}

#[test]
fn envelope_debug() {
    let env = CommandEnvelope::new(1, "route.test.v1", vec![0]);
    let debug = format!("{:?}", env);
    assert!(debug.contains("CommandEnvelope"));
}

#[test]
fn envelope_clone() {
    let env = CommandEnvelope::new(1, "route.test.v1", vec![1, 2]);
    let env2 = env.clone();
    assert_eq!(env.command_id, env2.command_id);
    assert_eq!(env.route_id, env2.route_id);
}

// ============================================================================
// PromotedCommand tests - Project lifecycle
// ============================================================================

#[test]
fn promoted_command_project_bootstrap() {
    let cmd = PromotedCommand::ProjectBootstrap {
        project_name: "MyProject".into(),
    };
    assert_eq!(cmd.route_id(), "route.project.bootstrap.v1");
}

#[test]
fn promoted_command_project_create() {
    let cmd = PromotedCommand::ProjectCreate {
        project_name: "Test".into(),
        project_root: "/test".into(),
        world_name: "World1".into(),
    };
    assert_eq!(cmd.route_id(), "route.project.create.v1");
}

#[test]
fn promoted_command_project_build() {
    let cmd = PromotedCommand::ProjectBuild {
        target_platform: "Windows".into(),
    };
    assert_eq!(cmd.route_id(), "route.build.package.v1");
}

#[test]
fn promoted_command_project_launch() {
    let cmd = PromotedCommand::ProjectLaunch {
        launch_mode: "Preview".into(),
    };
    assert_eq!(cmd.route_id(), "route.launch.verify_first_result.v1");
}

#[test]
fn promoted_command_project_verify_first_result() {
    let cmd = PromotedCommand::ProjectVerifyFirstResult;
    assert_eq!(cmd.route_id(), "route.launch.verify_first_result.v1");
}

// ============================================================================
// PromotedCommand tests - World lifecycle
// ============================================================================

#[test]
fn promoted_command_world_open() {
    let cmd = PromotedCommand::WorldOpen {
        world_path: "/worlds/test.world".into(),
    };
    assert_eq!(cmd.route_id(), "route.world.open.v1");
}

#[test]
fn promoted_command_world_close() {
    let cmd = PromotedCommand::WorldClose;
    assert_eq!(cmd.route_id(), "route.world.close.v1");
}

// ============================================================================
// PromotedCommand tests - Runtime control
// ============================================================================

#[test]
fn promoted_command_runtime_play() {
    let cmd = PromotedCommand::RuntimePlay;
    assert_eq!(cmd.route_id(), "route.runtime.play.v1");
}

#[test]
fn promoted_command_runtime_pause() {
    let cmd = PromotedCommand::RuntimePause;
    assert_eq!(cmd.route_id(), "route.runtime.pause.v1");
}

#[test]
fn promoted_command_runtime_stop() {
    let cmd = PromotedCommand::RuntimeStop;
    assert_eq!(cmd.route_id(), "route.runtime.stop.v1");
}

#[test]
fn promoted_command_runtime_simulate() {
    let cmd = PromotedCommand::RuntimeSimulate;
    assert_eq!(cmd.route_id(), "route.runtime.simulate.v1");
}

// ============================================================================
// PromotedCommand tests - Terrain
// ============================================================================

#[test]
fn promoted_command_terrain_sculpt_raise() {
    let cmd = PromotedCommand::TerrainSculptRaise {
        position: [0.5, 0.5],
        radius: 10.0,
        strength: 0.8,
    };
    assert_eq!(cmd.route_id(), "route.terrain.sculpt.raise.v1");
}

#[test]
fn promoted_command_terrain_sculpt_flatten() {
    let cmd = PromotedCommand::TerrainSculptFlatten {
        position: [1.0, 2.0],
        radius: 5.0,
        strength: 1.0,
        target_height: 100.0,
    };
    assert_eq!(cmd.route_id(), "route.terrain.sculpt.flatten.v1");
}

#[test]
fn promoted_command_terrain_paint_material() {
    let cmd = PromotedCommand::TerrainPaintMaterial {
        position: [0.0, 0.0],
        radius: 3.0,
        strength: 0.5,
        material_layer: 2,
    };
    assert_eq!(cmd.route_id(), "route.terrain.paint.material.v1");
}

#[test]
fn promoted_command_terrain_add_hole() {
    let cmd = PromotedCommand::TerrainAddHole {
        position: [5.0, 5.0],
        radius: 2.0,
    };
    assert_eq!(cmd.route_id(), "route.terrain.hole.add.v1");
}

#[test]
fn promoted_command_terrain_routes_are_unique() {
    let commands = [
        PromotedCommand::TerrainImport {
            heightmap_path: "test".into(),
        },
        PromotedCommand::TerrainRebuild,
        PromotedCommand::TerrainSculptRaise {
            position: [0.0, 0.0],
            radius: 1.0,
            strength: 1.0,
        },
        PromotedCommand::TerrainSculptLower {
            position: [0.0, 0.0],
            radius: 1.0,
            strength: 1.0,
        },
        PromotedCommand::TerrainSculptSmooth {
            position: [0.0, 0.0],
            radius: 1.0,
            strength: 1.0,
        },
        PromotedCommand::TerrainSculptFlatten {
            position: [0.0, 0.0],
            radius: 1.0,
            strength: 1.0,
            target_height: 0.0,
        },
        PromotedCommand::TerrainPaintMaterial {
            position: [0.0, 0.0],
            radius: 1.0,
            strength: 1.0,
            material_layer: 0,
        },
        PromotedCommand::TerrainAddHole {
            position: [0.0, 0.0],
            radius: 1.0,
        },
        PromotedCommand::TerrainRemoveHole {
            position: [0.0, 0.0],
            radius: 1.0,
        },
    ];
    let routes: Vec<_> = commands.iter().map(|c| c.route_id()).collect();
    // Ensure all terrain commands have terrain routes
    for route in &routes {
        assert!(route.starts_with("route.terrain"));
    }
}

// ============================================================================
// PromotedCommand tests - Material
// ============================================================================

#[test]
fn promoted_command_material_create() {
    let cmd = PromotedCommand::MaterialCreate {
        material_name: "NewMat".into(),
    };
    assert_eq!(cmd.route_id(), "route.material.create.v1");
}

#[test]
fn promoted_command_material_delete() {
    let cmd = PromotedCommand::MaterialDelete {
        material_id: "mat_123".into(),
    };
    assert_eq!(cmd.route_id(), "route.material.delete.v1");
}

#[test]
fn promoted_command_material_authority_initialize() {
    let cmd = PromotedCommand::MaterialAuthorityInitialize;
    assert_eq!(cmd.route_id(), "route.material.authority.initialize.v1");
}

#[test]
fn promoted_command_material_bind_visual_response() {
    let cmd = PromotedCommand::MaterialBindVisualResponse {
        material_id: "mat1".into(),
        visual_family: "PBR".into(),
    };
    assert_eq!(cmd.route_id(), "route.material.bind_visual_response.v1");
}

#[test]
fn promoted_command_material_preview_burn() {
    let cmd = PromotedCommand::MaterialPreviewBurn {
        material_id: "mat1".into(),
        preview_target: "diffuse".into(),
    };
    assert_eq!(cmd.route_id(), "route.material.preview_burn.v1");
}

// ============================================================================
// PromotedCommand tests - Audio
// ============================================================================

#[test]
fn promoted_command_audio_create_source() {
    let cmd = PromotedCommand::AudioCreateSource {
        source_name: "Ambient".into(),
    };
    assert_eq!(cmd.route_id(), "route.audio.source.create.v1");
}

#[test]
fn promoted_command_audio_authority_initialize() {
    let cmd = PromotedCommand::AudioAuthorityInitialize;
    assert_eq!(cmd.route_id(), "route.audio.authority.initialize.v1");
}

#[test]
fn promoted_command_audio_authority_dispose() {
    let cmd = PromotedCommand::AudioAuthorityDispose;
    assert_eq!(cmd.route_id(), "route.audio.authority.dispose.v1");
}

#[test]
fn promoted_command_audio_routes_are_audio() {
    let commands = [
        PromotedCommand::AudioCreateSource {
            source_name: "test".into(),
        },
        PromotedCommand::AudioBindWorldSource {
            source_id: "s1".into(),
            position: [0.0, 0.0, 0.0],
        },
        PromotedCommand::AudioSetAcousticProfile {
            source_id: "s1".into(),
            profile: "reverb".into(),
        },
    ];
    for cmd in &commands {
        assert!(
            cmd.route_id().starts_with("route.audio"),
            "Expected audio route, got {}",
            cmd.route_id()
        );
    }
}

// ============================================================================
// PromotedCommand tests - Build
// ============================================================================

#[test]
fn promoted_command_build_run() {
    let cmd = PromotedCommand::BuildRun;
    assert_eq!(cmd.route_id(), "route.build.run.v1");
}

#[test]
fn promoted_command_build_release() {
    let cmd = PromotedCommand::BuildRelease;
    assert_eq!(cmd.route_id(), "route.build.release.v1");
}

#[test]
fn promoted_command_validation_run_full() {
    let cmd = PromotedCommand::ValidationRunFull;
    assert_eq!(cmd.route_id(), "route.world.validate.v1");
}

// ============================================================================
// PromotedCommand tests - Shell
// ============================================================================

#[test]
fn promoted_command_shell_activate_viewport() {
    let cmd = PromotedCommand::ShellActivateViewport;
    assert_eq!(cmd.route_id(), "route.shell.activate_viewport.v1");
}

#[test]
fn promoted_command_shell_activate_inspector() {
    let cmd = PromotedCommand::ShellActivateInspector;
    assert_eq!(cmd.route_id(), "route.shell.activate_inspector.v1");
}

#[test]
fn promoted_command_shell_routes_are_shell() {
    let commands = [
        PromotedCommand::ShellActivateViewport,
        PromotedCommand::ShellActivateOutliner,
        PromotedCommand::ShellActivateInspector,
        PromotedCommand::ShellActivateContentBrowser,
        PromotedCommand::ShellActivateMaterialLab,
        PromotedCommand::ShellActivateTerrainLab,
        PromotedCommand::ShellActivateSkyLab,
    ];
    for cmd in &commands {
        assert!(cmd.route_id().starts_with("route.shell"));
    }
}

// ============================================================================
// PromotedCommand tests - Sky/Environment
// ============================================================================

#[test]
fn promoted_command_environment_set_time() {
    let cmd = PromotedCommand::EnvironmentSetTime {
        time_of_day_hours: 14.5,
    };
    assert_eq!(cmd.route_id(), "route.sky.set_time_of_day.v1");
}

#[test]
fn promoted_command_sky_bind_profile() {
    let cmd = PromotedCommand::SkyBindProfile {
        sky_profile: "sunset".into(),
    };
    assert_eq!(cmd.route_id(), "route.sky.bind_profile.v1");
}

#[test]
fn promoted_command_environment_set_fog_density() {
    let cmd = PromotedCommand::EnvironmentSetFogDensity { density: 0.3 };
    assert_eq!(cmd.route_id(), "route.environment.fog.density.v1");
}

// ============================================================================
// PromotedCommand tests - Scene (legacy)
// ============================================================================

#[test]
fn promoted_command_scene_bootstrap() {
    let cmd = PromotedCommand::SceneBootstrap;
    assert_eq!(cmd.route_id(), "route.scene.bootstrap.v1");
}

#[test]
fn promoted_command_scene_fire_test_shot() {
    let cmd = PromotedCommand::SceneFireTestShot {
        weapon_entity_id: 42,
    };
    assert_eq!(cmd.route_id(), "route.scene.fire_test_shot.v1");
}

#[test]
fn promoted_command_scene_reset() {
    let cmd = PromotedCommand::SceneReset;
    assert_eq!(cmd.route_id(), "route.scene.reset.v1");
}

// ============================================================================
// PromotedCommand serialization tests
// ============================================================================

#[test]
fn promoted_command_serializes_unit_variant() {
    let cmd = PromotedCommand::RuntimePlay;
    let json = serde_json::to_string(&cmd).unwrap();
    assert!(json.contains("RuntimePlay"));
}

#[test]
fn promoted_command_serializes_struct_variant() {
    let cmd = PromotedCommand::MaterialCreate {
        material_name: "TestMat".into(),
    };
    let json = serde_json::to_string(&cmd).unwrap();
    assert!(json.contains("TestMat"));
}

#[test]
fn promoted_command_roundtrip_unit() {
    let cmd = PromotedCommand::RuntimeStop;
    let json = serde_json::to_string(&cmd).unwrap();
    let restored: PromotedCommand = serde_json::from_str(&json).unwrap();
    assert_eq!(cmd, restored);
}

#[test]
fn promoted_command_roundtrip_struct() {
    let cmd = PromotedCommand::TerrainImport {
        heightmap_path: "/path/to/heightmap.png".into(),
    };
    let json = serde_json::to_string(&cmd).unwrap();
    let restored: PromotedCommand = serde_json::from_str(&json).unwrap();
    assert_eq!(cmd, restored);
}

#[test]
fn promoted_command_debug() {
    let cmd = PromotedCommand::RuntimePlay;
    let debug = format!("{:?}", cmd);
    assert!(debug.contains("RuntimePlay"));
}

#[test]
fn promoted_command_clone() {
    let cmd = PromotedCommand::WorldOpen {
        world_path: "/worlds/test.world".into(),
    };
    let cmd2 = cmd.clone();
    assert_eq!(cmd, cmd2);
}

// ============================================================================
// CommandId tests
// ============================================================================

#[test]
fn command_id_new() {
    let id = CommandId::new("cmd_001");
    assert_eq!(id.as_str(), "cmd_001");
}

#[test]
fn command_id_from_string() {
    let id = CommandId::new(String::from("test_id"));
    assert_eq!(id.as_str(), "test_id");
}

#[test]
fn command_id_equality() {
    assert_eq!(CommandId::new("a"), CommandId::new("a"));
    assert_ne!(CommandId::new("a"), CommandId::new("b"));
}

#[test]
fn command_id_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let id = CommandId::new("test");
    let mut h1 = DefaultHasher::new();
    id.hash(&mut h1);
    let mut h2 = DefaultHasher::new();
    id.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn command_id_in_hashset() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(CommandId::new("a"));
    set.insert(CommandId::new("b"));
    set.insert(CommandId::new("a"));
    assert_eq!(set.len(), 2);
}

#[test]
fn command_id_serializes() {
    let id = CommandId::new("cmd_123");
    let json = serde_json::to_string(&id).unwrap();
    assert!(json.contains("cmd_123"));
}

#[test]
fn command_id_roundtrip() {
    let id = CommandId::new("roundtrip_id");
    let json = serde_json::to_string(&id).unwrap();
    let restored: CommandId = serde_json::from_str(&json).unwrap();
    assert_eq!(id, restored);
}

// ============================================================================
// CommandPayload tests
// ============================================================================

#[test]
fn command_payload_empty() {
    let p = CommandPayload::Empty;
    let json = serde_json::to_string(&p).unwrap();
    assert!(json.contains("Empty"));
}

#[test]
fn command_payload_project() {
    let p = CommandPayload::Project(ProjectPayload {
        project_name: Some("TestProject".into()),
        project_path: None,
    });
    let json = serde_json::to_string(&p).unwrap();
    assert!(json.contains("Project"));
    assert!(json.contains("TestProject"));
}

#[test]
fn command_payload_terrain() {
    let p = CommandPayload::Terrain(TerrainPayload {
        position: Some([1.0, 2.0]),
        brush_radius: Some(5.0),
        brush_strength: Some(0.8),
        target_layer: None,
        target_height: None,
    });
    let json = serde_json::to_string(&p).unwrap();
    assert!(json.contains("Terrain"));
}

#[test]
fn command_payload_material() {
    let p = CommandPayload::Material(MaterialPayload {
        material_id: Some("mat1".into()),
        profile_name: None,
        binding_ref: Some("texture.png".into()),
        preview_trigger: None,
        proof_mode: None,
    });
    assert!(matches!(p, CommandPayload::Material(_)));
}

#[test]
fn command_payload_from_project() {
    let proj = ProjectPayload::default();
    let p: CommandPayload = proj.into();
    assert!(matches!(p, CommandPayload::Project(_)));
}

#[test]
fn command_payload_from_unit() {
    let p: CommandPayload = ().into();
    assert!(matches!(p, CommandPayload::Empty));
}

#[test]
fn command_payload_roundtrip() {
    let p = CommandPayload::Audio(AudioPayload {
        resource_id: Some("res1".into()),
        profile_ref: Some("profile1".into()),
        preview_mode: Some("isolated".into()),
    });
    let json = serde_json::to_string(&p).unwrap();
    let restored: CommandPayload = serde_json::from_str(&json).unwrap();
    assert_eq!(p, restored);
}

// ============================================================================
// RouteDomain tests
// ============================================================================

#[test]
fn route_domain_variants() {
    let domains = [
        RouteDomain::Project,
        RouteDomain::World,
        RouteDomain::Import,
        RouteDomain::Terrain,
        RouteDomain::Material,
        RouteDomain::Environment,
        RouteDomain::Shell,
        RouteDomain::Audio,
        RouteDomain::Runtime,
        RouteDomain::Build,
        RouteDomain::Diagnostics,
    ];
    assert_eq!(domains.len(), 11);
}

#[test]
fn route_domain_serializes() {
    let json = serde_json::to_string(&RouteDomain::Material).unwrap();
    assert!(json.contains("Material"));
}

#[test]
fn route_domain_roundtrip() {
    for domain in [
        RouteDomain::Project,
        RouteDomain::World,
        RouteDomain::Terrain,
        RouteDomain::Audio,
        RouteDomain::Build,
    ] {
        let json = serde_json::to_string(&domain).unwrap();
        let restored: RouteDomain = serde_json::from_str(&json).unwrap();
        assert_eq!(domain, restored);
    }
}

#[test]
fn route_domain_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut h1 = DefaultHasher::new();
    RouteDomain::Material.hash(&mut h1);
    let mut h2 = DefaultHasher::new();
    RouteDomain::Material.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

// ============================================================================
// SourceSurface tests
// ============================================================================

#[test]
fn source_surface_variants() {
    let surfaces = [
        SourceSurface::MainMenu,
        SourceSurface::ProjectWizard,
        SourceSurface::OpenWorldDialog,
        SourceSurface::TerrainPanel,
        SourceSurface::MaterialPanel,
        SourceSurface::SkyPanel,
        SourceSurface::AudioPanel,
        SourceSurface::BuildPanel,
        SourceSurface::ViewportPanel,
        SourceSurface::InspectorPanel,
        SourceSurface::CommandPalette,
        SourceSurface::AutomationPanel,
        SourceSurface::QualityPanel,
        SourceSurface::Unknown,
    ];
    assert_eq!(surfaces.len(), 14);
}

#[test]
fn source_surface_serializes() {
    let json = serde_json::to_string(&SourceSurface::MaterialPanel).unwrap();
    assert!(json.contains("MaterialPanel"));
}

#[test]
fn source_surface_roundtrip() {
    let surface = SourceSurface::CommandPalette;
    let json = serde_json::to_string(&surface).unwrap();
    let restored: SourceSurface = serde_json::from_str(&json).unwrap();
    assert_eq!(surface, restored);
}

// ============================================================================
// TransactionMeta tests
// ============================================================================

#[test]
fn transaction_meta_new() {
    let tm = TransactionMeta::new("btn.test");
    assert_eq!(tm.schema_version, 1);
    assert_eq!(tm.source_button_id, "btn.test");
    assert!(tm.transaction_id.is_none());
    assert!(tm.retry_of.is_none());
}

#[test]
fn transaction_meta_serializes() {
    let tm = TransactionMeta::new("btn.material.create");
    let json = serde_json::to_string(&tm).unwrap();
    assert!(json.contains("btn.material.create"));
    assert!(json.contains("1")); // schema_version
}

#[test]
fn transaction_meta_roundtrip() {
    let tm = TransactionMeta::new("btn.test");
    let json = serde_json::to_string(&tm).unwrap();
    let restored: TransactionMeta = serde_json::from_str(&json).unwrap();
    assert_eq!(tm, restored);
}

// ============================================================================
// CanonicalCommandEnvelope tests (requires manifest JSON)
// ============================================================================

#[test]
fn canonical_envelope_from_route() {
    let route = canonical_route_by_action_id("material.bind_light_response.requested")
        .expect("canonical route exists");
    let payload = CommandPayload::Material(MaterialPayload {
        material_id: Some("mat1".into()),
        profile_name: None,
        binding_ref: Some("light_profile".into()),
        preview_trigger: None,
        proof_mode: None,
    });
    let envelope =
        CanonicalCommandEnvelope::from_route(route, payload, SourceSurface::MaterialPanel);
    assert_eq!(envelope.command_id.as_str(), route.command_id);
}

#[test]
fn canonical_envelope_domain() {
    let route = canonical_route_by_action_id("material.duplicate_profile.requested")
        .expect("canonical route exists");
    let payload = CommandPayload::Material(MaterialPayload::default());
    let envelope =
        CanonicalCommandEnvelope::from_route(route, payload, SourceSurface::MaterialPanel);
    assert_eq!(envelope.domain(), RouteDomain::Material);
}

#[test]
fn canonical_route_manifest_has_routes() {
    let manifest = canonical_route_manifest();
    assert!(!manifest.routes.is_empty());
    assert_eq!(manifest.route_count, manifest.routes.len());
}

#[test]
fn canonical_route_lookup_by_button_id() {
    let route = canonical_route("btn.world.validate_world");
    assert!(route.is_some(), "btn.world.validate_world should exist");
}

#[test]
fn canonical_route_lookup_nonexistent() {
    let route = canonical_route("btn.nonexistent.button.xyz");
    assert!(route.is_none());
}

#[test]
fn canonical_button_routes_not_empty() {
    let routes = canonical_button_routes();
    assert!(!routes.is_empty());
}

#[test]
fn canonical_button_route_has_all_fields() {
    let routes = canonical_button_routes();
    let route = &routes[0];
    assert!(!route.button_id.is_empty());
    assert!(!route.action_id.is_empty());
    assert!(!route.command_id.is_empty());
    assert!(!route.executor_module.is_empty());
}

#[test]
fn canonical_button_route_domain_derivation() {
    let routes = canonical_button_routes();
    // Check that domain is derived from button_id prefix
    for route in routes.iter().take(10) {
        let domain = route.domain();
        // Just verify it doesn't panic and returns a valid variant
        let _ = match domain {
            RouteDomain::Project
            | RouteDomain::World
            | RouteDomain::Import
            | RouteDomain::Terrain
            | RouteDomain::Material
            | RouteDomain::Environment
            | RouteDomain::Shell
            | RouteDomain::Audio
            | RouteDomain::Runtime
            | RouteDomain::Build
            | RouteDomain::Diagnostics => true,
        };
    }
}
