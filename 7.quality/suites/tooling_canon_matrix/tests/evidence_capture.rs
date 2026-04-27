//! Evidence Capture: проверка системы захвата доказательств

use tool_evidence_capture::{
    ArtifactRef, ArtifactType, Baseline, BaselineRegistry, CertificationEngine,
    CertificationVerdict, CompareEngine, CompareMode, EvidenceBundle, EvidenceVerdict,
};

fn evidence_bundle(pack_id: &str, verdict: EvidenceVerdict) -> EvidenceBundle {
    EvidenceBundle {
        bundle_id: format!("bundle:{pack_id}"),
        pack_id: pack_id.to_string(),
        scenario_id: "scenario.alpha".to_string(),
        build_profile: "first-result".to_string(),
        schema_revision: "v1".to_string(),
        timestamp: 42,
        artifacts: vec![
            ArtifactRef {
                artifact_id: "frame:001".to_string(),
                artifact_type: ArtifactType::FrameCapture,
                path: "captures/frame.png".to_string(),
                checksum: "abc123".to_string(),
            },
            ArtifactRef {
                artifact_id: "log:001".to_string(),
                artifact_type: ArtifactType::DiagnosticsLog,
                path: "captures/runtime.log".to_string(),
                checksum: "def456".to_string(),
            },
        ],
        verdict,
        operator_signoff: Some("qa".to_string()),
    }
}

#[test]
fn capture_screenshot() {
    let bundle = evidence_bundle("pack.alpha", EvidenceVerdict::Green);

    assert_eq!(
        bundle.artifacts[0].artifact_type,
        ArtifactType::FrameCapture
    );
    assert_eq!(bundle.artifacts[0].path, "captures/frame.png");
    assert_eq!(bundle.verdict, EvidenceVerdict::Green);
}

#[test]
fn capture_metrics() {
    let mut registry = BaselineRegistry::new();
    registry.register_baseline(Baseline {
        baseline_id: "baseline:pack.alpha".to_string(),
        pack_id: "pack.alpha".to_string(),
        scenario_id: "scenario.alpha".to_string(),
        timestamp: 1,
        artifacts: evidence_bundle("pack.alpha", EvidenceVerdict::Green).artifacts,
        protected: false,
    });

    assert!(registry.can_replace_baseline("pack.alpha", &EvidenceVerdict::Green));
    assert!(!registry.can_replace_baseline("pack.alpha", &EvidenceVerdict::Orange));
    assert!(!registry.can_replace_baseline("pack.alpha", &EvidenceVerdict::Red));
}

#[test]
fn capture_logs() {
    let bundle = evidence_bundle("pack.beta", EvidenceVerdict::Green);
    let mut compare = CompareEngine::new();
    compare.create_triplet(
        "pack.beta".to_string(),
        "baseline://pack.beta".to_string(),
        CompareMode::Exact,
    );
    let compare_result = compare.execute_compare("pack.beta").unwrap();
    let mut certification = CertificationEngine::new();
    let result = certification.certify_pack(
        "pack.beta".to_string(),
        "scenario.alpha".to_string(),
        &bundle,
        &compare_result,
    );

    assert_eq!(result.verdict, CertificationVerdict::Certified);
    assert_eq!(result.evidence_bundle_ref, "bundle:pack.beta");
    assert_eq!(result.compare_result_ref, "pack.beta");
}
