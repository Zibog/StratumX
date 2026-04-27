#![allow(unused_imports)]
use super::*;

#[test]
fn submit_requires_non_zero_compressed_bytes() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 2,
        max_inflight_uploads: 2,
    });
    assert!(s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 0,
            decoded_bytes: 1,
            upload_bytes: 1
        })
        .is_err());
}
#[test]
fn submit_requires_non_zero_upload_bytes() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 2,
        max_inflight_uploads: 2,
    });
    assert!(s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 1,
            decoded_bytes: 1,
            upload_bytes: 0
        })
        .is_err());
}
#[test]
fn submit_tracks_inflight_counts() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 2,
        max_inflight_uploads: 2,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 1,
            decoded_bytes: 2,
            upload_bytes: 3,
        })
        .unwrap();
    assert_eq!(r.inflight_decodes, 1);
    assert_eq!(r.inflight_uploads, 1);
}
#[test]
fn complete_decode_pops_front() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 2,
        max_inflight_uploads: 2,
    });
    s.submit(TransferRequest {
        asset_key: 1,
        compressed_bytes: 1,
        decoded_bytes: 2,
        upload_bytes: 3,
    })
    .unwrap();
    assert!(s.complete_decode().is_some());
}
#[test]
fn complete_upload_pops_front() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 2,
        max_inflight_uploads: 2,
    });
    s.submit(TransferRequest {
        asset_key: 1,
        compressed_bytes: 1,
        decoded_bytes: 2,
        upload_bytes: 3,
    })
    .unwrap();
    assert!(s.complete_upload().is_some());
}
