#[test]
fn transfer_control_submit_case_11() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 12,
            decoded_bytes: 13,
            upload_bytes: 14,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_12() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 13,
            decoded_bytes: 14,
            upload_bytes: 15,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_13() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 14,
            decoded_bytes: 15,
            upload_bytes: 16,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_14() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 15,
            decoded_bytes: 16,
            upload_bytes: 17,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_15() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 16,
            decoded_bytes: 17,
            upload_bytes: 18,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_16() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 17,
            decoded_bytes: 18,
            upload_bytes: 19,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_17() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 18,
            decoded_bytes: 19,
            upload_bytes: 20,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_18() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 19,
            decoded_bytes: 20,
            upload_bytes: 21,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_19() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 20,
            decoded_bytes: 21,
            upload_bytes: 22,
        })
        .unwrap();
    assert!(r.accepted);
}
