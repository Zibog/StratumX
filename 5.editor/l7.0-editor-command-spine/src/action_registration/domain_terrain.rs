//! Terrain Domain Action Registration
//!
//! Registers terrain manipulation actions (sculpt, paint, configure layers).
//! Per canonical boundary law: editor defines routes, tooling executes.

use crate::{ActionDefinition, ActionFamily, ActionId, ActionRegistry, MutationClass};

// ============================================================================
// Registration
// ============================================================================

pub fn register(registry: &mut ActionRegistry) {
    registry.register(ActionDefinition {
        action_id: ActionId::new("terrain.sculpt"),
        display_label: "Sculpt Terrain".to_string(),
        action_family: ActionFamily::Terrain,
        tooling_route: "route.terrain.sculpt.v1".to_string(),
        sdk_packet_family: "packet.terrain.*".to_string(),
        engine_truth_owner: "engine/l0".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec!["WLD_NO_ACTIVE".to_string(), "TRN_NO_TERRAIN".to_string()],
    });

    registry.register(ActionDefinition {
        action_id: ActionId::new("terrain.paint"),
        display_label: "Paint Terrain".to_string(),
        action_family: ActionFamily::Terrain,
        tooling_route: "route.terrain.paint.v1".to_string(),
        sdk_packet_family: "packet.terrain.*".to_string(),
        engine_truth_owner: "engine/l0".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec!["WLD_NO_ACTIVE".to_string(), "TRN_NO_TERRAIN".to_string()],
    });

    registry.register(ActionDefinition {
        action_id: ActionId::new("terrain.configure_layers"),
        display_label: "Configure Terrain Layers".to_string(),
        action_family: ActionFamily::Terrain,
        tooling_route: "route.terrain.configure_layers.v1".to_string(),
        sdk_packet_family: "packet.terrain.*".to_string(),
        engine_truth_owner: "engine/l0".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec!["WLD_NO_ACTIVE".to_string(), "TRN_NO_TERRAIN".to_string()],
    });
}
