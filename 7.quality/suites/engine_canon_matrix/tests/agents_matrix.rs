mod common;
use common::*;

#[test]
fn agents_simulate_case_0() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                action_intent_count: 0,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 0);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_1() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (1, 0, 0),
                action_intent_count: 1,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 1);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_2() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (2, 0, 0),
                action_intent_count: 2,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 2);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_3() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (3, 0, 0),
                action_intent_count: 3,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 3);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_4() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (4, 0, 0),
                action_intent_count: 4,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 4);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_5() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (5, 0, 0),
                action_intent_count: 0,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 0);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_6() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (6, 0, 0),
                action_intent_count: 1,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 1);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_7() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (7, 0, 0),
                action_intent_count: 2,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 2);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_8() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (8, 0, 0),
                action_intent_count: 3,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 3);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_9() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (9, 0, 0),
                action_intent_count: 4,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 4);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_10() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (10, 0, 0),
                action_intent_count: 0,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 0);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_11() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (11, 0, 0),
                action_intent_count: 1,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 1);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_12() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (12, 0, 0),
                action_intent_count: 2,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 2);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_13() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (13, 0, 0),
                action_intent_count: 3,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 3);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_14() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (14, 0, 0),
                action_intent_count: 4,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 4);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_15() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (15, 0, 0),
                action_intent_count: 0,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 0);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_16() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (16, 0, 0),
                action_intent_count: 1,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 1);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
#[test]
fn agents_simulate_case_17() {
    let fam = AgentsFamily::new(AgentsConfig {
        max_action_intents: 64,
    });
    let (delta, metrics) = fam
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (17, 0, 0),
                action_intent_count: 2,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 2);
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
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
