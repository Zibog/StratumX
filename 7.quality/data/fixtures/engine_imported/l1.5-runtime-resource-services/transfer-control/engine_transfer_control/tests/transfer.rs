use engine_transfer_control::{TransferConfig, TransferControlService, TransferRequest};

#[test]
fn transfer_service_accepts_valid_requests() {
    let mut service = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 2,
        max_inflight_uploads: 2,
    });
    let result = service
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 10,
            decoded_bytes: 20,
            upload_bytes: 30,
        })
        .unwrap();
    assert!(result.accepted);
}
