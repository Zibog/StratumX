#[cfg(test)]
mod tests {
    use stratumx_shell_test_support::{execute_shell_button, shell_routes};

    #[test]
    fn view_buttons_are_shell_owned() {
        for route in shell_routes() {
            assert_eq!(route.executor_module, "common::executor_shell");
            assert_eq!(route.owner_service, "WorkspaceLayoutManager");
            assert_eq!(route.publication_kinds, vec!["ShellChanged".to_string()]);
            assert_eq!(route.persistence_kind, "none");
        }
    }

    #[test]
    fn shell_routes_publish_focus_changes() {
        for button_id in [
            "btn.view.viewport",
            "btn.view.inspector",
            "btn.view.material_lab",
        ] {
            let (ctx, result) = execute_shell_button(button_id);
            assert!(ctx
                .executed_buttons
                .iter()
                .any(|button| button == button_id));
            assert!(result.focus_target.is_some());
            assert_eq!(result.denial, None);
        }
    }
}
