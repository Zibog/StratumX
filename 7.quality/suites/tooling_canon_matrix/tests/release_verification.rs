//! Release Verification: проверка системы верификации релизов

use stratumx_tooling_l6_14_release_runtime::first_result_verification::{
    BuildArtifact, ExportArtifact, FirstResultVerifier, LaunchTrace, ReleaseChain,
    FIRST_RESULT_SIGNATURE,
};

#[test]
fn verify_release_package() {
    let mut chain = ReleaseChain::new();
    chain.register_build(BuildArtifact {
        build_id: "build-1".to_string(),
        project_id: "project-1".to_string(),
        target_platform: "windows".to_string(),
        executable_path: "bin/game.exe".to_string(),
        build_timestamp: 1,
        build_profile: "release".to_string(),
    });
    chain.register_export(ExportArtifact {
        export_id: "export-1".to_string(),
        build_id: "build-1".to_string(),
        export_path: "dist/project-1".to_string(),
        export_timestamp: 2,
        launchable: true,
    });
    chain.register_launch(LaunchTrace {
        launch_id: "launch-1".to_string(),
        export_id: "export-1".to_string(),
        launch_timestamp: 3,
        process_id: Some(1234),
        exit_code: Some(0),
        runtime_diagnostics: vec!["launch ok".to_string()],
        runtime_signature: Some(FIRST_RESULT_SIGNATURE.to_string()),
    });

    let verification = chain.verify_chain("launch-1").unwrap();

    assert!(verification.chain_valid);
    assert_eq!(verification.build_id, "build-1");
    assert_eq!(verification.project_id, "project-1");
}

#[test]
fn verify_release_signatures() {
    let mut verifier = FirstResultVerifier::new();
    let result = verifier.verify_signature(
        "verify-1".to_string(),
        "project-1".to_string(),
        "build-1".to_string(),
        FIRST_RESULT_SIGNATURE.to_string(),
        vec!["signature matched".to_string()],
    );

    assert!(result.verified);
    assert!(verifier.is_verified("verify-1"));
    assert_eq!(
        verifier.get_last_verification().unwrap().runtime_signature,
        FIRST_RESULT_SIGNATURE
    );
}

#[test]
fn verify_release_integrity() {
    let mut verifier = FirstResultVerifier::new();
    let result = verifier.verify_signature(
        "verify-2".to_string(),
        "project-1".to_string(),
        "build-2".to_string(),
        "unexpected signature".to_string(),
        vec!["signature mismatch".to_string()],
    );

    assert!(!result.verified);
    assert!(!verifier.is_verified("verify-2"));
}
