use engine_stream_control::{
    StreamControlConfig, StreamControlService, StreamReason, StreamRequest,
};

#[test]
fn stream_queue_enforces_inflight_ceiling() {
    let mut service = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 1,
        prefetch_radius_regions: 1,
    });
    service
        .queue_request(StreamRequest {
            region_key: (0, 0, 0),
            priority: 1,
            reason: StreamReason::Visibility,
        })
        .unwrap();
    assert!(service
        .queue_request(StreamRequest {
            region_key: (1, 0, 0),
            priority: 1,
            reason: StreamReason::Prefetch
        })
        .is_err());
}

#[test]
fn tick_activates_all_pending_regions_and_clears_queue() {
    let mut service = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 2,
    });
    for region in [(0, 0, 0), (1, 0, 0)] {
        service
            .queue_request(StreamRequest {
                region_key: region,
                priority: 1,
                reason: StreamReason::Recovery,
            })
            .unwrap();
    }
    let result = service.tick();
    assert_eq!(result.activated_regions.len(), 2);
    assert_eq!(result.deferred, 0);
    assert!(service.active_regions().contains(&(1, 0, 0)));
}
