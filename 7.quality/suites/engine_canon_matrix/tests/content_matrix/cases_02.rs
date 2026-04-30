#[test]
fn content_ingest_case_10() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 11,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 11);
}
#[test]
fn content_ingest_case_11() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 12,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1, 1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 12);
}
#[test]
fn content_ingest_case_12() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 13,
                label: "asset".to_string(),
            },
            bytes: vec![1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 13);
}
#[test]
fn content_ingest_case_13() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 14,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 14);
}
#[test]
fn content_ingest_case_14() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 15,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 15);
}
#[test]
fn content_ingest_case_15() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 16,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1, 1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 16);
}
#[test]
fn content_ingest_case_16() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 17,
                label: "asset".to_string(),
            },
            bytes: vec![1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 17);
}
#[test]
fn content_ingest_case_17() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 18,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 18);
}
#[test]
fn content_ingest_case_18() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 19,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 19);
}
#[test]
fn content_ingest_case_19() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 20,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1, 1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 20);
}
