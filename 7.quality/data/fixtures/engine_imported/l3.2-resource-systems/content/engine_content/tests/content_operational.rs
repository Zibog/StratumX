use engine_content::{
    ContentConfig, ContentDescriptor, ContentLocator, ContentPipeline, ContentRequest,
};

#[test]
fn runtime_pack_product_is_stable_for_same_manifest() {
    let pipeline = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let result = pipeline
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 7,
                label: "asset".to_string(),
            },
            bytes: vec![1, 2, 3],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let a = pipeline.build_runtime_pack_product(&result.manifest);
    let b = pipeline.build_runtime_pack_product(&result.manifest);
    assert_eq!(a, b);
}
