use engine_agents::{AgentsConfig, AgentsContext, AgentsFamily};
use engine_core::Tick;
use engine_ecs::EcsSubstrate;
use engine_world::WorldState;

#[test]
fn agents_reject_action_intent_overflow() {
    let family = AgentsFamily::new(AgentsConfig {
        max_action_intents: 1,
    });
    let result = family.simulate(
        &EcsSubstrate::new(),
        &WorldState::new(),
        AgentsContext {
            tick: Tick(0),
            region_key: (0, 0, 0),
            action_intent_count: 2,
        },
    );
    assert!(result.is_err());
}

#[test]
fn agents_metrics_reflect_action_intent_count() {
    let family = AgentsFamily::new(AgentsConfig {
        max_action_intents: 4,
    });
    let (_, metrics) = family
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                action_intent_count: 3,
            },
        )
        .unwrap();
    assert_eq!(metrics.action_intent_count, 3);
}
