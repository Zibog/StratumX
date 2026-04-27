use engine_generation::{
    GenerationConfig, GenerationContext, GenerationRequest, GenerationService, ModelDescriptor,
    ModelWeights,
};
use engine_inference::{InferenceConfig, InferenceModel, InferenceService};
use engine_world::WorldState;

#[test]
fn generation_service_wraps_inference_output() {
    let inference = InferenceService::new(
        InferenceConfig { max_batch_items: 4 },
        InferenceModel {
            model_id: "mock".to_string(),
        },
    );
    let generation = GenerationService::new(
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
    let result = generation
        .generate(
            &WorldState::new(),
            &inference,
            GenerationRequest {
                prompt: "hello".to_string(),
            },
        )
        .unwrap();
    assert!(result.output.artifact.contains("gen:abc"));
}
