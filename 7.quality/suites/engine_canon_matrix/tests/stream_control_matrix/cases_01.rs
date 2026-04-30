#[test]
fn stream_control_activation_case_0() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (0, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(0, 0, 0)));
}
#[test]
fn stream_control_activation_case_1() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (1, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(1, 0, 0)));
}
#[test]
fn stream_control_activation_case_2() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (2, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(2, 0, 0)));
}
#[test]
fn stream_control_activation_case_3() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (3, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(3, 0, 0)));
}
#[test]
fn stream_control_activation_case_4() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (4, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(4, 0, 0)));
}
#[test]
fn stream_control_activation_case_5() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (5, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(5, 0, 0)));
}
#[test]
fn stream_control_activation_case_6() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (6, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(6, 0, 0)));
}
#[test]
fn stream_control_activation_case_7() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (7, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(7, 0, 0)));
}
#[test]
fn stream_control_activation_case_8() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (8, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(8, 0, 0)));
}
#[test]
fn stream_control_activation_case_9() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (9, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(9, 0, 0)));
}
#[test]
fn stream_control_activation_case_10() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (10, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(10, 0, 0)));
}
