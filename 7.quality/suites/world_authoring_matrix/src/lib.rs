#[cfg(test)]
mod tests {
    use stratumx_route_test_support::{execute_button, route};

    #[test]
    fn project_world_and_import_routes_are_world_owned() {
        let expectations = [
            (
                "btn.project.new_project",
                "project::executor_project",
                "ProjectBootstrapService",
            ),
            (
                "btn.project.open_project",
                "project::executor_project",
                "ProjectBootstrapService",
            ),
            (
                "btn.project.save_project",
                "project::executor_project",
                "ProjectBootstrapService",
            ),
            (
                "btn.project.save_project_as",
                "project::executor_project",
                "ProjectBootstrapService",
            ),
            (
                "btn.world.open_world_package",
                "world::executor_world",
                "WorldLifecycleService",
            ),
            (
                "btn.world.save_world_package",
                "world::executor_world",
                "WorldLifecycleService",
            ),
            (
                "btn.world.validate_world",
                "diagnostics::executor_diagnostics",
                "DiagnosticsService",
            ),
            (
                "btn.import.heightmap_source",
                "terrain::executor_terrain",
                "ImportExportPipelineService",
            ),
        ];

        for (button_id, executor_module, owner_service) in expectations {
            let route = route(button_id);
            assert_eq!(route.executor_module, executor_module);
            assert_eq!(route.owner_service, owner_service);
            assert!(route.publication_kinds.iter().any(|kind| {
                kind == "ProjectChanged"
                    || kind == "WorldChanged"
                    || kind == "DiagnosticsChanged"
                    || kind == "ShellChanged"
            }));
        }
    }

    #[test]
    fn representative_world_routes_execute_through_the_facade() {
        for button_id in [
            "btn.project.new_project",
            "btn.world.open_world_package",
            "btn.world.validate_world",
        ] {
            let (ctx, result) = execute_button(button_id, ());
            assert!(ctx
                .executed_buttons
                .iter()
                .any(|button| button == button_id));
            assert!(result.focus_target.is_some());
            assert!(result.recovery_anchor.is_some());
            assert!(!result.publications.is_empty());
        }
    }
}
