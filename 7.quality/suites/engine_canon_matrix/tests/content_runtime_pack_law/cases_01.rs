#[test]
fn same_content_bytes_produce_same_content_digest() {
    let pipeline = ContentPipeline::new(ContentConfig { max_packs: 10 });

    let bytes = vec![1, 2, 3, 4, 5];
    let digest1 = pipeline.compute_content_digest(&bytes);
    let digest2 = pipeline.compute_content_digest(&bytes);

    assert_eq!(digest1, digest2);
}

#[test]
fn different_content_bytes_produce_different_digest() {
    let pipeline = ContentPipeline::new(ContentConfig { max_packs: 10 });

    let digest1 = pipeline.compute_content_digest(&[1, 2, 3]);
    let digest2 = pipeline.compute_content_digest(&[4, 5, 6]);

    assert_ne!(digest1, digest2);
}

#[test]
fn invalid_locator_is_rejected_with_exact_reason() {
    let pipeline = ContentPipeline::new(ContentConfig { max_packs: 10 });

    let result = pipeline.ingest(ContentRequest {
        descriptor: ContentDescriptor {
            content_id: 1,
            label: "asset".to_string(),
        },
        bytes: vec![1, 2, 3],
        locator: ContentLocator {
            uri: "missing-scheme".to_string(),
        },
    });

    assert_eq!(
        result,
        Err(EngineCoreError::InvalidDescriptor(
            "content locator requires explicit scheme",
        ))
    );
}

#[test]
fn duplicate_content_digest_requires_canonical_reuse() {
    let pipeline = ContentPipeline::new(ContentConfig { max_packs: 10 });

    pipeline
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 1,
                label: "asset-a".to_string(),
            },
            bytes: vec![9, 9, 9],
            locator: ContentLocator {
                uri: "mem://asset-a".to_string(),
            },
        })
        .unwrap();

    let result = pipeline.ingest(ContentRequest {
        descriptor: ContentDescriptor {
            content_id: 2,
            label: "asset-b".to_string(),
        },
        bytes: vec![9, 9, 9],
        locator: ContentLocator {
            uri: "mem://asset-b".to_string(),
        },
    });

    assert_eq!(
        result,
        Err(EngineCoreError::InvalidDescriptor(
            "duplicate content digest requires canonical reuse of existing pack id",
        ))
    );
}

#[test]
fn idempotent_reingest_of_same_resource_is_stable() {
    let pipeline = ContentPipeline::new(ContentConfig { max_packs: 10 });
    let request = ContentRequest {
        descriptor: ContentDescriptor {
            content_id: 7,
            label: "asset".to_string(),
        },
        bytes: vec![1, 2, 3, 4],
        locator: ContentLocator {
            uri: "mem://asset".to_string(),
        },
    };

    let first = pipeline.ingest(request.clone()).unwrap();
    let second = pipeline.ingest(request).unwrap();

    assert_eq!(first.pack, second.pack);
    assert_eq!(
        pipeline
            .build_runtime_pack_product(&first.manifest)
            .deterministic_digest,
        pipeline
            .build_runtime_pack_product(&second.manifest)
            .deterministic_digest
    );
}

#[test]
fn empty_content_is_rejected() {
    let pipeline = ContentPipeline::new(ContentConfig { max_packs: 10 });

    let result = pipeline.build_runtime_pack_manifest(RuntimePackId(1), &[], &[vec![1, 2, 3]]);

    assert_eq!(
        result,
        Err(EngineCoreError::InvalidDescriptor(
            "cannot build manifest from empty content",
        ))
    );
}

#[test]
fn chunk_size_zero_is_rejected() {
    let pipeline = ContentPipeline::new(ContentConfig { max_packs: 10 });

    let result = pipeline.build_runtime_pack_manifest(RuntimePackId(1), &[1, 2, 3], &[vec![]]);

    assert_eq!(
        result,
        Err(EngineCoreError::InvalidDescriptor(
            "chunk size cannot be zero"
        ))
    );
}

#[test]
fn runtime_pack_manifest_contains_sorted_chunk_digests_and_dependencies() {
    let pipeline = ContentPipeline::new(ContentConfig { max_packs: 10 });

    let manifest = pipeline
        .build_runtime_pack_manifest_with_dependencies(
            RuntimePackId(7),
            &[1, 2, 3, 4, 5],
            &[vec![5, 4, 3], vec![1, 2], vec![9, 8, 7]],
            &[RuntimePackId(9), RuntimePackId(3)],
        )
        .unwrap();

    let mut sorted_chunk_digests = manifest.chunk_digests.clone();
    sorted_chunk_digests.sort_by_key(|digest| digest.0);

    assert_eq!(manifest.chunk_digests, sorted_chunk_digests);
    assert_eq!(
        manifest.dependency_pack_ids,
        vec![RuntimePackId(3), RuntimePackId(9)]
    );
    assert_eq!(manifest.dependency_count, 2);
}

#[test]
fn self_dependency_is_rejected_with_exact_reason() {
    let pipeline = ContentPipeline::new(ContentConfig { max_packs: 10 });

    let result = pipeline.build_runtime_pack_manifest_with_dependencies(
        RuntimePackId(4),
        &[1, 2, 3],
        &[vec![1, 2, 3]],
        &[RuntimePackId(4)],
    );

    assert_eq!(
        result,
        Err(EngineCoreError::InvalidDescriptor(
            "runtime pack cannot depend on itself",
        ))
    );
}

