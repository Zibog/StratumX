#[cfg(test)]
mod tests {
    use stratumx_route_test_support::{execute_button, route};

    #[test]
    fn audio_routes_are_first_class_and_owner_backed() {
        for button_id in [
            "btn.audio.assign_emitter_class_world_source",
            "btn.audio.bind_zone_profile_world_surface",
            "btn.audio.bind_priority_ducking_policy",
            "btn.audio.preview_audibility_free_camera",
            "btn.audio.preview_obstruction_vs_occlusion",
            "btn.audio.preview_indoor_outdoor_transition",
            "btn.audio.preview_voice_subtitle_legality",
            "btn.audio.inspect_listener_profile",
            "btn.audio.inspect_bus_ducking",
        ] {
            let route = route(button_id);
            assert_eq!(route.owner_service, "AudioAuthoringService");
            assert!(route.executor_module.contains("audio"));
            assert!(route
                .publication_kinds
                .iter()
                .any(|kind| kind == "AudioChanged" || kind == "DiagnosticsChanged"));
        }
    }

    #[test]
    fn audio_preview_and_inspect_routes_execute_with_focus() {
        for button_id in [
            "btn.audio.preview_audibility_free_camera",
            "btn.audio.inspect_listener_profile",
        ] {
            let (_, result) = execute_button(button_id, ());
            assert!(result.focus_target.is_some());
            assert!(result.retry_target.is_some());
        }
    }
}
