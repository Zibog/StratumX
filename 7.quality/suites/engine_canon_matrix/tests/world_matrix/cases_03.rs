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
