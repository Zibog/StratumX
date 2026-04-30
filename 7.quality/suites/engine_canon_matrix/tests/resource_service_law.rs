mod common;
use common::*;

#[test]
fn residency_transition_follows_canonical_ladder() {
    let mut service = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 2,
        streaming_item_budget: 2,
    });

    service.transition(9, ResidencySet::StagingBacked).unwrap();
    service
        .transition(9, ResidencySet::StreamingResident)
        .unwrap();
    service.transition(9, ResidencySet::Hot).unwrap();

    assert_eq!(
        service.descriptor(9).unwrap().residency_set,
        ResidencySet::Hot
    );
    assert!(service.transition(10, ResidencySet::Hot).is_err());
}

#[test]
fn stream_control_activates_requests_in_deterministic_priority_order() {
    let mut service = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 8,
        prefetch_radius_regions: 2,
    });
    service
        .queue_request(StreamRequest {
            region_key: (2, 0, 0),
            priority: 1,
            reason: StreamReason::Prefetch,
        })
        .unwrap();
    service
        .queue_request(StreamRequest {
            region_key: (1, 0, 0),
            priority: 7,
            reason: StreamReason::Visibility,
        })
        .unwrap();
    service
        .queue_request(StreamRequest {
            region_key: (0, 0, 0),
            priority: 7,
            reason: StreamReason::Recovery,
        })
        .unwrap();

    let result = service.tick();

    assert_eq!(
        result.activated_regions,
        vec![(0, 0, 0), (1, 0, 0), (2, 0, 0)]
    );
}

#[test]
fn transfer_control_preserves_fifo_and_rejects_invalid_request() {
    let mut service = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 4,
        max_inflight_uploads: 4,
    });
    assert!(service
        .submit(TransferRequest {
            asset_key: 1,
            compressed_bytes: 0,
            decoded_bytes: 8,
            upload_bytes: 8,
        })
        .is_err());

    service
        .submit(TransferRequest {
            asset_key: 11,
            compressed_bytes: 4,
            decoded_bytes: 8,
            upload_bytes: 8,
        })
        .unwrap();
    service
        .submit(TransferRequest {
            asset_key: 12,
            compressed_bytes: 5,
            decoded_bytes: 9,
            upload_bytes: 9,
        })
        .unwrap();

    assert_eq!(service.complete_decode().unwrap().asset_key, 11);
    assert_eq!(service.complete_decode().unwrap().asset_key, 12);
    assert_eq!(service.complete_upload().unwrap().asset_key, 11);
    assert_eq!(service.complete_upload().unwrap().asset_key, 12);
}
