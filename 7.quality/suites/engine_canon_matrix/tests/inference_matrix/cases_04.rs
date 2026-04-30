#[test]
fn inference_detached_case_33() {
    let s = InferenceService::new(
        InferenceConfig { max_batch_items: 8 },
        InferenceModel {
            model_id: "mock".to_string(),
        },
    );
    let r = s
        .infer_detached(InferenceRequest {
            prompt: "hello world".to_string(),
            batch_size: 2,
        })
        .unwrap();
    assert_eq!(r.tokens[0], "mock".to_string());
}
#[test]
fn inference_detached_case_34() {
    let s = InferenceService::new(
        InferenceConfig { max_batch_items: 8 },
        InferenceModel {
            model_id: "mock".to_string(),
        },
    );
    let r = s
        .infer_detached(InferenceRequest {
            prompt: "hello world".to_string(),
            batch_size: 3,
        })
        .unwrap();
    assert_eq!(r.tokens[0], "mock".to_string());
}
