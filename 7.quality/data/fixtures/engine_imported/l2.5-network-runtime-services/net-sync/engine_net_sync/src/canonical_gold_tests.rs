#![allow(unused_imports)]
use super::*;

#[test]
fn snapshot_rejects_interest_overflow() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 1,
    });
    assert!(s
        .snapshot(&engine_world::WorldState::new(), vec![(0, 0, 0), (1, 0, 0)])
        .is_err());
}
#[test]
fn snapshot_returns_positive_byte_count() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 4,
    });
    let (_, m) = s
        .snapshot(&engine_world::WorldState::new(), vec![(0, 0, 0)])
        .unwrap();
    assert!(m.snapshot_bytes > 0);
}
#[test]
fn snapshot_tracks_interest_count() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 4,
    });
    let (_, m) = s
        .snapshot(&engine_world::WorldState::new(), vec![(0, 0, 0), (1, 0, 0)])
        .unwrap();
    assert_eq!(m.interest_region_count, 2);
}
#[test]
fn delta_preserves_tick_range() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 4,
    });
    let d = s.delta(Tick(1), Tick(4));
    assert_eq!(d.from_tick, Tick(1));
    assert_eq!(d.to_tick, Tick(4));
}
#[test]
fn queue_snapshot_sends_to_transport() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 4,
    });
    let (snap, _) = s
        .snapshot(&engine_world::WorldState::new(), vec![(0, 0, 0)])
        .unwrap();
    let mut runtime = engine_runtime::RuntimeKernel::new(
        engine_world::WorldState::new(),
        engine_runtime::RuntimeConfig {
            profile: engine_runtime::RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut transport =
        engine_net_transport::NetTransportService::new(engine_net_transport::NetTransportConfig {
            max_packet_size_bytes: 4096,
        });
    transport.open_session(engine_net_transport::ConnectionHandle(1));
    assert!(s
        .queue_snapshot(
            &transport,
            &mut runtime,
            engine_net_transport::ConnectionHandle(1),
            &snap
        )
        .is_ok());
}
