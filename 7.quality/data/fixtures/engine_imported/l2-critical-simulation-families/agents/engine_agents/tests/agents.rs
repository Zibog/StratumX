use engine_agents::{AgentsConfig, AgentsContext, AgentsFamily};
use engine_core::Tick;
use engine_ecs::EcsSubstrate;
use engine_world::WorldState;

#[test]
fn agents_family_produces_bounded_delta() {
    let family = AgentsFamily::new(AgentsConfig {
        max_action_intents: 4,
    });
    let (delta, _) = family
        .simulate(
            &EcsSubstrate::new(),
            &WorldState::new(),
            AgentsContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                action_intent_count: 2,
            },
        )
        .unwrap();
    assert_eq!(delta.apply_segments[0].family_tags[0], 30);
}
