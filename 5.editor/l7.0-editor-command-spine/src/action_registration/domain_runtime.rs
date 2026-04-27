//! Runtime Domain Action Registration
//!
//! Registers runtime control actions (play, pause, step).
//! Per canonical boundary law: editor defines routes, tooling executes.

use crate::{ActionDefinition, ActionFamily, ActionId, ActionRegistry, MutationClass};

// ============================================================================
// Registration
// ============================================================================

pub fn register(registry: &mut ActionRegistry) {
    registry.register(ActionDefinition {
        action_id: ActionId::new("runtime.play"),
        display_label: "Play".to_string(),
        action_family: ActionFamily::Runtime,
        tooling_route: "route.runtime.play.v1".to_string(),
        sdk_packet_family: "packet.runtime.*".to_string(),
        engine_truth_owner: "engine/l1".to_string(),
        mutation_class: MutationClass::Simulate,
        possible_denial_families: vec![
            "WLD_NO_ACTIVE".to_string(),
            "RT_KERNEL_UNAVAILABLE".to_string(),
        ],
    });

    registry.register(ActionDefinition {
        action_id: ActionId::new("runtime.pause"),
        display_label: "Pause".to_string(),
        action_family: ActionFamily::Runtime,
        tooling_route: "route.runtime.pause.v1".to_string(),
        sdk_packet_family: "packet.runtime.*".to_string(),
        engine_truth_owner: "engine/l1".to_string(),
        mutation_class: MutationClass::Simulate,
        possible_denial_families: vec![
            "WLD_NO_ACTIVE".to_string(),
            "RT_KERNEL_UNAVAILABLE".to_string(),
        ],
    });

    registry.register(ActionDefinition {
        action_id: ActionId::new("runtime.step"),
        display_label: "Step".to_string(),
        action_family: ActionFamily::Runtime,
        tooling_route: "route.runtime.step.v1".to_string(),
        sdk_packet_family: "packet.runtime.*".to_string(),
        engine_truth_owner: "engine/l1".to_string(),
        mutation_class: MutationClass::Simulate,
        possible_denial_families: vec![
            "WLD_NO_ACTIVE".to_string(),
            "RT_KERNEL_UNAVAILABLE".to_string(),
        ],
    });
}
