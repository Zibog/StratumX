// Tests for l6.14-release-runtime: FirstResultVerification, ReleaseChain

use stratumx_tooling_l6_14_release_runtime::*;
use stratumx_tooling_l6_14_release_runtime::first_result_verification::*;

// ============================================================================
// CANONICAL_LEVEL constant tests
// ============================================================================

#[test]
fn canonical_level_value() {
    assert_eq!(CANONICAL_LEVEL, "l6.14-release-runtime");
}

#[test]
fn canonical_level_not_empty() {
    assert!(!CANONICAL_LEVEL.is_empty());
}

#[test]
fn canonical_level_starts_with_l6() {
    assert!(CANONICAL_LEVEL.starts_with("l6"));
}

#[test]
fn canonical_level_contains_release() {
    assert!(CANONICAL_LEVEL.contains("release"));
}

// ============================================================================
// L614ReleaseRuntimeMarker tests
// ============================================================================

#[test]
fn marker_default() {
    let m = L614ReleaseRuntimeMarker::default();
    assert_eq!(m, L614ReleaseRuntimeMarker);
}

#[test]
fn marker_equality() {
    assert_eq!(L614ReleaseRuntimeMarker, L614ReleaseRuntimeMarker);
}

#[test]
fn marker_copy() {
    let a = L614ReleaseRuntimeMarker;
    let _b = a;
    let _c = a;
}

#[test]
fn marker_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut h1 = DefaultHasher::new();
    L614ReleaseRuntimeMarker.hash(&mut h1);
    let mut h2 = DefaultHasher::new();
    L614ReleaseRuntimeMarker.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn marker_in_hashset() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(L614ReleaseRuntimeMarker);
    set.insert(L614ReleaseRuntimeMarker);
    assert_eq!(set.len(), 1);
}

#[test]
fn marker_is_unit_struct() {
    assert_eq!(std::mem::size_of::<L614ReleaseRuntimeMarker>(), 0);
}

#[test]
fn marker_debug() {
    let debug = format!("{:?}", L614ReleaseRuntimeMarker);
    assert!(debug.contains("L614ReleaseRuntimeMarker"));
}

// ============================================================================
// FIRST_RESULT_SIGNATURE constant tests
// ============================================================================

#[test]
fn first_result_signature_value() {
    assert_eq!(FIRST_RESULT_SIGNATURE, "ты победил");
}

#[test]
fn first_result_signature_not_empty() {
    assert!(!FIRST_RESULT_SIGNATURE.is_empty());
}

// ============================================================================
// FirstResultVerification tests
// ============================================================================

#[test]
fn first_result_verification_constructs() {
    let v = FirstResultVerification {
        verification_id: "v1".into(),
        project_id: "proj1".into(),
        build_id: "build1".into(),
        launch_timestamp: 12345,
        runtime_signature: "ты победил".into(),
        verified: true,
        diagnostics_trace: vec!["step1".into()],
    };
    assert!(v.verified);
    assert_eq!(v.runtime_signature, "ты победил");
}

#[test]
fn first_result_verification_unverified() {
    let v = FirstResultVerification {
        verification_id: "v2".into(),
        project_id: "proj1".into(),
        build_id: "build1".into(),
        launch_timestamp: 12345,
        runtime_signature: "wrong".into(),
        verified: false,
        diagnostics_trace: vec![],
    };
    assert!(!v.verified);
}

#[test]
fn first_result_verification_serializes() {
    let v = FirstResultVerification {
        verification_id: "v1".into(),
        project_id: "p1".into(),
        build_id: "b1".into(),
        launch_timestamp: 0,
        runtime_signature: "sig".into(),
        verified: true,
        diagnostics_trace: vec!["trace1".into()],
    };
    let json = serde_json::to_string(&v).unwrap();
    assert!(json.contains("v1"));
    assert!(json.contains("true"));
}

#[test]
fn first_result_verification_roundtrip() {
    let v = FirstResultVerification {
        verification_id: "rt".into(),
        project_id: "proj".into(),
        build_id: "build".into(),
        launch_timestamp: 99999,
        runtime_signature: "sig".into(),
        verified: false,
        diagnostics_trace: vec!["a".into(), "b".into()],
    };
    let json = serde_json::to_string(&v).unwrap();
    let restored: FirstResultVerification = serde_json::from_str(&json).unwrap();
    assert_eq!(v, restored);
}

#[test]
fn first_result_verification_equality() {
    let a = FirstResultVerification {
        verification_id: "eq".into(),
        project_id: "p".into(),
        build_id: "b".into(),
        launch_timestamp: 0,
        runtime_signature: "sig".into(),
        verified: true,
        diagnostics_trace: vec![],
    };
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn first_result_verification_debug() {
    let v = FirstResultVerification {
        verification_id: "dbg".into(),
        project_id: "p".into(),
        build_id: "b".into(),
        launch_timestamp: 0,
        runtime_signature: "sig".into(),
        verified: true,
        diagnostics_trace: vec![],
    };
    let debug = format!("{:?}", v);
    assert!(debug.contains("FirstResultVerification"));
}

// ============================================================================
// FirstResultVerifier tests
// ============================================================================

#[test]
fn verifier_new() {
    let v = FirstResultVerifier::new();
    assert!(v.get_last_verification().is_none());
}

#[test]
fn verifier_default() {
    let v = FirstResultVerifier::default();
    assert!(v.get_last_verification().is_none());
}

#[test]
fn verifier_verify_signature_correct() {
    let mut v = FirstResultVerifier::new();
    let result = v.verify_signature(
        "v1".into(),
        "proj1".into(),
        "build1".into(),
        "ты победил".into(),
        vec!["trace1".into()],
    );
    assert!(result.verified);
    assert_eq!(result.verification_id, "v1");
    assert_eq!(result.project_id, "proj1");
    assert_eq!(result.build_id, "build1");
}

#[test]
fn verifier_verify_signature_incorrect() {
    let mut v = FirstResultVerifier::new();
    let result = v.verify_signature(
        "v2".into(),
        "proj1".into(),
        "build1".into(),
        "wrong_signature".into(),
        vec![],
    );
    assert!(!result.verified);
}

#[test]
fn verifier_get_verification() {
    let mut v = FirstResultVerifier::new();
    v.verify_signature(
        "v1".into(),
        "p1".into(),
        "b1".into(),
        "sig".into(),
        vec![],
    );
    let retrieved = v.get_verification("v1");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().verification_id, "v1");
}

#[test]
fn verifier_get_nonexistent_verification() {
    let v = FirstResultVerifier::new();
    assert!(v.get_verification("nonexistent").is_none());
}

#[test]
fn verifier_get_last_verification() {
    let mut v = FirstResultVerifier::new();
    v.verify_signature("v1".into(), "p1".into(), "b1".into(), "ты победил".into(), vec![]);
    v.verify_signature("v2".into(), "p1".into(), "b2".into(), "wrong".into(), vec![]);
    let last = v.get_last_verification();
    assert!(last.is_some());
    assert_eq!(last.unwrap().verification_id, "v2");
    assert!(!last.unwrap().verified);
}

#[test]
fn verifier_is_verified_true() {
    let mut v = FirstResultVerifier::new();
    v.verify_signature("v1".into(), "p1".into(), "b1".into(), "ты победил".into(), vec![]);
    assert!(v.is_verified("v1"));
}

#[test]
fn verifier_is_verified_false() {
    let mut v = FirstResultVerifier::new();
    v.verify_signature("v1".into(), "p1".into(), "b1".into(), "wrong".into(), vec![]);
    assert!(!v.is_verified("v1"));
}

#[test]
fn verifier_is_verified_nonexistent() {
    let v = FirstResultVerifier::new();
    assert!(!v.is_verified("nonexistent"));
}

#[test]
fn verifier_stores_multiple_verifications() {
    let mut v = FirstResultVerifier::new();
    v.verify_signature("v1".into(), "p1".into(), "b1".into(), "ты победил".into(), vec![]);
    v.verify_signature("v2".into(), "p1".into(), "b2".into(), "ты победил".into(), vec![]);
    assert!(v.is_verified("v1"));
    assert!(v.is_verified("v2"));
}

#[test]
fn verifier_diagnostics_trace_stored() {
    let mut v = FirstResultVerifier::new();
    let result = v.verify_signature(
        "v1".into(),
        "p1".into(),
        "b1".into(),
        "sig".into(),
        vec!["step1".into(), "step2".into(), "step3".into()],
    );
    assert_eq!(result.diagnostics_trace.len(), 3);
}

// ============================================================================
// BuildArtifact tests
// ============================================================================

#[test]
fn build_artifact_constructs() {
    let ba = BuildArtifact {
        build_id: "build_001".into(),
        project_id: "proj1".into(),
        target_platform: "Windows".into(),
        executable_path: "C:/game.exe".into(),
        build_timestamp: 12345,
        build_profile: "Release".into(),
    };
    assert_eq!(ba.build_id, "build_001");
}

#[test]
fn build_artifact_serializes() {
    let ba = BuildArtifact {
        build_id: "b1".into(),
        project_id: "p1".into(),
        target_platform: "Linux".into(),
        executable_path: "/game".into(),
        build_timestamp: 0,
        build_profile: "Debug".into(),
    };
    let json = serde_json::to_string(&ba).unwrap();
    assert!(json.contains("b1"));
    assert!(json.contains("Linux"));
}

#[test]
fn build_artifact_roundtrip() {
    let ba = BuildArtifact {
        build_id: "rt".into(),
        project_id: "proj".into(),
        target_platform: "MacOS".into(),
        executable_path: "/app/game.app".into(),
        build_timestamp: 99999,
        build_profile: "Shipping".into(),
    };
    let json = serde_json::to_string(&ba).unwrap();
    let restored: BuildArtifact = serde_json::from_str(&json).unwrap();
    assert_eq!(ba, restored);
}

#[test]
fn build_artifact_equality() {
    let a = BuildArtifact {
        build_id: "eq".into(),
        project_id: "p".into(),
        target_platform: "Win".into(),
        executable_path: "/g".into(),
        build_timestamp: 0,
        build_profile: "R".into(),
    };
    let b = a.clone();
    assert_eq!(a, b);
}

// ============================================================================
// ExportArtifact tests
// ============================================================================

#[test]
fn export_artifact_constructs() {
    let ea = ExportArtifact {
        export_id: "exp_001".into(),
        build_id: "build_001".into(),
        export_path: "/exports/game.zip".into(),
        export_timestamp: 12345,
        launchable: true,
    };
    assert!(ea.launchable);
}

#[test]
fn export_artifact_serializes() {
    let ea = ExportArtifact {
        export_id: "e1".into(),
        build_id: "b1".into(),
        export_path: "/export".into(),
        export_timestamp: 0,
        launchable: false,
    };
    let json = serde_json::to_string(&ea).unwrap();
    assert!(json.contains("e1"));
    assert!(json.contains("false"));
}

#[test]
fn export_artifact_roundtrip() {
    let ea = ExportArtifact {
        export_id: "rt".into(),
        build_id: "b_rt".into(),
        export_path: "/path".into(),
        export_timestamp: 11111,
        launchable: true,
    };
    let json = serde_json::to_string(&ea).unwrap();
    let restored: ExportArtifact = serde_json::from_str(&json).unwrap();
    assert_eq!(ea, restored);
}

// ============================================================================
// LaunchTrace tests
// ============================================================================

#[test]
fn launch_trace_constructs() {
    let lt = LaunchTrace {
        launch_id: "launch_001".into(),
        export_id: "exp_001".into(),
        launch_timestamp: 12345,
        process_id: Some(1234),
        exit_code: Some(0),
        runtime_diagnostics: vec!["all good".into()],
        runtime_signature: Some("ты победил".into()),
    };
    assert_eq!(lt.exit_code, Some(0));
}

#[test]
fn launch_trace_no_process() {
    let lt = LaunchTrace {
        launch_id: "l1".into(),
        export_id: "e1".into(),
        launch_timestamp: 0,
        process_id: None,
        exit_code: None,
        runtime_diagnostics: vec![],
        runtime_signature: None,
    };
    assert!(lt.process_id.is_none());
    assert!(lt.exit_code.is_none());
}

#[test]
fn launch_trace_serializes() {
    let lt = LaunchTrace {
        launch_id: "lt1".into(),
        export_id: "e1".into(),
        launch_timestamp: 0,
        process_id: Some(42),
        exit_code: Some(0),
        runtime_diagnostics: vec!["diag1".into()],
        runtime_signature: Some("sig".into()),
    };
    let json = serde_json::to_string(&lt).unwrap();
    assert!(json.contains("lt1"));
    assert!(json.contains("42"));
}

#[test]
fn launch_trace_roundtrip() {
    let lt = LaunchTrace {
        launch_id: "rt".into(),
        export_id: "e_rt".into(),
        launch_timestamp: 77777,
        process_id: None,
        exit_code: Some(-1),
        runtime_diagnostics: vec!["err1".into(), "err2".into()],
        runtime_signature: Some("fail".into()),
    };
    let json = serde_json::to_string(&lt).unwrap();
    let restored: LaunchTrace = serde_json::from_str(&json).unwrap();
    assert_eq!(lt, restored);
}

// ============================================================================
// ReleaseChain tests
// ============================================================================

#[test]
fn release_chain_new() {
    let rc = ReleaseChain::new();
    assert!(rc.get_last_build().is_none());
    assert!(rc.get_last_export().is_none());
    assert!(rc.get_last_launch().is_none());
}

#[test]
fn release_chain_default() {
    let rc = ReleaseChain::default();
    assert!(rc.get_last_build().is_none());
}

#[test]
fn release_chain_register_build() {
    let mut rc = ReleaseChain::new();
    let ba = BuildArtifact {
        build_id: "b1".into(),
        project_id: "p1".into(),
        target_platform: "Windows".into(),
        executable_path: "/game.exe".into(),
        build_timestamp: 0,
        build_profile: "Release".into(),
    };
    rc.register_build(ba);
    let retrieved = rc.get_build("b1");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().build_id, "b1");
}

#[test]
fn release_chain_register_export() {
    let mut rc = ReleaseChain::new();
    let ea = ExportArtifact {
        export_id: "e1".into(),
        build_id: "b1".into(),
        export_path: "/export".into(),
        export_timestamp: 0,
        launchable: true,
    };
    rc.register_export(ea);
    let retrieved = rc.get_export("e1");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().export_id, "e1");
}

#[test]
fn release_chain_register_launch() {
    let mut rc = ReleaseChain::new();
    let lt = LaunchTrace {
        launch_id: "l1".into(),
        export_id: "e1".into(),
        launch_timestamp: 0,
        process_id: None,
        exit_code: Some(0),
        runtime_diagnostics: vec![],
        runtime_signature: Some("sig".into()),
    };
    rc.register_launch(lt);
    let retrieved = rc.get_launch("l1");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().launch_id, "l1");
}

#[test]
fn release_chain_get_last_build() {
    let mut rc = ReleaseChain::new();
    rc.register_build(BuildArtifact {
        build_id: "b1".into(),
        project_id: "p1".into(),
        target_platform: "W".into(),
        executable_path: "/g".into(),
        build_timestamp: 1,
        build_profile: "R".into(),
    });
    rc.register_build(BuildArtifact {
        build_id: "b2".into(),
        project_id: "p1".into(),
        target_platform: "W".into(),
        executable_path: "/g".into(),
        build_timestamp: 2,
        build_profile: "R".into(),
    });
    let last = rc.get_last_build();
    assert!(last.is_some());
    assert_eq!(last.unwrap().build_id, "b2");
}

#[test]
fn release_chain_get_last_export() {
    let mut rc = ReleaseChain::new();
    rc.register_export(ExportArtifact {
        export_id: "e1".into(),
        build_id: "b1".into(),
        export_path: "/e1".into(),
        export_timestamp: 1,
        launchable: true,
    });
    rc.register_export(ExportArtifact {
        export_id: "e2".into(),
        build_id: "b1".into(),
        export_path: "/e2".into(),
        export_timestamp: 2,
        launchable: true,
    });
    assert_eq!(rc.get_last_export().unwrap().export_id, "e2");
}

#[test]
fn release_chain_get_last_launch() {
    let mut rc = ReleaseChain::new();
    rc.register_launch(LaunchTrace {
        launch_id: "l1".into(),
        export_id: "e1".into(),
        launch_timestamp: 1,
        process_id: None,
        exit_code: Some(0),
        runtime_diagnostics: vec![],
        runtime_signature: None,
    });
    rc.register_launch(LaunchTrace {
        launch_id: "l2".into(),
        export_id: "e1".into(),
        launch_timestamp: 2,
        process_id: None,
        exit_code: Some(0),
        runtime_diagnostics: vec![],
        runtime_signature: None,
    });
    assert_eq!(rc.get_last_launch().unwrap().launch_id, "l2");
}

#[test]
fn release_chain_verify_chain() {
    let mut rc = ReleaseChain::new();
    rc.register_build(BuildArtifact {
        build_id: "b1".into(),
        project_id: "proj_alpha".into(),
        target_platform: "Windows".into(),
        executable_path: "/game.exe".into(),
        build_timestamp: 100,
        build_profile: "Release".into(),
    });
    rc.register_export(ExportArtifact {
        export_id: "e1".into(),
        build_id: "b1".into(),
        export_path: "/export.zip".into(),
        export_timestamp: 200,
        launchable: true,
    });
    rc.register_launch(LaunchTrace {
        launch_id: "l1".into(),
        export_id: "e1".into(),
        launch_timestamp: 300,
        process_id: Some(1234),
        exit_code: Some(0),
        runtime_diagnostics: vec![],
        runtime_signature: Some("ты победил".into()),
    });

    let verification = rc.verify_chain("l1").unwrap();
    assert!(verification.chain_valid);
    assert_eq!(verification.launch_id, "l1");
    assert_eq!(verification.export_id, "e1");
    assert_eq!(verification.build_id, "b1");
    assert_eq!(verification.project_id, "proj_alpha");
}

#[test]
fn release_chain_verify_chain_launch_not_found() {
    let rc = ReleaseChain::new();
    let result = rc.verify_chain("nonexistent");
    assert!(result.is_err());
}

#[test]
fn release_chain_verify_chain_export_not_found() {
    let mut rc = ReleaseChain::new();
    rc.register_launch(LaunchTrace {
        launch_id: "l1".into(),
        export_id: "nonexistent_export".into(),
        launch_timestamp: 0,
        process_id: None,
        exit_code: None,
        runtime_diagnostics: vec![],
        runtime_signature: None,
    });
    let result = rc.verify_chain("l1");
    assert!(result.is_err());
}

#[test]
fn release_chain_verify_chain_build_not_found() {
    let mut rc = ReleaseChain::new();
    rc.register_export(ExportArtifact {
        export_id: "e1".into(),
        build_id: "nonexistent_build".into(),
        export_path: "/e".into(),
        export_timestamp: 0,
        launchable: true,
    });
    rc.register_launch(LaunchTrace {
        launch_id: "l1".into(),
        export_id: "e1".into(),
        launch_timestamp: 0,
        process_id: None,
        exit_code: None,
        runtime_diagnostics: vec![],
        runtime_signature: None,
    });
    let result = rc.verify_chain("l1");
    assert!(result.is_err());
}

// ============================================================================
// ChainVerification tests
// ============================================================================

#[test]
fn chain_verification_constructs() {
    let cv = ChainVerification {
        launch_id: "l1".into(),
        export_id: "e1".into(),
        build_id: "b1".into(),
        project_id: "p1".into(),
        chain_valid: true,
    };
    assert!(cv.chain_valid);
}

#[test]
fn chain_verification_serializes() {
    let cv = ChainVerification {
        launch_id: "l1".into(),
        export_id: "e1".into(),
        build_id: "b1".into(),
        project_id: "p1".into(),
        chain_valid: true,
    };
    let json = serde_json::to_string(&cv).unwrap();
    assert!(json.contains("chain_valid"));
    assert!(json.contains("true"));
}

#[test]
fn chain_verification_roundtrip() {
    let cv = ChainVerification {
        launch_id: "rt".into(),
        export_id: "e_rt".into(),
        build_id: "b_rt".into(),
        project_id: "p_rt".into(),
        chain_valid: false,
    };
    let json = serde_json::to_string(&cv).unwrap();
    let restored: ChainVerification = serde_json::from_str(&json).unwrap();
    assert_eq!(cv, restored);
}

#[test]
fn chain_verification_equality() {
    let a = ChainVerification {
        launch_id: "eq".into(),
        export_id: "e".into(),
        build_id: "b".into(),
        project_id: "p".into(),
        chain_valid: true,
    };
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn chain_verification_debug() {
    let cv = ChainVerification {
        launch_id: "dbg".into(),
        export_id: "e".into(),
        build_id: "b".into(),
        project_id: "p".into(),
        chain_valid: true,
    };
    let debug = format!("{:?}", cv);
    assert!(debug.contains("ChainVerification"));
}
