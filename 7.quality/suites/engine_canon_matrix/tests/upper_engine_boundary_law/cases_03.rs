#[test]
fn startup_rejects_missing_required_service() {
    let config = StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::InteractiveHostAware, // Invalid combination
        runtime_manifests: vec![],
        service_wiring: create_test_service_wiring(),
    };

    let assembly = StartupAssembly::new(config);
    let decision = assembly.validate();
    assert!(!decision.accepted);
    assert!(!decision.reasons.is_empty());
}

#[test]
fn startup_launch_plan_validates_profile_role_combination() {
    let config = StartupConfig {
        profile: RuntimeProfile::Interactive60,
        network_role: NetworkRole::HeadlessHost, // Invalid
        runtime_manifests: vec![],
        service_wiring: create_test_service_wiring(),
    };

    let assembly = StartupAssembly::new(config);
    let result = assembly.runtime_launch_plan();
    assert!(result.is_err());
}

#[test]
fn startup_headless_launch_requires_headless_profile() {
    let config = StartupConfig {
        profile: RuntimeProfile::Interactive60, // Wrong profile
        network_role: NetworkRole::InteractiveHostAware,
        runtime_manifests: vec![],
        service_wiring: create_test_service_wiring(),
    };

    let assembly = StartupAssembly::new(config);
    let world = WorldState::new();
    let result = assembly.launch_headless(world);
    assert!(result.is_err());
}

#[test]
fn startup_realtime_launch_rejects_headless_profile() {
    let config = StartupConfig {
        profile: RuntimeProfile::Headless20, // Wrong profile
        network_role: NetworkRole::HeadlessHost,
        runtime_manifests: vec![],
        service_wiring: create_test_service_wiring(),
    };

    let assembly = StartupAssembly::new(config);
    let world = WorldState::new();
    let result = assembly.launch_realtime(world);
    assert!(result.is_err());
}

#[test]
fn content_runtime_pack_product_deterministic() {
    let config = ContentConfig { max_packs: 100 };
    let mut pipeline = ContentPipeline::new(config);

    let request = ContentRequest {
        descriptor: ContentDescriptor {
            content_id: 1,
            label: "test".to_string(),
        },
        bytes: vec![1, 2, 3],
        locator: ContentLocator {
            uri: "file://test.pak".to_string(),
        },
    };

    let result = pipeline.ingest(request).unwrap();
    let product1 = pipeline.build_runtime_pack_product(&result.manifest);
    let product2 = pipeline.build_runtime_pack_product(&result.manifest);

    assert_eq!(product1.pack_ids, product2.pack_ids);
}
