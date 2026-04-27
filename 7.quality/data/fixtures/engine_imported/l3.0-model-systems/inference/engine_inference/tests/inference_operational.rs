use engine_ecs::EcsSubstrate;
use engine_inference::{InferenceConfig, InferenceModel, InferenceRequest, InferenceService};
use engine_world::WorldState;

#[test]
fn inference_is_stable_for_same_prompt_and_model() {
    let service = InferenceService::new(
        InferenceConfig { max_batch_items: 4 },
        InferenceModel {
            model_id: "mock".to_string(),
        },
    );
    let a = service
        .infer(
            &WorldState::new(),
            &EcsSubstrate::new(),
            InferenceRequest {
                prompt: "hello".to_string(),
                batch_size: 1,
            },
        )
        .unwrap();
    let b = service
        .infer(
            &WorldState::new(),
            &EcsSubstrate::new(),
            InferenceRequest {
                prompt: "hello".to_string(),
                batch_size: 1,
            },
        )
        .unwrap();
    assert_eq!(a, b);
}
