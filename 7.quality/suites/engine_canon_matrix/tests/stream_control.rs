// Stream Control Service Tests

use engine_stream_control::{
    StreamControlConfig, StreamControlService, StreamReason, StreamRequest,
};

#[test]
fn test_stream_control_queue() {
    let mut service = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 10,
        prefetch_radius_regions: 2,
    });
    service
        .queue_request(StreamRequest {
            region_key: (1, 2, 3),
            priority: 7,
            reason: StreamReason::Visibility,
        })
        .expect("queue");
    let result = service.tick();
    assert!(result.accepted);
    assert_eq!(result.activated_regions, vec![(1, 2, 3)]);
    assert!(service.active_regions().contains(&(1, 2, 3)));
}
