#[test]
fn agents_simulate_case_36() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (36, 0, 0),
                action_intent_count: 1,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 1);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_37() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (37, 0, 0),
                action_intent_count: 2,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 2);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_38() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (38, 0, 0),
                action_intent_count: 3,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 3);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_39() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (39, 0, 0),
                action_intent_count: 4,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 4);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
