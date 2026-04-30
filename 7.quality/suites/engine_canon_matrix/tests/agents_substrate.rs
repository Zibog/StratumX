mod common;
use common::*;

#[test]
fn agent_tick_preserves_identity_and_emits_typed_intents() {
    let mut substrate = AgentsSubstrate::new();
    substrate
        .register_agent(AgentState {
            id: AgentId(1),
            region_key: (0, 0, 0),
            energy: 6,
            alertness: 8,
        })
        .unwrap();
    substrate
        .register_agent(AgentState {
            id: AgentId(2),
            region_key: (0, 0, 0),
            energy: 1,
            alertness: 4,
        })
        .unwrap();

    let summary = substrate.tick();

    assert_eq!(summary.intents.len(), 2);
    assert_eq!(summary.intents[0].agent_id, AgentId(1));
    assert_eq!(summary.intents[0].kind, AgentIntentKind::Reposition);
    assert_eq!(summary.intents[1].agent_id, AgentId(2));
    assert_eq!(summary.intents[1].kind, AgentIntentKind::Recover);
}

#[test]
fn agent_tick_is_deterministic_for_same_state() {
    let state = AgentState {
        id: AgentId(3),
        region_key: (1, 0, 0),
        energy: 5,
        alertness: 2,
    };
    let mut left = AgentsSubstrate::new();
    let mut right = AgentsSubstrate::new();
    left.register_agent(state.clone()).unwrap();
    right.register_agent(state).unwrap();

    assert_eq!(left.tick(), right.tick());
}

#[test]
fn agent_registration_rejects_invalid_identity() {
    let mut substrate = AgentsSubstrate::new();
    assert!(substrate
        .register_agent(AgentState {
            id: AgentId(0),
            region_key: (0, 0, 0),
            energy: 5,
            alertness: 5,
        })
        .is_err());
}
