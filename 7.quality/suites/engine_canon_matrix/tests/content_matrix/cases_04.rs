#[test]
fn content_ingest_case_29() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 30,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 30);
}

