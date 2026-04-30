// Tests for l6.7-tool-evidence-capture: EvidenceBundle, Compare, Certification, FreezeGate, Waivers

extern crate tool_evidence_capture;
use tool_evidence_capture::*;

// ============================================================================
// ArtifactType tests
// ============================================================================

#[test]
fn artifact_type_variants() {
    let types = [
        ArtifactType::FrameCapture,
        ArtifactType::AudioCapture,
        ArtifactType::StateSnapshot,
        ArtifactType::DiagnosticsLog,
        ArtifactType::CompareDigest,
        ArtifactType::RecoveryTrace,
    ];
    assert_eq!(types.len(), 6);
}

#[test]
fn artifact_type_serializes() {
    let json = serde_json::to_string(&ArtifactType::FrameCapture).unwrap();
    assert!(json.contains("FrameCapture"));
}

#[test]
fn artifact_type_roundtrip() {
    for at in [
        ArtifactType::FrameCapture,
        ArtifactType::AudioCapture,
        ArtifactType::StateSnapshot,
        ArtifactType::DiagnosticsLog,
        ArtifactType::CompareDigest,
        ArtifactType::RecoveryTrace,
    ] {
        let json = serde_json::to_string(&at).unwrap();
        let restored: ArtifactType = serde_json::from_str(&json).unwrap();
        assert_eq!(at, restored);
    }
}

// ============================================================================
// ArtifactRef tests
// ============================================================================

#[test]
fn artifact_ref_constructs() {
    let ar = ArtifactRef {
        artifact_id: "art_001".into(),
        artifact_type: ArtifactType::FrameCapture,
        path: "/captures/frame_001.png".into(),
        checksum: "sha256:abc".into(),
    };
    assert_eq!(ar.artifact_id, "art_001");
    assert_eq!(ar.path, "/captures/frame_001.png");
}

#[test]
fn artifact_ref_serializes() {
    let ar = ArtifactRef {
        artifact_id: "art_001".into(),
        artifact_type: ArtifactType::StateSnapshot,
        path: "/snapshots/s1.bin".into(),
        checksum: "md5:123".into(),
    };
    let json = serde_json::to_string(&ar).unwrap();
    assert!(json.contains("art_001"));
    assert!(json.contains("StateSnapshot"));
}

#[test]
fn artifact_ref_roundtrip() {
    let ar = ArtifactRef {
        artifact_id: "art_002".into(),
        artifact_type: ArtifactType::CompareDigest,
        path: "/digests/d1.json".into(),
        checksum: "sha256:xyz".into(),
    };
    let json = serde_json::to_string(&ar).unwrap();
    let restored: ArtifactRef = serde_json::from_str(&json).unwrap();
    assert_eq!(ar, restored);
}

#[test]
fn artifact_ref_equality() {
    let a = ArtifactRef {
        artifact_id: "a1".into(),
        artifact_type: ArtifactType::FrameCapture,
        path: "/p1".into(),
        checksum: "c1".into(),
    };
    let b = a.clone();
    assert_eq!(a, b);
}

// ============================================================================
// EvidenceVerdict tests
// ============================================================================

#[test]
fn evidence_verdict_variants() {
    let verdicts = [
        EvidenceVerdict::Green,
        EvidenceVerdict::Orange,
        EvidenceVerdict::Red,
    ];
    assert_eq!(verdicts.len(), 3);
}

#[test]
fn evidence_verdict_equality() {
    assert_eq!(EvidenceVerdict::Green, EvidenceVerdict::Green);
    assert_ne!(EvidenceVerdict::Green, EvidenceVerdict::Red);
}

#[test]
fn evidence_verdict_serializes() {
    assert!(serde_json::to_string(&EvidenceVerdict::Green)
        .unwrap()
        .contains("Green"));
    assert!(serde_json::to_string(&EvidenceVerdict::Orange)
        .unwrap()
        .contains("Orange"));
    assert!(serde_json::to_string(&EvidenceVerdict::Red)
        .unwrap()
        .contains("Red"));
}

#[test]
fn evidence_verdict_roundtrip() {
    for v in [
        EvidenceVerdict::Green,
        EvidenceVerdict::Orange,
        EvidenceVerdict::Red,
    ] {
        let json = serde_json::to_string(&v).unwrap();
        let restored: EvidenceVerdict = serde_json::from_str(&json).unwrap();
        assert_eq!(v, restored);
    }
}

// ============================================================================
// EvidenceBundle tests
// ============================================================================

#[test]
fn evidence_bundle_constructs() {
    let bundle = EvidenceBundle {
        bundle_id: "bundle_001".into(),
        pack_id: "pack_001".into(),
        scenario_id: "scenario_a".into(),
        build_profile: "Debug".into(),
        schema_revision: "v1".into(),
        timestamp: 0,
        artifacts: vec![],
        verdict: EvidenceVerdict::Green,
        operator_signoff: None,
    };
    assert_eq!(bundle.bundle_id, "bundle_001");
    assert!(bundle.operator_signoff.is_none());
}

#[test]
fn evidence_bundle_with_artifacts() {
    let artifact = ArtifactRef {
        artifact_id: "art_001".into(),
        artifact_type: ArtifactType::FrameCapture,
        path: "/cap.png".into(),
        checksum: "c1".into(),
    };
    let bundle = EvidenceBundle {
        bundle_id: "b1".into(),
        pack_id: "p1".into(),
        scenario_id: "s1".into(),
        build_profile: "Release".into(),
        schema_revision: "v2".into(),
        timestamp: 12345,
        artifacts: vec![artifact],
        verdict: EvidenceVerdict::Green,
        operator_signoff: Some("operator1".into()),
    };
    assert_eq!(bundle.artifacts.len(), 1);
    assert_eq!(bundle.operator_signoff, Some("operator1".into()));
}

#[test]
fn evidence_bundle_serializes() {
    let bundle = EvidenceBundle {
        bundle_id: "b1".into(),
        pack_id: "p1".into(),
        scenario_id: "s1".into(),
        build_profile: "Debug".into(),
        schema_revision: "v1".into(),
        timestamp: 0,
        artifacts: vec![],
        verdict: EvidenceVerdict::Orange,
        operator_signoff: None,
    };
    let json = serde_json::to_string(&bundle).unwrap();
    assert!(json.contains("Orange"));
    assert!(json.contains("b1"));
}

#[test]
fn evidence_bundle_roundtrip() {
    let bundle = EvidenceBundle {
        bundle_id: "roundtrip".into(),
        pack_id: "pack_rt".into(),
        scenario_id: "scenario_rt".into(),
        build_profile: "Test".into(),
        schema_revision: "v3".into(),
        timestamp: 999,
        artifacts: vec![ArtifactRef {
            artifact_id: "art_rt".into(),
            artifact_type: ArtifactType::DiagnosticsLog,
            path: "/log.txt".into(),
            checksum: "cs".into(),
        }],
        verdict: EvidenceVerdict::Green,
        operator_signoff: Some("ops".into()),
    };
    let json = serde_json::to_string(&bundle).unwrap();
    let restored: EvidenceBundle = serde_json::from_str(&json).unwrap();
    assert_eq!(bundle, restored);
}

// ============================================================================
// CompareMode tests
// ============================================================================

#[test]
fn compare_mode_variants() {
    let modes = [
        CompareMode::Exact,
        CompareMode::Threshold,
        CompareMode::Visual,
        CompareMode::Semantic,
    ];
    assert_eq!(modes.len(), 4);
}

#[test]
fn compare_mode_roundtrip() {
    for mode in [
        CompareMode::Exact,
        CompareMode::Threshold,
        CompareMode::Visual,
        CompareMode::Semantic,
    ] {
        let json = serde_json::to_string(&mode).unwrap();
        let restored: CompareMode = serde_json::from_str(&json).unwrap();
        assert_eq!(mode, restored);
    }
}

// ============================================================================
// CompareVerdict tests
// ============================================================================

#[test]
fn compare_verdict_variants() {
    let verdicts = [
        CompareVerdict::Pass,
        CompareVerdict::Failed,
        CompareVerdict::Recovered,
        CompareVerdict::NoBaseline,
    ];
    assert_eq!(verdicts.len(), 4);
}

#[test]
fn compare_verdict_roundtrip() {
    for v in [
        CompareVerdict::Pass,
        CompareVerdict::Failed,
        CompareVerdict::Recovered,
        CompareVerdict::NoBaseline,
    ] {
        let json = serde_json::to_string(&v).unwrap();
        let restored: CompareVerdict = serde_json::from_str(&json).unwrap();
        assert_eq!(v, restored);
    }
}

// ============================================================================
// CompareTriplet tests
// ============================================================================

#[test]
fn compare_triplet_constructs() {
    let triplet = CompareTriplet {
        baseline_ref: "baseline_v1".into(),
        failed_ref: None,
        recovery_ref: None,
        compare_mode: CompareMode::Exact,
        diff_artifacts: vec![],
    };
    assert_eq!(triplet.baseline_ref, "baseline_v1");
    assert!(triplet.failed_ref.is_none());
}

#[test]
fn compare_triplet_with_failed_and_recovery() {
    let triplet = CompareTriplet {
        baseline_ref: "bl".into(),
        failed_ref: Some("failed_run".into()),
        recovery_ref: Some("recovery_run".into()),
        compare_mode: CompareMode::Threshold,
        diff_artifacts: vec![],
    };
    assert!(triplet.failed_ref.is_some());
    assert!(triplet.recovery_ref.is_some());
}

#[test]
fn compare_triplet_serializes() {
    let triplet = CompareTriplet {
        baseline_ref: "bl".into(),
        failed_ref: None,
        recovery_ref: None,
        compare_mode: CompareMode::Visual,
        diff_artifacts: vec![],
    };
    let json = serde_json::to_string(&triplet).unwrap();
    assert!(json.contains("Visual"));
}

// ============================================================================
// CompareEngine tests
// ============================================================================

#[test]
fn compare_engine_new() {
    let eng = CompareEngine::new();
    assert!(eng.get_triplet("any").is_none());
}

#[test]
fn compare_engine_default() {
    let eng = CompareEngine::default();
    assert!(eng.get_triplet("any").is_none());
}

#[test]
fn compare_engine_create_triplet() {
    let mut eng = CompareEngine::new();
    let id = eng.create_triplet("pack1".into(), "baseline_v1".into(), CompareMode::Exact);
    assert_eq!(id, "pack1");
}

#[test]
fn compare_engine_get_triplet() {
    let mut eng = CompareEngine::new();
    eng.create_triplet("pack1".into(), "bl1".into(), CompareMode::Exact);
    let triplet = eng.get_triplet("pack1");
    assert!(triplet.is_some());
    assert_eq!(triplet.unwrap().baseline_ref, "bl1");
}

#[test]
fn compare_engine_add_failed_run() {
    let mut eng = CompareEngine::new();
    eng.create_triplet("pack1".into(), "bl1".into(), CompareMode::Exact);
    eng.add_failed_run("pack1", "failed_run_1".into());
    let triplet = eng.get_triplet("pack1").unwrap();
    assert_eq!(triplet.failed_ref, Some("failed_run_1".into()));
}

#[test]
fn compare_engine_add_recovery_run() {
    let mut eng = CompareEngine::new();
    eng.create_triplet("pack1".into(), "bl1".into(), CompareMode::Exact);
    eng.add_recovery_run("pack1", "recovery_run_1".into());
    let triplet = eng.get_triplet("pack1").unwrap();
    assert_eq!(triplet.recovery_ref, Some("recovery_run_1".into()));
}

#[test]
fn compare_engine_execute_compare_pass() {
    let mut eng = CompareEngine::new();
    eng.create_triplet("pack1".into(), "bl1".into(), CompareMode::Exact);
    let result = eng.execute_compare("pack1").unwrap();
    assert_eq!(result.verdict, CompareVerdict::Pass);
}

#[test]
fn compare_engine_execute_compare_no_baseline() {
    let mut eng = CompareEngine::new();
    eng.create_triplet("pack1".into(), "".into(), CompareMode::Exact);
    let result = eng.execute_compare("pack1").unwrap();
    assert_eq!(result.verdict, CompareVerdict::NoBaseline);
}

#[test]
fn compare_engine_execute_compare_failed_no_recovery() {
    let mut eng = CompareEngine::new();
    eng.create_triplet("pack1".into(), "bl1".into(), CompareMode::Exact);
    eng.add_failed_run("pack1", "f1".into());
    let result = eng.execute_compare("pack1").unwrap();
    assert_eq!(result.verdict, CompareVerdict::Failed);
}

#[test]
fn compare_engine_execute_compare_recovered() {
    let mut eng = CompareEngine::new();
    eng.create_triplet("pack1".into(), "bl1".into(), CompareMode::Exact);
    eng.add_failed_run("pack1", "f1".into());
    eng.add_recovery_run("pack1", "r1".into());
    let result = eng.execute_compare("pack1").unwrap();
    assert_eq!(result.verdict, CompareVerdict::Recovered);
}

#[test]
fn compare_engine_execute_compare_nonexistent_pack() {
    let mut eng = CompareEngine::new();
    let result = eng.execute_compare("nonexistent");
    assert!(result.is_err());
}

#[test]
fn compare_result_serializes() {
    let result = CompareResult {
        pack_id: "pack1".into(),
        verdict: CompareVerdict::Pass,
        diff_summary: "Compare mode: Exact".into(),
    };
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("Pass"));
}

// ============================================================================
// Baseline tests
// ============================================================================

#[test]
fn baseline_constructs() {
    let baseline = Baseline {
        baseline_id: "bl_001".into(),
        pack_id: "pack_001".into(),
        scenario_id: "scenario_a".into(),
        timestamp: 12345,
        artifacts: vec![],
        protected: false,
    };
    assert!(!baseline.protected);
}

#[test]
fn baseline_protected() {
    let baseline = Baseline {
        baseline_id: "bl_golden".into(),
        pack_id: "pack_001".into(),
        scenario_id: "scenario_a".into(),
        timestamp: 12345,
        artifacts: vec![],
        protected: true,
    };
    assert!(baseline.protected);
}

#[test]
fn baseline_serializes() {
    let baseline = Baseline {
        baseline_id: "bl1".into(),
        pack_id: "p1".into(),
        scenario_id: "s1".into(),
        timestamp: 0,
        artifacts: vec![],
        protected: true,
    };
    let json = serde_json::to_string(&baseline).unwrap();
    assert!(json.contains("true"));
}

// ============================================================================
// BaselineRegistry tests
// ============================================================================

#[test]
fn baseline_registry_new() {
    let reg = BaselineRegistry::new();
    assert!(reg.get_baseline("any").is_none());
}

#[test]
fn baseline_registry_default() {
    let reg = BaselineRegistry::default();
    assert!(reg.get_baseline("any").is_none());
}

#[test]
fn baseline_registry_register_and_get() {
    let mut reg = BaselineRegistry::new();
    let baseline = Baseline {
        baseline_id: "bl1".into(),
        pack_id: "pack1".into(),
        scenario_id: "s1".into(),
        timestamp: 0,
        artifacts: vec![],
        protected: false,
    };
    reg.register_baseline(baseline);
    let retrieved = reg.get_baseline("pack1");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().baseline_id, "bl1");
}

#[test]
fn baseline_registry_can_replace_green() {
    let reg = BaselineRegistry::new();
    assert!(reg.can_replace_baseline("pack1", &EvidenceVerdict::Green));
}

#[test]
fn baseline_registry_cannot_replace_orange() {
    let reg = BaselineRegistry::new();
    assert!(!reg.can_replace_baseline("pack1", &EvidenceVerdict::Orange));
}

#[test]
fn baseline_registry_cannot_replace_red() {
    let reg = BaselineRegistry::new();
    assert!(!reg.can_replace_baseline("pack1", &EvidenceVerdict::Red));
}

#[test]
fn baseline_registry_update_non_protected() {
    let mut reg = BaselineRegistry::new();
    let bl1 = Baseline {
        baseline_id: "bl1".into(),
        pack_id: "pack1".into(),
        scenario_id: "s1".into(),
        timestamp: 0,
        artifacts: vec![],
        protected: false,
    };
    reg.register_baseline(bl1);

    let bl2 = Baseline {
        baseline_id: "bl2".into(),
        pack_id: "pack1".into(),
        scenario_id: "s2".into(),
        timestamp: 1,
        artifacts: vec![],
        protected: false,
    };
    assert!(reg.update_baseline("pack1", bl2).is_ok());
}

#[test]
fn baseline_registry_update_protected_fails() {
    let mut reg = BaselineRegistry::new();
    let bl1 = Baseline {
        baseline_id: "bl1".into(),
        pack_id: "pack1".into(),
        scenario_id: "s1".into(),
        timestamp: 0,
        artifacts: vec![],
        protected: true,
    };
    reg.register_baseline(bl1);

    let bl2 = Baseline {
        baseline_id: "bl2".into(),
        pack_id: "pack1".into(),
        scenario_id: "s2".into(),
        timestamp: 1,
        artifacts: vec![],
        protected: false,
    };
    assert!(reg.update_baseline("pack1", bl2).is_err());
}

// ============================================================================
// CertificationVerdict tests
// ============================================================================

#[test]
fn certification_verdict_variants() {
    let verdicts = [
        CertificationVerdict::Certified,
        CertificationVerdict::ConditionalPass,
        CertificationVerdict::Blocked,
    ];
    assert_eq!(verdicts.len(), 3);
}

#[test]
fn certification_verdict_roundtrip() {
    for v in [
        CertificationVerdict::Certified,
        CertificationVerdict::ConditionalPass,
        CertificationVerdict::Blocked,
    ] {
        let json = serde_json::to_string(&v).unwrap();
        let restored: CertificationVerdict = serde_json::from_str(&json).unwrap();
        assert_eq!(v, restored);
    }
}

// ============================================================================
// CertificationEngine tests
// ============================================================================

#[test]
fn certification_engine_new() {
    let eng = CertificationEngine::new();
    assert!(!eng.is_freeze_ready("any"));
}

#[test]
fn certification_engine_default() {
    let eng = CertificationEngine::default();
    assert!(!eng.is_freeze_ready("any"));
}

#[test]
fn certification_engine_certify_green_pass() {
    let mut eng = CertificationEngine::new();
    let mut compare_eng = CompareEngine::new();
    compare_eng.create_triplet("pack1".into(), "bl1".into(), CompareMode::Exact);

    let evidence = EvidenceBundle {
        bundle_id: "b1".into(),
        pack_id: "pack1".into(),
        scenario_id: "s1".into(),
        build_profile: "Debug".into(),
        schema_revision: "v1".into(),
        timestamp: 100,
        artifacts: vec![ArtifactRef {
            artifact_id: "a1".into(),
            artifact_type: ArtifactType::FrameCapture,
            path: "/cap.png".into(),
            checksum: "c1".into(),
        }],
        verdict: EvidenceVerdict::Green,
        operator_signoff: None,
    };
    let compare_result = compare_eng.execute_compare("pack1").unwrap();

    let result = eng.certify_pack("pack1".into(), "s1".into(), &evidence, &compare_result);
    assert_eq!(result.verdict, CertificationVerdict::Certified);
    assert!(result.blockers.is_empty());
}

#[test]
fn certification_engine_red_blocked() {
    let mut eng = CertificationEngine::new();
    let mut compare_eng = CompareEngine::new();
    compare_eng.create_triplet("pack1".into(), "bl1".into(), CompareMode::Exact);

    let evidence = EvidenceBundle {
        bundle_id: "b1".into(),
        pack_id: "pack1".into(),
        scenario_id: "s1".into(),
        build_profile: "Debug".into(),
        schema_revision: "v1".into(),
        timestamp: 100,
        artifacts: vec![],
        verdict: EvidenceVerdict::Red,
        operator_signoff: None,
    };
    let compare_result = compare_eng.execute_compare("pack1").unwrap();

    let result = eng.certify_pack("pack1".into(), "s1".into(), &evidence, &compare_result);
    assert_eq!(result.verdict, CertificationVerdict::Blocked);
    assert!(!result.blockers.is_empty());
}

#[test]
fn certification_engine_is_freeze_ready() {
    let mut eng = CertificationEngine::new();
    let mut compare_eng = CompareEngine::new();
    compare_eng.create_triplet("pack1".into(), "bl1".into(), CompareMode::Exact);

    let evidence = EvidenceBundle {
        bundle_id: "b1".into(),
        pack_id: "pack1".into(),
        scenario_id: "s1".into(),
        build_profile: "Debug".into(),
        schema_revision: "v1".into(),
        timestamp: 100,
        artifacts: vec![ArtifactRef {
            artifact_id: "a1".into(),
            artifact_type: ArtifactType::FrameCapture,
            path: "/cap.png".into(),
            checksum: "c1".into(),
        }],
        verdict: EvidenceVerdict::Green,
        operator_signoff: None,
    };
    let compare_result = compare_eng.execute_compare("pack1").unwrap();
    eng.certify_pack("pack1".into(), "s1".into(), &evidence, &compare_result);

    assert!(eng.is_freeze_ready("pack1"));
    assert!(!eng.is_freeze_ready("nonexistent"));
}

#[test]
fn certification_result_serializes() {
    let result = CertificationResult {
        pack_id: "pack1".into(),
        scenario_id: "s1".into(),
        verdict: CertificationVerdict::Certified,
        blockers: vec![],
        evidence_bundle_ref: "b1".into(),
        compare_result_ref: "pack1".into(),
        timestamp: 100,
    };
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("Certified"));
}

// ============================================================================
// FreezePosture tests
// ============================================================================

#[test]
fn freeze_posture_variants() {
    let postures = [
        FreezePosture::LocalDocumentFreeze,
        FreezePosture::PartialLive,
        FreezePosture::RuntimeProven,
        FreezePosture::ProductionGold,
    ];
    assert_eq!(postures.len(), 4);
}

#[test]
fn freeze_posture_roundtrip() {
    for p in [
        FreezePosture::LocalDocumentFreeze,
        FreezePosture::PartialLive,
        FreezePosture::RuntimeProven,
        FreezePosture::ProductionGold,
    ] {
        let json = serde_json::to_string(&p).unwrap();
        let restored: FreezePosture = serde_json::from_str(&json).unwrap();
        assert_eq!(p, restored);
    }
}

// ============================================================================
// FreezeBundle tests
// ============================================================================

#[test]
fn freeze_bundle_constructs() {
    let bundle = FreezeBundle {
        freeze_id: "freeze_001".into(),
        pack_ids: vec!["pack1".into(), "pack2".into()],
        certification_refs: vec!["cert1".into()],
        build_profile: "Release".into(),
        timestamp: 0,
        operator_signoff: "ops_lead".into(),
        freeze_posture: FreezePosture::PartialLive,
    };
    assert_eq!(bundle.pack_ids.len(), 2);
}

#[test]
fn freeze_bundle_serializes() {
    let bundle = FreezeBundle {
        freeze_id: "f1".into(),
        pack_ids: vec!["p1".into()],
        certification_refs: vec![],
        build_profile: "Debug".into(),
        timestamp: 0,
        operator_signoff: "op1".into(),
        freeze_posture: FreezePosture::ProductionGold,
    };
    let json = serde_json::to_string(&bundle).unwrap();
    assert!(json.contains("ProductionGold"));
}

// ============================================================================
// FreezeEngine tests
// ============================================================================

#[test]
fn freeze_engine_new() {
    let eng = FreezeEngine::new();
    assert!(eng.get_freeze("any").is_none());
}

#[test]
fn freeze_engine_default() {
    let eng = FreezeEngine::default();
    assert!(eng.get_freeze("any").is_none());
}

#[test]
fn freeze_engine_create_freeze() {
    let mut eng = FreezeEngine::new();
    let result = eng.create_freeze(
        "freeze_001".into(),
        vec!["pack1".into()],
        vec!["cert1".into()],
        "Release".into(),
        "ops_lead".into(),
    );
    assert!(result.is_ok());
    let bundle = result.unwrap();
    assert_eq!(bundle.freeze_id, "freeze_001");
}

#[test]
fn freeze_engine_get_freeze() {
    let mut eng = FreezeEngine::new();
    eng.create_freeze(
        "f1".into(),
        vec!["p1".into()],
        vec![],
        "Debug".into(),
        "op1".into(),
    )
    .unwrap();
    let retrieved = eng.get_freeze("f1");
    assert!(retrieved.is_some());
}

#[test]
fn freeze_engine_get_nonexistent() {
    let eng = FreezeEngine::new();
    assert!(eng.get_freeze("nonexistent").is_none());
}

// ============================================================================
// FreezeBlocker tests
// ============================================================================

#[test]
fn freeze_blocker_variants() {
    let blockers = [
        FreezeBlocker::NoBaseline,
        FreezeBlocker::NoCompareDigest,
        FreezeBlocker::NoRecoverPath,
        FreezeBlocker::NoHardwareFloorResult,
        FreezeBlocker::UndocumentedFallback,
        FreezeBlocker::CertificationBlocked,
        FreezeBlocker::MissingArtifacts,
    ];
    assert_eq!(blockers.len(), 7);
}

#[test]
fn freeze_blocker_descriptions() {
    assert!(FreezeBlocker::NoBaseline.description().contains("baseline"));
    assert!(FreezeBlocker::NoCompareDigest
        .description()
        .contains("compare digest"));
    assert!(FreezeBlocker::MissingArtifacts
        .description()
        .contains("missing"));
    assert!(FreezeBlocker::CertificationBlocked
        .description()
        .contains("blocked"));
}

#[test]
fn freeze_blocker_serializes() {
    let json = serde_json::to_string(&FreezeBlocker::NoBaseline).unwrap();
    assert!(json.contains("NoBaseline"));
}

#[test]
fn freeze_blocker_roundtrip() {
    for b in [
        FreezeBlocker::NoBaseline,
        FreezeBlocker::NoCompareDigest,
        FreezeBlocker::MissingArtifacts,
    ] {
        let json = serde_json::to_string(&b).unwrap();
        let restored: FreezeBlocker = serde_json::from_str(&json).unwrap();
        assert_eq!(b, restored);
    }
}

// ============================================================================
// TexturePressureLevel tests
// ============================================================================

#[test]
fn texture_pressure_level_variants() {
    let levels = [
        TexturePressureLevel::Healthy,
        TexturePressureLevel::Elevated,
        TexturePressureLevel::Critical,
    ];
    assert_eq!(levels.len(), 3);
}

#[test]
fn texture_pressure_level_roundtrip() {
    for lvl in [
        TexturePressureLevel::Healthy,
        TexturePressureLevel::Elevated,
        TexturePressureLevel::Critical,
    ] {
        let json = serde_json::to_string(&lvl).unwrap();
        let restored: TexturePressureLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(lvl, restored);
    }
}

// ============================================================================
// HardwareFloorResult tests
// ============================================================================

#[test]
fn hardware_floor_result_new() {
    let result = HardwareFloorResult::new("pack1".into(), "scenario_a".into());
    assert_eq!(result.pack_id, "pack1");
    assert_eq!(result.frame_time_ms, 0.0);
    assert_eq!(result.memory_usage_mb, 0.0);
    assert_eq!(result.texture_pressure, TexturePressureLevel::Healthy);
}

#[test]
fn hardware_floor_result_is_within_budget() {
    let mut result = HardwareFloorResult::new("pack1".into(), "s1".into());
    result.frame_time_ms = 16.0;
    result.memory_usage_mb = 512.0;
    assert!(result.is_within_budget(16.67, 1024.0));
    assert!(!result.is_within_budget(15.0, 1024.0));
}

#[test]
fn hardware_floor_result_add_degrade_step() {
    let mut result = HardwareFloorResult::new("pack1".into(), "s1".into());
    let step = DegradeLadderStep {
        step_name: "disable_shadows".into(),
        description: "Turn off shadow rendering".into(),
        frame_time_impact_ms: -2.0,
        memory_savings_mb: 64.0,
        visual_quality_impact: "Moderate".into(),
    };
    result.add_degrade_step(step);
    assert_eq!(result.degrade_ladder.len(), 1);
}

#[test]
fn hardware_floor_result_calculate_degrade_savings() {
    let mut result = HardwareFloorResult::new("pack1".into(), "s1".into());
    result.add_degrade_step(DegradeLadderStep {
        step_name: "step1".into(),
        description: "d1".into(),
        frame_time_impact_ms: -1.0,
        memory_savings_mb: 32.0,
        visual_quality_impact: "Low".into(),
    });
    result.add_degrade_step(DegradeLadderStep {
        step_name: "step2".into(),
        description: "d2".into(),
        frame_time_impact_ms: -2.0,
        memory_savings_mb: 64.0,
        visual_quality_impact: "High".into(),
    });
    let (frame_savings, memory_savings) = result.calculate_degrade_savings();
    assert!((frame_savings - (-3.0)).abs() < 0.001);
    assert!((memory_savings - 96.0).abs() < 0.001);
}

#[test]
fn hardware_floor_result_serializes() {
    let result = HardwareFloorResult::new("pack1".into(), "s1".into());
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("pack1"));
}

// ============================================================================
// DegradeLadderStep tests
// ============================================================================

#[test]
fn degrade_ladder_step_constructs() {
    let step = DegradeLadderStep {
        step_name: "low_res_textures".into(),
        description: "Use 512x512 textures".into(),
        frame_time_impact_ms: -3.0,
        memory_savings_mb: 128.0,
        visual_quality_impact: "Moderate".into(),
    };
    assert_eq!(step.step_name, "low_res_textures");
}

#[test]
fn degrade_ladder_step_serializes() {
    let step = DegradeLadderStep {
        step_name: "step".into(),
        description: "desc".into(),
        frame_time_impact_ms: -1.0,
        memory_savings_mb: 16.0,
        visual_quality_impact: "Low".into(),
    };
    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains("step"));
}

// ============================================================================
// Waiver tests
// ============================================================================

#[test]
fn waiver_constructs() {
    let waiver = Waiver {
        waiver_id: "w1".into(),
        pack_id: "pack1".into(),
        waiver_type: WaiverType::OrangeBaselineReplacement,
        description: "Temporary waiver for orange baseline".into(),
        approved_by: "manager".into(),
        timestamp: 12345,
    };
    assert_eq!(waiver.waiver_id, "w1");
}

#[test]
fn waiver_serializes() {
    let waiver = Waiver {
        waiver_id: "w1".into(),
        pack_id: "p1".into(),
        waiver_type: WaiverType::MissingHardwareFloor,
        description: "No hardware floor test available".into(),
        approved_by: "lead".into(),
        timestamp: 0,
    };
    let json = serde_json::to_string(&waiver).unwrap();
    assert!(json.contains("MissingHardwareFloor"));
}

#[test]
fn waiver_roundtrip() {
    let waiver = Waiver {
        waiver_id: "w2".into(),
        pack_id: "p2".into(),
        waiver_type: WaiverType::ConditionalCertification,
        description: "Conditional pass".into(),
        approved_by: "qa".into(),
        timestamp: 54321,
    };
    let json = serde_json::to_string(&waiver).unwrap();
    let restored: Waiver = serde_json::from_str(&json).unwrap();
    assert_eq!(waiver, restored);
}

#[test]
fn waiver_type_variants() {
    let types = [
        WaiverType::OrangeBaselineReplacement,
        WaiverType::MissingHardwareFloor,
        WaiverType::AlternativeCompareMode,
        WaiverType::ConditionalCertification,
    ];
    assert_eq!(types.len(), 4);
}

#[test]
fn waiver_type_roundtrip() {
    for wt in [
        WaiverType::OrangeBaselineReplacement,
        WaiverType::MissingHardwareFloor,
        WaiverType::AlternativeCompareMode,
        WaiverType::ConditionalCertification,
    ] {
        let json = serde_json::to_string(&wt).unwrap();
        let restored: WaiverType = serde_json::from_str(&json).unwrap();
        assert_eq!(wt, restored);
    }
}

// ============================================================================
// FreezeGateEngine integration tests
// ============================================================================

#[test]
fn freeze_gate_engine_new_equals_default() {
    let eng1 = FreezeGateEngine::new();
    let _eng2 = FreezeGateEngine::default();
    // Both should be empty with no hardware floor results
    // Just verify they construct properly
    assert_eq!(
        eng1.check_freeze_gate(
            "",
            &BaselineRegistry::new(),
            &CompareEngine::new(),
            &CertificationEngine::new(),
            &EvidenceBundle {
                bundle_id: "".into(),
                pack_id: "".into(),
                scenario_id: "".into(),
                build_profile: "".into(),
                schema_revision: "".into(),
                timestamp: 0,
                artifacts: vec![],
                verdict: EvidenceVerdict::Green,
                operator_signoff: None,
            }
        )
        .pack_id,
        ""
    );
}

#[test]
fn freeze_gate_check_all_blockers() {
    let gate_eng = FreezeGateEngine::new();
    let baseline_reg = BaselineRegistry::new();
    let compare_eng = CompareEngine::new();
    let cert_eng = CertificationEngine::new();
    let evidence = EvidenceBundle {
        bundle_id: "b1".into(),
        pack_id: "pack1".into(),
        scenario_id: "s1".into(),
        build_profile: "Debug".into(),
        schema_revision: "v1".into(),
        timestamp: 0,
        artifacts: vec![],
        verdict: EvidenceVerdict::Green,
        operator_signoff: None,
    };

    let result =
        gate_eng.check_freeze_gate("pack1", &baseline_reg, &compare_eng, &cert_eng, &evidence);
    assert!(!result.can_freeze);
    assert!(!result.blockers.is_empty());
}

#[test]
fn freeze_gate_result_serializes() {
    let result = FreezeGateResult {
        pack_id: "pack1".into(),
        can_freeze: false,
        blockers: vec![FreezeBlocker::NoBaseline],
        warnings: vec!["No operator signoff".into()],
    };
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("NoBaseline"));
}
