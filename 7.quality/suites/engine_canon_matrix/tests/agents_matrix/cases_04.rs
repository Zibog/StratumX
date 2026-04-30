#[test]
fn agents_simulate_case_27() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (27, 0, 0),
                action_intent_count: 2,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 2);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_28() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (28, 0, 0),
                action_intent_count: 3,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 3);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_29() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (29, 0, 0),
                action_intent_count: 4,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 4);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_30() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (30, 0, 0),
                action_intent_count: 0,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 0);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_31() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (31, 0, 0),
                action_intent_count: 1,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 1);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_32() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (32, 0, 0),
                action_intent_count: 2,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 2);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_33() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (33, 0, 0),
                action_intent_count: 3,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 3);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_34() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (34, 0, 0),
                action_intent_count: 4,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 4);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_35() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (35, 0, 0),
                action_intent_count: 0,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 0);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
