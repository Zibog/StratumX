//! Freeze Gating: проверка системы freeze gating

use tool_evidence_capture::{
    ArtifactRef, ArtifactType, Baseline, BaselineRegistry, CertificationEngine, CompareEngine,
    CompareMode, EvidenceBundle, EvidenceVerdict, FreezeBlocker, FreezeGateEngine,
    HardwareFloorResult,
};

fn sample_bundle(pack_id: &str, signoff: Option<&str>) -> EvidenceBundle {
    EvidenceBundle {
        bundle_id: format!("bundle:{pack_id}"),
        pack_id: pack_id.to_string(),
        scenario_id: "scenario.freeze".to_string(),
        build_profile: "release".to_string(),
        schema_revision: "v1".to_string(),
        timestamp: 7,
        artifacts: vec![ArtifactRef {
            artifact_id: "compare:001".to_string(),
            artifact_type: ArtifactType::CompareDigest,
            path: "evidence/compare.json".to_string(),
            checksum: "hash".to_string(),
        }],
        verdict: EvidenceVerdict::Green,
        operator_signoff: signoff.map(str::to_string),
    }
}

fn ready_freeze_gate_state(
    signoff: Option<&str>,
) -> (
    FreezeGateEngine,
    BaselineRegistry,
    CompareEngine,
    CertificationEngine,
    EvidenceBundle,
) {
    let pack_id = "pack.freeze";
    let evidence = sample_bundle(pack_id, signoff);
    let mut baseline_registry = BaselineRegistry::new();
    baseline_registry.register_baseline(Baseline {
        baseline_id: "baseline:pack.freeze".to_string(),
        pack_id: pack_id.to_string(),
        scenario_id: "scenario.freeze".to_string(),
        timestamp: 1,
        artifacts: evidence.artifacts.clone(),
        protected: false,
    });

    let mut compare_engine = CompareEngine::new();
    compare_engine.create_triplet(
        pack_id.to_string(),
        "baseline://pack.freeze".to_string(),
        CompareMode::Exact,
    );
    let compare_result = compare_engine.execute_compare(pack_id).unwrap();

    let mut certification_engine = CertificationEngine::new();
    certification_engine.certify_pack(
        pack_id.to_string(),
        "scenario.freeze".to_string(),
        &evidence,
        &compare_result,
    );

    let mut freeze_gate = FreezeGateEngine::new();
    freeze_gate.register_hardware_floor_result(
        pack_id.to_string(),
        HardwareFloorResult::new(pack_id.to_string(), "scenario.freeze".to_string()),
    );

    (
        freeze_gate,
        baseline_registry,
        compare_engine,
        certification_engine,
        evidence,
    )
}

#[test]
fn freeze_gate_validation() {
    let evidence = EvidenceBundle {
        bundle_id: "bundle:missing".to_string(),
        pack_id: "pack.missing".to_string(),
        scenario_id: "scenario.freeze".to_string(),
        build_profile: "release".to_string(),
        schema_revision: "v1".to_string(),
        timestamp: 0,
        artifacts: Vec::new(),
        verdict: EvidenceVerdict::Green,
        operator_signoff: None,
    };
    let result = FreezeGateEngine::new().check_freeze_gate(
        "pack.missing",
        &BaselineRegistry::new(),
        &CompareEngine::new(),
        &CertificationEngine::new(),
        &evidence,
    );

    assert!(!result.can_freeze);
    assert!(result.blockers.contains(&FreezeBlocker::NoBaseline));
    assert!(result
        .blockers
        .contains(&FreezeBlocker::NoHardwareFloorResult));
    assert!(result.blockers.contains(&FreezeBlocker::MissingArtifacts));
}

#[test]
fn freeze_gate_enforcement() {
    let (freeze_gate, baseline_registry, compare_engine, certification_engine, evidence) =
        ready_freeze_gate_state(Some("qa"));

    let result = freeze_gate.check_freeze_gate(
        "pack.freeze",
        &baseline_registry,
        &compare_engine,
        &certification_engine,
        &evidence,
    );

    assert!(result.can_freeze);
    assert!(result.blockers.is_empty());
    assert!(result.warnings.is_empty());
}

#[test]
fn freeze_gate_reporting() {
    let (freeze_gate, baseline_registry, compare_engine, certification_engine, evidence) =
        ready_freeze_gate_state(None);

    let result = freeze_gate.check_freeze_gate(
        "pack.freeze",
        &baseline_registry,
        &compare_engine,
        &certification_engine,
        &evidence,
    );

    assert!(result.can_freeze);
    assert_eq!(
        result.warnings,
        vec!["No operator signoff recorded".to_string()]
    );
}
