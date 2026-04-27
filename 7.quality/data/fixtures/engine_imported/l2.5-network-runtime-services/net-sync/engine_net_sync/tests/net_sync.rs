use engine_net_sync::{NetSyncConfig, NetSyncService};
use engine_world::WorldState;

#[test]
fn net_sync_produces_snapshot_metrics() {
    let service = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 4,
    });
    let (_snapshot, metrics) = service
        .snapshot(&WorldState::new(), vec![(0, 0, 0)])
        .unwrap();
    assert!(metrics.snapshot_bytes > 0);
}
