use engine_net_latency::{LatencyBucket, NetLatencyConfig, NetLatencyService};

#[test]
fn history_window_evicts_oldest_samples() {
    let mut service = NetLatencyService::new(NetLatencyConfig { history_size: 2 }).unwrap();
    service.record_sample(20);
    service.record_sample(30);
    service.record_sample(200);
    let metrics = service.metrics();
    assert_eq!(metrics.bucket, LatencyBucket::Medium);
    assert_eq!(metrics.estimate.round_trip_ms, 115);
}
