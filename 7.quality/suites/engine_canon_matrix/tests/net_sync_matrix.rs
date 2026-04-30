mod common;
use common::*;
use engine_net_transport::{ConnectionHandle, NetTransportConfig, NetTransportService};

fn run_snapshot_case(interest_regions: usize) {
    let mut transport = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 1024,
    });
    let session = transport.open_session(ConnectionHandle(1));
    let mut sync = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let interest = vec![(0, 0, 0); interest_regions];

    let (_snapshot, metrics) = sync
        .snapshot(&transport, &session, &WorldState::new(), interest)
        .unwrap();

    assert_eq!(metrics.interest_region_count, interest_regions);
    assert!(metrics.snapshot_bytes > 0);
}

macro_rules! snapshot_case {
    ($name:ident, $interest_regions:expr) => {
        #[test]
        fn $name() {
            run_snapshot_case($interest_regions);
        }
    };
}

snapshot_case!(net_sync_snapshot_case_0, 1);
snapshot_case!(net_sync_snapshot_case_1, 2);
snapshot_case!(net_sync_snapshot_case_2, 3);
snapshot_case!(net_sync_snapshot_case_3, 1);
snapshot_case!(net_sync_snapshot_case_4, 2);
snapshot_case!(net_sync_snapshot_case_5, 3);
snapshot_case!(net_sync_snapshot_case_6, 1);
snapshot_case!(net_sync_snapshot_case_7, 2);
snapshot_case!(net_sync_snapshot_case_8, 3);
snapshot_case!(net_sync_snapshot_case_9, 1);
snapshot_case!(net_sync_snapshot_case_10, 2);
snapshot_case!(net_sync_snapshot_case_11, 3);
snapshot_case!(net_sync_snapshot_case_12, 1);
snapshot_case!(net_sync_snapshot_case_13, 2);
snapshot_case!(net_sync_snapshot_case_14, 3);
snapshot_case!(net_sync_snapshot_case_15, 1);
snapshot_case!(net_sync_snapshot_case_16, 2);
snapshot_case!(net_sync_snapshot_case_17, 3);
snapshot_case!(net_sync_snapshot_case_18, 1);
snapshot_case!(net_sync_snapshot_case_19, 2);
snapshot_case!(net_sync_snapshot_case_20, 3);
snapshot_case!(net_sync_snapshot_case_21, 1);
snapshot_case!(net_sync_snapshot_case_22, 2);
snapshot_case!(net_sync_snapshot_case_23, 3);
snapshot_case!(net_sync_snapshot_case_24, 1);
snapshot_case!(net_sync_snapshot_case_25, 2);
snapshot_case!(net_sync_snapshot_case_26, 3);
snapshot_case!(net_sync_snapshot_case_27, 1);
snapshot_case!(net_sync_snapshot_case_28, 2);
snapshot_case!(net_sync_snapshot_case_29, 3);
