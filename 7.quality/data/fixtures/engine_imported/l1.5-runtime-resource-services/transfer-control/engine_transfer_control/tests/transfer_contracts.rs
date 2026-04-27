use engine_transfer_control::{TransferConfig, TransferControlService, TransferRequest};

#[test]
fn transfer_rejects_zero_sized_edges() {
    let mut service = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 1,
        max_inflight_uploads: 1,
    });
    let result = service.submit(TransferRequest {
        asset_key: 1,
        compressed_bytes: 0,
        decoded_bytes: 10,
        upload_bytes: 10,
    });
    assert!(result.is_err());
}

#[test]
fn transfer_queue_ceilings_and_fifo_completion_are_preserved() {
    let mut service = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 1,
        max_inflight_uploads: 1,
    });
    let request = TransferRequest {
        asset_key: 7,
        compressed_bytes: 10,
        decoded_bytes: 20,
        upload_bytes: 30,
    };
    service.submit(request.clone()).unwrap();
    assert!(service.submit(request.clone()).is_err());
    assert_eq!(service.complete_decode().unwrap().asset_key, 7);
    assert_eq!(service.complete_upload().unwrap().asset_key, 7);
}
