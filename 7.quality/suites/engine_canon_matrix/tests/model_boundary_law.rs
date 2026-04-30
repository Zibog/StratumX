use engine_core::EngineCoreError;
use engine_generation::{
    GenerationConfig, GenerationContext, GenerationRequest, GenerationService, ModelDescriptor,
    ModelWeights,
};
use engine_inference::{InferenceConfig, InferenceModel, InferenceRequest, InferenceService};

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

    let gen_a = generation("gen-a", "abc");
    let gen_b = generation("gen-b", "abc");

    let receipt_a = gen_a
        .generate_with_receipt(&inference, request.clone())
        .unwrap();
    let receipt_b = gen_b.generate_with_receipt(&inference, request).unwrap();

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

    let model_a = inference("test-a");
    let model_b = inference("test-b");

    let receipt_a = model_a.infer_with_receipt(request.clone()).unwrap();
    let receipt_b = model_b.infer_with_receipt(request).unwrap();

    assert_ne!(receipt_a.model_policy_id, receipt_b.model_policy_id);
    assert_ne!(
        receipt_a.deterministic_digest,
        receipt_b.deterministic_digest
    );
}

#[test]
fn generation_missing_model_family_is_rejected_with_exact_reason() {
    let inference = inference("test");
    let generation = generation("", "abc");

    let result = generation.generate_with_receipt(
        &inference,
        GenerationRequest {
            prompt: "hello".to_string(),
        },
    );

    assert_eq!(
        result,
        Err(EngineCoreError::InvalidDescriptor(
            "generation context requires model family",
        ))
    );
}

#[test]
fn generation_missing_weights_are_rejected_with_exact_reason() {
    let inference = inference("test");
    let generation = generation("gen", "");

    let result = generation.generate_with_receipt(
        &inference,
        GenerationRequest {
            prompt: "hello".to_string(),
        },
    );

    assert_eq!(
        result,
        Err(EngineCoreError::InvalidDescriptor(
            "generation context requires weights checksum",
        ))
    );
}

#[test]
fn inference_missing_model_is_rejected_with_exact_reason() {
    let inference = inference("");

    let result = inference.infer_with_receipt(InferenceRequest {
        prompt: "hello".to_string(),
        batch_size: 1,
    });

    assert_eq!(
        result,
        Err(EngineCoreError::InvalidDescriptor(
            "inference service requires non-empty model id",
        ))
    );
}

#[test]
fn inference_empty_prompt_is_rejected_with_exact_reason() {
    let inference = inference("test");

    let result = inference.infer_with_receipt(InferenceRequest {
        prompt: "   ".to_string(),
        batch_size: 1,
    });

    assert_eq!(
        result,
        Err(EngineCoreError::InvalidDescriptor(
            "inference request requires non-empty prompt",
        ))
    );
}
