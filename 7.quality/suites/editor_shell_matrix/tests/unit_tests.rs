use std::fs;

use tempfile::tempdir;

use stratumx_editor_l8_0_editor_shell::{
    filter_command_palette, DockPosition, DockingManager, GeneratedQualitySummary, LayoutState,
    ProjectDialogKind, ShellRuntime, WorkspaceStage,
};
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

#[test]
fn shell_runtime_persists_and_restores_layout() {
    let temp = tempdir().expect("tempdir");
    let layout_path = temp.path().join("layout.json");

    let mut runtime = ShellRuntime::new();
    runtime.toggle_panel("terrain");
    runtime.toggle_panel("environment");

    let saved = runtime
        .save_workspace_layout(&layout_path)
        .expect("save layout");
    assert!(layout_path.exists());

    let mut restored = ShellRuntime::new();
    restored
        .restore_workspace_layout(saved.clone())
        .expect("restore layout");

    assert_eq!(restored.get_open_panels(), saved.visible_panel_ids());
}

#[test]
fn shell_runtime_loads_generated_quality_summary() {
    let temp = tempdir().expect("tempdir");
    let generated = temp.path().join("generated");
    fs::create_dir_all(generated.join("test-results")).expect("test-results dir");
    fs::create_dir_all(generated.join("metrics")).expect("metrics dir");

    fs::write(
        generated.join("test-results").join("verify-summary.json"),
        r#"{"status":"passed"}"#,
    )
    .expect("verify");
    fs::write(
        generated.join("test-results").join("smoke-summary.json"),
        r#"{"status":"passed"}"#,
    )
    .expect("smoke");
    fs::write(
        generated.join("test-results").join("full-summary.json"),
        r#"{"status":"warning"}"#,
    )
    .expect("full");
    fs::write(
        generated.join("metrics").join("metrics-summary.json"),
        r#"{"declared_tests":42,"route_coverage":57,"workspace_packages":230}"#,
    )
    .expect("metrics");

    let mut runtime = ShellRuntime::new();
    runtime.set_generated_artifacts_root(generated);
    runtime
        .refresh_quality_artifacts()
        .expect("quality summary");

    assert_eq!(
        runtime
            .status_bar
            .quality_summary
            .as_ref()
            .and_then(|summary| summary.verify_status.clone()),
        Some("passed".to_string())
    );
    assert_eq!(
        runtime
            .status_bar
            .quality_summary
            .as_ref()
            .map(|summary| summary.declared_tests),
        Some(42)
    );
}

#[test]
fn command_palette_filter_returns_matching_entries() {
    let filtered = filter_command_palette("save");
    assert!(filtered.iter().any(|entry| entry.id == "file.save"));
    assert!(!filtered.is_empty());
}

#[test]
fn generated_quality_summary_load_reads_status_and_metrics() {
    let temp = tempdir().expect("tempdir");
    let generated = temp.path().join("generated");
    fs::create_dir_all(generated.join("test-results")).expect("test-results dir");
    fs::create_dir_all(generated.join("metrics")).expect("metrics dir");

    fs::write(
        generated.join("test-results").join("verify-summary.json"),
        r#"{"status":"passed"}"#,
    )
    .expect("verify");
    fs::write(
        generated.join("test-results").join("smoke-summary.json"),
        r#"{"status":"passed"}"#,
    )
    .expect("smoke");
    fs::write(
        generated.join("test-results").join("full-summary.json"),
        r#"{"status":"passed"}"#,
    )
    .expect("full");
    fs::write(
        generated.join("metrics").join("metrics-summary.json"),
        r#"{"declared_tests":8946,"route_coverage":57,"workspace_packages":230}"#,
    )
    .expect("metrics");

    let summary = GeneratedQualitySummary::load(&generated).expect("summary");
    assert_eq!(summary.full_status.as_deref(), Some("passed"));
    assert_eq!(summary.declared_tests, 8946);
    assert_eq!(summary.route_coverage, 57);
    assert_eq!(summary.workspace_packages, 230);
}

#[test]
fn layout_state_load_round_trip_matches_saved_panels() {
    let temp = tempdir().expect("tempdir");
    let path = temp.path().join("layout.json");
    let layout = LayoutState::from_visible_panels(&[
        "viewport".to_string(),
        "inspector".to_string(),
        "terrain".to_string(),
    ]);

    layout
        .save(path.to_string_lossy().as_ref())
        .expect("save layout");
    let loaded = LayoutState::load(path.to_string_lossy().as_ref()).expect("load layout");

    assert_eq!(loaded.visible_panel_ids(), layout.visible_panel_ids());
}

#[test]
fn shell_runtime_stage_activation_sets_expected_panels() {
    let mut runtime = ShellRuntime::new();
    runtime.activate_stage(WorkspaceStage::Terrain);
    assert_eq!(
        runtime.get_open_panels(),
        vec![
            "viewport".to_string(),
            "terrain".to_string(),
            "inspector".to_string(),
        ]
    );

    runtime.activate_stage(WorkspaceStage::Capture);
    assert_eq!(
        runtime.get_open_panels(),
        vec![
            "viewport".to_string(),
            "diagnostics".to_string(),
            "outliner".to_string(),
        ]
    );
}

#[test]
fn shell_runtime_new_project_dialog_submits_project_create_command() {
    let mut runtime = ShellRuntime::new();
    runtime.show_project_dialog(ProjectDialogKind::NewProject);
    runtime.project_dialogs.new_project.project_name = "ProofLane".to_string();
    runtime.project_dialogs.new_project.project_path = "E:/tmp/prooflane".to_string();
    runtime.project_dialogs.new_project.world_name = "first_world".to_string();

    let command_id = runtime
        .submit_new_project_dialog()
        .expect("submit new project dialog");
    let pending = runtime.drain_pending_commands();

    assert_eq!(command_id, 1);
    assert_eq!(pending.len(), 1);
    assert!(!runtime.project_dialogs.new_project.is_open);
    assert!(matches!(
        pending[0].1,
        PromotedCommand::ProjectCreate {
            ref project_name,
            ref project_root,
            ref world_name
        } if project_name == "ProofLane"
            && project_root == "E:/tmp/prooflane"
            && world_name == "first_world"
    ));
}

#[test]
fn shell_runtime_open_world_dialog_requires_world_json() {
    let temp = tempdir().expect("tempdir");
    let world_dir = temp.path().join("broken_world");
    fs::create_dir_all(&world_dir).expect("world dir");

    let mut runtime = ShellRuntime::new();
    runtime.show_project_dialog(ProjectDialogKind::OpenWorld);
    runtime.project_dialogs.open_world.world_path = world_dir.to_string_lossy().to_string();

    let error = runtime
        .submit_open_world_dialog()
        .expect_err("missing world.json should fail");
    assert!(error.contains("world.json"));
}

#[test]
fn layout_round_trip_preserves_docking_position() {
    let temp = tempdir().expect("tempdir");
    let path = temp.path().join("layout.json");
    let mut layout =
        LayoutState::from_visible_panels(&["viewport".to_string(), "inspector".to_string()]);
    if let Some(panel) = layout
        .panels
        .iter_mut()
        .find(|panel| panel.id == "inspector")
    {
        panel.dock_position = DockPosition::Right;
    }

    layout
        .save(path.to_string_lossy().as_ref())
        .expect("save layout");
    let loaded = LayoutState::load(path.to_string_lossy().as_ref()).expect("load layout");
    let inspector = loaded
        .panels
        .iter()
        .find(|panel| panel.id == "inspector")
        .expect("inspector panel");

    assert_eq!(inspector.dock_position, DockPosition::Right);
}

#[test]
fn docking_manager_drag_panel_snaps_to_edge() {
    let mut docking = DockingManager::new();
    docking.drag_panel("terrain", (1595.0, 80.0), [1600.0, 900.0]);

    let terrain = docking.panel_geometry("terrain").expect("terrain geometry");
    assert_eq!(terrain.dock_position, DockPosition::Right);
}
