#[test]
fn transfer_control_submit_case_0() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 1,
            decoded_bytes: 2,
            upload_bytes: 3,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_1() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 2,
            decoded_bytes: 3,
            upload_bytes: 4,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_2() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 3,
            decoded_bytes: 4,
            upload_bytes: 5,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_3() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 4,
            decoded_bytes: 5,
            upload_bytes: 6,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_4() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 5,
            decoded_bytes: 6,
            upload_bytes: 7,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_5() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 6,
            decoded_bytes: 7,
            upload_bytes: 8,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_6() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 7,
            decoded_bytes: 8,
            upload_bytes: 9,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_7() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 8,
            decoded_bytes: 9,
            upload_bytes: 10,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_8() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 9,
            decoded_bytes: 10,
            upload_bytes: 11,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_9() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 10,
            decoded_bytes: 11,
            upload_bytes: 12,
        })
        .unwrap();
    assert!(r.accepted);
}
#[test]
fn transfer_control_submit_case_10() {
    let mut s = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let r = s
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 11,
            decoded_bytes: 12,
            upload_bytes: 13,
        })
        .unwrap();
    assert!(r.accepted);
}
