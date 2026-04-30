use engine_content::{
    ContentConfig, ContentDescriptor, ContentLocator, ContentPipeline, ContentRequest,
    ContentResult, RuntimePackId,
};
use engine_core::{EngineCoreError, EngineCoreResult};

fn request(content_id: u64, uri: &str, bytes: &[u8]) -> ContentRequest {
    ContentRequest {
        descriptor: ContentDescriptor {
            content_id,
            label: format!("asset-{content_id}"),
        },
        bytes: bytes.to_vec(),
        locator: ContentLocator {
            uri: uri.to_string(),
        },
    }
}

#[test]
fn content_pipeline_ingest_requires_mutable_owner() {
    let _: fn(&mut ContentPipeline, ContentRequest) -> EngineCoreResult<ContentResult> =
        ContentPipeline::ingest;
    let _: fn(
        &mut ContentPipeline,
        ContentRequest,
    ) -> engine_content::ContentPipelineResult<ContentResult> = ContentPipeline::try_ingest;
}

#[test]
fn content_pipeline_state_changes_are_explicit() {
    let mut pipeline = ContentPipeline::new(ContentConfig { max_packs: 4 });

    assert_eq!(pipeline.ingested_pack_count(), 0);
    let result = pipeline
        .ingest(request(1, "mem://explicit-owner", &[1, 2, 3]))
        .unwrap();

    assert_eq!(result.pack.pack_id, 1);
    assert_eq!(pipeline.ingested_pack_count(), 1);
    assert!(pipeline.has_ingested_pack(1));
}

#[test]
fn content_pipeline_duplicate_policy_still_works_after_refcell_removal() {
    let mut pipeline = ContentPipeline::new(ContentConfig { max_packs: 4 });

    pipeline
        .ingest(request(1, "mem://asset-a", &[9, 9, 9]))
        .unwrap();
    let duplicate = pipeline.ingest(request(2, "mem://asset-b", &[9, 9, 9]));

    assert_eq!(
        duplicate,
        Err(EngineCoreError::InvalidDescriptor(
            "duplicate content digest requires canonical reuse of existing pack id",
        ))
    );
}

#[test]
fn content_pipeline_dependency_manifest_still_uses_ledger_state() {
    let mut pipeline = ContentPipeline::new(ContentConfig { max_packs: 8 });

    pipeline.ingest(request(1, "mem://dep-a", &[1])).unwrap();
    pipeline.ingest(request(2, "mem://dep-b", &[2])).unwrap();

    assert_eq!(pipeline.ingested_pack_count(), 2);
    assert!(pipeline.has_ingested_pack(1));
    assert!(pipeline.has_ingested_pack(2));

    let manifest = pipeline
        .build_runtime_pack_manifest_with_dependencies(
            RuntimePackId(3),
            &[3, 3, 3],
            &[vec![3]],
            &[RuntimePackId(2), RuntimePackId(1)],
        )
        .unwrap();

    assert_eq!(
        manifest.dependency_pack_ids,
        vec![RuntimePackId(1), RuntimePackId(2)]
    );
    assert_eq!(manifest.dependency_count, 2);
    assert_eq!(pipeline.ingested_pack_count(), 2);
}
