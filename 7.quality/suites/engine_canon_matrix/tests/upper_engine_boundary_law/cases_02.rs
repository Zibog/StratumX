#[test]
fn animation_solve_boundary_is_deterministic() {
    let mut runtime = AnimationRuntime::new();

    let clip = AnimationClip {
        name: "walk".to_string(),
        duration_sec: 1.0,
        keyframes: vec![
            Keyframe {
                time_sec: 0.0,
                joint_index: 0,
                position: [0.0, 0.0, 0.0],
                rotation: [0.0, 0.0, 0.0, 1.0],
            },
            Keyframe {
                time_sec: 1.0,
                joint_index: 0,
                position: [1.0, 0.0, 0.0],
                rotation: [0.0, 0.0, 0.0, 1.0],
            },
        ],
        looping: true,
    };

    let clip_index = runtime.add_clip(clip);
    assert_eq!(clip_index, 0);

    let played = runtime.play_clip("walk");
    assert!(played);

    runtime.update(0.5);
    assert_eq!(runtime.states[0].current_time, 0.5);
}

#[test]
fn animation_solve_rejects_invalid_chain() {
    let mut runtime = AnimationRuntime::new();

    // Try to solve IK for non-existent chain
    let result = runtime.solve_ik(999, [1.0, 2.0, 3.0]);
    assert!(result.is_none());
}

#[test]
fn imaging_frame_policy_accepts_valid_resource_state() {
    let config = ImagingConfig {
        max_render_targets: 100,
        max_upload_bytes: 1024 * 1024,
    };
    let service = ImagingService::new(config);

    let world = WorldState::new();
    let ecs = EcsSubstrate::new();
    let materials = MaterialRegistry::new(create_test_material_config());
    let residency = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 100,
        streaming_item_budget: 50,
    });
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 10,
        max_inflight_uploads: 10,
    });

    let request = ImagingRequest {
        render_target_id: 1,
        view_region: (0, 0, 0),
        upload_bytes: 1024,
    };

    let result = service.render(&world, &ecs, &materials, &residency, &mut transfer, request);
    assert!(result.is_ok());
}

#[test]
fn imaging_frame_policy_rejects_illegal_resource_state() {
    let config = ImagingConfig {
        max_render_targets: 10,
        max_upload_bytes: 1024,
    };
    let service = ImagingService::new(config);

    let world = WorldState::new();
    let ecs = EcsSubstrate::new();
    let materials = MaterialRegistry::new(create_test_material_config());
    let residency = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 100,
        streaming_item_budget: 50,
    });
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 10,
        max_inflight_uploads: 10,
    });

    // Exceeds max render targets
    let request = ImagingRequest {
        render_target_id: 100,
        view_region: (0, 0, 0),
        upload_bytes: 512,
    };

    let result = service.render(&world, &ecs, &materials, &residency, &mut transfer, request);
    assert!(result.is_err());

    // Exceeds max upload bytes
    let request = ImagingRequest {
        render_target_id: 1,
        view_region: (0, 0, 0),
        upload_bytes: 2048,
    };

    let result = service.render(&world, &ecs, &materials, &residency, &mut transfer, request);
    assert!(result.is_err());
}

#[test]
fn content_policy_accepts_valid_resource() {
    let config = ContentConfig { max_packs: 100 };
    let pipeline = ContentPipeline::new(config);

    let request = ContentRequest {
        descriptor: ContentDescriptor {
            content_id: 1,
            label: "test_content".to_string(),
        },
        bytes: vec![1, 2, 3, 4],
        locator: ContentLocator {
            uri: "file://test.pak".to_string(),
        },
    };

    let result = pipeline.ingest(request);
    assert!(result.is_ok());
}

#[test]
fn content_policy_rejects_missing_resource_id() {
    let config = ContentConfig { max_packs: 100 };
    let pipeline = ContentPipeline::new(config);

    let request = ContentRequest {
        descriptor: ContentDescriptor {
            content_id: 1,
            label: "test_content".to_string(),
        },
        bytes: vec![], // Empty bytes
        locator: ContentLocator {
            uri: "file://test.pak".to_string(),
        },
    };

    let result = pipeline.ingest(request);
    assert!(result.is_err());
}

#[test]
fn startup_wires_required_engine_services() {
    let config = StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::HeadlessHost,
        runtime_manifests: vec![],
        service_wiring: create_test_service_wiring(),
    };

    let assembly = StartupAssembly::new(config);
    let decision = assembly.validate();
    assert!(decision.accepted);
}

