// PHASE 6: CERTIFICATION, FREEZE, AND FIRST-RESULT VERIFICATION
// Integration test demonstrating complete certification chain

use stratumx_tooling_l6_14_release_runtime::first_result_verification::{
    BuildArtifact, ExportArtifact, FirstResultVerifier, LaunchTrace, ReleaseChain,
    FIRST_RESULT_SIGNATURE,
};
use tool_evidence_capture::{
    ArtifactRef, ArtifactType, Baseline, BaselineRegistry, CertificationEngine,
    CertificationVerdict, CompareEngine, CompareMode, CompareVerdict, EvidenceBundle,
    EvidenceVerdict, FreezeEngine,
};

#[test]
fn test_complete_certification_chain() {
    // Step 1: Create baseline
    let mut baseline_registry = BaselineRegistry::new();
    let baseline = Baseline {
        baseline_id: "baseline_brutal_proof_region".to_string(),
        pack_id: "pack_brutal_proof_region".to_string(),
        scenario_id: "scenario_brutal_proof_region".to_string(),
        timestamp: 1000,
        artifacts: vec![ArtifactRef {
            artifact_id: "artifact_baseline_1".to_string(),
            artifact_type: ArtifactType::StateSnapshot,
            path: "/baselines/brutal_proof_region.snapshot".to_string(),
            checksum: "abc123".to_string(),
        }],
        protected: true,
    };
    baseline_registry.register_baseline(baseline);

    // Step 2: Create compare triplet
    let mut compare_engine = CompareEngine::new();
    let pack_id = compare_engine.create_triplet(
        "pack_brutal_proof_region".to_string(),
        "baseline_brutal_proof_region".to_string(),
        CompareMode::Exact,
    );

    // Step 3: Execute compare (no failed run, should pass)
    let compare_result = compare_engine.execute_compare(&pack_id).unwrap();
    assert_eq!(compare_result.verdict, CompareVerdict::Pass);

    // Step 4: Create evidence bundle
    let evidence = EvidenceBundle {
        bundle_id: "evidence_brutal_proof_region".to_string(),
        pack_id: "pack_brutal_proof_region".to_string(),
        scenario_id: "scenario_brutal_proof_region".to_string(),
        build_profile: "desktop".to_string(),
        schema_revision: "1.0".to_string(),
        timestamp: 2000,
        artifacts: vec![
            ArtifactRef {
                artifact_id: "artifact_run_1".to_string(),
                artifact_type: ArtifactType::StateSnapshot,
                path: "/runs/brutal_proof_region_run1.snapshot".to_string(),
                checksum: "def456".to_string(),
            },
            ArtifactRef {
                artifact_id: "artifact_diagnostics_1".to_string(),
                artifact_type: ArtifactType::DiagnosticsLog,
                path: "/runs/brutal_proof_region_run1.log".to_string(),
                checksum: "ghi789".to_string(),
            },
        ],
        verdict: EvidenceVerdict::Green,
        operator_signoff: Some("operator_1".to_string()),
    };

    // Step 5: Certify pack
    let mut cert_engine = CertificationEngine::new();
    let cert_result = cert_engine.certify_pack(
        "pack_brutal_proof_region".to_string(),
        "scenario_brutal_proof_region".to_string(),
        &evidence,
        &compare_result,
    );

    assert_eq!(cert_result.verdict, CertificationVerdict::Certified);
    assert!(cert_result.blockers.is_empty());
    assert!(cert_engine.is_freeze_ready("pack_brutal_proof_region"));

    // Step 6: Create freeze
    let mut freeze_engine = FreezeEngine::new();
    let freeze = freeze_engine
        .create_freeze(
            "freeze_brutal_proof_region".to_string(),
            vec!["pack_brutal_proof_region".to_string()],
            vec!["cert_brutal_proof_region".to_string()],
            "desktop".to_string(),
            "operator_1".to_string(),
        )
        .unwrap();

    assert_eq!(freeze.freeze_id, "freeze_brutal_proof_region");
    assert_eq!(freeze.pack_ids.len(), 1);

    // Step 7: Build/Export/Launch chain
    let mut release_chain = ReleaseChain::new();

    let build = BuildArtifact {
        build_id: "build_brutal_proof_region".to_string(),
        project_id: "project_brutal_proof_region".to_string(),
        target_platform: "desktop".to_string(),
        executable_path: "/builds/brutal_proof_region.exe".to_string(),
        build_timestamp: 3000,
        build_profile: "release".to_string(),
    };
    release_chain.register_build(build);

    let export = ExportArtifact {
        export_id: "export_brutal_proof_region".to_string(),
        build_id: "build_brutal_proof_region".to_string(),
        export_path: "/exports/brutal_proof_region".to_string(),
        export_timestamp: 4000,
        launchable: true,
    };
    release_chain.register_export(export);

    let launch = LaunchTrace {
        launch_id: "launch_brutal_proof_region".to_string(),
        export_id: "export_brutal_proof_region".to_string(),
        launch_timestamp: 5000,
        process_id: Some(1234),
        exit_code: Some(0),
        runtime_diagnostics: vec![
            "Engine started".to_string(),
            "World loaded".to_string(),
            "Proof region bootstrapped".to_string(),
        ],
        runtime_signature: Some(FIRST_RESULT_SIGNATURE.to_string()),
    };
    release_chain.register_launch(launch);

    // Step 8: Verify chain
    let chain_verification = release_chain
        .verify_chain("launch_brutal_proof_region")
        .unwrap();
    assert!(chain_verification.chain_valid);
    assert_eq!(chain_verification.project_id, "project_brutal_proof_region");

    // Step 9: Verify first result signature
    let mut verifier = FirstResultVerifier::new();
    let first_result = verifier.verify_signature(
        "verify_brutal_proof_region".to_string(),
        "project_brutal_proof_region".to_string(),
        "build_brutal_proof_region".to_string(),
        FIRST_RESULT_SIGNATURE.to_string(),
        vec![
            "Engine started".to_string(),
            "World loaded".to_string(),
            "Proof region bootstrapped".to_string(),
        ],
    );

    assert!(first_result.verified);
    assert_eq!(first_result.runtime_signature, FIRST_RESULT_SIGNATURE);
}

#[test]
fn test_certification_blocked_on_failed_compare() {
    let mut compare_engine = CompareEngine::new();
    let pack_id = compare_engine.create_triplet(
        "pack_test".to_string(),
        "baseline_test".to_string(),
        CompareMode::Exact,
    );

    // Add failed run
    compare_engine.add_failed_run(&pack_id, "failed_run_1".to_string());

    let compare_result = compare_engine.execute_compare(&pack_id).unwrap();
    assert_eq!(compare_result.verdict, CompareVerdict::Failed);

    // Try to certify with failed compare
    let evidence = EvidenceBundle {
        bundle_id: "evidence_test".to_string(),
        pack_id: "pack_test".to_string(),
        scenario_id: "scenario_test".to_string(),
        build_profile: "desktop".to_string(),
        schema_revision: "1.0".to_string(),
        timestamp: 1000,
        artifacts: vec![],
        verdict: EvidenceVerdict::Green,
        operator_signoff: None,
    };

    let mut cert_engine = CertificationEngine::new();
    let cert_result = cert_engine.certify_pack(
        "pack_test".to_string(),
        "scenario_test".to_string(),
        &evidence,
        &compare_result,
    );

    assert_eq!(cert_result.verdict, CertificationVerdict::Blocked);
    assert!(!cert_result.blockers.is_empty());
    assert!(!cert_engine.is_freeze_ready("pack_test"));
}

#[test]
fn test_baseline_protection_from_red_run() {
    let mut baseline_registry = BaselineRegistry::new();

    let baseline = Baseline {
        baseline_id: "baseline_1".to_string(),
        pack_id: "pack_1".to_string(),
        scenario_id: "scenario_1".to_string(),
        timestamp: 1000,
        artifacts: vec![],
        protected: true,
    };

    baseline_registry.register_baseline(baseline);

    // Red verdict cannot replace baseline
    assert!(!baseline_registry.can_replace_baseline("pack_1", &EvidenceVerdict::Red));

    // Orange verdict cannot replace baseline (without waiver)
    assert!(!baseline_registry.can_replace_baseline("pack_1", &EvidenceVerdict::Orange));

    // Green verdict can replace baseline
    assert!(baseline_registry.can_replace_baseline("pack_1", &EvidenceVerdict::Green));
}

#[test]
fn test_recovery_run_clears_blocker() {
    let mut compare_engine = CompareEngine::new();
    let pack_id = compare_engine.create_triplet(
        "pack_recovery".to_string(),
        "baseline_recovery".to_string(),
        CompareMode::Exact,
    );

    // Add failed run
    compare_engine.add_failed_run(&pack_id, "failed_run_1".to_string());
    let result = compare_engine.execute_compare(&pack_id).unwrap();
    assert_eq!(result.verdict, CompareVerdict::Failed);

    // Add recovery run
    compare_engine.add_recovery_run(&pack_id, "recovery_run_1".to_string());
    let result = compare_engine.execute_compare(&pack_id).unwrap();
    assert_eq!(result.verdict, CompareVerdict::Recovered);

    // Now certification should pass
    let evidence = EvidenceBundle {
        bundle_id: "evidence_recovery".to_string(),
        pack_id: "pack_recovery".to_string(),
        scenario_id: "scenario_recovery".to_string(),
        build_profile: "desktop".to_string(),
        schema_revision: "1.0".to_string(),
        timestamp: 1000,
        artifacts: vec![],
        verdict: EvidenceVerdict::Green,
        operator_signoff: Some("operator_1".to_string()),
    };

    let mut cert_engine = CertificationEngine::new();
    let cert_result = cert_engine.certify_pack(
        "pack_recovery".to_string(),
        "scenario_recovery".to_string(),
        &evidence,
        &result,
    );

    assert_eq!(cert_result.verdict, CertificationVerdict::Certified);
    assert!(cert_result.blockers.is_empty());
}

#[test]
fn test_first_result_verification_failure() {
    let mut verifier = FirstResultVerifier::new();

    let verification = verifier.verify_signature(
        "verify_fail".to_string(),
        "project_fail".to_string(),
        "build_fail".to_string(),
        "wrong signature".to_string(),
        vec!["Engine started".to_string()],
    );

    assert!(!verification.verified);
    assert_ne!(verification.runtime_signature, FIRST_RESULT_SIGNATURE);
}
