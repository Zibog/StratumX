#[test]
fn content_ingest_case_20() {
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 21,
                label: "asset".to_string(),
            },
            bytes: vec![1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 21);
}
#[test]
fn content_ingest_case_21() {
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 22,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 22);
}
#[test]
fn content_ingest_case_22() {
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 23,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 23);
}
#[test]
fn content_ingest_case_23() {
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 24,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1, 1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 24);
}
#[test]
fn content_ingest_case_24() {
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 25,
                label: "asset".to_string(),
            },
            bytes: vec![1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 25);
}
#[test]
fn content_ingest_case_25() {
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 26,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 26);
}
#[test]
fn content_ingest_case_26() {
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 27,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 27);
}
#[test]
fn content_ingest_case_27() {
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 28,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1, 1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 28);
}
#[test]
fn content_ingest_case_28() {
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 29,
                label: "asset".to_string(),
            },
            bytes: vec![1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 29);
}
