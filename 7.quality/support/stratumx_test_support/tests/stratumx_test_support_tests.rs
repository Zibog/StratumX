//! Comprehensive tests for stratumx_test_support crate

use stratumx_test_support::*;

// ---------------------------------------------------------------------------
// Bridge types tests
// ---------------------------------------------------------------------------

#[test]
fn bridge_version_new_sets_fields() {
    let version = BridgeVersion::new(2, 5, 1);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 5);
    assert_eq!(version.patch, 1);
}

#[test]
fn runtime_handle_opaque_tag_format() {
    let handle = RuntimeHandle::new(42);
    let tag = handle.opaque_tag();
    assert_eq!(tag, "runtime_000000000000002a");
}

#[test]
fn runtime_handle_display() {
    let handle = RuntimeHandle::new(7);
    assert_eq!(format!("{}", handle), "Runtime(7)");
}

#[test]
fn runtime_handle_debug_contains_opaque() {
    let handle = RuntimeHandle::new(1);
    let debug = format!("{:?}", handle);
    assert!(debug.contains("<opaque:"));
}

#[test]
fn session_handle_opaque_tag_format() {
    let handle = SessionHandle::new(100);
    let tag = handle.opaque_tag();
    assert_eq!(tag, "session_0000000000000064");
}

#[test]
fn object_handle_opaque_tag_format() {
    let handle = ObjectHandle::new(255);
    let tag = handle.opaque_tag();
    assert_eq!(tag, "object_00000000000000ff");
}

#[test]
fn object_handle_equality() {
    let a = ObjectHandle::new(1);
    let b = ObjectHandle::new(1);
    let c = ObjectHandle::new(2);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn default_bridge_config_sane_values() {
    let config = BridgeConfig::default();
    assert_eq!(config.runtime, RuntimeHandle::new(1));
    assert_eq!(config.version, BridgeVersion::new(1, 0, 0));
    assert_eq!(config.max_queue_depth, 1024);
    assert_eq!(config.max_objects_per_snapshot, 8192);
}

#[test]
fn default_transport_policy_ordered_control() {
    let policy = default_transport_policy(TransportLane::OrderedControl);
    assert_eq!(policy.lane, TransportLane::OrderedControl);
    assert_eq!(policy.max_payload_bytes, 1024);
    assert_eq!(policy.max_batch_size, 1);
    assert!(!policy.compression_enabled);
}

#[test]
fn default_transport_policy_bounded_preview() {
    let policy = default_transport_policy(TransportLane::BoundedPreview);
    assert_eq!(policy.max_payload_bytes, 64);
    assert_eq!(policy.max_batch_size, 4);
    assert!(policy.compression_enabled);
}

#[test]
fn default_transport_policy_metrics_only() {
    let policy = default_transport_policy(TransportLane::MetricsOnly);
    assert_eq!(policy.max_payload_bytes, 128);
    assert_eq!(policy.max_batch_size, 64);
    assert!(policy.compression_enabled);
}

#[test]
fn default_transport_policy_artifact_only() {
    let policy = default_transport_policy(TransportLane::ArtifactOnly);
    assert_eq!(policy.max_payload_bytes, 512);
    assert_eq!(policy.max_batch_size, 16);
    assert!(!policy.compression_enabled);
}

// ---------------------------------------------------------------------------
// Bridge runtime tests
// ---------------------------------------------------------------------------

#[test]
fn bridge_runtime_new_has_initial_snapshot() {
    let config = BridgeConfig::default();
    let runtime = BridgeRuntime::new(config);
    assert_eq!(runtime.snapshots.len(), 1);
    assert_eq!(runtime.snapshots[0].epoch, 0);
    assert_eq!(runtime.snapshots[0].label, "initial");
}

#[test]
fn bridge_runtime_open_session_assigns_handle() {
    let config = BridgeConfig::default();
    let mut runtime = BridgeRuntime::new(config);
    let handle = runtime.open_session("test_session");
    assert_eq!(handle, SessionHandle::new(1));
    assert_eq!(runtime.session_count(), 1);
}

#[test]
fn bridge_runtime_multiple_sessions_increment() {
    let config = BridgeConfig::default();
    let mut runtime = BridgeRuntime::new(config);
    let s1 = runtime.open_session("first");
    let s2 = runtime.open_session("second");
    assert_ne!(s1, s2);
    assert_eq!(runtime.session_count(), 2);
}

#[test]
fn bridge_runtime_register_object_succeeds() {
    let config = BridgeConfig::default();
    let mut runtime = BridgeRuntime::new(config);
    runtime.open_session("session1");
    let handle = runtime.register_object("my_world", ObjectClass::World);
    assert!(handle.is_ok());
}

#[test]
fn bridge_runtime_object_view_returns_data() {
    let config = BridgeConfig::default();
    let mut runtime = BridgeRuntime::new(config);
    runtime.open_session("session1");
    let handle = runtime
        .register_object("test_obj", ObjectClass::Entity)
        .unwrap();
    let view = runtime.object_view(handle);
    assert!(view.is_some());
    let view = view.unwrap();
    assert_eq!(view.label, "test_obj");
    assert_eq!(view.class, ObjectClass::Entity);
}

#[test]
fn bridge_runtime_record_metric_stores_value() {
    let config = BridgeConfig::default();
    let mut runtime = BridgeRuntime::new(config);
    runtime.record_metric("fps", 60.0, "frames_per_second");
    let batch = runtime.read_metric_batch(0, 10);
    assert_eq!(batch.records.len(), 1);
    assert_eq!(batch.records[0].name, "fps");
    assert_eq!(batch.records[0].value, 60.0);
}

#[test]
fn bridge_runtime_read_observation_batch_empty() {
    let config = BridgeConfig::default();
    let runtime = BridgeRuntime::new(config);
    let batch = runtime.read_observation_batch(0, 10);
    assert_eq!(batch.records.len(), 0);
    assert_eq!(batch.next_cursor, 0);
}

#[test]
fn bridge_runtime_latest_snapshot_initial() {
    let config = BridgeConfig::default();
    let runtime = BridgeRuntime::new(config);
    let snapshot = runtime.latest_snapshot();
    assert_eq!(snapshot.epoch, 0);
    assert_eq!(snapshot.label, "initial");
}

#[test]
fn bridge_runtime_apply_control_set_field() {
    let config = BridgeConfig::default();
    let mut runtime = BridgeRuntime::new(config);
    let session = runtime.open_session("session1");
    let handle = runtime.register_object("obj", ObjectClass::World).unwrap();

    let control = BridgeControl {
        session,
        sequence: 1,
        object: Some(handle),
        kind: BridgeControlKind::SetField {
            key: "color".to_string(),
            value: "red".to_string(),
        },
    };
    assert!(runtime.apply_control(control).is_ok());
}

#[test]
fn bridge_runtime_apply_control_add_tag() {
    let config = BridgeConfig::default();
    let mut runtime = BridgeRuntime::new(config);
    let session = runtime.open_session("session1");
    let handle = runtime.register_object("obj", ObjectClass::World).unwrap();

    let control = BridgeControl {
        session,
        sequence: 1,
        object: Some(handle),
        kind: BridgeControlKind::AddTag {
            tag: "tagged".to_string(),
        },
    };
    assert!(runtime.apply_control(control).is_ok());
}

#[test]
fn bridge_runtime_apply_control_pause_resume_reset() {
    let config = BridgeConfig::default();
    let mut runtime = BridgeRuntime::new(config);
    let session = runtime.open_session("session1");

    for kind in &[
        BridgeControlKind::Pause,
        BridgeControlKind::Resume,
        BridgeControlKind::Reset,
    ] {
        let control = BridgeControl {
            session,
            sequence: 1,
            object: None,
            kind: kind.clone(),
        };
        assert!(runtime.apply_control(control).is_ok());
    }
}

#[test]
fn bridge_runtime_transport_legality_payload_too_large() {
    let config = BridgeConfig::default();
    let runtime = BridgeRuntime::new(config);
    let policy = default_transport_policy(TransportLane::OrderedControl);
    let verdict = runtime.transport_legality(&policy, 2048, true, true);
    assert!(matches!(verdict, LegalityVerdict::Illegal(_)));
}

#[test]
fn bridge_runtime_transport_legality_ordered_control_ok() {
    let config = BridgeConfig::default();
    let runtime = BridgeRuntime::new(config);
    let policy = default_transport_policy(TransportLane::OrderedControl);
    let verdict = runtime.transport_legality(&policy, 512, true, true);
    assert!(matches!(verdict, LegalityVerdict::Legal));
}

#[test]
fn bridge_runtime_compatibility_version_too_old() {
    let config = BridgeConfig::default();
    let runtime = BridgeRuntime::new(config);
    let verdict = runtime.compatibility_verdict(
        BridgeVersion::new(0, 1, 0),
        &[Capability::Controls],
        CompatibilityProfile::ToolRuntime,
    );
    assert!(matches!(verdict, CompatibilityVerdict::VersionTooOld));
}

#[test]
fn bridge_runtime_compatibility_tool_runtime_ok() {
    let config = BridgeConfig::default();
    let runtime = BridgeRuntime::new(config);
    let verdict = runtime.compatibility_verdict(
        BridgeVersion::new(1, 0, 0),
        &[Capability::Controls],
        CompatibilityProfile::ToolRuntime,
    );
    assert!(matches!(verdict, CompatibilityVerdict::Compatible));
}

#[test]
fn bridge_runtime_compatibility_missing_capability() {
    let config = BridgeConfig::default();
    let runtime = BridgeRuntime::new(config);
    let verdict = runtime.compatibility_verdict(
        BridgeVersion::new(1, 0, 0),
        &[],
        CompatibilityProfile::ToolRuntime,
    );
    assert!(matches!(
        verdict,
        CompatibilityVerdict::MissingCapability(Capability::Controls)
    ));
}

#[test]
fn bridge_runtime_publish_snapshot_increments_epoch() {
    let config = BridgeConfig::default();
    let mut runtime = BridgeRuntime::new(config);
    let snapshot = runtime.publish_snapshot("first").unwrap();
    assert_eq!(snapshot.epoch, 1);
    assert_eq!(snapshot.label, "first");
    assert_eq!(runtime.snapshots.len(), 2); // initial + first
}

#[test]
fn bridge_runtime_ingest_packet_requires_session() {
    let config = BridgeConfig::default();
    let mut runtime = BridgeRuntime::new(config);
    let session = runtime.open_session("session1");

    let packet = BridgePacket {
        session,
        sequence: 1,
        lane: PacketLane::Control,
        topic: "test".to_string(),
        payload: vec![],
    };
    assert!(runtime.ingest_packet(packet).is_ok());
}

#[test]
fn bridge_runtime_ingest_packet_invalid_session() {
    let config = BridgeConfig::default();
    let mut runtime = BridgeRuntime::new(config);

    let packet = BridgePacket {
        session: SessionHandle::new(999),
        sequence: 1,
        lane: PacketLane::Control,
        topic: "test".to_string(),
        payload: vec![],
    };
    assert!(runtime.ingest_packet(packet).is_err());
}

#[test]
fn bridge_runtime_session_label_lookup() {
    let config = BridgeConfig::default();
    let mut runtime = BridgeRuntime::new(config);
    let handle = runtime.open_session("my_session");
    assert_eq!(runtime.session_label(handle), Some("my_session"));
}

#[test]
fn bridge_runtime_session_label_not_found() {
    let config = BridgeConfig::default();
    let runtime = BridgeRuntime::new(config);
    assert_eq!(runtime.session_label(SessionHandle::new(999)), None);
}

#[test]
fn bridge_runtime_estimated_hot_path_bytes_grows() {
    let config = BridgeConfig::default();
    let mut runtime = BridgeRuntime::new(config);
    let initial = runtime.estimated_hot_path_bytes();
    runtime.open_session("s1");
    let after = runtime.estimated_hot_path_bytes();
    assert!(after >= initial);
}

// ---------------------------------------------------------------------------
// Engine bridge harness tests
// ---------------------------------------------------------------------------

#[test]
fn engine_bridge_harness_new_succeeds() {
    let startup = canonical_demo_startup_config(RuntimeProfile::Interactive60);
    let config = BridgeConfig::default();
    let harness = EngineBridgeHarness::new(startup, config);
    assert!(harness.is_ok());
}

#[test]
fn engine_bridge_harness_bootstrap_demo_world() {
    let startup = canonical_demo_startup_config(RuntimeProfile::Interactive60);
    let config = BridgeConfig::default();
    let mut harness = EngineBridgeHarness::new(startup, config).unwrap();
    let result = harness.bootstrap_demo_world(4, 2);
    assert!(result.is_ok());
    let (handle, snapshot) = result.unwrap();
    assert_eq!(snapshot.label, "world_bootstrapped");
    assert!(handle.0 > 0);
}

// ---------------------------------------------------------------------------
// ID and clock fixture tests
// ---------------------------------------------------------------------------

#[test]
fn test_clock_initial_time_is_zero() {
    let clock = create_test_clock();
    assert_eq!(clock.now(), 0);
}

#[test]
fn test_clock_advance_increments_time() {
    let clock = create_test_clock();
    clock.advance(100);
    assert_eq!(clock.now(), 100);
}

#[test]
fn test_clock_set_overwrites_time() {
    let clock = create_test_clock();
    clock.advance(50);
    clock.set(200);
    assert_eq!(clock.now(), 200);
}

#[test]
fn test_clock_multiple_advances_accumulate() {
    let clock = create_test_clock();
    clock.advance(10);
    clock.advance(20);
    clock.advance(30);
    assert_eq!(clock.now(), 60);
}

#[test]
fn next_test_id_produces_unique_values() {
    let id1 = next_test_id();
    let id2 = next_test_id();
    assert_ne!(id1, id2);
}

// ---------------------------------------------------------------------------
// File fixture tests
// ---------------------------------------------------------------------------

#[test]
fn test_file_path_is_valid() {
    let path = test_file_path();
    assert!(path.to_str().is_some());
}

#[test]
fn test_dir_path_is_valid() {
    let path = test_dir_path();
    assert!(path.to_str().is_some());
}

// ---------------------------------------------------------------------------
// Temp fs fixture tests
// ---------------------------------------------------------------------------

#[test]
fn temp_dir_creates_and_exists() {
    let dir = create_temp_dir();
    assert!(dir.path().exists());
    assert!(dir.path().is_dir());
}

#[test]
fn temp_dir_create_file_writes_content() {
    let dir = create_temp_dir();
    let file_path = dir.create_file("test.txt", "hello world");
    assert!(file_path.exists());
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "hello world");
}

#[test]
fn temp_dir_create_dir_creates_directory() {
    let dir = create_temp_dir();
    let subdir = dir.create_dir("subdir");
    assert!(subdir.exists());
    assert!(subdir.is_dir());
}

// ---------------------------------------------------------------------------
// Chunk fixture tests
// ---------------------------------------------------------------------------

#[test]
fn create_test_chunk_has_zero_coords() {
    let chunk = create_test_chunk();
    assert_eq!(chunk.chunk_x, 0);
    assert_eq!(chunk.chunk_y, 0);
    assert!(!chunk.loaded);
    assert!(!chunk.dirty);
    assert!(!chunk.mesh_built);
}

#[test]
fn create_chunk_at_sets_coordinates() {
    let chunk = create_chunk_at(5, 10);
    assert_eq!(chunk.chunk_x, 5);
    assert_eq!(chunk.chunk_y, 10);
}

// ---------------------------------------------------------------------------
// Sky fixture tests
// ---------------------------------------------------------------------------

#[test]
fn create_test_sky_is_valid() {
    let sky = create_test_sky();
    // Sky should be created without panic; scattered weather regime set by default
    assert!(matches!(
        sky.weather_director.target_regime,
        engine_material::WeatherRegime::Clear | engine_material::WeatherRegime::Scattered
    ));
}

#[test]
fn create_sky_at_time_sets_time() {
    let sky = create_sky_at_time(12.0);
    assert!((sky.celestial.time_of_day_hours - 12.0).abs() < 0.01);
}

// ---------------------------------------------------------------------------
// Terrain fixture tests
// ---------------------------------------------------------------------------

#[test]
fn create_test_terrain_has_expected_dimensions() {
    let terrain = create_test_terrain();
    assert_eq!(terrain.world_size, [100.0, 100.0]);
    assert_eq!(terrain.resolution, [64, 64]);
    assert_eq!(terrain.chunk_grid, [4, 4]);
}

#[test]
fn create_terrain_with_size_updates_dimensions() {
    let terrain = create_terrain_with_size(200, 300);
    assert_eq!(terrain.world_size, [200.0, 300.0]);
}

// ---------------------------------------------------------------------------
// Package fixture tests
// ---------------------------------------------------------------------------

#[test]
fn create_test_package_has_valid_fields() {
    let package = create_test_package();
    assert_eq!(package.world_label, "Test World");
    assert_eq!(package.version, "1.0.0");
    assert_eq!(package.source_lineage.created_by, "test");
}

// ---------------------------------------------------------------------------
// World fixture tests
// ---------------------------------------------------------------------------

#[test]
fn create_test_world_is_initially_empty() {
    let world = create_test_world();
    assert!(world.proof_region_scene().is_none());
}

#[test]
fn create_world_with_scene_has_scene() {
    let world = create_world_with_scene();
    assert!(world.proof_region_scene().is_some());
}

#[test]
fn test_world_id_is_stable() {
    let id = test_world_id();
    assert!(!id.0.to_string().is_empty());
}

// ---------------------------------------------------------------------------
// Tooling types tests
// ---------------------------------------------------------------------------

#[test]
fn derived_summary_from_snapshot_counts_objects() {
    let snapshot = ToolSnapshot {
        generation: 1,
        objects: vec![
            ToolObject {
                handle: ObjectHandle::new(1),
                class: ObjectClass::World,
                label: "world".to_string(),
                fields: Default::default(),
                tags: Default::default(),
            },
            ToolObject {
                handle: ObjectHandle::new(2),
                class: ObjectClass::Entity,
                label: "entity".to_string(),
                fields: Default::default(),
                tags: Default::default(),
            },
        ],
        transactions: vec![],
    };
    let summary = DerivedSummary::from_snapshot(&snapshot);
    assert_eq!(summary.object_count, 2);
    assert_eq!(summary.active_objects, 2);
    assert_eq!(summary.transaction_count, 0);
}

// ---------------------------------------------------------------------------
// Tooling runtime tests
// ---------------------------------------------------------------------------

#[test]
fn tooling_runtime_new_is_default() {
    let runtime = ToolingRuntime::new();
    // generation and ledger are private; use public methods
    let snapshot = runtime.snapshot();
    assert_eq!(snapshot.generation, 0);
    assert_eq!(runtime.ledger().len(), 0);
}

#[test]
fn tooling_runtime_create_object_succeeds() {
    let mut runtime = ToolingRuntime::new();
    let handle = runtime.create_object("test", ObjectClass::World);
    assert!(handle.is_ok());
}

#[test]
fn tooling_runtime_snapshot_reflects_objects() {
    let mut runtime = ToolingRuntime::new();
    runtime.create_object("obj1", ObjectClass::World).unwrap();
    let snapshot = runtime.snapshot();
    assert_eq!(snapshot.objects.len(), 1);
    assert_eq!(snapshot.generation, 1);
}

#[test]
fn tooling_runtime_index_returns_objects() {
    let mut runtime = ToolingRuntime::new();
    runtime.create_object("obj1", ObjectClass::World).unwrap();
    let index = runtime.index();
    assert_eq!(index.objects.len(), 1);
}

#[test]
fn tooling_runtime_plan_goal_returns_plan() {
    let runtime = ToolingRuntime::new();
    let plan = runtime.plan_goal("create world");
    assert_eq!(plan.goal, "create world");
    assert!(!plan.suggested_commands.is_empty());
}

#[test]
fn tooling_runtime_set_workspace_focus() {
    let mut runtime = ToolingRuntime::new();
    runtime.set_workspace_focus("inspector");
    assert_eq!(runtime.workspace().focused_view, "inspector");
}

#[test]
fn tooling_runtime_clear_workspace() {
    let mut runtime = ToolingRuntime::new();
    runtime.set_workspace_focus("viewport");
    runtime.clear_workspace();
    assert!(runtime.workspace().focused_view.is_empty());
    assert!(runtime.workspace().open_views.is_empty());
}

#[test]
fn tooling_runtime_validate_snapshot_empty_workspace_errors() {
    let mut runtime = ToolingRuntime::new();
    runtime.clear_workspace();
    let diagnostics = runtime.validate_snapshot();
    assert!(!diagnostics.is_empty());
}

#[test]
fn tooling_runtime_preview_object_nonexistent_fails() {
    let mut runtime = ToolingRuntime::new();
    let result = runtime.preview_object(ObjectHandle::new(999));
    assert!(result.is_err());
}

#[test]
fn tooling_runtime_build_current_returns_artifact() {
    let mut runtime = ToolingRuntime::new();
    let artifact = runtime.build_current();
    assert!(!artifact.digest.is_empty());
    assert_eq!(artifact.object_count, 0);
}

#[test]
fn tooling_runtime_stage_and_approve_proposal() {
    let mut runtime = ToolingRuntime::new();
    let commands = vec![ToolCommand::CreateObject {
        label: "test".to_string(),
        class: ObjectClass::World,
    }];
    let id = runtime.stage_proposal("goal", commands);
    assert!(runtime.approve_proposal(id).is_ok());
}

#[test]
fn tooling_runtime_approve_nonexistent_proposal_fails() {
    let mut runtime = ToolingRuntime::new();
    assert!(runtime.approve_proposal(999).is_err());
}

#[test]
fn tooling_runtime_apply_unapproved_proposal_fails() {
    let mut runtime = ToolingRuntime::new();
    let commands = vec![ToolCommand::CreateObject {
        label: "test".to_string(),
        class: ObjectClass::World,
    }];
    let id = runtime.stage_proposal("goal", commands);
    assert!(runtime.apply_proposal(id).is_err());
}

#[test]
fn tooling_runtime_revert_proposal_returns_commands() {
    let mut runtime = ToolingRuntime::new();
    let commands = vec![ToolCommand::CreateObject {
        label: "test".to_string(),
        class: ObjectClass::World,
    }];
    let id = runtime.stage_proposal("goal", commands.clone());
    let reverted = runtime.revert_proposal(id).unwrap();
    assert_eq!(reverted.len(), 1);
}

// ---------------------------------------------------------------------------
// Editor types tests
// ---------------------------------------------------------------------------

#[test]
fn viewport_state_default_is_empty() {
    let state = ViewportState::default();
    assert!(state.selected.is_empty());
}

#[test]
fn tool_context_default_mode_is_select() {
    let ctx = ToolContextState::default();
    assert!(matches!(ctx.active_mode, ToolMode::Select));
}

#[test]
fn assistant_surface_default_has_no_goal() {
    let surface = AssistantSurfaceState::default();
    assert!(surface.active_goal.is_none());
    assert!(surface.staged_proposal.is_none());
}

// ---------------------------------------------------------------------------
// Editor product tests
// ---------------------------------------------------------------------------

#[test]
fn editor_product_new_demo_succeeds() {
    let product = EditorProduct::new_demo("test_project");
    assert!(product.is_ok());
}

#[test]
fn editor_product_new_demo_has_correct_title() {
    let product = EditorProduct::new_demo("test_project").unwrap();
    assert_eq!(product.title, "StratumX Editor");
}

#[test]
fn editor_product_new_demo_has_project_name() {
    let product = EditorProduct::new_demo("my_project").unwrap();
    assert_eq!(product.project_name, "my_project");
}

#[test]
fn editor_product_anchored_panels_contains_viewport() {
    let product = EditorProduct::new_demo("test").unwrap();
    let panels = product.anchored_panels();
    assert!(panels.contains(&EditorPanel::Viewport));
    assert!(panels.contains(&EditorPanel::Outliner));
    assert!(panels.contains(&EditorPanel::BuildRelease));
}

#[test]
fn editor_product_toggle_panel_adds_panel() {
    let mut product = EditorProduct::new_demo("test").unwrap();
    let initial_count = product.workspace_layout.open_panels.len();
    product.toggle_panel(EditorPanel::Assistant);
    assert_eq!(
        product.workspace_layout.open_panels.len(),
        initial_count + 1
    );
}

#[test]
fn editor_product_set_tool_mode_changes_mode() {
    let mut product = EditorProduct::new_demo("test").unwrap();
    product.set_tool_mode(ToolMode::Translate);
    assert!(matches!(
        product.tool_context.active_mode,
        ToolMode::Translate
    ));
}

#[test]
fn editor_product_create_world_anchor_succeeds() {
    let mut product = EditorProduct::new_demo("test").unwrap();
    let handle = product.create_world_anchor("root");
    assert!(handle.is_ok());
}

#[test]
fn editor_product_create_scene_entity_succeeds() {
    let mut product = EditorProduct::new_demo("test").unwrap();
    let handle = product.create_scene_entity("player");
    assert!(handle.is_ok());
}

#[test]
fn editor_product_create_material_succeeds() {
    let mut product = EditorProduct::new_demo("test").unwrap();
    let handle = product.create_material("concrete");
    assert!(handle.is_ok());
}

#[test]
fn editor_product_create_logic_node_succeeds() {
    let mut product = EditorProduct::new_demo("test").unwrap();
    let handle = product.create_logic_node("trigger");
    assert!(handle.is_ok());
}

#[test]
fn editor_product_select_object_succeeds() {
    let mut product = EditorProduct::new_demo("test").unwrap();
    let handle = product.create_world_anchor("root").unwrap();
    assert!(product.select_object(handle).is_ok());
}

#[test]
fn editor_product_stage_assistant_goal_returns_id() {
    let mut product = EditorProduct::new_demo("test").unwrap();
    let proposal_id = product.stage_assistant_goal("create a house");
    assert!(proposal_id > 0);
}

#[test]
fn editor_product_run_build_succeeds() {
    let mut product = EditorProduct::new_demo("test").unwrap();
    let artifact = product.run_build();
    assert!(artifact.is_ok());
}

#[test]
fn editor_product_run_release_succeeds() {
    let mut product = EditorProduct::new_demo("test").unwrap();
    product.run_build().unwrap();
    let release = product.run_release("stable");
    assert!(release.is_ok());
}

#[test]
fn editor_product_run_playtest_capture_sets_state() {
    let mut product = EditorProduct::new_demo("test").unwrap();
    product.run_playtest_capture("test_capture");
    assert!(product.playtest_surface.session_active);
    assert_eq!(
        product.playtest_surface.last_capture_label,
        Some("test_capture".to_string())
    );
}

// ---------------------------------------------------------------------------
// Startup config tests
// ---------------------------------------------------------------------------

#[test]
fn canonical_demo_startup_config_has_physics_enabled() {
    let config = canonical_demo_startup_config(RuntimeProfile::Interactive60);
    assert!(config.enable_physics);
    assert!(!config.enable_audio);
    assert!(!config.enable_networking);
}

#[test]
fn canonical_demo_startup_config_profile_matches() {
    for profile in &[
        RuntimeProfile::Interactive60,
        RuntimeProfile::ListenHost60,
        RuntimeProfile::Headless20,
    ] {
        let config = canonical_demo_startup_config(*profile);
        assert_eq!(config.runtime_profile, *profile);
    }
}
