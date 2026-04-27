// Network Latency Service Tests

use engine_core::Tick;
use engine_net_latency::{LatencyBucket, NetLatencyConfig, NetLatencyService, PredictionContext};

#[test]
fn test_latency_service() {
    let mut service =
        NetLatencyService::new(NetLatencyConfig { history_size: 10 }).expect("service");
    service.record_sample(30);
    service.record_sample(50);
    let metrics = service.metrics();
    assert_eq!(metrics.estimate.round_trip_ms, 40);
    assert_eq!(metrics.bucket, LatencyBucket::Low);
    let reconcile = service.reconcile(PredictionContext {
        authoritative_tick: Tick(5),
        predicted_tick: Tick(7),
    });
    assert!(reconcile.rewound);
    assert_eq!(reconcile.delta_ticks, 2);
}
