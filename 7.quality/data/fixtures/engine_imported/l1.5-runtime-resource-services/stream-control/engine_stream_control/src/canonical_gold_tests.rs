#![allow(unused_imports)]
use super::*;

#[test]
fn queue_request_accepts_under_ceiling() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 2,
        prefetch_radius_regions: 1,
    });
    assert!(s
        .queue_request(StreamRequest {
            region_key: (0, 0, 0),
            priority: 1,
            reason: StreamReason::Visibility
        })
        .is_ok());
}
#[test]
fn queue_request_rejects_over_ceiling() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 1,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (0, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    assert!(s
        .queue_request(StreamRequest {
            region_key: (1, 0, 0),
            priority: 1,
            reason: StreamReason::Visibility
        })
        .is_err());
}
#[test]
fn tick_activates_pending_regions() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 2,
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
}
#[test]
fn active_regions_contains_activated_key() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 2,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (0, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    s.tick();
    assert!(s.active_regions().contains(&(0, 0, 0)));
}
#[test]
fn tick_drains_queue() {
    let mut s = StreamControlService::new(StreamControlConfig {
        max_inflight_requests: 2,
        prefetch_radius_regions: 1,
    });
    s.queue_request(StreamRequest {
        region_key: (0, 0, 0),
        priority: 1,
        reason: StreamReason::Visibility,
    })
    .unwrap();
    let r = s.tick();
    assert_eq!(r.deferred, 0);
}
