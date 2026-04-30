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
