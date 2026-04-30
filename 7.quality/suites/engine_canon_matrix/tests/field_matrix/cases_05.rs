#[test]
fn field_simulate_case_36() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (36, 0, 0),
                region_delta_count: 1,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 1);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_37() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (37, 0, 0),
                region_delta_count: 2,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 2);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_38() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (38, 0, 0),
                region_delta_count: 3,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 3);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
#[test]
fn field_simulate_case_39() {
    let fam = FieldFamily::new(FieldConfig {
        max_region_deltas: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (39, 0, 0),
                region_delta_count: 4,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 4);
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}
