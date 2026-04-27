mod common;
use common::*;

#[test]
fn content_ingest_case_0() {
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
#[test]
fn content_ingest_case_10() {
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
#[test]
fn content_ingest_case_29() {
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
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
