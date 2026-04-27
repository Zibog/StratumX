#[cfg(test)]
mod tests {
    use stratumx_tooling_l6_0_tool_session::{
        CanonicalCommandExecutor, EditorPublication, ToolSessionContext,
    };
    use stratumx_tooling_l6_1_command_envelopes::{
        CanonicalCommandEnvelope, CommandId, CommandPayload, RouteDomain, RouteMetadata,
        SourceSurface, TransactionMeta,
    };

    fn build_route() -> RouteMetadata {
        RouteMetadata {
            button_id: "btn.build.release_package".to_string(),
            action_id: "build.release_package.requested".to_string(),
            executor_module: "build::executor_build".to_string(),
            owner_service: "BuildReleaseService".to_string(),
            owner_state: "release_readiness".to_string(),
            publication_kinds: vec!["BuildChanged".to_string(), "DiagnosticsChanged".to_string()],
            persistence_kind: "artifact_output".to_string(),
            diagnostics_kind: "release_readiness".to_string(),
            focus_success: "build.release.dashboard".to_string(),
            focus_retry: "build.release.retry".to_string(),
            focus_failure: "build.release.blockers".to_string(),
            recovery_anchor: "release.freeze.checkpoint".to_string(),
            evidence_family: "release_artifacts".to_string(),
            status: "cataloged".to_string(),
            domain: RouteDomain::Build,
        }
    }

    #[test]
    fn build_result_contract_carries_release_artifacts_focus_and_recovery() {
        let route = build_route();
        let envelope = CanonicalCommandEnvelope {
            command_id: CommandId::new(route.action_id.clone()),
            payload: CommandPayload::Empty,
            transaction_meta: TransactionMeta::new(route.button_id.clone()),
            source_surface: SourceSurface::BuildPanel,
            route_metadata: route,
        };

        let mut ctx = ToolSessionContext::default();
        let result = CanonicalCommandExecutor::execute(envelope, &mut ctx);
        assert!(result
            .publications
            .iter()
            .any(|publication| matches!(publication, EditorPublication::BuildChanged(_))));
        assert!(result
            .publications
            .iter()
            .any(|publication| matches!(publication, EditorPublication::DiagnosticsChanged(_))));
        assert!(result.artifact_ref.is_some());
        assert_eq!(
            result.focus_target.as_deref(),
            Some("build.release.dashboard")
        );
        assert_eq!(
            result.recovery_anchor.as_deref(),
            Some("release.freeze.checkpoint")
        );
    }
}
