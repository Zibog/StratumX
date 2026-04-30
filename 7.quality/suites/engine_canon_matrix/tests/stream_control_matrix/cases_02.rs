#[test]
fn stream_control_activation_case_11() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (11, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(11, 0, 0)));
}
#[test]
fn stream_control_activation_case_12() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (12, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(12, 0, 0)));
}
#[test]
fn stream_control_activation_case_13() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (13, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(13, 0, 0)));
}
#[test]
fn stream_control_activation_case_14() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (14, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(14, 0, 0)));
}
#[test]
fn stream_control_activation_case_15() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (15, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(15, 0, 0)));
}
#[test]
fn stream_control_activation_case_16() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (16, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(16, 0, 0)));
}
#[test]
fn stream_control_activation_case_17() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (17, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(17, 0, 0)));
}
#[test]
fn stream_control_activation_case_18() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (18, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(18, 0, 0)));
}
#[test]
fn stream_control_activation_case_19() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (19, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(19, 0, 0)));
}
#[test]
fn stream_control_activation_case_20() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (20, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(20, 0, 0)));
}
#[test]
fn stream_control_activation_case_21() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 4,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (21, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert!(r.accepted);
    assert!(s.active_regions().contains(&(21, 0, 0)));
}
