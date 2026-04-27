use engine_core::Tick;
use engine_net_latency::{NetLatencyConfig, NetLatencyService, PredictionContext};

#[test]
fn net_latency_produces_bucketed_metrics() {
    let mut service = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    service.record_sample(30);
    service.record_sample(50);
    let metrics = service.metrics();
    assert!(metrics.estimate.round_trip_ms >= 30);
    let reconcile = service.reconcile(PredictionContext {
        authoritative_tick: Tick(1),
        predicted_tick: Tick(3),
    });
    assert!(reconcile.rewound);
}
