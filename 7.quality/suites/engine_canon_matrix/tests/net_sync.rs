// Network Sync Service Tests

use engine_net_sync::{NetSyncConfig, NetSyncService};
use engine_net_transport::{ConnectionHandle, NetTransportConfig, NetTransportService};
use engine_world::WorldState;

#[test]
fn test_net_sync_snapshot() {
    let mut transport = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 1024,
    });
    let session = transport.open_session(ConnectionHandle(1));
    let mut service = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 10,
    });

    let (_snapshot, metrics) = service
        .snapshot(&transport, &session, &WorldState::new(), vec![(0, 0, 0)])
        .expect("snapshot");

    assert_eq!(metrics.interest_region_count, 1);
    assert!(metrics.snapshot_bytes > 0);
}
