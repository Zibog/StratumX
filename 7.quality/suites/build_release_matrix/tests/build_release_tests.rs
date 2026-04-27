// ============================================================================
// Build -> Release -> Evidence -> Launch Pipeline Integration Tests
// Covers: build artifact creation -> release package -> evidence capture -> launch verification
// ============================================================================

use stratumx_tooling_l6_14_release_runtime::first_result_verification::{
    BuildArtifact, ExportArtifact, FIRST_RESULT_SIGNATURE, FirstResultVerifier, LaunchTrace,
    ReleaseChain,
};
use tool_evidence_capture::{
    ArtifactRef, ArtifactType, CertificationEngine, CertificationVerdict, CompareEngine,
    CompareMode, CompareVerdict, EvidenceBundle, EvidenceVerdict, FreezeEngine, FreezePosture,
};

// ============================================================================
// Build Artifact Creation
// ============================================================================

#[test]
fn build_artifact_windows_release() {
    let artifact = BuildArtifact {
        build_id: "build_win_rel_001".into(),
        project_id: "test_project".into(),
        target_platform: "Windows".into(),
        executable_path: "C:/stratumx/builds/game.exe".into(),
        build_timestamp: 1000,
        build_profile: "Release".into(),
    };

    assert_eq!(artifact.build_id, "build_win_rel_001");
    assert_eq!(artifact.target_platform, "Windows");
    assert_eq!(artifact.build_profile, "Release");
}

#[test]
fn build_artifact_windows_debug() {
    let artifact = BuildArtifact {
        build_id: "build_win_dbg_001".into(),
        project_id: "test_project".into(),
        target_platform: "Windows".into(),
        executable_path: "C:/stratumx/builds/game_d.exe".into(),
        build_timestamp: 1000,
        build_profile: "Debug".into(),
    };

    assert_eq!(artifact.build_profile, "Debug");
}

#[test]
fn build_artifact_linux_release() {
    let artifact = BuildArtifact {
        build_id: "build_linux_001".into(),
        project_id: "test_project".into(),
        target_platform: "Linux".into(),
        executable_path: "/home/stratumx/builds/game".into(),
        build_timestamp: 2000,
        build_profile: "Shipping".into(),
    };

    assert_eq!(artifact.target_platform, "Linux");
    assert_eq!(artifact.build_profile, "Shipping");
}

#[test]
fn build_artifact_serialization_roundtrip() {
    let artifact = BuildArtifact {
        build_id: "build_rt".into(),
        project_id: "proj_rt".into(),
        target_platform: "MacOS".into(),
        executable_path: "/Applications/game.app".into(),
        build_timestamp: 3000,
        build_profile: "Release".into(),
    };

    let json = serde_json::to_string(&artifact).unwrap();
    let restored: BuildArtifact = serde_json::from_str(&json).unwrap();
    assert_eq!(artifact, restored);
}

// ============================================================================
// Release Package (Export Artifact)
// ============================================================================

#[test]
fn release_package_creation() {
    let export = ExportArtifact {
        export_id: "export_001".into(),
        build_id: "build_win_rel_001".into(),
        export_path: "C:/stratumx/exports/game_v1.zip".into(),
        export_timestamp: 5000,
        launchable: true,
    };

    assert!(export.launchable);
    assert_eq!(export.build_id, "build_win_rel_001");
}

#[test]
fn release_package_non_launchable() {
    let export = ExportArtifact {
        export_id: "export_dev".into(),
        build_id: "build_win_dbg_001".into(),
        export_path: "C:/stratumx/exports/dev.zip".into(),
        export_timestamp: 5000,
        launchable: false,
    };

    assert!(!export.launchable);
}

#[test]
fn release_package_serialization_roundtrip() {
    let export = ExportArtifact {
        export_id: "exp_rt".into(),
        build_id: "build_rt".into(),
        export_path: "/exports/rt.zip".into(),
        export_timestamp: 6000,
        launchable: true,
    };

    let json = serde_json::to_string(&export).unwrap();
    let restored: ExportArtifact = serde_json::from_str(&json).unwrap();
    assert_eq!(export, restored);
}

// ============================================================================
// Evidence Capture
// ============================================================================

#[test]
fn evidence_capture_build_artifacts() {
    let bundle = EvidenceBundle {
        bundle_id: "build_evidence_001".into(),
        pack_id: "build_pack".into(),
        scenario_id: "build_verification".into(),
        build_profile: "Release".into(),
        schema_revision: "v1".into(),
        timestamp: 7000,
        artifacts: vec![
            ArtifactRef {
                artifact_id: "build_log".into(),
                artifact_type: ArtifactType::DiagnosticsLog,
                path: "/evidence/build.log".into(),
                checksum: "sha256:buildlog".into(),
            },
            ArtifactRef {
                artifact_id: "build_snapshot".into(),
                artifact_type: ArtifactType::StateSnapshot,
                path: "/evidence/build_state.bin".into(),
                checksum: "sha256:buildstate".into(),
            },
        ],
        verdict: EvidenceVerdict::Green,
        operator_signoff: Some("build_manager".into()),
    };

    assert_eq!(bundle.artifacts.len(), 2);
    assert_eq!(bundle.verdict, EvidenceVerdict::Green);
}

#[test]
fn evidence_capture_release_artifacts() {
    let bundle = EvidenceBundle {
        bundle_id: "release_evidence_001".into(),
        pack_id: "release_pack".into(),
        scenario_id: "release_verification".into(),
        build_profile: "Shipping".into(),
        schema_revision: "v1".into(),
        timestamp: 8000,
        artifacts: vec![ArtifactRef {
            artifact_id: "release_digest".into(),
            artifact_type: ArtifactType::CompareDigest,
            path: "/evidence/release_digest.json".into(),
            checksum: "sha256:releasedigest".into(),
        }],
        verdict: EvidenceVerdict::Green,
        operator_signoff: Some("release_manager".into()),
    };

    assert_eq!(bundle.verdict, EvidenceVerdict::Green);
    assert_eq!(bundle.artifacts[0].artifact_type, ArtifactType::CompareDigest);
}

#[test]
fn evidence_capture_red_verdict() {
    let bundle = EvidenceBundle {
        bundle_id: "failed_evidence".into(),
        pack_id: "failed_pack".into(),
        scenario_id: "failed_test".into(),
        build_profile: "Debug".into(),
        schema_revision: "v1".into(),
        timestamp: 9000,
        artifacts: vec![ArtifactRef {
            artifact_id: "failure_log".into(),
            artifact_type: ArtifactType::DiagnosticsLog,
            path: "/evidence/failure.log".into(),
            checksum: "sha256:failure".into(),
        }],
        verdict: EvidenceVerdict::Red,
        operator_signoff: None,
    };

    assert_eq!(bundle.verdict, EvidenceVerdict::Red);
}

#[test]
fn evidence_bundle_serialization_roundtrip() {
    let bundle = EvidenceBundle {
        bundle_id: "bundle_rt".into(),
        pack_id: "pack_rt".into(),
        scenario_id: "scenario_rt".into(),
        build_profile: "Release".into(),
        schema_revision: "v2".into(),
        timestamp: 10000,
        artifacts: vec![ArtifactRef {
            artifact_id: "art_rt".into(),
            artifact_type: ArtifactType::FrameCapture,
            path: "/evidence/frame.png".into(),
            checksum: "sha256:frame".into(),
        }],
        verdict: EvidenceVerdict::Green,
        operator_signoff: Some("ops".into()),
    };

    let json = serde_json::to_string(&bundle).unwrap();
    let restored: EvidenceBundle = serde_json::from_str(&json).unwrap();
    assert_eq!(bundle, restored);
}

// ============================================================================
// Launch Verification
// ============================================================================

#[test]
fn launch_trace_successful() {
    let trace = LaunchTrace {
        launch_id: "launch_001".into(),
        export_id: "export_001".into(),
        launch_timestamp: 11000,
        process_id: Some(12345),
        exit_code: Some(0),
        runtime_diagnostics: vec!["init_ok".into(), "render_ok".into(), "audio_ok".into()],
        runtime_signature: Some(FIRST_RESULT_SIGNATURE.to_string()),
    };

    assert_eq!(trace.exit_code, Some(0));
    assert!(trace.runtime_signature.is_some());
    assert_eq!(
        trace.runtime_signature.as_deref(),
        Some(FIRST_RESULT_SIGNATURE)
    );
}

#[test]
fn launch_trace_failed_exit_code() {
    let trace = LaunchTrace {
        launch_id: "launch_failed".into(),
        export_id: "export_001".into(),
        launch_timestamp: 11000,
        process_id: Some(12345),
        exit_code: Some(-1),
        runtime_diagnostics: vec!["init_failed".into()],
        runtime_signature: Some("error".into()),
    };

    assert_eq!(trace.exit_code, Some(-1));
}

#[test]
fn launch_trace_no_process() {
    let trace = LaunchTrace {
        launch_id: "launch_no_process".into(),
        export_id: "export_001".into(),
        launch_timestamp: 11000,
        process_id: None,
        exit_code: None,
        runtime_diagnostics: vec![],
        runtime_signature: None,
    };

    assert!(trace.process_id.is_none());
    assert!(trace.exit_code.is_none());
}

#[test]
fn launch_trace_serialization_roundtrip() {
    let trace = LaunchTrace {
        launch_id: "launch_rt".into(),
        export_id: "export_rt".into(),
        launch_timestamp: 12000,
        process_id: Some(9999),
        exit_code: Some(0),
        runtime_diagnostics: vec!["diag1".into(), "diag2".into()],
        runtime_signature: Some(FIRST_RESULT_SIGNATURE.to_string()),
    };

    let json = serde_json::to_string(&trace).unwrap();
    let restored: LaunchTrace = serde_json::from_str(&json).unwrap();
    assert_eq!(trace, restored);
}

// ============================================================================
// Full Pipeline: Build -> Release -> Evidence -> Launch
// ============================================================================

#[test]
fn full_pipeline_build_release_evidence_launch() {
    // Step 1: Build artifact creation
    let mut chain = ReleaseChain::new();
    let build = BuildArtifact {
        build_id: "pipeline_build_001".into(),
        project_id: "pipeline_project".into(),
        target_platform: "Windows".into(),
        executable_path: "C:/pipeline/game.exe".into(),
        build_timestamp: 100,
        build_profile: "Release".into(),
    };
    chain.register_build(build);

    // Step 2: Release package
    let export = ExportArtifact {
        export_id: "pipeline_export_001".into(),
        build_id: "pipeline_build_001".into(),
        export_path: "C:/pipeline/export.zip".into(),
        export_timestamp: 200,
        launchable: true,
    };
    chain.register_export(export);

    // Step 3: Evidence capture
    let evidence = EvidenceBundle {
        bundle_id: "pipeline_evidence".into(),
        pack_id: "pipeline_pack".into(),
        scenario_id: "pipeline_verification".into(),
        build_profile: "Release".into(),
        schema_revision: "v1".into(),
        timestamp: 300,
        artifacts: vec![ArtifactRef {
            artifact_id: "pipeline_artifact".into(),
            artifact_type: ArtifactType::CompareDigest,
            path: "/evidence/pipeline.json".into(),
            checksum: "sha256:pipeline".into(),
        }],
        verdict: EvidenceVerdict::Green,
        operator_signoff: Some("pipeline_manager".into()),
    };
    assert_eq!(evidence.verdict, EvidenceVerdict::Green);

    // Step 4: Launch verification
    let launch = LaunchTrace {
        launch_id: "pipeline_launch".into(),
        export_id: "pipeline_export_001".into(),
        launch_timestamp: 400,
        process_id: Some(54321),
        exit_code: Some(0),
        runtime_diagnostics: vec!["all_systems_go".into()],
        runtime_signature: Some(FIRST_RESULT_SIGNATURE.to_string()),
    };
    chain.register_launch(launch);

    // Verify the full chain
    let verification = chain.verify_chain("pipeline_launch").unwrap();
    assert!(verification.chain_valid);
    assert_eq!(verification.build_id, "pipeline_build_001");
    assert_eq!(verification.export_id, "pipeline_export_001");
    assert_eq!(verification.launch_id, "pipeline_launch");
    assert_eq!(verification.project_id, "pipeline_project");
}

#[test]
fn full_pipeline_with_comparison() {
    let mut chain = ReleaseChain::new();
    let mut compare_engine = CompareEngine::new();
    let mut cert_engine = CertificationEngine::new();

    // Build
    chain.register_build(BuildArtifact {
        build_id: "cmp_build".into(),
        project_id: "cmp_project".into(),
        target_platform: "Windows".into(),
        executable_path: "/game.exe".into(),
        build_timestamp: 1,
        build_profile: "Release".into(),
    });

    // Export
    chain.register_export(ExportArtifact {
        export_id: "cmp_export".into(),
        build_id: "cmp_build".into(),
        export_path: "/export.zip".into(),
        export_timestamp: 2,
        launchable: true,
    });

    // Compare baseline
    compare_engine.create_triplet("cmp_pack".into(), "baseline_v1".into(), CompareMode::Exact);

    // Evidence
    let evidence = EvidenceBundle {
        bundle_id: "cmp_evidence".into(),
        pack_id: "cmp_pack".into(),
        scenario_id: "cmp_scenario".into(),
        build_profile: "Release".into(),
        schema_revision: "v1".into(),
        timestamp: 3,
        artifacts: vec![],
        verdict: EvidenceVerdict::Green,
        operator_signoff: None,
    };

    let compare_result = compare_engine.execute_compare("cmp_pack").unwrap();
    assert_eq!(compare_result.verdict, CompareVerdict::Pass);

    let cert_result = cert_engine.certify_pack(
        "cmp_pack".into(),
        "cmp_scenario".into(),
        &evidence,
        &compare_result,
    );
    assert_eq!(cert_result.verdict, CertificationVerdict::Certified);

    // Launch
    chain.register_launch(LaunchTrace {
        launch_id: "cmp_launch".into(),
        export_id: "cmp_export".into(),
        launch_timestamp: 4,
        process_id: Some(100),
        exit_code: Some(0),
        runtime_diagnostics: vec![],
        runtime_signature: Some(FIRST_RESULT_SIGNATURE.to_string()),
    });

    let chain_verification = chain.verify_chain("cmp_launch").unwrap();
    assert!(chain_verification.chain_valid);
}

#[test]
fn full_pipeline_with_freeze_gate() {
    let mut chain = ReleaseChain::new();
    let mut compare_engine = CompareEngine::new();
    let mut cert_engine = CertificationEngine::new();
    let mut freeze_engine = FreezeEngine::new();

    // Build
    chain.register_build(BuildArtifact {
        build_id: "freeze_build".into(),
        project_id: "freeze_project".into(),
        target_platform: "Windows".into(),
        executable_path: "/game.exe".into(),
        build_timestamp: 1,
        build_profile: "Shipping".into(),
    });

    // Export
    chain.register_export(ExportArtifact {
        export_id: "freeze_export".into(),
        build_id: "freeze_build".into(),
        export_path: "/export.zip".into(),
        export_timestamp: 2,
        launchable: true,
    });

    // Compare
    compare_engine.create_triplet(
        "freeze_pack".into(),
        "golden_baseline".into(),
        CompareMode::Exact,
    );

    // Evidence (Green)
    let evidence = EvidenceBundle {
        bundle_id: "freeze_evidence".into(),
        pack_id: "freeze_pack".into(),
        scenario_id: "freeze_scenario".into(),
        build_profile: "Shipping".into(),
        schema_revision: "v1".into(),
        timestamp: 3,
        artifacts: vec![ArtifactRef {
            artifact_id: "freeze_digest".into(),
            artifact_type: ArtifactType::CompareDigest,
            path: "/freeze.json".into(),
            checksum: "sha256:freeze".into(),
        }],
        verdict: EvidenceVerdict::Green,
        operator_signoff: None,
    };

    let compare_result = compare_engine.execute_compare("freeze_pack").unwrap();
    let cert_result = cert_engine.certify_pack(
        "freeze_pack".into(),
        "freeze_scenario".into(),
        &evidence,
        &compare_result,
    );
    assert_eq!(cert_result.verdict, CertificationVerdict::Certified);

    // Freeze gate
    let freeze_result = freeze_engine.create_freeze(
        "freeze_001".into(),
        vec!["freeze_pack".into()],
        vec![cert_result.pack_id.clone()],
        "Shipping".into(),
        "freeze_operator".into(),
    );
    assert!(freeze_result.is_ok());
    let freeze_bundle = freeze_result.unwrap();
    assert_eq!(freeze_bundle.freeze_posture, FreezePosture::PartialLive);

    // Launch
    chain.register_launch(LaunchTrace {
        launch_id: "freeze_launch".into(),
        export_id: "freeze_export".into(),
        launch_timestamp: 4,
        process_id: Some(200),
        exit_code: Some(0),
        runtime_diagnostics: vec!["gold_verified".into()],
        runtime_signature: Some(FIRST_RESULT_SIGNATURE.to_string()),
    });

    let verification = chain.verify_chain("freeze_launch").unwrap();
    assert!(verification.chain_valid);
}

#[test]
fn pipeline_first_result_verification() {
    let mut verifier = FirstResultVerifier::new();

    // Simulate pipeline completion with correct signature
    let result = verifier.verify_signature(
        "pipeline_verification".into(),
        "pipeline_project".into(),
        "pipeline_build".into(),
        FIRST_RESULT_SIGNATURE.to_string(),
        vec![
            "build_complete".into(),
            "export_complete".into(),
            "evidence_captured".into(),
            "launch_verified".into(),
        ],
    );

    assert!(result.verified);
    assert_eq!(result.verification_id, "pipeline_verification");
    assert_eq!(result.diagnostics_trace.len(), 4);
}

#[test]
fn pipeline_broken_chain_detection() {
    let mut chain = ReleaseChain::new();

    // Build with one ID
    chain.register_build(BuildArtifact {
        build_id: "good_build".into(),
        project_id: "broken_project".into(),
        target_platform: "Windows".into(),
        executable_path: "/game.exe".into(),
        build_timestamp: 1,
        build_profile: "Release".into(),
    });

    // Export references a DIFFERENT build (broken chain)
    chain.register_export(ExportArtifact {
        export_id: "good_export".into(),
        build_id: "missing_build".into(), // References non-existent build
        export_path: "/export.zip".into(),
        export_timestamp: 2,
        launchable: true,
    });

    // Launch
    chain.register_launch(LaunchTrace {
        launch_id: "good_launch".into(),
        export_id: "good_export".into(),
        launch_timestamp: 3,
        process_id: Some(300),
        exit_code: Some(0),
        runtime_diagnostics: vec![],
        runtime_signature: Some(FIRST_RESULT_SIGNATURE.to_string()),
    });

    // Chain verification should fail due to missing build
    let result = chain.verify_chain("good_launch");
    assert!(result.is_err());
}
