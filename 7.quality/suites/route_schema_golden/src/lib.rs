#[cfg(test)]
mod tests {
    use stratumx_route_test_support::dispatch_button;
    use stratumx_tooling_l6_1_command_envelopes::canonical_route_manifest;

    #[test]
    fn manifest_json_shape_is_stable() {
        let manifest = canonical_route_manifest();
        let json = serde_json::to_value(manifest).expect("manifest json");
        assert_eq!(json["manifest_version"], "phase4.v1");
        assert_eq!(json["route_count"], 85);
        assert!(json["routes"].as_array().is_some());
        let first = &json["routes"][0];
        for key in [
            "button_id",
            "action_id",
            "command_id",
            "executor_module",
            "owner_service",
            "owner_state",
            "publication_kinds",
            "focus_success",
            "focus_retry",
            "focus_failure",
            "recovery_anchor",
            "evidence_family",
            "status",
        ] {
            assert!(first.get(key).is_some(), "missing manifest key {key}");
        }
    }

    #[test]
    fn envelope_json_shape_is_stable() {
        let envelope = dispatch_button("btn.material.capture_proof_artifacts", ())
            .expect("cataloged envelope");
        let json = serde_json::to_value(&envelope).expect("envelope json");
        assert!(json.get("command_id").is_some());
        assert!(json.get("payload").is_some());
        assert!(json.get("transaction_meta").is_some());
        assert!(json.get("source_surface").is_some());
        assert!(json.get("route_metadata").is_some());
    }
}
