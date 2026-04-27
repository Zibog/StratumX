use engine_content::{
    ContentConfig, ContentDescriptor, ContentLocator, ContentPipeline, ContentRequest,
};

#[test]
fn content_pipeline_builds_runtime_pack_products() {
    let pipeline = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let result = pipeline
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 1,
                label: "asset".to_string(),
            },
            bytes: vec![1, 2, 3],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let product = pipeline.build_runtime_pack_product(&result.manifest);
    assert_eq!(product.pack_ids, vec![1]);
}
