#![allow(unused_imports)]
use super::*;

#[test]
fn infer_rejects_zero_batch_size() {
    let s = InferenceService::new(
        InferenceConfig { max_batch_items: 4 },
        InferenceModel {
            model_id: "mock".to_string(),
        },
    );
    assert!(s
        .infer_detached(InferenceRequest {
            prompt: "hello".to_string(),
            batch_size: 0
        })
        .is_err());
}
#[test]
fn infer_rejects_batch_over_limit() {
    let s = InferenceService::new(
        InferenceConfig { max_batch_items: 1 },
        InferenceModel {
            model_id: "mock".to_string(),
        },
    );
    assert!(s
        .infer_detached(InferenceRequest {
            prompt: "hello".to_string(),
            batch_size: 2
        })
        .is_err());
}
#[test]
fn infer_detached_includes_model_id_token() {
    let s = InferenceService::new(
        InferenceConfig { max_batch_items: 4 },
        InferenceModel {
            model_id: "mock".to_string(),
        },
    );
    assert_eq!(
        s.infer_detached(InferenceRequest {
            prompt: "hello".to_string(),
            batch_size: 1
        })
        .unwrap()
        .tokens[0],
        "mock".to_string()
    );
}
#[test]
fn infer_splits_prompt_words() {
    let s = InferenceService::new(
        InferenceConfig { max_batch_items: 4 },
        InferenceModel {
            model_id: "mock".to_string(),
        },
    );
    assert!(s
        .infer_detached(InferenceRequest {
            prompt: "hello world".to_string(),
            batch_size: 1
        })
        .unwrap()
        .tokens
        .contains(&"world".to_string()));
}
#[test]
fn infer_delegates_from_attached_variant() {
    let s = InferenceService::new(
        InferenceConfig { max_batch_items: 4 },
        InferenceModel {
            model_id: "mock".to_string(),
        },
    );
    assert!(s
        .infer(
            &engine_world::WorldState::new(),
            &engine_ecs::EcsSubstrate::new(),
            InferenceRequest {
                prompt: "hello".to_string(),
                batch_size: 1
            }
        )
        .is_ok());
}
