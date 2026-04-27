#[cfg(test)]
fn material_route(
    button_id: &str,
) -> &'static stratumx_tooling_l6_1_command_envelopes::CanonicalButtonRoute {
    use stratumx_tooling_l6_1_command_envelopes::canonical_button_routes;

    canonical_button_routes()
        .iter()
        .find(|route| route.button_id == button_id)
        .expect("material route must exist")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn material_first_routes_are_cataloged() {
        let buttons = [
            "btn.material.new_profile",
            "btn.material.duplicate_profile",
            "btn.material.assign_archetype",
            "btn.material.bind_surface_family",
            "btn.material.bind_response_profile",
            "btn.material.bind_texture_stack",
            "btn.material.bind_microdetail_profile",
            "btn.material.bind_weather_modulation",
            "btn.material.bind_visual_response_family",
            "btn.material.bind_acoustic_profile",
            "btn.material.bind_light_response",
            "btn.material.inspect_branch_coverage",
            "btn.material.set_cheap_runtime_rung",
            "btn.material.preview_bullet_hit",
            "btn.material.preview_blast",
            "btn.material.preview_wetness",
            "btn.material.preview_burn",
            "btn.material.capture_proof_artifacts",
            "btn.material.review_freeze_blockers",
        ];

        for button_id in buttons {
            let route = material_route(button_id);
            assert_eq!(route.owner_service, "MaterialAuthoringService");
            assert_eq!(route.executor_module, "material::executor_material");
            assert!(route
                .publication_kinds
                .iter()
                .any(|kind| kind == "MaterialChanged"));
            assert!(route
                .publication_kinds
                .iter()
                .any(|kind| kind == "DiagnosticsChanged"));
        }
    }

    #[test]
    fn proof_routes_emit_evidence_publications() {
        for button_id in [
            "btn.material.preview_bullet_hit",
            "btn.material.preview_blast",
            "btn.material.preview_wetness",
            "btn.material.preview_burn",
            "btn.material.capture_proof_artifacts",
            "btn.material.review_freeze_blockers",
        ] {
            let route = material_route(button_id);
            assert!(route
                .publication_kinds
                .iter()
                .any(|kind| kind == "EvidenceChanged"));
            assert_ne!(route.evidence_family, "none");
        }
    }
}
