#![allow(unused_imports)]
use super::*;

#[test]
fn constructor_rejects_zero_history() {
    assert!(NetLatencyService::new(NetLatencyConfig { history_size: 0 }).is_err());
}
#[test]
fn metrics_bucket_low_at_fifty_or_less() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(40);
    s.record_sample(50);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn metrics_bucket_medium_above_fifty() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(60);
    s.record_sample(80);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn history_window_evicts_oldest_samples() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 2 }).unwrap();
    s.record_sample(40);
    s.record_sample(100);
    s.record_sample(140);
    let metrics = s.metrics();
    assert_eq!(metrics.bucket, LatencyBucket::Medium);
    assert_eq!(metrics.estimate.round_trip_ms, 120);
}
#[test]
fn reconcile_reports_delta_ticks() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(1),
        predicted_tick: Tick(3),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 2);
}
