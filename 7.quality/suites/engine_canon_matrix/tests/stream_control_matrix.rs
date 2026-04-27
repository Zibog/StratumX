mod common;
use common::*;

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
