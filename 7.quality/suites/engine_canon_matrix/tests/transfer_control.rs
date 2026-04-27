// Transfer Control Service Tests

use engine_transfer_control::{TransferConfig, TransferControlService, TransferRequest};

#[test]
fn test_transfer_control_submit() {
    let mut service = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 10,
        max_inflight_uploads: 10,
    });
    let result = service
        .submit(TransferRequest {
            asset_key: 9,
            compressed_bytes: 4,
            decoded_bytes: 8,
            upload_bytes: 8,
        })
        .expect("submit");
    assert!(result.accepted);
    assert_eq!(result.inflight_decodes, 1);
    assert_eq!(service.complete_decode().unwrap().asset_key, 9);
    assert_eq!(service.complete_upload().unwrap().asset_key, 9);
}
