use engine_ecs::EcsSubstrate;
use engine_inference::{InferenceConfig, InferenceModel, InferenceRequest, InferenceService};
use engine_world::WorldState;

#[test]
fn inference_service_returns_tokenized_output() {
    let service = InferenceService::new(
        InferenceConfig { max_batch_items: 4 },
        InferenceModel {
            model_id: "mock".to_string(),
        },
    );
    let result = service
        .infer(
            &WorldState::new(),
            &EcsSubstrate::new(),
            InferenceRequest {
                prompt: "hello world".to_string(),
                batch_size: 1,
            },
        )
        .unwrap();
    assert!(result.tokens.len() >= 3);
}
