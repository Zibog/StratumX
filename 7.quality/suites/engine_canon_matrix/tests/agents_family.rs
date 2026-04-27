// Agents Family Tests

use engine_agents::{AgentsConfig, AgentsContext, AgentsFamily};
use engine_core::Tick;
use engine_ecs::EcsSubstrate;
use engine_world::WorldState;

#[test]
fn test_agents_family_simulate() {
    let family = AgentsFamily::new(AgentsConfig {
        max_action_intents: 10,
    });
    let (delta, metrics) = family
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                action_intent_count: 2,
            },
        )
        .expect("simulate");
    assert_eq!(delta.apply_segments.len(), 1);
    assert_eq!(metrics.action_intent_count, 2);
}
