use engine_content::{
    ContentConfig, ContentDescriptor, ContentLocator, ContentPipeline, ContentRequest,
};

#[test]
fn content_ingest_requires_non_empty_bytes() {
    let pipeline = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let result = pipeline.ingest(ContentRequest {
        descriptor: ContentDescriptor {
            content_id: 1,
            label: "asset".to_string(),
        },
        bytes: vec![],
        locator: ContentLocator {
            uri: "mem://asset".to_string(),
        },
    });
    assert!(result.is_err());
}

#[test]
fn content_ingest_respects_pack_ceiling() {
    let pipeline = ContentPipeline::new(ContentConfig { max_packs: 0 });
    let result = pipeline.ingest(ContentRequest {
        descriptor: ContentDescriptor {
            content_id: 1,
            label: "asset".to_string(),
        },
        bytes: vec![1],
        locator: ContentLocator {
            uri: "mem://asset".to_string(),
        },
    });
    assert!(result.is_err());
}
