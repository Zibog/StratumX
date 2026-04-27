use editor_dto_law::*;
use uuid::Uuid;

fn test_uuid() -> Uuid {
    uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
}

fn stable_world_id() -> StableWorldId {
    StableWorldId(test_uuid())
}

// ============================================================================
// Identity Tests
// ============================================================================

#[test]
fn test_stable_entity_id_creation() {
    let id = StableEntityId(test_uuid());
    assert_eq!(id.0, test_uuid());
}

#[test]
fn test_stable_entity_id_equality() {
    let a = StableEntityId(test_uuid());
    let b = StableEntityId(test_uuid());
    assert_eq!(a, b);
}

#[test]
fn test_stable_entity_id_copy() {
    let a = StableEntityId(test_uuid());
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn test_stable_component_id_creation() {
    let id = StableComponentId(test_uuid());
    assert_eq!(id.0, test_uuid());
}

#[test]
fn test_stable_world_id_creation() {
    let id = StableWorldId(test_uuid());
    assert_eq!(id.0, test_uuid());
}

#[test]
fn test_startup_world_ref_creation() {
    let id = StartupWorldRef(test_uuid());
    assert_eq!(id.0, test_uuid());
}

#[test]
fn test_validation_world_ref_creation() {
    let id = ValidationWorldRef(test_uuid());
    assert_eq!(id.0, test_uuid());
}

#[test]
fn test_demo_world_ref_creation() {
    let id = DemoWorldRef(test_uuid());
    assert_eq!(id.0, test_uuid());
}

#[test]
fn test_terrain_binding_ref_creation() {
    let id = TerrainBindingRef(test_uuid());
    assert_eq!(id.0, test_uuid());
}

#[test]
fn test_sky_environment_binding_ref_creation() {
    let id = SkyEnvironmentBindingRef(test_uuid());
    assert_eq!(id.0, test_uuid());
}

#[test]
fn test_runtime_entry_ref_creation() {
    let id = RuntimeEntryRef(test_uuid());
    assert_eq!(id.0, test_uuid());
}

#[test]
fn test_workspace_ref_creation() {
    let id = WorkspaceRef(test_uuid());
    assert_eq!(id.0, test_uuid());
}

#[test]
fn test_profile_ref_creation() {
    let id = ProfileRef(test_uuid());
    assert_eq!(id.0, test_uuid());
}

#[test]
fn test_interaction_target_ref_creation() {
    let id = InteractionTargetRef(test_uuid());
    assert_eq!(id.0, test_uuid());
}

// ============================================================================
// ViewportId Tests
// ============================================================================

#[test]
fn test_viewport_id_variants() {
    let _ = ViewportId::Primary;
    let _ = ViewportId::Secondary;
    let _ = ViewportId::Tertiary;
    let _ = ViewportId::Quaternary;
    let custom = ViewportId::Custom(42);
    if let ViewportId::Custom(v) = custom {
        assert_eq!(v, 42);
    }
}

#[test]
fn test_viewport_id_equality() {
    let a = ViewportId::Primary;
    let b = ViewportId::Primary;
    assert_eq!(a, b);
}

#[test]
fn test_viewport_id_inequality() {
    let a = ViewportId::Primary;
    let b = ViewportId::Secondary;
    assert_ne!(a, b);
}

#[test]
fn test_viewport_id_serialize_roundtrip() {
    let id = ViewportId::Primary;
    let json = serde_json::to_string(&id).unwrap();
    let parsed: ViewportId = serde_json::from_str(&json).unwrap();
    assert_eq!(id, parsed);
}

#[test]
fn test_viewport_id_custom_serialize_roundtrip() {
    let id = ViewportId::Custom(99);
    let json = serde_json::to_string(&id).unwrap();
    let parsed: ViewportId = serde_json::from_str(&json).unwrap();
    assert_eq!(id, parsed);
}

// ============================================================================
// PanelId Tests
// ============================================================================

#[test]
fn test_panel_id_all_variants() {
    let _ = PanelId::WorldOutliner;
    let _ = PanelId::ContentBrowser;
    let _ = PanelId::PropertyInspector;
    let _ = PanelId::RuntimeInspector;
    let _ = PanelId::DiagnosticsPanel;
    let _ = PanelId::ConsolePanel;
    let _ = PanelId::BuildPanel;
    let _ = PanelId::ViewportStatusRail;
    let custom = PanelId::Custom("my-panel".to_string());
    if let PanelId::Custom(label) = custom {
        assert_eq!(label, "my-panel");
    }
}

#[test]
fn test_panel_id_equality() {
    let a = PanelId::WorldOutliner;
    let b = PanelId::WorldOutliner;
    assert_eq!(a, b);
}

#[test]
fn test_panel_id_serialize_roundtrip() {
    let id = PanelId::PropertyInspector;
    let json = serde_json::to_string(&id).unwrap();
    let parsed: PanelId = serde_json::from_str(&json).unwrap();
    assert_eq!(id, parsed);
}

// ============================================================================
// World Lifecycle Tests
// ============================================================================

#[test]
fn test_open_mode_variants() {
    let _ = OpenMode::Startup;
    let _ = OpenMode::Recent;
    let _ = OpenMode::RestoreSession;
    let _ = OpenMode::Explicit;
}

#[test]
fn test_restore_policy_variants() {
    let _ = RestorePolicy::RestoreFull;
    let _ = RestorePolicy::RestorePartial;
    let _ = RestorePolicy::FreshOpen;
}

#[test]
fn test_world_open_request_creation() {
    let request = WorldOpenRequest {
        world_ref: Some(stable_world_id()),
        open_mode: OpenMode::Startup,
        restore_policy: RestorePolicy::RestoreFull,
    };
    assert!(request.world_ref.is_some());
}

#[test]
fn test_world_open_request_null_world() {
    let request = WorldOpenRequest {
        world_ref: None,
        open_mode: OpenMode::Recent,
        restore_policy: RestorePolicy::FreshOpen,
    };
    assert!(request.world_ref.is_none());
}

#[test]
fn test_world_open_result_accepted() {
    let result = WorldOpenResult {
        accepted: true,
        world_ref: Some(stable_world_id()),
        world_label: Some("My World".to_string()),
        failure_class: None,
        recovery_hints: vec![],
    };
    assert!(result.accepted);
}

#[test]
fn test_world_open_result_failure() {
    let result = WorldOpenResult {
        accepted: false,
        world_ref: None,
        world_label: None,
        failure_class: Some(FailureClass::WorldNotFound),
        recovery_hints: vec!["Check path".to_string()],
    };
    assert!(!result.accepted);
    assert!(matches!(
        result.failure_class,
        Some(FailureClass::WorldNotFound)
    ));
}

#[test]
fn test_failure_class_all_variants() {
    let _ = FailureClass::WorldNotFound;
    let _ = FailureClass::CorruptedData;
    let _ = FailureClass::IncompatibleVersion;
    let _ = FailureClass::MissingDependencies;
    let _ = FailureClass::PermissionDenied;
    let _ = FailureClass::EngineError;
    let _ = FailureClass::Unknown("custom".to_string());
}

#[test]
fn test_bind_posture_variants() {
    let _ = BindPosture::Unbound;
    let _ = BindPosture::Binding;
    let _ = BindPosture::Bound;
    let _ = BindPosture::Degraded;
    let _ = BindPosture::Failed;
}

#[test]
fn test_world_bind_state_creation() {
    let state = WorldBindState {
        world_ref: stable_world_id(),
        terrain_ref: Some(TerrainBindingRef(test_uuid())),
        environment_ref: Some(SkyEnvironmentBindingRef(test_uuid())),
        posture: BindPosture::Bound,
    };
    assert!(state.terrain_ref.is_some());
    assert!(state.environment_ref.is_some());
}

#[test]
fn test_world_bind_state_unbound() {
    let state = WorldBindState {
        world_ref: stable_world_id(),
        terrain_ref: None,
        environment_ref: None,
        posture: BindPosture::Unbound,
    };
    assert!(state.terrain_ref.is_none());
}

#[test]
fn test_world_summary_dto_creation() {
    let summary = WorldSummaryDto {
        world_ref: stable_world_id(),
        world_label: "Test World".to_string(),
        world_role: "startup".to_string(),
        open_mode: OpenMode::Startup,
        bind_posture: BindPosture::Bound,
        active_selection_count: 5,
    };
    assert_eq!(summary.active_selection_count, 5);
}

// ============================================================================
// Content State Tests
// ============================================================================

#[test]
fn test_lod_posture_variants() {
    let _ = LodPosture::Full;
    let _ = LodPosture::Reduced;
    let _ = LodPosture::Minimal;
}

#[test]
fn test_terrain_state_dto_creation() {
    let state = TerrainStateDto {
        world_ref: stable_world_id(),
        terrain_binding_ref: Some(TerrainBindingRef(test_uuid())),
        present: true,
        walkable: true,
        material_profile_ref: None,
        lod_posture: LodPosture::Full,
        degraded: false,
    };
    assert!(state.present);
}

#[test]
fn test_cloud_posture_variants() {
    let _ = CloudPosture::Clear;
    let _ = CloudPosture::Scattered;
    let _ = CloudPosture::Overcast;
    let _ = CloudPosture::Storm;
}

#[test]
fn test_fog_posture_variants() {
    let _ = FogPosture::None;
    let _ = FogPosture::Light;
    let _ = FogPosture::Medium;
    let _ = FogPosture::Heavy;
}

#[test]
fn test_precipitation_posture_variants() {
    let _ = PrecipitationPosture::None;
    let _ = PrecipitationPosture::LightRain;
    let _ = PrecipitationPosture::HeavyRain;
    let _ = PrecipitationPosture::Snow;
    let _ = PrecipitationPosture::Hail;
}

#[test]
fn test_weather_regime_variants() {
    let _ = WeatherRegime::Clear;
    let _ = WeatherRegime::Scattered;
    let _ = WeatherRegime::Overcast;
    let _ = WeatherRegime::IncomingStorm;
    let _ = WeatherRegime::HeavyStorm;
    let _ = WeatherRegime::PostStormCalm;
    let _ = WeatherRegime::FogMorning;
    let _ = WeatherRegime::WindyOvercast;
}

#[test]
fn test_weather_regime_serialize_roundtrip() {
    let regime = WeatherRegime::HeavyStorm;
    let json = serde_json::to_string(&regime).unwrap();
    let parsed: WeatherRegime = serde_json::from_str(&json).unwrap();
    assert_eq!(regime, parsed);
}

#[test]
fn test_cloud_shadow_posture_variants() {
    let _ = CloudShadowPosture::Full;
    let _ = CloudShadowPosture::Simplified;
    let _ = CloudShadowPosture::Fallback;
    let _ = CloudShadowPosture::Disabled;
}

#[test]
fn test_cloud_shadow_posture_serialize_roundtrip() {
    let posture = CloudShadowPosture::Full;
    let json = serde_json::to_string(&posture).unwrap();
    let parsed: CloudShadowPosture = serde_json::from_str(&json).unwrap();
    assert_eq!(posture, parsed);
}

#[test]
fn test_sky_state_dto_creation() {
    let state = SkyStateDto {
        world_ref: stable_world_id(),
        environment_binding_ref: Some(SkyEnvironmentBindingRef(test_uuid())),
        present: true,
        time_of_day: 12.0,
        date_or_cycle_ref: None,
        cloud_posture: CloudPosture::Clear,
        fog_posture: FogPosture::None,
        precipitation_posture: PrecipitationPosture::None,
        degraded: false,
    };
    assert_eq!(state.time_of_day, 12.0);
}

// ============================================================================
// Runtime Entry Tests
// ============================================================================

#[test]
fn test_runtime_mode_variants() {
    let _ = RuntimeMode::Play;
    let _ = RuntimeMode::Simulate;
}

#[test]
fn test_camera_policy_variants() {
    let _ = CameraPolicy::FreeCam;
    let _ = CameraPolicy::PossessWalkPawn;
    let _ = CameraPolicy::FollowEntity;
    let _ = CameraPolicy::Fixed;
}

#[test]
fn test_runtime_entry_request_creation() {
    let request = RuntimeEntryRequest {
        world_ref: stable_world_id(),
        mode: RuntimeMode::Play,
        camera_policy: CameraPolicy::FreeCam,
    };
    assert!(matches!(request.mode, RuntimeMode::Play));
}

#[test]
fn test_runtime_entry_result_accepted() {
    let result = RuntimeEntryResult {
        accepted: true,
        runtime_ref: Some(RuntimeEntryRef(test_uuid())),
        deny_reason: None,
    };
    assert!(result.accepted);
}

#[test]
fn test_runtime_entry_result_denied() {
    let result = RuntimeEntryResult {
        accepted: false,
        runtime_ref: None,
        deny_reason: Some("Engine busy".to_string()),
    };
    assert!(!result.accepted);
    assert!(result.deny_reason.is_some());
}

#[test]
fn test_runtime_entry_dto_creation() {
    let dto = RuntimeEntryDto {
        accepted: true,
        runtime_ref: Some(RuntimeEntryRef(test_uuid())),
        mode: RuntimeMode::Play,
        world_ref: stable_world_id(),
        deny_reason: None,
    };
    assert!(dto.accepted);
}

#[test]
fn test_return_to_authoring_result_creation() {
    let result = ReturnToAuthoringResult {
        accepted: true,
        world_ref: stable_world_id(),
        camera_policy: CameraPolicy::FreeCam,
        discarded_runtime_state: true,
    };
    assert!(result.discarded_runtime_state);
}

#[test]
fn test_preview_state_dto_creation() {
    let dto = PreviewStateDto {
        session_active: true,
        world_ref: Some(stable_world_id()),
        frame_posture: "live".to_string(),
        runtime_attached: true,
        degraded: false,
    };
    assert!(dto.session_active);
}

// ============================================================================
// Viewport Frame Tests
// ============================================================================

#[test]
fn test_frame_posture_variants() {
    let _ = FramePosture::Boot;
    let _ = FramePosture::Live;
    let _ = FramePosture::Degraded;
    let _ = FramePosture::Failure;
    let _ = FramePosture::PlayAttached;
    let _ = FramePosture::SimulateAttached;
}

#[test]
fn test_boot_surface_kind_variants() {
    let _ = BootSurfaceKind::Empty;
    let _ = BootSurfaceKind::Bound;
    let _ = BootSurfaceKind::Degraded;
}

#[test]
fn test_viewport_boot_frame_creation() {
    let frame = ViewportBootFrame {
        world_ref: stable_world_id(),
        viewport_id: ViewportId::Primary,
        posture: FramePosture::Boot,
        surface_kind: BootSurfaceKind::Empty,
    };
    assert!(matches!(frame.posture, FramePosture::Boot));
}

#[test]
fn test_viewport_live_frame_creation() {
    let frame = ViewportLiveFrame {
        world_ref: stable_world_id(),
        viewport_id: ViewportId::Primary,
        frame_ref: 100,
        terrain_ref: Some(TerrainBindingRef(test_uuid())),
        environment_ref: Some(SkyEnvironmentBindingRef(test_uuid())),
        weather_state: None,
    };
    assert_eq!(frame.frame_ref, 100);
}

#[test]
fn test_weather_state_snapshot_creation() {
    let snapshot = WeatherStateSnapshot {
        time_of_day: 14.0,
        sun_elevation_deg: 45.0,
        cloud_coverage: 0.5,
        fog_density: 0.1,
        rain_enabled: false,
        weather_regime: "clear".to_string(),
        cloud_shadow_active: true,
        weather_cells_count: 10,
    };
    assert_eq!(snapshot.weather_cells_count, 10);
}

#[test]
fn test_degraded_reason_variants() {
    let _ = DegradedReason::LowMemory;
    let _ = DegradedReason::HighLatency;
    let _ = DegradedReason::MissingAssets;
    let _ = DegradedReason::PartialBinding;
    let _ = DegradedReason::Unknown("custom".to_string());
}

#[test]
fn test_fidelity_class_variants() {
    let _ = FidelityClass::Full;
    let _ = FidelityClass::Reduced;
    let _ = FidelityClass::Minimal;
    let _ = FidelityClass::Placeholder;
}

#[test]
fn test_viewport_degraded_frame_creation() {
    let frame = ViewportDegradedFrame {
        world_ref: stable_world_id(),
        viewport_id: ViewportId::Primary,
        degraded_reason: DegradedReason::LowMemory,
        fidelity_class: FidelityClass::Reduced,
    };
    assert!(matches!(frame.degraded_reason, DegradedReason::LowMemory));
}

#[test]
fn test_viewport_failure_projection_creation() {
    let proj = ViewportFailureProjection {
        viewport_id: ViewportId::Primary,
        failure_class: FailureClass::EngineError,
        recovery_actions: vec!["Restart".to_string()],
    };
    assert!(!proj.recovery_actions.is_empty());
}

#[test]
fn test_camera_state_creation() {
    let state = CameraState {
        position: [0.0, 5.0, -10.0],
        rotation: [0.0, 0.0, 0.0, 1.0],
        fov: 90.0,
    };
    assert_eq!(state.fov, 90.0);
}

#[test]
fn test_viewport_play_attached_frame_creation() {
    let frame = ViewportPlayAttachedFrame {
        runtime_ref: RuntimeEntryRef(test_uuid()),
        world_ref: stable_world_id(),
        viewport_id: ViewportId::Primary,
        camera_state: CameraState {
            position: [0.0; 3],
            rotation: [0.0, 0.0, 0.0, 1.0],
            fov: 90.0,
        },
    };
    assert!(matches!(frame.viewport_id, ViewportId::Primary));
}

#[test]
fn test_viewport_simulate_attached_frame_creation() {
    let frame = ViewportSimulateAttachedFrame {
        runtime_ref: RuntimeEntryRef(test_uuid()),
        world_ref: stable_world_id(),
        viewport_id: ViewportId::Custom(5),
        camera_state: CameraState {
            position: [1.0, 2.0, 3.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            fov: 60.0,
        },
    };
    if let ViewportId::Custom(v) = frame.viewport_id {
        assert_eq!(v, 5);
    }
}

#[test]
fn test_viewport_frame_dto_creation() {
    let dto = ViewportFrameDto {
        world_ref: stable_world_id(),
        viewport_id: ViewportId::Primary,
        frame_posture: FramePosture::Live,
        terrain_present: true,
        environment_present: true,
        runtime_attached: false,
        failure_class: None,
        weather_state: None,
    };
    assert!(dto.terrain_present);
}

#[test]
fn test_budget_posture_variants() {
    let _ = BudgetPosture::Healthy;
    let _ = BudgetPosture::Warning;
    let _ = BudgetPosture::Critical;
}

#[test]
fn test_viewport_stats_dto_creation() {
    let stats = ViewportStatsDto {
        fps: 60.0,
        frame_time_ms: 16.67,
        budget_posture: BudgetPosture::Healthy,
        terrain_posture: "full".to_string(),
        environment_posture: "clear".to_string(),
        runtime_posture: "active".to_string(),
        weather_posture: "none".to_string(),
        cloud_shadow_posture: "full".to_string(),
    };
    assert_eq!(stats.fps, 60.0);
}

// ============================================================================
// Diagnostics Tests
// ============================================================================

#[test]
fn test_posture_variants() {
    let _ = Posture::Healthy;
    let _ = Posture::Degraded;
    let _ = Posture::Failed;
    let _ = Posture::Unknown;
}

#[test]
fn test_diagnostics_summary_dto_creation() {
    let summary = DiagnosticsSummaryDto {
        host_posture: Posture::Healthy,
        world_posture: Posture::Healthy,
        terrain_posture: Posture::Degraded,
        environment_posture: Posture::Healthy,
        viewport_posture: Posture::Healthy,
        runtime_posture: Posture::Healthy,
        bridge_posture: Posture::Healthy,
        degradation_posture: Posture::Healthy,
        weather_diagnostics: None,
    };
    assert!(matches!(summary.host_posture, Posture::Healthy));
}

#[test]
fn test_weather_diagnostics_creation() {
    let diag = WeatherDiagnostics {
        sky_bundle_status: "loaded".to_string(),
        asset_root_resolved: Some("/path".to_string()),
        stars_present: true,
        moon_albedo_present: true,
        moon_height_present: false,
        moon_normal_present: false,
        sun_profile_active: true,
        weather_director_active: true,
        weather_cells_count: 8,
        cloud_shadow_posture: "full".to_string(),
        weather_regime: "clear".to_string(),
        render_pass_active: true,
        viewport_mode: "perspective".to_string(),
    };
    assert_eq!(diag.weather_cells_count, 8);
}

#[test]
fn test_diagnostic_level_variants() {
    let _ = DiagnosticLevel::Info;
    let _ = DiagnosticLevel::Warning;
    let _ = DiagnosticLevel::Error;
}

#[test]
fn test_diagnostic_entry_creation() {
    let entry = DiagnosticEntry {
        level: DiagnosticLevel::Warning,
        message: "Low memory".to_string(),
        source: "memory".to_string(),
        location: Some("heap".to_string()),
    };
    assert!(matches!(entry.level, DiagnosticLevel::Warning));
}

#[test]
fn test_viewport_diagnostics_projection_creation() {
    let proj = ViewportDiagnosticsProjection {
        host: Posture::Healthy,
        world: Posture::Healthy,
        terrain: Posture::Degraded,
        environment: Posture::Healthy,
        runtime: Posture::Healthy,
        budgets: Posture::Degraded,
    };
    assert!(matches!(proj.host, Posture::Healthy));
}

// ============================================================================
// World Package Tests
// ============================================================================

#[test]
fn test_world_role_variants() {
    let _ = WorldRole::Startup;
    let _ = WorldRole::Reference;
    let _ = WorldRole::Demo;
    let _ = WorldRole::Content;
}

#[test]
fn test_world_role_serialize_roundtrip() {
    for role in [
        WorldRole::Startup,
        WorldRole::Reference,
        WorldRole::Demo,
        WorldRole::Content,
    ] {
        let json = serde_json::to_string(&role).unwrap();
        let parsed: WorldRole = serde_json::from_str(&json).unwrap();
        assert_eq!(role, parsed);
    }
}

#[test]
fn test_source_lineage_creation() {
    let lineage = SourceLineage {
        created_at: "2024-01-01".to_string(),
        created_by: "author".to_string(),
        import_source: Some("external".to_string()),
        last_modified: "2024-01-02".to_string(),
    };
    assert!(lineage.import_source.is_some());
}

#[test]
fn test_source_lineage_no_import() {
    let lineage = SourceLineage {
        created_at: "2024-01-01".to_string(),
        created_by: "author".to_string(),
        import_source: None,
        last_modified: "2024-01-01".to_string(),
    };
    assert!(lineage.import_source.is_none());
}

#[test]
fn test_world_package_manifest_creation() {
    let manifest = WorldPackageManifest {
        world_id: test_uuid(),
        world_label: "My World".to_string(),
        world_role: WorldRole::Startup,
        version: "1.0.0".to_string(),
        terrain_root_ref: Some("terrain_001".to_string()),
        environment_root_ref: Some("sky_default".to_string()),
        streaming_profile_ref: None,
        source_lineage: SourceLineage {
            created_at: "2024-01-01".to_string(),
            created_by: "author".to_string(),
            import_source: None,
            last_modified: "2024-01-01".to_string(),
        },
    };
    assert_eq!(manifest.world_label, "My World");
}

#[test]
fn test_chunk_header_new() {
    let header = ChunkHeader::new(256, 256);
    assert_eq!(header.magic, ChunkHeader::MAGIC);
    assert_eq!(header.version, ChunkHeader::VERSION);
    assert!(header.is_valid());
}

#[test]
fn test_chunk_header_invalid_magic() {
    let mut header = ChunkHeader::new(64, 64);
    header.magic = *b"XXXX";
    assert!(!header.is_valid());
}

#[test]
fn test_chunk_data_new() {
    let data = ChunkData::new(4, 4);
    assert_eq!(data.heights.len(), 16);
    assert_eq!(data.material_weights.len(), 16);
    assert!(data.header.is_valid());
}

#[test]
fn test_chunk_data_to_bytes_roundtrip() {
    let data = ChunkData::new(2, 2);
    let bytes = data.to_bytes();
    let parsed = ChunkData::from_bytes(&bytes).unwrap();
    assert_eq!(data.header.width, parsed.header.width);
}

#[test]
fn test_chunk_data_from_bytes_invalid() {
    let bytes = [0u8; 4];
    assert!(ChunkData::from_bytes(&bytes).is_err());
}

#[test]
fn test_chunk_data_from_bytes_bad_magic() {
    let mut bytes = vec![0u8; ChunkHeader::SIZE];
    bytes[0..4].copy_from_slice(b"XXXX");
    assert!(ChunkData::from_bytes(&bytes).is_err());
}

#[test]
fn test_sky_binding_creation() {
    let binding = SkyBinding {
        sky_bundle_ref: "sky/default".to_string(),
        time_of_day: 12.0,
        day_of_year: 180,
        latitude_deg: 45.0,
        weather_regime: "clear".to_string(),
        cloud_coverage: 0.3,
        fog_density: 0.1,
    };
    assert_eq!(binding.time_of_day, 12.0);
}

#[test]
fn test_terrain_manifest_creation() {
    let manifest = TerrainManifest {
        terrain_id: test_uuid(),
        origin: [0.0; 3],
        world_size: [1000.0; 2],
        resolution: [1024, 1024],
        chunk_grid: [4, 4],
        chunk_size: 256,
        height_range: [-100.0, 500.0],
        material_layers: vec![],
        chunks: vec![],
    };
    assert_eq!(manifest.chunk_size, 256);
}

#[test]
fn test_material_layer_creation() {
    let layer = MaterialLayer {
        layer_id: 0,
        material_family: "rock".to_string(),
        density_kg_m3: 2500.0,
        albedo_texture_ref: Some("rock.png".to_string()),
        normal_texture_ref: None,
        orm_texture_ref: None,
        uv_scale: [1.0, 1.0],
        base_tint: [1.0; 4],
    };
    assert_eq!(layer.density_kg_m3, 2500.0);
}

#[test]
fn test_chunk_descriptor_creation() {
    let desc = ChunkDescriptor {
        chunk_x: 3,
        chunk_y: 5,
        data_file: "chunk_3_5.dat".to_string(),
        revision: 42,
    };
    assert_eq!(desc.revision, 42);
}

// ============================================================================
// Serialization Roundtrip Tests (types that support it)
// ============================================================================

#[test]
fn test_stable_entity_id_serialize_roundtrip() {
    let id = StableEntityId(test_uuid());
    let json = serde_json::to_string(&id).unwrap();
    let parsed: StableEntityId = serde_json::from_str(&json).unwrap();
    assert_eq!(id.0, parsed.0);
}

#[test]
fn test_stable_world_id_serialize_roundtrip() {
    let id = StableWorldId(test_uuid());
    let json = serde_json::to_string(&id).unwrap();
    let parsed: StableWorldId = serde_json::from_str(&json).unwrap();
    assert_eq!(id.0, parsed.0);
}

#[test]
fn test_weather_regime_all_serialize_roundtrip() {
    let regimes = [
        WeatherRegime::Clear,
        WeatherRegime::Scattered,
        WeatherRegime::Overcast,
        WeatherRegime::IncomingStorm,
        WeatherRegime::HeavyStorm,
        WeatherRegime::PostStormCalm,
        WeatherRegime::FogMorning,
        WeatherRegime::WindyOvercast,
    ];
    for regime in regimes {
        let json = serde_json::to_string(&regime).unwrap();
        let parsed: WeatherRegime = serde_json::from_str(&json).unwrap();
        assert_eq!(regime, parsed);
    }
}

#[test]
fn test_viewport_id_all_serialize_roundtrip() {
    let ids = [
        ViewportId::Primary,
        ViewportId::Secondary,
        ViewportId::Tertiary,
        ViewportId::Quaternary,
        ViewportId::Custom(42),
    ];
    for id in ids {
        let json = serde_json::to_string(&id).unwrap();
        let parsed: ViewportId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }
}

#[test]
fn test_panel_id_all_serialize_roundtrip() {
    let ids = [
        PanelId::WorldOutliner,
        PanelId::ContentBrowser,
        PanelId::PropertyInspector,
        PanelId::Custom("test".to_string()),
    ];
    for id in ids {
        let json = serde_json::to_string(&id).unwrap();
        let parsed: PanelId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }
}

#[test]
fn test_chunk_header_serialize_roundtrip() {
    let header = ChunkHeader::new(512, 512);
    let json = serde_json::to_string(&header).unwrap();
    let parsed: ChunkHeader = serde_json::from_str(&json).unwrap();
    assert_eq!(header.width, parsed.width);
    assert_eq!(header.height, parsed.height);
}

#[test]
fn test_world_package_manifest_serialize_roundtrip() {
    let manifest = WorldPackageManifest {
        world_id: test_uuid(),
        world_label: "Test".to_string(),
        world_role: WorldRole::Demo,
        version: "0.1.0".to_string(),
        terrain_root_ref: None,
        environment_root_ref: None,
        streaming_profile_ref: None,
        source_lineage: SourceLineage {
            created_at: "2024-01-01".to_string(),
            created_by: "test".to_string(),
            import_source: None,
            last_modified: "2024-01-01".to_string(),
        },
    };
    let json = serde_json::to_string(&manifest).unwrap();
    let parsed: WorldPackageManifest = serde_json::from_str(&json).unwrap();
    assert_eq!(manifest.world_label, parsed.world_label);
    assert_eq!(manifest.world_role, parsed.world_role);
}

#[test]
fn test_camera_state_serialize_roundtrip() {
    let state = CameraState {
        position: [0.0, 5.0, -10.0],
        rotation: [0.0, 0.0, 0.0, 1.0],
        fov: 90.0,
    };
    let json = serde_json::to_string(&state).unwrap();
    let parsed: CameraState = serde_json::from_str(&json).unwrap();
    assert_eq!(state.position, parsed.position);
    assert_eq!(state.fov, parsed.fov);
}

#[test]
fn test_sky_binding_serialize_roundtrip() {
    let binding = SkyBinding {
        sky_bundle_ref: "sky/default".to_string(),
        time_of_day: 12.0,
        day_of_year: 180,
        latitude_deg: 45.0,
        weather_regime: "clear".to_string(),
        cloud_coverage: 0.3,
        fog_density: 0.1,
    };
    let json = serde_json::to_string(&binding).unwrap();
    let parsed: SkyBinding = serde_json::from_str(&json).unwrap();
    assert_eq!(binding.time_of_day, parsed.time_of_day);
    assert_eq!(binding.weather_regime, parsed.weather_regime);
}
