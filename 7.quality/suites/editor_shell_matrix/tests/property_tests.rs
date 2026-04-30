use stratumx_editor_l8_0_editor_shell::{filter_command_palette, LayoutState, ShellRuntime};
use stratumx_test_support::create_temp_dir;

fn sample_panel_sets() -> Vec<Vec<String>> {
    vec![
        vec![],
        vec!["viewport".to_string()],
        vec!["viewport".to_string(), "inspector".to_string()],
        vec![
            "viewport".to_string(),
            "outliner".to_string(),
            "terrain".to_string(),
        ],
        vec![
            "viewport".to_string(),
            "diagnostics".to_string(),
            "environment".to_string(),
            "inspector".to_string(),
        ],
    ]
}

#[test]
fn workspace_layout_round_trip_for_sample_panel_sets() {
    for panels in sample_panel_sets() {
        let layout = LayoutState::from_visible_panels(&panels);
        let mut runtime = ShellRuntime::new();
        runtime
            .restore_workspace_layout(layout.clone())
            .expect("restore layout");

        let restored_panels = runtime.get_open_panels();
        assert_eq!(restored_panels, layout.visible_panel_ids());
    }
}

#[test]
fn command_palette_filter_is_stable_for_sample_queries() {
    for query in ["", "a", "save", "diag", "terrain", "view"] {
        let first = filter_command_palette(query)
            .into_iter()
            .map(|entry| entry.id)
            .collect::<Vec<_>>();
        let second = filter_command_palette(query)
            .into_iter()
            .map(|entry| entry.id)
            .collect::<Vec<_>>();

        assert_eq!(first, second);
    }
}

#[test]
fn layout_round_trip_preserves_panel_geometry_for_sample_panel_sets() {
    for panels in sample_panel_sets() {
        let layout = LayoutState::from_visible_panels(&panels);
        let mut runtime = ShellRuntime::new();
        runtime
            .restore_workspace_layout(layout.clone())
            .expect("restore layout");

        let temp = create_temp_dir();
        let path = temp.path().join("layout.json");
        let saved = runtime.save_workspace_layout(&path).expect("save layout");

        assert_eq!(saved.visible_panel_ids(), layout.visible_panel_ids());
        assert_eq!(saved.panel_geometries(), layout.panel_geometries());
    }
}
