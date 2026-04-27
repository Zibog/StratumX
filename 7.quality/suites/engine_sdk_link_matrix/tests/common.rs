use engine_runtime::RuntimeProfile;
use stratumx_test_support::{
    canonical_demo_startup_config, BridgeConfig, Capability, CompatibilityProfile,
    CompatibilityVerdict, EngineBridgeHarness,
};

pub fn run_engine_sdk_case(case: usize) {
    let profile = match case % 3 {
        0 => RuntimeProfile::Interactive60,
        1 => RuntimeProfile::ListenHost60,
        _ => RuntimeProfile::Headless20,
    };
    let startup = canonical_demo_startup_config(profile);
    let mut harness =
        EngineBridgeHarness::new(startup, BridgeConfig::default()).expect("engine-sdk harness");
    let segments = (case % 2) + 1;
    let passes = if case.checked_rem(2) == Some(0) { 1 } else { 2 };
    let (_world, snapshot) = harness
        .bootstrap_demo_world(segments, passes)
        .expect("bootstrap demo world");
    let projection = harness.projection();
    assert!(snapshot.object_count >= 1);
    assert_eq!(snapshot.epoch, projection.snapshot_epoch);
    assert!(projection.launch_plan.runtime_pack_count >= 1);
    let batch = harness.bridge().read_observation_batch(0, 32);
    assert!(!batch.records.is_empty());
    let verdict = harness.bridge().compatibility_verdict(
        BridgeConfig::default().version,
        &[
            Capability::Snapshots,
            Capability::Observations,
            Capability::Metrics,
            Capability::Controls,
            Capability::ArtifactRefs,
        ],
        match case % 4 {
            0 => CompatibilityProfile::ToolRuntime,
            1 => CompatibilityProfile::EditorSurface,
            2 => CompatibilityProfile::Automation,
            _ => CompatibilityProfile::Diagnostics,
        },
    );
    assert!(matches!(verdict, CompatibilityVerdict::Compatible));
}

// __stratumx_ready_common_smoke_test__
#[test]
fn common_smoke() {
    run_engine_sdk_case(0);
}
