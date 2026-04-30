#[test]
fn field_simulate_case_0() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                region_delta_count: 0,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 0);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_1() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (1, 0, 0),
                region_delta_count: 1,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 1);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_2() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (2, 0, 0),
                region_delta_count: 2,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 2);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_3() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (3, 0, 0),
                region_delta_count: 3,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 3);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_4() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (4, 0, 0),
                region_delta_count: 4,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 4);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_5() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (5, 0, 0),
                region_delta_count: 0,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 0);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_6() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (6, 0, 0),
                region_delta_count: 1,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 1);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_7() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (7, 0, 0),
                region_delta_count: 2,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 2);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_8() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (8, 0, 0),
                region_delta_count: 3,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 3);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
