mod common;
use common::*;

#[test]
fn world_apply_under_budget_0() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_1() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_2() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2, 3],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_3() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2, 3, 4],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_4() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_5() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_6() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2, 3],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_7() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2, 3, 4],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_8() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_9() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_10() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2, 3],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_11() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2, 3, 4],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_12() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_13() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_14() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2, 3],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_15() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2, 3, 4],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_16() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_17() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_18() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2, 3],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_19() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2, 3, 4],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_20() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_21() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_22() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2, 3],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_23() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2, 3, 4],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_24() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_25() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_26() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2, 3],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_27() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2, 3, 4],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_28() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_under_budget_29() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2],
    };
    assert!(w.apply(&[seg], 1).is_ok());
    assert_eq!(w.read_model().tick.0, 1);
}
#[test]
fn world_apply_rejects_fanout_0() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![0u16; MAX_FAMILY_FANOUT_PER_SEGMENT + 1],
    };
    assert!(w.apply(&[seg], 1).is_err());
}
#[test]
fn world_apply_rejects_fanout_1() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![0u16; MAX_FAMILY_FANOUT_PER_SEGMENT + 1],
    };
    assert!(w.apply(&[seg], 1).is_err());
}
#[test]
fn world_apply_rejects_fanout_2() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![0u16; MAX_FAMILY_FANOUT_PER_SEGMENT + 1],
    };
    assert!(w.apply(&[seg], 1).is_err());
}
#[test]
fn world_apply_rejects_fanout_3() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![0u16; MAX_FAMILY_FANOUT_PER_SEGMENT + 1],
    };
    assert!(w.apply(&[seg], 1).is_err());
}
#[test]
fn world_apply_rejects_fanout_4() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![0u16; MAX_FAMILY_FANOUT_PER_SEGMENT + 1],
    };
    assert!(w.apply(&[seg], 1).is_err());
}
#[test]
fn world_apply_rejects_fanout_5() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![0u16; MAX_FAMILY_FANOUT_PER_SEGMENT + 1],
    };
    assert!(w.apply(&[seg], 1).is_err());
}
#[test]
fn world_apply_rejects_fanout_6() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![0u16; MAX_FAMILY_FANOUT_PER_SEGMENT + 1],
    };
    assert!(w.apply(&[seg], 1).is_err());
}
#[test]
fn world_apply_rejects_fanout_7() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![0u16; MAX_FAMILY_FANOUT_PER_SEGMENT + 1],
    };
    assert!(w.apply(&[seg], 1).is_err());
}
#[test]
fn world_apply_rejects_fanout_8() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![0u16; MAX_FAMILY_FANOUT_PER_SEGMENT + 1],
    };
    assert!(w.apply(&[seg], 1).is_err());
}
#[test]
fn world_apply_rejects_fanout_9() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![0u16; MAX_FAMILY_FANOUT_PER_SEGMENT + 1],
    };
    assert!(w.apply(&[seg], 1).is_err());
}
#[test]
fn world_apply_rejects_fanout_10() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![0u16; MAX_FAMILY_FANOUT_PER_SEGMENT + 1],
    };
    assert!(w.apply(&[seg], 1).is_err());
}
#[test]
fn world_apply_rejects_fanout_11() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![0u16; MAX_FAMILY_FANOUT_PER_SEGMENT + 1],
    };
    assert!(w.apply(&[seg], 1).is_err());
}
#[test]
fn world_apply_rejects_fanout_12() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![0u16; MAX_FAMILY_FANOUT_PER_SEGMENT + 1],
    };
    assert!(w.apply(&[seg], 1).is_err());
}
#[test]
fn world_apply_rejects_fanout_13() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![0u16; MAX_FAMILY_FANOUT_PER_SEGMENT + 1],
    };
    assert!(w.apply(&[seg], 1).is_err());
}
#[test]
fn world_apply_rejects_fanout_14() {
    let mut w = WorldState::new();
    let seg = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![0u16; MAX_FAMILY_FANOUT_PER_SEGMENT + 1],
    };
    assert!(w.apply(&[seg], 1).is_err());
}
#[test]
fn world_snapshot_bytes_non_empty_0() {
    let w = WorldState::new();
    assert!(!w.snapshot_bytes(0).unwrap().is_empty());
}
#[test]
fn world_snapshot_bytes_non_empty_1() {
    let w = WorldState::new();
    assert!(!w.snapshot_bytes(1).unwrap().is_empty());
}
#[test]
fn world_snapshot_bytes_non_empty_2() {
    let w = WorldState::new();
    assert!(!w.snapshot_bytes(2).unwrap().is_empty());
}
#[test]
fn world_snapshot_bytes_non_empty_3() {
    let w = WorldState::new();
    assert!(!w.snapshot_bytes(3).unwrap().is_empty());
}
#[test]
fn world_snapshot_bytes_non_empty_4() {
    let w = WorldState::new();
    assert!(!w.snapshot_bytes(4).unwrap().is_empty());
}
#[test]
fn world_snapshot_bytes_non_empty_5() {
    let w = WorldState::new();
    assert!(!w.snapshot_bytes(5).unwrap().is_empty());
}
#[test]
fn world_snapshot_bytes_non_empty_6() {
    let w = WorldState::new();
    assert!(!w.snapshot_bytes(6).unwrap().is_empty());
}
#[test]
fn world_snapshot_bytes_non_empty_7() {
    let w = WorldState::new();
    assert!(!w.snapshot_bytes(7).unwrap().is_empty());
}
#[test]
fn world_snapshot_bytes_non_empty_8() {
    let w = WorldState::new();
    assert!(!w.snapshot_bytes(8).unwrap().is_empty());
}
#[test]
fn world_snapshot_bytes_non_empty_9() {
    let w = WorldState::new();
    assert!(!w.snapshot_bytes(9).unwrap().is_empty());
}
#[test]
fn world_snapshot_bytes_non_empty_10() {
    let w = WorldState::new();
    assert!(!w.snapshot_bytes(10).unwrap().is_empty());
}
#[test]
fn world_snapshot_bytes_non_empty_11() {
    let w = WorldState::new();
    assert!(!w.snapshot_bytes(11).unwrap().is_empty());
}
#[test]
fn world_snapshot_bytes_non_empty_12() {
    let w = WorldState::new();
    assert!(!w.snapshot_bytes(12).unwrap().is_empty());
}
#[test]
fn world_snapshot_bytes_non_empty_13() {
    let w = WorldState::new();
    assert!(!w.snapshot_bytes(13).unwrap().is_empty());
}
#[test]
fn world_snapshot_bytes_non_empty_14() {
    let w = WorldState::new();
    assert!(!w.snapshot_bytes(14).unwrap().is_empty());
}
