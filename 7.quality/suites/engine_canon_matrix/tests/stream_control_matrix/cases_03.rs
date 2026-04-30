#[test]
fn stream_control_activation_case_22() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (22, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(22, 0, 0)));
}
#[test]
fn stream_control_activation_case_23() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (23, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(23, 0, 0)));
}
#[test]
fn stream_control_activation_case_24() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (24, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(24, 0, 0)));
}
