use engine_runtime::RuntimeProfile;
use stratumx_test_support::{canonical_demo_startup_config, BridgeConfig, EngineBridgeHarness};
use stratumx_test_support::{
    ApprovalClass, BudgetClass, CommandOrigin, ObjectClass, ToolCommand, ToolingRuntime,
};

pub fn run_sdk_tooling_case(case: usize) {
    let profile = match case % 3 {
        0 => RuntimeProfile::Interactive60,
        1 => RuntimeProfile::ListenHost60,
        _ => RuntimeProfile::Headless20,
    };
    let startup = canonical_demo_startup_config(profile);
    let mut harness =
        EngineBridgeHarness::new(startup, BridgeConfig::default()).expect("stack harness");
    let (_world, _snapshot) = harness
        .bootstrap_demo_world(
            (case % 2) + 1,
            if case.checked_rem(2) == Some(0) { 1 } else { 2 },
        )
        .expect("world snapshot");
    let mut tooling = ToolingRuntime::with_bridge(harness.into_bridge());
    let class = match case % 5 {
        0 => ObjectClass::World,
        1 => ObjectClass::Scene,
        2 => ObjectClass::Material,
        3 => ObjectClass::Terrain,
        _ => ObjectClass::Logic,
    };
    let handle = tooling
        .create_object(format!("linked-{case}"), class)
        .expect("create object");
    tooling
        .apply_command(
            ToolCommand::UpsertField {
                handle,
                key: "family".into(),
                value: format!("family-{case}"),
            },
            CommandOrigin::User,
            ApprovalClass::None,
            BudgetClass::Interactive,
        )
        .expect("family field");
    if case.checked_rem(2) == Some(0) {
        tooling
            .apply_command(
                ToolCommand::AddTag {
                    handle,
                    tag: "linked".into(),
                },
                CommandOrigin::Automation,
                ApprovalClass::None,
                BudgetClass::Background,
            )
            .expect("tag add");
    }
    let diagnostics = tooling.validate_snapshot();
    assert!(diagnostics.iter().all(|d| !d.message.contains("label")));
    let preview = tooling.preview_object(handle).expect("preview");
    let build = tooling.build_current();
    let release = tooling
        .release_build(&build, "dev", true)
        .expect("release build");
    assert_eq!(preview.handle, handle);
    assert!(build.object_count >= 2);
    assert_eq!(release.build_digest, build.digest);
    assert!(tooling.snapshot().generation > 0);
}

// __stratumx_ready_common_smoke_test__
#[test]
fn common_smoke() {
    run_sdk_tooling_case(0);
}
