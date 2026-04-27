use engine_generation::{
    GenerationConfig, GenerationContext, GenerationRequest, GenerationService, ModelDescriptor,
    ModelWeights,
};
use engine_inference::{InferenceConfig, InferenceModel, InferenceService};
use engine_world::WorldState;

fn inference() -> InferenceService {
    InferenceService::new(
        InferenceConfig { max_batch_items: 4 },
        InferenceModel {
            model_id: "mock".to_string(),
        },
    )
}

#[test]
fn generation_rejects_artifact_that_exceeds_ceiling() {
    let service = GenerationService::new(
        GenerationConfig {
            max_output_chars: 4,
        },
        GenerationContext {
            descriptor: ModelDescriptor {
                model_family: "gen".to_string(),
            },
            weights: ModelWeights {
                checksum: "abc".to_string(),
            },
        },
    );
    let result = service.generate(
        &WorldState::new(),
        &inference(),
        GenerationRequest {
            prompt: "long prompt".to_string(),
        },
    );
    assert!(result.is_err());
}

#[test]
fn generation_artifact_contains_family_and_checksum() {
    let service = GenerationService::new(
        GenerationConfig {
            max_output_chars: 256,
        },
        GenerationContext {
            descriptor: ModelDescriptor {
                model_family: "gen".to_string(),
            },
            weights: ModelWeights {
                checksum: "abc".to_string(),
            },
        },
    );
    let result = service
        .generate(
            &WorldState::new(),
            &inference(),
            GenerationRequest {
                prompt: "hello".to_string(),
            },
        )
        .unwrap();
    assert!(result.output.artifact.starts_with("gen:abc:"));
}
