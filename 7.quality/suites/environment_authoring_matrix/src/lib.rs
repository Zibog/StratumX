#[cfg(test)]
mod tests {
    use stratumx_route_test_support::{execute_button, route};

    #[test]
    fn sky_routes_are_environment_owned() {
        for button_id in [
            "btn.sky.bind_sky_profile",
            "btn.sky.set_time_of_day",
            "btn.sky.set_weather_regime",
            "btn.sky.bind_cloud_profile",
        ] {
            let route = route(button_id);
            assert_eq!(route.executor_module, "environment::executor_environment");
            assert_eq!(route.owner_service, "EnvironmentAuthoringService");
            assert!(route
                .publication_kinds
                .iter()
                .any(|kind| kind == "EnvironmentChanged"));
        }
    }

    #[test]
    fn environment_routes_emit_focus_and_diagnostics() {
        for button_id in ["btn.sky.bind_sky_profile", "btn.sky.set_weather_regime"] {
            let (_, result) = execute_button(button_id, ());
            assert!(result.focus_target.is_some());
            assert!(!result.diagnostics.is_empty());
        }
    }
}
