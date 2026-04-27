#![allow(unused_imports)]
use super::*;

#[test]
fn simulate_rejects_too_many_action_intents() {
    let f = AgentsFamily::new(AgentsConfig {
        max_action_intents: 1,
    });
    assert!(f
        .simulate(
            &engine_ecs::EcsSubstrate::new(),
            &engine_world::WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                action_intent_count: 2
            }
        )
        .is_err());
}
#[test]
fn simulate_returns_metrics() {
    let f = AgentsFamily::new(AgentsConfig {
        max_action_intents: 4,
    });
    let (_, m) = f
        .simulate(
            &engine_ecs::EcsSubstrate::new(),
            &engine_world::WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                action_intent_count: 1,
            },
        )
        .unwrap();
    assert_eq!(m.action_intent_count, 1);
}
#[test]
fn simulate_tags_agents_family() {
    let f = AgentsFamily::new(AgentsConfig {
        max_action_intents: 4,
    });
    let (d, _) = f
        .simulate(
            &engine_ecs::EcsSubstrate::new(),
            &engine_world::WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                action_intent_count: 1,
            },
        )
        .unwrap();
    assert_eq!(d.apply_segments[0].family_tags[0], 30);
}
#[test]
fn simulate_preserves_region_key() {
    let f = AgentsFamily::new(AgentsConfig {
        max_action_intents: 4,
    });
    let (d, _) = f
        .simulate(
            &engine_ecs::EcsSubstrate::new(),
            &engine_world::WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (7, 0, 0),
                action_intent_count: 1,
            },
        )
        .unwrap();
    assert_eq!(d.apply_segments[0].region_key, (7, 0, 0));
}
#[test]
fn simulate_accepts_zero_intents() {
    let f = AgentsFamily::new(AgentsConfig {
        max_action_intents: 4,
    });
    assert!(f
        .simulate(
            &engine_ecs::EcsSubstrate::new(),
            &engine_world::WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                action_intent_count: 0
            }
        )
        .is_ok());
}
