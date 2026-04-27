use engine_core::Tick;
use engine_net_latency::{LatencyBucket, NetLatencyConfig, NetLatencyService, PredictionContext};

#[test]
fn history_size_must_be_non_zero() {
    assert!(NetLatencyService::new(NetLatencyConfig { history_size: 0 }).is_err());
}

#[test]
fn latency_bucket_and_jitter_follow_samples() {
    let mut service = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    for sample in [90, 130] {
        service.record_sample(sample);
    }
    let metrics = service.metrics();
    assert_eq!(metrics.bucket, LatencyBucket::Medium);
    assert_eq!(metrics.jitter.variance_ms, 40);
    let reconcile = service.reconcile(PredictionContext {
        authoritative_tick: Tick(2),
        predicted_tick: Tick(2),
    });
    assert!(!reconcile.rewound);
}
