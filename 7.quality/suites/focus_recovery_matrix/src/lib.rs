#[cfg(test)]
mod tests {
    use stratumx_route_test_support::{execute_button, route};

    #[test]
    fn canonical_routes_emit_focus_retry_and_recovery_contracts() {
        for button_id in [
            "btn.terrain.sculpt_primary",
            "btn.material.preview_blast",
            "btn.view.inspector",
            "btn.audio.preview_obstruction_vs_occlusion",
        ] {
            let route = route(button_id);
            let (ctx, result) = execute_button(button_id, ());
            assert_eq!(
                result.focus_target.as_deref(),
                Some(route.focus_success.as_str())
            );
            assert_eq!(
                result.retry_target.as_deref(),
                Some(route.focus_retry.as_str())
            );
            assert_eq!(
                result.recovery_anchor.as_deref(),
                Some(route.recovery_anchor.as_str())
            );
            assert!(ctx
                .focus_history
                .iter()
                .any(|focus| focus == &route.focus_success));
        }
    }
}
