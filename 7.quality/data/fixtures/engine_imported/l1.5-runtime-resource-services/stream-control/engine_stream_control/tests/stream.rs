use engine_stream_control::{
    StreamControlConfig, StreamControlService, StreamReason, StreamRequest,
};

#[test]
fn stream_control_activates_requested_regions() {
    let mut service = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    service
        .queue_request(StreamRequest {
            region_key: (0, 0, 0),
            priority: 1,
            reason: StreamReason::Visibility,
        })
        .unwrap();
    let result = service.tick();
    assert!(result.accepted);
    assert!(service.active_regions().contains(&(0, 0, 0)));
}
