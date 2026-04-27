#![allow(unused_imports)]
use super::*;

#[test]
fn ingest_rejects_empty_bytes() {
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    assert!(p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 1,
                label: "asset".to_string()
            },
            bytes: vec![],
            locator: ContentLocator {
                uri: "mem://asset".to_string()
            }
        })
        .is_err());
}
#[test]
fn ingest_builds_manifest() {
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
    assert_eq!(r.manifest.packs.len(), 1);
}
#[test]
fn ingest_uses_content_id_as_pack_id() {
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 7,
                label: "asset".to_string(),
            },
            bytes: vec![1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    assert_eq!(r.pack.pack_id, 7);
}
#[test]
fn build_runtime_pack_product_collects_ids() {
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    let r = p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 7,
                label: "asset".to_string(),
            },
            bytes: vec![1],
            locator: ContentLocator {
                uri: "mem://asset".to_string(),
            },
        })
        .unwrap();
    assert_eq!(p.build_runtime_pack_product(&r.manifest).pack_ids, vec![7]);
}
#[test]
fn ingest_accepts_non_empty_bytes() {
    let p = ContentPipeline::new(ContentConfig { max_packs: 4 });
    assert!(p
        .ingest(ContentRequest {
            descriptor: ContentDescriptor {
                content_id: 1,
                label: "asset".to_string()
            },
            bytes: vec![1],
            locator: ContentLocator {
                uri: "mem://asset".to_string()
            }
        })
        .is_ok());
}
