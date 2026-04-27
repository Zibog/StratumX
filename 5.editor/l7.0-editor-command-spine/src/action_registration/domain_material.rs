//! Material Domain Action Registration
//!
//! Registers material authoring actions (author profile, bind surface, inspect).
//! Per canonical boundary law: editor defines routes, tooling executes.

use crate::{ActionDefinition, ActionFamily, ActionId, ActionRegistry, MutationClass};

// ============================================================================
// Registration
// ============================================================================

pub fn register(registry: &mut ActionRegistry) {
    registry.register(ActionDefinition {
        action_id: ActionId::new("material.author_profile"),
        display_label: "Author Material Profile".to_string(),
        action_family: ActionFamily::Material,
        tooling_route: "route.material.author_response_profile.v1".to_string(),
        sdk_packet_family: "packet.material.*".to_string(),
        engine_truth_owner: "engine/l0".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec![
            "WLD_NO_ACTIVE".to_string(),
            "MAT_PROFILE_INVALID".to_string(),
        ],
    });

    registry.register(ActionDefinition {
        action_id: ActionId::new("material.bind_surface"),
        display_label: "Bind Material to Surface".to_string(),
        action_family: ActionFamily::Material,
        tooling_route: "route.material.bind_surface_family.v1".to_string(),
        sdk_packet_family: "packet.material.*".to_string(),
        engine_truth_owner: "engine/l0".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec![
            "WLD_NO_ACTIVE".to_string(),
            "MAT_BINDING_FAILED".to_string(),
        ],
    });

    registry.register(ActionDefinition {
        action_id: ActionId::new("material.inspect"),
        display_label: "Inspect Material".to_string(),
        action_family: ActionFamily::Material,
        tooling_route: "route.material.inspect.v1".to_string(),
        sdk_packet_family: "packet.material.*".to_string(),
        engine_truth_owner: "engine/l0".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec!["WLD_NO_ACTIVE".to_string()],
    });
}
