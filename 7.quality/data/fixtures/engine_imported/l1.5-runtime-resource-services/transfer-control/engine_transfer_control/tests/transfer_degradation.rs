use engine_transfer_control::{TransferConfig, TransferControlService, TransferRequest};

#[test]
fn zero_byte_transfer_is_rejected() {
    let mut service = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 2,
        max_inflight_uploads: 2,
    });
    let result = service.submit(TransferRequest {
        asset_key: 1,
        compressed_bytes: 0,
        decoded_bytes: 5,
        upload_bytes: 5,
    });
    assert!(result.is_err());
}

#[test]
fn completion_order_is_fifo() {
    let mut service = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 4,
        max_inflight_uploads: 4,
    });
    service
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 1,
            decoded_bytes: 2,
            upload_bytes: 3,
        })
        .unwrap();
    service
        .submit(TransferRequest {
            asset_key: 2,
            compressed_bytes: 1,
            decoded_bytes: 2,
            upload_bytes: 3,
        })
        .unwrap();
    assert_eq!(service.complete_decode().unwrap().asset_key, 1);
    assert_eq!(service.complete_decode().unwrap().asset_key, 2);
}
