//! World Domain Action Registration
//!
//! Registers world lifecycle actions (open, close, save).
//! Per canonical boundary law: editor defines routes, tooling executes.

use crate::{ActionDefinition, ActionFamily, ActionId, ActionRegistry, MutationClass};

// ============================================================================
// Registration
// ============================================================================

pub fn register(registry: &mut ActionRegistry) {
    registry.register(ActionDefinition {
        action_id: ActionId::new("world.open"),
        display_label: "Open World".to_string(),
        action_family: ActionFamily::World,
        tooling_route: "route.world.open.v1".to_string(),
        sdk_packet_family: "packet.world.*".to_string(),
        engine_truth_owner: "engine/l0".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec![
            "WLD_OPEN_FAILED".to_string(),
            "PRJ_NO_WORKSPACE".to_string(),
        ],
    });

    registry.register(ActionDefinition {
        action_id: ActionId::new("world.close"),
        display_label: "Close World".to_string(),
        action_family: ActionFamily::World,
        tooling_route: "route.world.close.v1".to_string(),
        sdk_packet_family: "packet.world.*".to_string(),
        engine_truth_owner: "engine/l0".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec!["WLD_CLOSE_FAILED".to_string()],
    });

    registry.register(ActionDefinition {
        action_id: ActionId::new("world.save"),
        display_label: "Save World".to_string(),
        action_family: ActionFamily::World,
        tooling_route: "route.world.save.v1".to_string(),
        sdk_packet_family: "packet.world.*".to_string(),
        engine_truth_owner: "engine/l0".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec![
            "WLD_SAVE_FAILED".to_string(),
            "PRJ_NO_WORKSPACE".to_string(),
        ],
    });

    registry.register(ActionDefinition {
        action_id: ActionId::new("environment.configure_sky"),
        display_label: "Configure Sky".to_string(),
        action_family: ActionFamily::SkyEnvironment,
        tooling_route: "route.environment.configure_sky.v1".to_string(),
        sdk_packet_family: "packet.environment.*".to_string(),
        engine_truth_owner: "engine/l0".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec![
            "WLD_NO_ACTIVE".to_string(),
            "ENV_SKY_PROFILE_INVALID".to_string(),
        ],
    });

    registry.register(ActionDefinition {
        action_id: ActionId::new("environment.configure_weather"),
        display_label: "Configure Weather".to_string(),
        action_family: ActionFamily::SkyEnvironment,
        tooling_route: "route.environment.configure_weather.v1".to_string(),
        sdk_packet_family: "packet.environment.*".to_string(),
        engine_truth_owner: "engine/l0".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec![
            "WLD_NO_ACTIVE".to_string(),
            "ENV_WEATHER_INVALID".to_string(),
        ],
    });

    registry.register(ActionDefinition {
        action_id: ActionId::new("environment.set_time"),
        display_label: "Set Time Of Day".to_string(),
        action_family: ActionFamily::SkyEnvironment,
        tooling_route: "route.environment.set_time.v1".to_string(),
        sdk_packet_family: "packet.environment.*".to_string(),
        engine_truth_owner: "engine/l0".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec!["WLD_NO_ACTIVE".to_string(), "ENV_TIME_INVALID".to_string()],
    });
}
