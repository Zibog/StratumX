#[test]
fn content_ingest_case_0() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 1,
                label: "asset".to_string(),
            },
            bytes: vec![1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 1);
}
#[test]
fn content_ingest_case_1() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 2,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 2);
}
#[test]
fn content_ingest_case_2() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 3,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 3);
}
#[test]
fn content_ingest_case_3() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 4,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1, 1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 4);
}
#[test]
fn content_ingest_case_4() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 5,
                label: "asset".to_string(),
            },
            bytes: vec![1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 5);
}
#[test]
fn content_ingest_case_5() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 6,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 6);
}
#[test]
fn content_ingest_case_6() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 7,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 7);
}
#[test]
fn content_ingest_case_7() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 8,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1, 1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 8);
}
#[test]
fn content_ingest_case_8() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 9,
                label: "asset".to_string(),
            },
            bytes: vec![1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 9);
}
#[test]
fn content_ingest_case_9() {
    let mut p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 10,
                label: "asset".to_string(),
            },
            bytes: vec![1, 1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    let out = p.build_runtime_pack_product(&r.manifest);
    assert_eq!(out.pack_ids[0], 10);
}
