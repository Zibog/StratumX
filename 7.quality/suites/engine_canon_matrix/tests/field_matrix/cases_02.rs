#[test]
fn field_simulate_case_9() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (9, 0, 0),
                region_delta_count: 4,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 4);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_10() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (10, 0, 0),
                region_delta_count: 0,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 0);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_11() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (11, 0, 0),
                region_delta_count: 1,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 1);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_12() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (12, 0, 0),
                region_delta_count: 2,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 2);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_13() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (13, 0, 0),
                region_delta_count: 3,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 3);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_14() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (14, 0, 0),
                region_delta_count: 4,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 4);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_15() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (15, 0, 0),
                region_delta_count: 0,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 0);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_16() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (16, 0, 0),
                region_delta_count: 1,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 1);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_17() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (17, 0, 0),
                region_delta_count: 2,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 2);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
