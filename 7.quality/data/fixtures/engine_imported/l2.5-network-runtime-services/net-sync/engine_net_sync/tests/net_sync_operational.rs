use engine_net_sync::{NetSyncConfig, NetSyncService};
use engine_net_transport::{ConnectionHandle, NetTransportConfig, NetTransportService};
use engine_runtime::{RuntimeConfig, RuntimeKernel, RuntimeProfile};
use engine_world::WorldState;

#[test]
fn queued_snapshot_uses_ordered_transport_path() {
    let service = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 4,
    });
    let (snapshot, _metrics) = service
        .snapshot(&WorldState::new(), vec![(0, 0, 0)])
        .unwrap();
    let mut runtime = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut transport = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 1024,
    });
    transport.open_session(ConnectionHandle(1));
    let metrics = service
        .queue_snapshot(&transport, &mut runtime, ConnectionHandle(1), &snapshot)
        .unwrap();
    assert_eq!(metrics.queued_packets, 1);
}

#[test]
fn snapshot_is_stable_for_same_world_and_interest_set() {
    let service = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 4,
    });
    let (a, _) = service
        .snapshot(&WorldState::new(), vec![(0, 0, 0)])
        .unwrap();
    let (b, _) = service
        .snapshot(&WorldState::new(), vec![(0, 0, 0)])
        .unwrap();
    assert_eq!(a, b);
}
