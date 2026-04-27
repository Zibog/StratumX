use engine_core::Tick;
use engine_net_sync::{NetSyncConfig, NetSyncService};
use engine_world::WorldState;

#[test]
fn sync_rejects_interest_region_overflow() {
    let service = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 1,
    });
    assert!(service
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (1, 0, 0)])
        .is_err());
}

#[test]
fn delta_preserves_tick_range() {
    let service = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 4,
    });
    let delta = service.delta(Tick(10), Tick(15));
    assert_eq!(delta.from_tick, Tick(10));
    assert_eq!(delta.to_tick, Tick(15));
}
