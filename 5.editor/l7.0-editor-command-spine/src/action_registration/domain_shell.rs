//! Shell Domain Action Registration
//!
//! Registers shell/UI actions (project lifecycle, panel management).
//! Per canonical boundary law: editor defines routes, tooling executes.

use crate::{ActionDefinition, ActionFamily, ActionId, ActionRegistry, MutationClass};

// ============================================================================
// Registration
// ============================================================================

pub fn register(registry: &mut ActionRegistry) {
    // Project actions
    registry.register(ActionDefinition {
        action_id: ActionId::new("project.new"),
        display_label: "New Project".to_string(),
        action_family: ActionFamily::ProjectFile,
        tooling_route: "route.project.new.v1".to_string(),
        sdk_packet_family: "packet.project.*".to_string(),
        engine_truth_owner: "editor/project".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec!["PRJ_NO_WORKSPACE".to_string()],
    });

    registry.register(ActionDefinition {
        action_id: ActionId::new("project.open"),
        display_label: "Open Project".to_string(),
        action_family: ActionFamily::ProjectFile,
        tooling_route: "route.project.open.v1".to_string(),
        sdk_packet_family: "packet.project.*".to_string(),
        engine_truth_owner: "editor/project".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec![
            "PRJ_NO_WORKSPACE".to_string(),
            "PRJ_OPEN_FAILED".to_string(),
        ],
    });

    registry.register(ActionDefinition {
        action_id: ActionId::new("project.close"),
        display_label: "Close Project".to_string(),
        action_family: ActionFamily::ProjectFile,
        tooling_route: "route.project.close.v1".to_string(),
        sdk_packet_family: "packet.project.*".to_string(),
        engine_truth_owner: "editor/project".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec!["PRJ_CLOSE_FAILED".to_string()],
    });

    // Panel actions
    registry.register(ActionDefinition {
        action_id: ActionId::new("panel.open"),
        display_label: "Open Panel".to_string(),
        action_family: ActionFamily::ViewPanel,
        tooling_route: "route.panel.open.v1".to_string(),
        sdk_packet_family: "packet.ui.*".to_string(),
        engine_truth_owner: "editor/ui".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec!["PNL_OPEN_FAILED".to_string()],
    });

    registry.register(ActionDefinition {
        action_id: ActionId::new("panel.close"),
        display_label: "Close Panel".to_string(),
        action_family: ActionFamily::ViewPanel,
        tooling_route: "route.panel.close.v1".to_string(),
        sdk_packet_family: "packet.ui.*".to_string(),
        engine_truth_owner: "editor/ui".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec!["PNL_CLOSE_FAILED".to_string()],
    });
}
