#[cfg(test)]
mod tests {
    use stratumx_route_test_support::{execute_button_in_context, ToolSessionContext};

    #[test]
    fn proof_contour_keeps_evidence_focus_and_history_through_multiple_routes() {
        let mut ctx = ToolSessionContext::default();

        for button_id in [
            "btn.project.new_project",
            "btn.world.open_world_package",
            "btn.material.preview_blast",
            "btn.material.capture_proof_artifacts",
            "btn.terrain.save_chunks",
            "btn.sky.set_weather_regime",
        ] {
            let result = execute_button_in_context(&mut ctx, button_id, ());
            assert!(result.focus_target.is_some());
            if button_id.starts_with("btn.material.preview_")
                || button_id == "btn.material.capture_proof_artifacts"
            {
                assert!(result.artifact_ref.is_some());
            }
        }

        assert_eq!(ctx.executed_buttons.len(), 6);
        assert!(ctx
            .executed_buttons
            .iter()
            .any(|button| button == "btn.material.capture_proof_artifacts"));
    }
}
