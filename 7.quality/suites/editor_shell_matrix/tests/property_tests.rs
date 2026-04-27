use proptest::prelude::*;
use stratumx_editor_l8_0_editor_shell::{filter_command_palette, LayoutState, ShellRuntime};

fn panel_id_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("viewport".to_string()),
        Just("outliner".to_string()),
        Just("inspector".to_string()),
        Just("diagnostics".to_string()),
        Just("terrain".to_string()),
        Just("environment".to_string()),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_workspace_layout_round_trip(panels in prop::collection::vec(panel_id_strategy(), 0..6)) {
        let layout = LayoutState::from_visible_panels(&panels);
        let mut runtime = ShellRuntime::new();
        runtime.restore_workspace_layout(layout.clone()).expect("restore layout");

        let restored_panels = runtime.get_open_panels();
        prop_assert_eq!(restored_panels, layout.visible_panel_ids());
    }

    #[test]
    fn prop_command_palette_filter_is_stable(query in "[a-z]{0,8}") {
        let first = filter_command_palette(&query)
            .into_iter()
            .map(|entry| entry.id)
            .collect::<Vec<_>>();
        let second = filter_command_palette(&query)
            .into_iter()
            .map(|entry| entry.id)
            .collect::<Vec<_>>();

        prop_assert_eq!(first, second);
    }

    #[test]
    fn prop_layout_round_trip_preserves_panel_geometry(panels in prop::collection::vec(panel_id_strategy(), 0..6)) {
        let layout = LayoutState::from_visible_panels(&panels);
        let mut runtime = ShellRuntime::new();
        runtime.restore_workspace_layout(layout.clone()).expect("restore layout");

        let temp = tempfile::tempdir().expect("tempdir");
        let path = temp.path().join("layout.json");
        let saved = runtime.save_workspace_layout(&path).expect("save layout");

        prop_assert_eq!(saved.visible_panel_ids(), layout.visible_panel_ids());
        prop_assert_eq!(saved.panel_geometries(), layout.panel_geometries());
    }
}
