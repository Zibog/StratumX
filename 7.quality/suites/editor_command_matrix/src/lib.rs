#[cfg(test)]
fn collect_button_ids(markdown: &str) -> Vec<String> {
    let mut ids = Vec::new();
    for chunk in markdown.split('`') {
        if chunk.starts_with("btn.")
            && chunk
                .chars()
                .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '.' || ch == '_')
        {
            ids.push(chunk.trim().to_string());
        }
    }
    ids.sort();
    ids.dedup();
    ids
}

#[cfg(test)]
mod tests {
    use super::*;
    use stratumx_tooling_l6_1_command_envelopes::{
        canonical_button_routes, canonical_route_manifest,
    };

    #[test]
    fn manifest_has_full_phase4_superset() {
        let manifest = canonical_route_manifest();
        assert_eq!(manifest.route_count, 85);
        assert_eq!(manifest.routes.len(), 85);
        assert_eq!(canonical_button_routes().len(), 85);
    }

    #[test]
    fn every_route_has_canonical_closure_fields() {
        for route in canonical_button_routes() {
            assert!(route.button_id.starts_with("btn."));
            assert!(!route.action_id.is_empty());
            assert!(!route.command_id.is_empty());
            assert!(!route.executor_module.is_empty());
            assert!(!route.owner_service.is_empty());
            assert!(!route.owner_state.is_empty());
            assert!(!route.publication_kinds.is_empty());
            assert!(!route.focus_success.is_empty());
            assert!(!route.focus_retry.is_empty());
            assert!(!route.focus_failure.is_empty());
            assert!(!route.recovery_anchor.is_empty());
            assert!(!route.status.is_empty());
        }
    }

    #[test]
    fn editor_docs_do_not_introduce_ids_outside_manifest() {
        let docs = [
            include_str!("../../../../1.docs/canonical/editor/110_EXACT_BUTTON_TO_ROUTE_MANIFEST_CANON.md"),
            include_str!("../../../../1.docs/canonical/editor/111_EDITOR_MENU_TREE_AND_BASE_COMMAND_GROUPS_CANON.md"),
            include_str!("../../../../1.docs/canonical/editor/112_MATERIAL_FIRST_EDITOR_AND_WORLD_OPERATOR_SEQUENCE_CANON.md"),
            include_str!("../../../../1.docs/canonical/editor/113_MATERIAL_CENTRIC_OPERATOR_SURFACE_CANON.md"),
        ];
        let manifest_ids: std::collections::BTreeSet<_> = canonical_button_routes()
            .iter()
            .map(|route| route.button_id.as_str())
            .collect();

        for doc in docs {
            for button_id in collect_button_ids(doc) {
                assert!(
                    manifest_ids.contains(button_id.as_str()),
                    "doc button id missing from manifest: {button_id}"
                );
            }
        }
    }

    #[test]
    fn manifest_contains_material_proof_rows() {
        for button_id in [
            "btn.material.capture_proof_artifacts",
            "btn.material.review_freeze_blockers",
        ] {
            let route = canonical_button_routes()
                .iter()
                .find(|route| route.button_id == button_id)
                .expect("material proof route must exist");
            assert!(route
                .publication_kinds
                .iter()
                .any(|kind| kind == "EvidenceChanged"));
        }
    }
}
