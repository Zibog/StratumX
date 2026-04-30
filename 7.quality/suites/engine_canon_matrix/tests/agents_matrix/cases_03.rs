#[test]
fn agents_simulate_case_18() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (18, 0, 0),
                action_intent_count: 3,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 3);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_19() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (19, 0, 0),
                action_intent_count: 4,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 4);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_20() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (20, 0, 0),
                action_intent_count: 0,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 0);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_21() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (21, 0, 0),
                action_intent_count: 1,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 1);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_22() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (22, 0, 0),
                action_intent_count: 2,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 2);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_23() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (23, 0, 0),
                action_intent_count: 3,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 3);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_24() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (24, 0, 0),
                action_intent_count: 4,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 4);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_25() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (25, 0, 0),
                action_intent_count: 0,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 0);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_26() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (26, 0, 0),
                action_intent_count: 1,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 1);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
