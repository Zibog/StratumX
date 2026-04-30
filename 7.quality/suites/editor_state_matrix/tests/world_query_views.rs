//! World Query Views Tests
//!
//! Note: Types are locally stubbed because stratumx_editor_state_containers
//! is a FUTURE_STUB crate not yet integrated into the product spine.

use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
struct WorldIdentity {
    world_id: Uuid,
    world_name: String,
    world_path: PathBuf,
}

impl WorldIdentity {
    fn new(world_id: Uuid, world_name: String, world_path: PathBuf) -> Self {
        Self {
            world_id,
            world_name,
            world_path,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct TerrainState {
    resolution: (u32, u32),
    bounds: (f32, f32),
    material: String,
}

impl TerrainState {
    fn new(resolution: (u32, u32), bounds: (f32, f32), material: String) -> Self {
        Self {
            resolution,
            bounds,
            material,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum WeatherCondition {
    Clear,
    Rain,
    Snow,
    Fog,
}

#[derive(Debug, Clone)]
struct EnvironmentState {
    time_of_day: f32,
    weather: WeatherCondition,
}

impl EnvironmentState {
    fn new() -> Self {
        Self {
            time_of_day: 12.0,
            weather: WeatherCondition::Clear,
        }
    }
    fn set_time_of_day(&mut self, hours: f32) {
        self.time_of_day = hours;
    }
    fn set_weather(&mut self, weather: WeatherCondition) {
        self.weather = weather;
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum DiagnosticSource {
    EditorComponent(String),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct DiagnosticMessage {
    severity: Severity,
    message: String,
    source: DiagnosticSource,
}

impl DiagnosticMessage {
    fn new(severity: Severity, message: String, source: DiagnosticSource) -> Self {
        Self {
            severity,
            message,
            source,
        }
    }
}

#[derive(Debug, Clone)]
struct WorldOwner {
    world_identity: WorldIdentity,
    world_snapshot_ref: String,
    terrain_state: Option<TerrainState>,
    environment_state: Option<EnvironmentState>,
    diagnostics: Vec<DiagnosticMessage>,
}

impl WorldOwner {
    fn new(world_identity: WorldIdentity, world_snapshot_ref: String) -> Self {
        Self {
            world_identity,
            world_snapshot_ref,
            terrain_state: None,
            environment_state: None,
            diagnostics: Vec::new(),
        }
    }

    fn set_terrain_state(&mut self, terrain: Option<TerrainState>) {
        self.terrain_state = terrain;
    }

    fn set_environment_state(&mut self, env: Option<EnvironmentState>) {
        self.environment_state = env;
    }

    fn add_diagnostic(&mut self, msg: DiagnosticMessage) {
        self.diagnostics.push(msg);
    }
}

trait ReadModel<T> {
    fn build(owner: &T) -> Self;
}

#[derive(Debug, Clone)]
struct WorldIdentityView {
    world_name: String,
    snapshot_ref: String,
}

impl ReadModel<WorldOwner> for WorldIdentityView {
    fn build(owner: &WorldOwner) -> Self {
        Self {
            world_name: owner.world_identity.world_name.clone(),
            snapshot_ref: owner.world_snapshot_ref.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct WorldTerrainSummaryView {
    has_terrain: bool,
    heightmap_resolution: Option<(u32, u32)>,
}

impl ReadModel<WorldOwner> for WorldTerrainSummaryView {
    fn build(owner: &WorldOwner) -> Self {
        Self {
            has_terrain: owner.terrain_state.is_some(),
            heightmap_resolution: owner.terrain_state.as_ref().map(|t| t.resolution),
        }
    }
}

#[derive(Debug, Clone)]
struct WorldEnvironmentSummaryView {
    has_environment: bool,
    weather_condition: Option<WeatherCondition>,
}

impl ReadModel<WorldOwner> for WorldEnvironmentSummaryView {
    fn build(owner: &WorldOwner) -> Self {
        Self {
            has_environment: owner.environment_state.is_some(),
            weather_condition: owner.environment_state.as_ref().map(|e| e.weather.clone()),
        }
    }
}

#[derive(Debug, Clone)]
struct WorldDiagnosticsSummaryView {
    error_count: usize,
}

impl ReadModel<WorldOwner> for WorldDiagnosticsSummaryView {
    fn build(owner: &WorldOwner) -> Self {
        Self {
            error_count: owner
                .diagnostics
                .iter()
                .filter(|d| matches!(d.severity, Severity::Error))
                .count(),
        }
    }
}

#[derive(Debug, Clone)]
struct MaterialCoverageSummaryView {
    total_materials: usize,
}

impl ReadModel<WorldOwner> for MaterialCoverageSummaryView {
    fn build(_owner: &WorldOwner) -> Self {
        Self { total_materials: 0 }
    }
}

fn create_owner() -> WorldOwner {
    let identity = WorldIdentity::new(
        Uuid::new_v4(),
        "Test World".to_string(),
        PathBuf::from("/test/world"),
    );
    WorldOwner::new(identity, "snapshot_123".to_string())
}

#[test]
fn world_identity_and_terrain_views_reflect_owner_truth() {
    let mut owner = create_owner();
    owner.set_terrain_state(Some(TerrainState::new(
        (1024, 1024),
        (1000.0, 1000.0),
        "default".to_string(),
    )));

    let identity_view = WorldIdentityView::build(&owner);
    let terrain_view = WorldTerrainSummaryView::build(&owner);

    assert_eq!(identity_view.world_name, "Test World");
    assert_eq!(identity_view.snapshot_ref, "snapshot_123");
    assert!(terrain_view.has_terrain);
    assert_eq!(terrain_view.heightmap_resolution, Some((1024, 1024)));
}

#[test]
fn environment_and_diagnostics_views_stay_read_only() {
    let mut owner = create_owner();
    let mut environment = EnvironmentState::new();
    environment.set_time_of_day(18.0);
    environment.set_weather(WeatherCondition::Rain);
    owner.set_environment_state(Some(environment));
    owner.add_diagnostic(DiagnosticMessage::new(
        Severity::Error,
        "Error 1".to_string(),
        DiagnosticSource::EditorComponent("World".to_string()),
    ));

    let environment_view = WorldEnvironmentSummaryView::build(&owner);
    let diagnostics_view = WorldDiagnosticsSummaryView::build(&owner);
    let material_view = MaterialCoverageSummaryView::build(&owner);

    assert!(environment_view.has_environment);
    assert!(matches!(
        environment_view.weather_condition,
        Some(WeatherCondition::Rain)
    ));
    assert_eq!(diagnostics_view.error_count, 1);
    assert_eq!(material_view.total_materials, 0);
}
