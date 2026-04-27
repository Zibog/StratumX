use engine_stream_control::{
    StreamControlConfig, StreamControlService, StreamReason, StreamRequest,
};

#[test]
fn empty_tick_is_non_accepting() {
    let mut service = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    let result = service.tick();
    assert!(!result.accepted);
    assert!(result.activated_regions.is_empty());
}

#[test]
fn duplicate_region_requests_collapse_to_one_active_region() {
    let mut service = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    for reason in [StreamReason::Visibility, StreamReason::Recovery] {
        service
            .queue_request(StreamRequest {
                region_key: (0, 0, 0),
                priority: 1,
                reason,
            })
            .unwrap();
    }
    let _ = service.tick();
    assert_eq!(service.active_regions().len(), 1);
}
