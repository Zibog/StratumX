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
