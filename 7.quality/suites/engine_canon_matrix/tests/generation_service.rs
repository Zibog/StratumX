// Generation Service Tests

use engine_generation::{
    GenerationConfig, GenerationContext, GenerationRequest, GenerationService, ModelDescriptor,
    ModelWeights,
};
use engine_inference::{InferenceConfig, InferenceModel, InferenceService};
use engine_world::WorldState;

#[test]
fn test_generation_service() {
    let context = GenerationContext {
        descriptor: ModelDescriptor {
            model_family: "family".into(),
        },
        weights: ModelWeights {
            checksum: "abc".into(),
        },
    };
    let generation = GenerationService::new(
        GenerationConfig {
            max_output_chars: 1000,
        },
        context,
    );
    let inference = InferenceService::new(
        InferenceConfig { max_batch_items: 4 },
        InferenceModel {
            model_id: "m".into(),
        },
    );
    let result = generation
        .generate(
            &WorldState::new(),
            &inference,
            GenerationRequest {
                prompt: "demo".into(),
            },
        )
        .expect("generate");
    assert!(result.output.artifact.contains("family:abc"));
}
