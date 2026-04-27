use engine_ecs::EcsSubstrate;
use engine_inference::{InferenceConfig, InferenceModel, InferenceRequest, InferenceService};
use engine_world::WorldState;

#[test]
fn inference_rejects_illegal_batch_sizes() {
    let service = InferenceService::new(
        InferenceConfig { max_batch_items: 2 },
        InferenceModel {
            model_id: "mock".to_string(),
        },
    );
    assert!(service
        .infer(
            &WorldState::new(),
            &EcsSubstrate::new(),
            InferenceRequest {
                prompt: "hello".to_string(),
                batch_size: 0
            }
        )
        .is_err());
    assert!(service
        .infer_detached(InferenceRequest {
            prompt: "hello".to_string(),
            batch_size: 3
        })
        .is_err());
}

#[test]
fn detached_inference_prefixes_model_id() {
    let service = InferenceService::new(
        InferenceConfig { max_batch_items: 2 },
        InferenceModel {
            model_id: "mock".to_string(),
        },
    );
    let result = service
        .infer_detached(InferenceRequest {
            prompt: "hello".to_string(),
            batch_size: 1,
        })
        .unwrap();
    assert_eq!(result.tokens[0], "mock");
}
