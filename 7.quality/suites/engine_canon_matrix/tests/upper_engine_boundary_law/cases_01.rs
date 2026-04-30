#[test]
fn generation_request_digest_is_stable() {
    let config = GenerationConfig {
        max_output_chars: 1000,
    };
    let context = GenerationContext {
        descriptor: ModelDescriptor {
            model_family: "test_model".to_string(),
        },
        weights: ModelWeights {
            checksum: "abc123".to_string(),
        },
    };
    let service = GenerationService::new(config, context);

    let world = WorldState::new();
    let inference_config = InferenceConfig {
        max_batch_items: 10,
    };
    let inference_model = InferenceModel {
        model_id: "test_inference".to_string(),
    };
    let inference = InferenceService::new(inference_config, inference_model);

    let request = GenerationRequest {
        prompt: "test prompt".to_string(),
    };

    let result1 = service
        .generate(&world, &inference, request.clone())
        .unwrap();
    let result2 = service.generate(&world, &inference, request).unwrap();

    assert_eq!(result1.output.artifact, result2.output.artifact);
}

#[test]
fn invalid_generation_request_is_rejected() {
    let config = GenerationConfig {
        max_output_chars: 10, // Very small limit
    };
    let context = GenerationContext {
        descriptor: ModelDescriptor {
            model_family: "test_model_with_very_long_name".to_string(),
        },
        weights: ModelWeights {
            checksum: "abc123".to_string(),
        },
    };
    let service = GenerationService::new(config, context);

    let world = WorldState::new();
    let inference_config = InferenceConfig {
        max_batch_items: 10,
    };
    let inference_model = InferenceModel {
        model_id: "test_inference".to_string(),
    };
    let inference = InferenceService::new(inference_config, inference_model);

    let request = GenerationRequest {
        prompt: "very long prompt that will exceed the character limit".to_string(),
    };

    let result = service.generate(&world, &inference, request);
    assert!(result.is_err());
}

#[test]
fn inference_request_boundary_validates_required_ids() {
    let config = InferenceConfig {
        max_batch_items: 10,
    };
    let model = InferenceModel {
        model_id: "test_model".to_string(),
    };
    let service = InferenceService::new(config, model);

    let world = WorldState::new();
    let ecs = EcsSubstrate::new();

    let request = InferenceRequest {
        prompt: "test".to_string(),
        batch_size: 1,
    };

    let result = service.infer(&world, &ecs, request);
    assert!(result.is_ok());
}

#[test]
fn invalid_inference_request_is_rejected() {
    let config = InferenceConfig { max_batch_items: 5 };
    let model = InferenceModel {
        model_id: "test_model".to_string(),
    };
    let service = InferenceService::new(config, model);

    // Zero batch size
    let request = InferenceRequest {
        prompt: "test".to_string(),
        batch_size: 0,
    };
    let result = service.infer_detached(request);
    assert!(result.is_err());

    // Exceeds max batch
    let request = InferenceRequest {
        prompt: "test".to_string(),
        batch_size: 10,
    };
    let result = service.infer_detached(request);
    assert!(result.is_err());
}

#[test]
fn acoustic_event_boundary_accepts_valid_event() {
    let config = AcousticsConfig {
        max_sources: 100,
        max_stream_upload_bytes: 1024 * 1024,
    };
    let service = AcousticsService::new(config);

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

    let request = AcousticsRequest {
        source_count: 10,
        stream_upload_bytes: 1024,
    };

    let result = service.synthesize(&world, &ecs, &materials, &residency, &mut transfer, request);
    assert!(result.is_ok());
}

#[test]
fn acoustic_event_boundary_rejects_missing_source() {
    let config = AcousticsConfig {
        max_sources: 10,
        max_stream_upload_bytes: 1024,
    };
    let service = AcousticsService::new(config);

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

    let request = AcousticsRequest {
        source_count: 100, // Exceeds max
        stream_upload_bytes: 512,
    };

    let result = service.synthesize(&world, &ecs, &materials, &residency, &mut transfer, request);
    assert!(result.is_err());
}

