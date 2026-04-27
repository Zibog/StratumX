mod common;
use common::*;

#[test]
fn net_sync_snapshot_case_0() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s.snapshot(&WorldState::new(), vec![(0, 0, 0)]).unwrap();
    assert_eq!(metrics.interest_region_count, 1);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_1() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 2);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_2() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 3);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_3() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s.snapshot(&WorldState::new(), vec![(0, 0, 0)]).unwrap();
    assert_eq!(metrics.interest_region_count, 1);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_4() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 2);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_5() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 3);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_6() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s.snapshot(&WorldState::new(), vec![(0, 0, 0)]).unwrap();
    assert_eq!(metrics.interest_region_count, 1);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_7() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 2);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_8() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 3);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_9() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s.snapshot(&WorldState::new(), vec![(0, 0, 0)]).unwrap();
    assert_eq!(metrics.interest_region_count, 1);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_10() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 2);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_11() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 3);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_12() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s.snapshot(&WorldState::new(), vec![(0, 0, 0)]).unwrap();
    assert_eq!(metrics.interest_region_count, 1);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_13() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 2);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_14() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 3);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_15() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s.snapshot(&WorldState::new(), vec![(0, 0, 0)]).unwrap();
    assert_eq!(metrics.interest_region_count, 1);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_16() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 2);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_17() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 3);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_18() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s.snapshot(&WorldState::new(), vec![(0, 0, 0)]).unwrap();
    assert_eq!(metrics.interest_region_count, 1);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_19() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 2);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_20() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 3);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_21() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s.snapshot(&WorldState::new(), vec![(0, 0, 0)]).unwrap();
    assert_eq!(metrics.interest_region_count, 1);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_22() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 2);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_23() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 3);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_24() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s.snapshot(&WorldState::new(), vec![(0, 0, 0)]).unwrap();
    assert_eq!(metrics.interest_region_count, 1);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_25() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 2);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_26() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 3);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_27() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s.snapshot(&WorldState::new(), vec![(0, 0, 0)]).unwrap();
    assert_eq!(metrics.interest_region_count, 1);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_28() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 2);
    assert!(metrics.snapshot_bytes > 0);
}
#[test]
fn net_sync_snapshot_case_29() {
    let s = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let (_snap, metrics) = s
        .snapshot(&WorldState::new(), vec![(0, 0, 0), (0, 0, 0), (0, 0, 0)])
        .unwrap();
    assert_eq!(metrics.interest_region_count, 3);
    assert!(metrics.snapshot_bytes > 0);
}
