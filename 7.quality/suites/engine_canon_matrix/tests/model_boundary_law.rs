use engine_core::EngineCoreError;
use engine_generation::{
    GenerationConfig, GenerationContext, GenerationRequest, GenerationService,
    ModelBoundaryFailure as GenerationFailure,
    ModelBoundaryFailureReason as GenerationFailureReason, ModelDescriptor, ModelWeights,
};
use engine_inference::{
    InferenceConfig, InferenceModel, InferenceRequest, InferenceService,
    ModelBoundaryFailure as InferenceFailure, ModelBoundaryFailureReason as InferenceFailureReason,
};

fn inference(model_id: &str) -> InferenceService {
    InferenceService::new(
        InferenceConfig { max_batch_items: 4 },
        InferenceModel {
            model_id: model_id.to_string(),
        },
    )
}

fn generation(model_family: &str, checksum: &str) -> GenerationService {
    GenerationService::new(
        GenerationConfig {
            max_output_chars: 256,
        },
        GenerationContext {
            descriptor: ModelDescriptor {
                model_family: model_family.to_string(),
            },
            weights: ModelWeights {
                checksum: checksum.to_string(),
            },
        },
    )
}

#[test]
fn generation_public_api_survives_module_split() {
    let world = engine_world::WorldState::new();
    let inference = inference("test");
    let generation = generation("gen", "abc");
    let result = generation
        .generate(
            &world,
            &inference,
            GenerationRequest {
                prompt: "hello world".to_string(),
            },
        )
        .unwrap();

    assert!(result.output.artifact.contains("gen:abc"));
}

#[test]
fn inference_public_api_survives_module_split() {
    let world = engine_world::WorldState::new();
    let ecs = engine_ecs::EcsSubstrate::new();
    let inference = inference("test");
    let result = inference
        .infer(
            &world,
            &ecs,
            InferenceRequest {
                prompt: "hello world".to_string(),
                batch_size: 1,
            },
        )
        .unwrap();

    assert_eq!(result.tokens[0], "test");
}

#[test]
fn generation_same_input_same_digest() {
    let inference = inference("test");
    let generation = generation("gen", "abc");
    let request = GenerationRequest {
        prompt: "hello world".to_string(),
    };

    let receipt1 = generation
        .generate_with_receipt(&inference, request.clone())
        .unwrap();
    let receipt2 = generation
        .generate_with_receipt(&inference, request)
        .unwrap();

    assert_eq!(receipt1.deterministic_digest, receipt2.deterministic_digest);
}

#[test]
fn inference_same_input_same_digest() {
    let inference = inference("test");
    let request = InferenceRequest {
        prompt: "hello world".to_string(),
        batch_size: 1,
    };

    let receipt1 = inference.infer_with_receipt(request.clone()).unwrap();
    let receipt2 = inference.infer_with_receipt(request).unwrap();

    assert_eq!(receipt1.deterministic_digest, receipt2.deterministic_digest);
}

#[test]
fn generation_policy_change_changes_digest() {
    let inference = inference("test");
    let request = GenerationRequest {
        prompt: "hello".to_string(),
    };
    let receipt_a = generation("gen-a", "abc")
        .generate_with_receipt(&inference, request.clone())
        .unwrap();
    let receipt_b = generation("gen-b", "abc")
        .generate_with_receipt(&inference, request)
        .unwrap();

    assert_ne!(receipt_a.model_policy_id, receipt_b.model_policy_id);
    assert_ne!(
        receipt_a.deterministic_digest,
        receipt_b.deterministic_digest
    );
}

#[test]
fn inference_policy_change_changes_digest() {
    let request = InferenceRequest {
        prompt: "hello".to_string(),
        batch_size: 1,
    };
    let receipt_a = inference("test-a")
        .infer_with_receipt(request.clone())
        .unwrap();
    let receipt_b = inference("test-b").infer_with_receipt(request).unwrap();

    assert_ne!(receipt_a.model_policy_id, receipt_b.model_policy_id);
    assert_ne!(
        receipt_a.deterministic_digest,
        receipt_b.deterministic_digest
    );
}

#[test]
fn generation_failure_reason_is_typed_primary_api() {
    let inference = inference("test");
    let result = generation("", "abc").generate_with_receipt(
        &inference,
        GenerationRequest {
            prompt: "hello".to_string(),
        },
    );

    assert_eq!(
        result.unwrap_err().reason,
        GenerationFailureReason::MissingModelFamily
    );
}

#[test]
fn inference_failure_reason_is_typed_primary_api() {
    let result = inference("").infer_with_receipt(InferenceRequest {
        prompt: "hello".to_string(),
        batch_size: 1,
    });

    assert_eq!(
        result.unwrap_err().reason,
        InferenceFailureReason::MissingModelId
    );
}

#[test]
fn inference_missing_model_is_rejected_with_exact_reason() {
    let result = inference("").infer_with_receipt(InferenceRequest {
        prompt: "hello".to_string(),
        batch_size: 1,
    });

    assert_eq!(
        result.unwrap_err().reason,
        InferenceFailureReason::MissingModelId
    );
}

#[test]
fn legacy_engine_core_error_bridge_preserves_reason_text() {
    let generation_bridge: EngineCoreError =
        GenerationFailure::for_reason(GenerationFailureReason::MissingWeightsChecksum).into();
    let inference_bridge: EngineCoreError =
        InferenceFailure::for_reason(InferenceFailureReason::InvalidBatchSize).into();

    assert_eq!(
        generation_bridge,
        EngineCoreError::InvalidDescriptor("generation context requires weights checksum")
    );
    assert_eq!(
        inference_bridge,
        EngineCoreError::InvalidDescriptor(
            "inference batch size is illegal for configured ceiling"
        )
    );
}
