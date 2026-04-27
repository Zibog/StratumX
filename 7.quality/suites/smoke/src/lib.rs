#[cfg(test)]
mod tests {
    use stratumx_route_test_support::execute_button;
    use stratumx_shell_test_support::execute_shell_button;

    #[test]
    fn minimal_editor_route_flow_executes_without_catastrophe() {
        for button_id in [
            "btn.project.new_project",
            "btn.world.open_world_package",
            "btn.terrain.sculpt_primary",
            "btn.material.new_profile",
            "btn.audio.preview_audibility_free_camera",
        ] {
            let (_, result) = execute_button(button_id, ());
            assert!(result.focus_target.is_some());
            assert!(!result.publications.is_empty());
        }
    }

    #[test]
    fn shell_surfaces_initialize_via_canonical_routes() {
        for button_id in [
            "btn.view.viewport",
            "btn.view.inspector",
            "btn.view.material_lab",
        ] {
            let (_, result) = execute_shell_button(button_id);
            assert!(result.focus_target.is_some());
            assert_eq!(result.denial, None);
        }
    }
}
