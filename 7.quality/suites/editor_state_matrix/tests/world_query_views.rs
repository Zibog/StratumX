use std::path::PathBuf;

use stratumx_editor_state_containers::owners::WorldOwner;
use stratumx_editor_state_containers::queries::world_queries::{
    MaterialCoverageSummaryView, WorldDiagnosticsSummaryView, WorldEnvironmentSummaryView,
    WorldIdentityView, WorldTerrainSummaryView,
};
use stratumx_editor_state_containers::queries::ReadModel;
use stratumx_editor_state_containers::{
    DiagnosticMessage, DiagnosticSource, EnvironmentState, Severity, TerrainState,
    WeatherCondition, WorldIdentity,
};
use uuid::Uuid;

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
    assert_eq!(environment_view.weather_condition, WeatherCondition::Rain);
    assert_eq!(diagnostics_view.error_count, 1);
    assert_eq!(material_view.total_materials, 0);
}
