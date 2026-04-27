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
fn generation_is_stable_for_same_prompt() {
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
    let a = service
        .generate(
            &WorldState::new(),
            &inference(),
            GenerationRequest {
                prompt: "hello".to_string(),
            },
        )
        .unwrap();
    let b = service
        .generate(
            &WorldState::new(),
            &inference(),
            GenerationRequest {
                prompt: "hello".to_string(),
            },
        )
        .unwrap();
    assert_eq!(a, b);
}
