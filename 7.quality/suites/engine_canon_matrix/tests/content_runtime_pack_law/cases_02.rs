#[test]
fn duplicate_dependencies_are_rejected_with_exact_reason() {
    let pipeline = ContentPipeline::new(ContentConfig { max_packs: 10 });

    let result = pipeline.build_runtime_pack_manifest_with_dependencies(
        RuntimePackId(4),
        &[1, 2, 3],
        &[vec![1, 2, 3]],
        &[RuntimePackId(9), RuntimePackId(9)],
    );

    assert_eq!(
        result,
        Err(EngineCoreError::InvalidDescriptor(
            "runtime pack dependencies must be unique",
        ))
    );
}

#[test]
fn same_manifest_inputs_produce_same_deterministic_digest() {
    let pipeline = ContentPipeline::new(ContentConfig { max_packs: 10 });

    let manifest1 = pipeline
        .build_runtime_pack_manifest_with_dependencies(
            RuntimePackId(1),
            &[1, 2, 3, 4, 5],
            &[vec![1, 2], vec![3, 4]],
            &[RuntimePackId(4)],
        )
        .unwrap();

    let manifest2 = pipeline
        .build_runtime_pack_manifest_with_dependencies(
            RuntimePackId(1),
            &[1, 2, 3, 4, 5],
            &[vec![1, 2], vec![3, 4]],
            &[RuntimePackId(4)],
        )
        .unwrap();

    assert_eq!(
        manifest1.deterministic_digest,
        manifest2.deterministic_digest
    );
}

#[test]
fn different_dependency_graph_changes_deterministic_digest() {
    let pipeline = ContentPipeline::new(ContentConfig { max_packs: 10 });

    let manifest1 = pipeline
        .build_runtime_pack_manifest_with_dependencies(
            RuntimePackId(1),
            &[1, 2, 3, 4, 5],
            &[vec![1, 2], vec![3, 4]],
            &[RuntimePackId(2)],
        )
        .unwrap();

    let manifest2 = pipeline
        .build_runtime_pack_manifest_with_dependencies(
            RuntimePackId(1),
            &[1, 2, 3, 4, 5],
            &[vec![1, 2], vec![3, 4]],
            &[RuntimePackId(3)],
        )
        .unwrap();

    assert_ne!(
        manifest1.deterministic_digest,
        manifest2.deterministic_digest
    );
}

