// FREEZE GATING INTEGRATION TEST
// Verifies that proof-region cannot freeze without meeting all lawful conditions

use engine_world::{ReferenceRegionBootstrapper, ReferenceRegionScene, WorldState};
use tool_evidence_capture::freeze_gating::{
    DegradeLadderStep, FreezeBlocker, FreezeGateEngine, HardwareFloorResult, TexturePressureLevel,
};
use tool_evidence_capture::{
    ArtifactRef, ArtifactType, Baseline, BaselineRegistry, CertificationEngine, CompareEngine,
    CompareMode, EvidenceBundle, EvidenceVerdict,
};

#[test]
fn freeze_gate_blocks_without_baseline() {
    let gate_engine = FreezeGateEngine::new();
    let baseline_registry = BaselineRegistry::new(); // Empty - no baseline
    let compare_engine = CompareEngine::new();
    let cert_engine = CertificationEngine::new();

    let evidence = create_test_evidence();

    let result = gate_engine.check_freeze_gate(
        "pack_proof_region",
        &baseline_registry,
        &compare_engine,
        &cert_engine,
        &evidence,
    );

    assert!(!result.can_freeze, "Should not freeze without baseline");
    assert!(result.blockers.contains(&FreezeBlocker::NoBaseline));
}

#[test]
fn freeze_gate_blocks_without_compare_digest() {
    let gate_engine = FreezeGateEngine::new();
    let mut baseline_registry = BaselineRegistry::new();
    let compare_engine = CompareEngine::new(); // Empty - no compare
    let cert_engine = CertificationEngine::new();

    // Add baseline
    baseline_registry.register_baseline(create_test_baseline());

    let evidence = create_test_evidence();

    let result = gate_engine.check_freeze_gate(
        "pack_proof_region",
        &baseline_registry,
        &compare_engine,
        &cert_engine,
        &evidence,
    );

    assert!(
        !result.can_freeze,
        "Should not freeze without compare digest"
    );
    assert!(result.blockers.contains(&FreezeBlocker::NoCompareDigest));
}

#[test]
fn freeze_gate_blocks_without_hardware_floor() {
    let gate_engine = FreezeGateEngine::new(); // No hardware floor registered
    let mut baseline_registry = BaselineRegistry::new();
    let mut compare_engine = CompareEngine::new();
    let cert_engine = CertificationEngine::new();

    // Add baseline
    baseline_registry.register_baseline(create_test_baseline());

    // Add compare
    compare_engine.create_triplet(
        "pack_proof_region".to_string(),
        "baseline_proof_region".to_string(),
        CompareMode::Exact,
    );

    let evidence = create_test_evidence();

    let result = gate_engine.check_freeze_gate(
        "pack_proof_region",
        &baseline_registry,
        &compare_engine,
        &cert_engine,
        &evidence,
    );

    assert!(
        !result.can_freeze,
        "Should not freeze without hardware floor"
    );
    assert!(result
        .blockers
        .contains(&FreezeBlocker::NoHardwareFloorResult));
}

#[test]
fn freeze_gate_blocks_without_recovery_path() {
    let gate_engine = FreezeGateEngine::new();
    let mut baseline_registry = BaselineRegistry::new();
    let mut compare_engine = CompareEngine::new();
    let cert_engine = CertificationEngine::new();

    // Add baseline
    baseline_registry.register_baseline(create_test_baseline());

    // Add compare with failed run but no recovery
    let pack_id = compare_engine.create_triplet(
        "pack_proof_region".to_string(),
        "baseline_proof_region".to_string(),
        CompareMode::Exact,
    );
    compare_engine.add_failed_run(&pack_id, "failed_run_1".to_string());
    // No recovery run added!

    let evidence = create_test_evidence();

    let result = gate_engine.check_freeze_gate(
        "pack_proof_region",
        &baseline_registry,
        &compare_engine,
        &cert_engine,
        &evidence,
    );

    assert!(
        !result.can_freeze,
        "Should not freeze without recovery path"
    );
    assert!(result.blockers.contains(&FreezeBlocker::NoRecoverPath));
}

#[test]
fn freeze_gate_allows_with_all_conditions_met() {
    let mut gate_engine = FreezeGateEngine::new();
    let mut baseline_registry = BaselineRegistry::new();
    let mut compare_engine = CompareEngine::new();
    let mut cert_engine = CertificationEngine::new();

    // Add baseline
    baseline_registry.register_baseline(create_test_baseline());

    // Add compare
    let pack_id = compare_engine.create_triplet(
        "pack_proof_region".to_string(),
        "baseline_proof_region".to_string(),
        CompareMode::Exact,
    );

    // Add hardware floor
    let hw_result = create_test_hardware_floor();
    gate_engine.register_hardware_floor_result("pack_proof_region".to_string(), hw_result);

    let evidence = create_test_evidence();

    // Certify
    let compare_result = compare_engine.execute_compare(&pack_id).unwrap();
    cert_engine.certify_pack(
        "pack_proof_region".to_string(),
        "scenario_proof_region".to_string(),
        &evidence,
        &compare_result,
    );

    let result = gate_engine.check_freeze_gate(
        "pack_proof_region",
        &baseline_registry,
        &compare_engine,
        &cert_engine,
        &evidence,
    );

    assert!(result.can_freeze, "Should freeze when all conditions met");
    assert!(result.blockers.is_empty());
}

#[test]
fn freeze_gate_proof_region_full_cycle() {
    // Full cycle: bootstrap -> run -> baseline -> compare -> certify -> hardware floor -> freeze gate

    let mut gate_engine = FreezeGateEngine::new();
    let mut baseline_registry = BaselineRegistry::new();
    let mut compare_engine = CompareEngine::new();
    let mut cert_engine = CertificationEngine::new();

    // Step 1: Run proof region
    let scene = ReferenceRegionScene::create_default();
    let mut world = WorldState::new();
    let bootstrapper = ReferenceRegionBootstrapper::new(scene);
    let state = bootstrapper.bootstrap(&mut world).expect("bootstrap");

    // Execute some proof cases
    let _explosion = state.apply_explosion(&mut world, [10.0, 0.0, 10.0], 50000.0);

    // Step 2: Create baseline from successful run
    baseline_registry.register_baseline(Baseline {
        baseline_id: "baseline_proof_region_run1".to_string(),
        pack_id: "pack_proof_region".to_string(),
        scenario_id: "scenario_proof_region_full".to_string(),
        timestamp: 1000,
        artifacts: vec![ArtifactRef {
            artifact_id: "snapshot_run1".to_string(),
            artifact_type: ArtifactType::StateSnapshot,
            path: "/baselines/proof_region_run1.snapshot".to_string(),
            checksum: "run1_checksum".to_string(),
        }],
        protected: true,
    });

    // Step 3: Create compare triplet
    let pack_id = compare_engine.create_triplet(
        "pack_proof_region".to_string(),
        "baseline_proof_region_run1".to_string(),
        CompareMode::Exact,
    );

    // Step 4: Execute compare
    let compare_result = compare_engine.execute_compare(&pack_id).unwrap();

    // Step 5: Create evidence
    let evidence = EvidenceBundle {
        bundle_id: "evidence_proof_region_full".to_string(),
        pack_id: "pack_proof_region".to_string(),
        scenario_id: "scenario_proof_region_full".to_string(),
        build_profile: "desktop".to_string(),
        schema_revision: "1.0".to_string(),
        timestamp: 2000,
        artifacts: vec![ArtifactRef {
            artifact_id: "run_snapshot".to_string(),
            artifact_type: ArtifactType::StateSnapshot,
            path: "/runs/proof_region_full.snapshot".to_string(),
            checksum: "full_checksum".to_string(),
        }],
        verdict: EvidenceVerdict::Green,
        operator_signoff: Some("operator_full_cycle".to_string()),
    };

    // Step 6: Certify
    let cert_result = cert_engine.certify_pack(
        "pack_proof_region".to_string(),
        "scenario_proof_region_full".to_string(),
        &evidence,
        &compare_result,
    );
    assert_eq!(
        cert_result.verdict,
        tool_evidence_capture::CertificationVerdict::Certified
    );

    // Step 7: Register hardware floor with degrade ladder
    let mut hw_result = HardwareFloorResult::new(
        "pack_proof_region".to_string(),
        "scenario_proof_region_full".to_string(),
    );
    hw_result.frame_time_ms = 14.5;
    hw_result.memory_usage_mb = 480.0;
    hw_result.texture_pressure = TexturePressureLevel::Healthy;

    hw_result.add_degrade_step(DegradeLadderStep {
        step_name: "Reduce shadow resolution".to_string(),
        description: "2048 -> 1024".to_string(),
        frame_time_impact_ms: 1.5,
        memory_savings_mb: 30.0,
        visual_quality_impact: "Minor".to_string(),
    });

    gate_engine.register_hardware_floor_result("pack_proof_region".to_string(), hw_result);

    // Step 8: Check freeze gate
    let freeze_result = gate_engine.check_freeze_gate(
        "pack_proof_region",
        &baseline_registry,
        &compare_engine,
        &cert_engine,
        &evidence,
    );

    assert!(freeze_result.can_freeze, "Full cycle should allow freeze");
    assert!(freeze_result.blockers.is_empty());
}

#[test]
fn hardware_floor_degrade_ladder_visible() {
    let mut hw_result = HardwareFloorResult::new(
        "pack_proof_region".to_string(),
        "scenario_proof_region".to_string(),
    );

    hw_result.frame_time_ms = 18.0;
    hw_result.memory_usage_mb = 520.0;
    hw_result.texture_pressure = TexturePressureLevel::Elevated;

    // Add visible degrade steps
    hw_result.add_degrade_step(DegradeLadderStep {
        step_name: "Reduce shadow quality".to_string(),
        description: "Lower shadow map resolution".to_string(),
        frame_time_impact_ms: 1.5,
        memory_savings_mb: 20.0,
        visual_quality_impact: "Slightly softer shadows".to_string(),
    });

    hw_result.add_degrade_step(DegradeLadderStep {
        step_name: "Disable SSAO".to_string(),
        description: "Turn off screen-space ambient occlusion".to_string(),
        frame_time_impact_ms: 2.0,
        memory_savings_mb: 15.0,
        visual_quality_impact: "Less depth in corners".to_string(),
    });

    hw_result.add_degrade_step(DegradeLadderStep {
        step_name: "Reduce particle count".to_string(),
        description: "50% particle reduction".to_string(),
        frame_time_impact_ms: 1.0,
        memory_savings_mb: 10.0,
        visual_quality_impact: "Thinner smoke/fire effects".to_string(),
    });

    // Verify degrade ladder is visible and documented
    assert_eq!(hw_result.degrade_ladder.len(), 3);

    let (total_frame_savings, total_memory_savings) = hw_result.calculate_degrade_savings();
    assert_eq!(total_frame_savings, 4.5);
    assert_eq!(total_memory_savings, 45.0);

    // Verify each step has visible impact description
    for step in &hw_result.degrade_ladder {
        assert!(
            !step.visual_quality_impact.is_empty(),
            "Degrade step must have visible quality impact description"
        );
    }
}

// Helper functions

fn create_test_baseline() -> Baseline {
    Baseline {
        baseline_id: "baseline_proof_region".to_string(),
        pack_id: "pack_proof_region".to_string(),
        scenario_id: "scenario_proof_region".to_string(),
        timestamp: 1000,
        artifacts: vec![ArtifactRef {
            artifact_id: "baseline_snapshot".to_string(),
            artifact_type: ArtifactType::StateSnapshot,
            path: "/baselines/proof_region.snapshot".to_string(),
            checksum: "baseline_checksum".to_string(),
        }],
        protected: true,
    }
}

fn create_test_evidence() -> EvidenceBundle {
    EvidenceBundle {
        bundle_id: "evidence_proof_region".to_string(),
        pack_id: "pack_proof_region".to_string(),
        scenario_id: "scenario_proof_region".to_string(),
        build_profile: "desktop".to_string(),
        schema_revision: "1.0".to_string(),
        timestamp: 2000,
        artifacts: vec![ArtifactRef {
            artifact_id: "run_snapshot".to_string(),
            artifact_type: ArtifactType::StateSnapshot,
            path: "/runs/proof_region.snapshot".to_string(),
            checksum: "run_checksum".to_string(),
        }],
        verdict: EvidenceVerdict::Green,
        operator_signoff: Some("operator_test".to_string()),
    }
}

fn create_test_hardware_floor() -> HardwareFloorResult {
    let mut hw_result = HardwareFloorResult::new(
        "pack_proof_region".to_string(),
        "scenario_proof_region".to_string(),
    );

    hw_result.frame_time_ms = 15.0;
    hw_result.memory_usage_mb = 500.0;
    hw_result.texture_pressure = TexturePressureLevel::Healthy;

    hw_result.add_degrade_step(DegradeLadderStep {
        step_name: "Reduce effects".to_string(),
        description: "Lower particle count".to_string(),
        frame_time_impact_ms: 1.0,
        memory_savings_mb: 10.0,
        visual_quality_impact: "Slightly thinner effects".to_string(),
    });

    hw_result
}
