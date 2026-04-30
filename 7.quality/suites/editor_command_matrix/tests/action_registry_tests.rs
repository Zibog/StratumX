//! Tests for Action Registry
//!
//! Moved from 5.editor/l7.0-editor-command-spine/src/action_registry.rs
//! during gold cleanup.

use stratumx_editor_l7_0_editor_command_spine::{
    ActionContext, ActionDefinition, ActionFamily, ActionId, ActionRegistry, MutationClass,
};

fn create_test_definition(action_id: &str, family: ActionFamily) -> ActionDefinition {
    ActionDefinition {
        action_id: ActionId::new(action_id),
        display_label: format!("Test {}", action_id),
        action_family: family,
        tooling_route: format!("route.{}.v1", action_id),
        sdk_packet_family: "packet.test.*".to_string(),
        engine_truth_owner: "engine/test".to_string(),
        mutation_class: MutationClass::Read,
        possible_denial_families: vec![],
    }
}

#[test]
fn test_new_registry_is_empty() {
    let registry = ActionRegistry::new();
    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);
}

#[test]
fn test_register_action() {
    let mut registry = ActionRegistry::new();
    let definition = create_test_definition("world.open", ActionFamily::World);

    registry.register(definition);

    assert_eq!(registry.len(), 1);
    assert!(!registry.is_empty());
}

#[test]
fn test_get_handler_found() {
    let mut registry = ActionRegistry::new();
    registry.register(create_test_definition("world.open", ActionFamily::World));

    let result = registry.get_handler(&ActionId::new("world.open"));
    assert!(result.is_some());
    assert_eq!(result.unwrap().action_id.as_str(), "world.open");
}

#[test]
fn test_stub_handler_execution() {
    use stratumx_editor_l7_0_editor_command_spine::action_registry::stub_handlers;

    let mut registry = ActionRegistry::new();
    registry.register(create_test_definition("world.open", ActionFamily::World));
    registry.register_stub_handler(
        ActionId::new("world.open"),
        stub_handlers::stub_viewport_focus,
    );

    let context = ActionContext::test_empty();
    let result = registry.invoke_stub(&ActionId::new("world.open"), &context);

    assert!(result.is_some());
    assert!(result.unwrap().success);
}
