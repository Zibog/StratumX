//! Unit tests for Action_Registry
//!
//! This test module validates the Action_Registry implementation for task 1.8:
//! - Test action registration with valid definition
//! - Test action lookup for registered actions
//! - Test action lookup for unregistered actions returns None
//! - Test query_available_actions with various contexts
//!
//! Validates: Requirements 2.1, 2.2, 2.7

use super::*;
use crate::{ActionFamily, ActionId, ActionResult, MutationClass, DenialFamily};
use uuid::Uuid;

// ============================================================================
// Test Helpers
// ============================================================================

/// Creates a test action definition with minimal required fields
fn create_test_action(
    action_id: &str,
    family: ActionFamily,
    mutation_class: MutationClass,
) -> ActionDefinition {
    fn dummy_handler(_context: ActionContext) -> ActionResult {
        ActionResult::success(FocusTarget::NoChange)
    }

    ActionDefinition {
        action_id: ActionId::new(action_id),
        display_label: format!("Test {}", action_id),
        action_family: family,
        tooling_route: format!("route.{}.v1", action_id),
        sdk_packet_family: format!("packet.{}.*", action_id.split('.').next().unwrap()),
        engine_truth_owner: "engine/test".to_string(),
        mutation_class,
        denial_families: vec![],
        handler: dummy_handler,
    }
}

/// Creates a test context with specified state
fn create_context(
    has_project: bool,
    has_world: bool,
    has_selection: bool,
    transaction_active: bool,
) -> ActionContext {
    ActionContext::from_legacy(
        if has_project { Some(Uuid::new_v4()) } else { None },
        if has_world { Some(Uuid::new_v4()) } else { None },
        if has_selection { vec![Uuid::new_v4()] } else { vec![] },
        None,
        transaction_active,
    )
}

// ============================================================================
// Action Registration Tests
// ============================================================================

#[test]
fn test_register_action_with_valid_definition() {
    let mut registry = ActionRegistry::new();
    
    let definition = create_test_action(
        "world.open",
        ActionFamily::World,
        MutationClass::Mutate,
    );
    
    registry.register(definition);
    
    // Verify action was registered
    assert_eq!(registry.len(), 1);
    assert!(!registry.is_empty());
    
    // Verify we can retrieve the action
    let retrieved = registry.get_handler(&ActionId::new("world.open"));
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().action_id.as_str(), "world.open");
}

#[test]
fn test_register_multiple_actions() {
    let mut registry = ActionRegistry::new();
    
    registry.register(create_test_action("world.open", ActionFamily::World, MutationClass::Mutate));
    registry.register(create_test_action("world.close", ActionFamily::World, MutationClass::Mutate));
    registry.register(create_test_action("world.save", ActionFamily::World, MutationClass::Mutate));
    registry.register(create_test_action("material.author", ActionFamily::Material, MutationClass::Mutate));
    registry.register(create_test_action("terrain.simulate", ActionFamily::Terrain, MutationClass::Simulate));
    
    assert_eq!(registry.len(), 5);
}

#[test]
fn test_register_action_preserves_all_fields() {
    let mut registry = ActionRegistry::new();
    
    fn test_handler(_context: ActionContext) -> ActionResult {
        ActionResult::success(Some("test_panel".to_string()))
    }
    
    let definition = ActionDefinition {
        action_id: ActionId::new("test.action"),
        display_label: "Test Action Label".to_string(),
        action_family: ActionFamily::Material,
        tooling_route: "route.test.action.v1".to_string(),
        sdk_packet_family: "packet.test.*".to_string(),
        engine_truth_owner: "engine/61".to_string(),
        mutation_class: MutationClass::Simulate,
        denial_families: vec![
            DenialFamily::MAT_PROFILE_INVALID,
            DenialFamily::MAT_BINDING_FAILED,
        ],
        handler: test_handler,
    };
    
    registry.register(definition);
    
    let retrieved = registry.get_handler(&ActionId::new("test.action")).unwrap();
    
    assert_eq!(retrieved.action_id.as_str(), "test.action");
    assert_eq!(retrieved.display_label, "Test Action Label");
    assert_eq!(retrieved.action_family, ActionFamily::Material);
    assert_eq!(retrieved.tooling_route, "route.test.action.v1");
    assert_eq!(retrieved.sdk_packet_family, "packet.test.*");
    assert_eq!(retrieved.engine_truth_owner, "engine/61");
    assert_eq!(retrieved.mutation_class, MutationClass::Simulate);
    assert_eq!(retrieved.denial_families.len(), 2);
    assert!(retrieved.denial_families.contains(&DenialFamily::MAT_PROFILE_INVALID));
    assert!(retrieved.denial_families.contains(&DenialFamily::MAT_BINDING_FAILED));
}

#[test]
fn test_register_replaces_existing_action() {
    let mut registry = ActionRegistry::new();
    
    // Register initial action
    let mut def1 = create_test_action("world.open", ActionFamily::World, MutationClass::Mutate);
    def1.display_label = "Original Label".to_string();
    registry.register(def1);
    
    assert_eq!(registry.len(), 1);
    
    // Register same action with different label
    let mut def2 = create_test_action("world.open", ActionFamily::World, MutationClass::Mutate);
    def2.display_label = "Updated Label".to_string();
    registry.register(def2);
    
    // Should still have only 1 action
    assert_eq!(registry.len(), 1);
    
    // Should have the updated label
    let retrieved = registry.get_handler(&ActionId::new("world.open")).unwrap();
    assert_eq!(retrieved.display_label, "Updated Label");
}

// ============================================================================
// Action Lookup Tests
// ============================================================================

#[test]
fn test_lookup_registered_action_returns_definition() {
    let mut registry = ActionRegistry::new();
    
    registry.register(create_test_action("world.open", ActionFamily::World, MutationClass::Mutate));
    registry.register(create_test_action("material.author", ActionFamily::Material, MutationClass::Mutate));
    
    // Lookup existing actions
    let world_open = registry.get_handler(&ActionId::new("world.open"));
    assert!(world_open.is_some());
    assert_eq!(world_open.unwrap().action_id.as_str(), "world.open");
    
    let material_author = registry.get_handler(&ActionId::new("material.author"));
    assert!(material_author.is_some());
    assert_eq!(material_author.unwrap().action_id.as_str(), "material.author");
}

#[test]
fn test_lookup_unregistered_action_returns_none() {
    let mut registry = ActionRegistry::new();
    
    registry.register(create_test_action("world.open", ActionFamily::World, MutationClass::Mutate));
    
    // Lookup non-existent action
    let result = registry.get_handler(&ActionId::new("nonexistent.action"));
    assert!(result.is_none());
    
    // Lookup similar but different action
    let result = registry.get_handler(&ActionId::new("world.close"));
    assert!(result.is_none());
}

#[test]
fn test_lookup_empty_registry_returns_none() {
    let registry = ActionRegistry::new();
    
    let result = registry.get_handler(&ActionId::new("any.action"));
    assert!(result.is_none());
}

#[test]
fn test_lookup_case_sensitive() {
    let mut registry = ActionRegistry::new();
    
    registry.register(create_test_action("world.open", ActionFamily::World, MutationClass::Mutate));
    
    // Exact match should work
    assert!(registry.get_handler(&ActionId::new("world.open")).is_some());
    
    // Different case should not match
    assert!(registry.get_handler(&ActionId::new("World.Open")).is_none());
    assert!(registry.get_handler(&ActionId::new("WORLD.OPEN")).is_none());
}

// ============================================================================
// Query Available Actions Tests
// ============================================================================

#[test]
fn test_query_available_actions_no_project() {
    let mut registry = ActionRegistry::new();
    
    // Register actions from different families
    registry.register(create_test_action("project.new", ActionFamily::ProjectFile, MutationClass::Mutate));
    registry.register(create_test_action("project.open", ActionFamily::ProjectFile, MutationClass::Mutate));
    registry.register(create_test_action("world.open", ActionFamily::World, MutationClass::Mutate));
    registry.register(create_test_action("panel.open", ActionFamily::ViewPanel, MutationClass::Read));
    registry.register(create_test_action("material.author", ActionFamily::Material, MutationClass::Mutate));
    
    let context = create_context(false, false, false, false);
    let available = registry.query_available_actions(&context);
    
    // Without a project, only ViewPanel and "new" actions should be available
    assert!(available.contains(&ActionId::new("panel.open")));
    assert!(available.contains(&ActionId::new("project.new")));
    
    // These should NOT be available without a project
    assert!(!available.contains(&ActionId::new("project.open")));
    assert!(!available.contains(&ActionId::new("world.open")));
    assert!(!available.contains(&ActionId::new("material.author")));
}

#[test]
fn test_query_available_actions_with_project_no_world() {
    let mut registry = ActionRegistry::new();
    
    registry.register(create_test_action("world.open", ActionFamily::World, MutationClass::Mutate));
    registry.register(create_test_action("world.close", ActionFamily::World, MutationClass::Mutate));
    registry.register(create_test_action("world.save", ActionFamily::World, MutationClass::Mutate));
    registry.register(create_test_action("material.author", ActionFamily::Material, MutationClass::Mutate));
    registry.register(create_test_action("panel.open", ActionFamily::ViewPanel, MutationClass::Read));
    
    let context = create_context(true, false, false, false);
    let available = registry.query_available_actions(&context);
    
    // With project but no world, world.open should be available
    assert!(available.contains(&ActionId::new("world.open")));
    assert!(available.contains(&ActionId::new("panel.open")));
    
    // These require an active world
    assert!(!available.contains(&ActionId::new("world.close")));
    assert!(!available.contains(&ActionId::new("world.save")));
    assert!(!available.contains(&ActionId::new("material.author")));
}

#[test]
fn test_query_available_actions_with_project_and_world() {
    let mut registry = ActionRegistry::new();
    
    registry.register(create_test_action("world.open", ActionFamily::World, MutationClass::Mutate));
    registry.register(create_test_action("world.close", ActionFamily::World, MutationClass::Mutate));
    registry.register(create_test_action("world.save", ActionFamily::World, MutationClass::Mutate));
    registry.register(create_test_action("material.author", ActionFamily::Material, MutationClass::Mutate));
    registry.register(create_test_action("terrain.simulate", ActionFamily::Terrain, MutationClass::Simulate));
    registry.register(create_test_action("sky.author", ActionFamily::SkyEnvironment, MutationClass::Mutate));
    registry.register(create_test_action("runtime.play", ActionFamily::Runtime, MutationClass::Mutate));
    
    let context = create_context(true, true, false, false);
    let available = registry.query_available_actions(&context);
    
    // With project and world, all these should be available
    assert!(available.contains(&ActionId::new("world.open")));
    assert!(available.contains(&ActionId::new("world.close")));
    assert!(available.contains(&ActionId::new("world.save")));
    assert!(available.contains(&ActionId::new("material.author")));
    assert!(available.contains(&ActionId::new("terrain.simulate")));
    assert!(available.contains(&ActionId::new("sky.author")));
    assert!(available.contains(&ActionId::new("runtime.play")));
}

#[test]
fn test_query_available_actions_diagnostics_and_build_families() {
    let mut registry = ActionRegistry::new();
    
    registry.register(create_test_action("diagnostics.inspect", ActionFamily::DiagnosticsProof, MutationClass::Read));
    registry.register(create_test_action("build.export", ActionFamily::BuildRelease, MutationClass::Mutate));
    
    // Without project
    let context_no_project = create_context(false, false, false, false);
    let available = registry.query_available_actions(&context_no_project);
    assert!(!available.contains(&ActionId::new("diagnostics.inspect")));
    assert!(!available.contains(&ActionId::new("build.export")));
    
    // With project
    let context_with_project = create_context(true, false, false, false);
    let available = registry.query_available_actions(&context_with_project);
    assert!(available.contains(&ActionId::new("diagnostics.inspect")));
    assert!(available.contains(&ActionId::new("build.export")));
}

#[test]
fn test_query_available_actions_view_panel_always_available() {
    let mut registry = ActionRegistry::new();
    
    registry.register(create_test_action("panel.open", ActionFamily::ViewPanel, MutationClass::Read));
    registry.register(create_test_action("panel.close", ActionFamily::ViewPanel, MutationClass::Read));
    registry.register(create_test_action("panel.dock", ActionFamily::ViewPanel, MutationClass::Mutate));
    
    // Test with no project, no world
    let context1 = create_context(false, false, false, false);
    let available1 = registry.query_available_actions(&context1);
    assert_eq!(available1.len(), 3);
    assert!(available1.contains(&ActionId::new("panel.open")));
    assert!(available1.contains(&ActionId::new("panel.close")));
    assert!(available1.contains(&ActionId::new("panel.dock")));
    
    // Test with project and world - should still be available
    let context2 = create_context(true, true, true, false);
    let available2 = registry.query_available_actions(&context2);
    assert!(available2.contains(&ActionId::new("panel.open")));
    assert!(available2.contains(&ActionId::new("panel.close")));
    assert!(available2.contains(&ActionId::new("panel.dock")));
}

#[test]
fn test_query_available_actions_empty_registry() {
    let registry = ActionRegistry::new();
    
    let context = create_context(true, true, true, false);
    let available = registry.query_available_actions(&context);
    
    assert!(available.is_empty());
}

#[test]
fn test_query_available_actions_with_selection() {
    let mut registry = ActionRegistry::new();
    
    // Register various actions
    registry.register(create_test_action("world.open", ActionFamily::World, MutationClass::Mutate));
    registry.register(create_test_action("material.author", ActionFamily::Material, MutationClass::Mutate));
    
    // Selection doesn't affect basic availability (that's handled in dispatch validation)
    let context_no_selection = create_context(true, true, false, false);
    let available_no_selection = registry.query_available_actions(&context_no_selection);
    
    let context_with_selection = create_context(true, true, true, false);
    let available_with_selection = registry.query_available_actions(&context_with_selection);
    
    // Both should return the same actions (selection validation happens at dispatch time)
    assert_eq!(available_no_selection.len(), available_with_selection.len());
}

#[test]
fn test_query_available_actions_with_transaction() {
    let mut registry = ActionRegistry::new();
    
    registry.register(create_test_action("world.save", ActionFamily::World, MutationClass::Mutate));
    registry.register(create_test_action("material.author", ActionFamily::Material, MutationClass::Mutate));
    
    // Transaction state doesn't affect basic availability (that's handled in dispatch validation)
    let context_no_transaction = create_context(true, true, false, false);
    let available_no_transaction = registry.query_available_actions(&context_no_transaction);
    
    let context_with_transaction = create_context(true, true, false, true);
    let available_with_transaction = registry.query_available_actions(&context_with_transaction);
    
    // Both should return the same actions (transaction validation happens at dispatch time)
    assert_eq!(available_no_transaction.len(), available_with_transaction.len());
}

// ============================================================================
// Registry Utility Method Tests
// ============================================================================

#[test]
fn test_registry_len_and_is_empty() {
    let mut registry = ActionRegistry::new();
    
    assert_eq!(registry.len(), 0);
    assert!(registry.is_empty());
    
    registry.register(create_test_action("world.open", ActionFamily::World, MutationClass::Mutate));
    assert_eq!(registry.len(), 1);
    assert!(!registry.is_empty());
    
    registry.register(create_test_action("world.close", ActionFamily::World, MutationClass::Mutate));
    assert_eq!(registry.len(), 2);
    assert!(!registry.is_empty());
}

#[test]
fn test_registry_action_ids_iterator() {
    let mut registry = ActionRegistry::new();
    
    registry.register(create_test_action("world.open", ActionFamily::World, MutationClass::Mutate));
    registry.register(create_test_action("world.close", ActionFamily::World, MutationClass::Mutate));
    registry.register(create_test_action("material.author", ActionFamily::Material, MutationClass::Mutate));
    
    let ids: Vec<&ActionId> = registry.action_ids().collect();
    assert_eq!(ids.len(), 3);
    
    // Verify all expected IDs are present
    let id_strings: Vec<&str> = ids.iter().map(|id| id.as_str()).collect();
    assert!(id_strings.contains(&"world.open"));
    assert!(id_strings.contains(&"world.close"));
    assert!(id_strings.contains(&"material.author"));
}

#[test]
fn test_registry_definitions_iterator() {
    let mut registry = ActionRegistry::new();
    
    registry.register(create_test_action("world.open", ActionFamily::World, MutationClass::Mutate));
    registry.register(create_test_action("material.author", ActionFamily::Material, MutationClass::Mutate));
    
    let defs: Vec<&ActionDefinition> = registry.definitions().collect();
    assert_eq!(defs.len(), 2);
    
    // Verify definitions have correct families
    let families: Vec<ActionFamily> = defs.iter().map(|def| def.action_family).collect();
    assert!(families.contains(&ActionFamily::World));
    assert!(families.contains(&ActionFamily::Material));
}

#[test]
fn test_registry_default_creates_empty() {
    let registry = ActionRegistry::default();
    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_register_action_with_empty_denial_families() {
    let mut registry = ActionRegistry::new();
    
    let mut definition = create_test_action("test.action", ActionFamily::World, MutationClass::Read);
    definition.denial_families = vec![];
    
    registry.register(definition);
    
    let retrieved = registry.get_handler(&ActionId::new("test.action")).unwrap();
    assert!(retrieved.denial_families.is_empty());
}

#[test]
fn test_register_action_with_multiple_denial_families() {
    let mut registry = ActionRegistry::new();
    
    let mut definition = create_test_action("test.action", ActionFamily::Material, MutationClass::Mutate);
    definition.denial_families = vec![
        DenialFamily::MAT_PROFILE_INVALID,
        DenialFamily::MAT_BINDING_FAILED,
        DenialFamily::PRJ_NO_WORKSPACE,
        DenialFamily::WLD_OPEN_FAILED,
    ];
    
    registry.register(definition);
    
    let retrieved = registry.get_handler(&ActionId::new("test.action")).unwrap();
    assert_eq!(retrieved.denial_families.len(), 4);
}

#[test]
fn test_query_available_actions_all_action_families() {
    let mut registry = ActionRegistry::new();
    
    // Register one action from each family
    registry.register(create_test_action("project.new", ActionFamily::ProjectFile, MutationClass::Mutate));
    registry.register(create_test_action("panel.open", ActionFamily::ViewPanel, MutationClass::Read));
    registry.register(create_test_action("world.open", ActionFamily::World, MutationClass::Mutate));
    registry.register(create_test_action("terrain.simulate", ActionFamily::Terrain, MutationClass::Simulate));
    registry.register(create_test_action("material.author", ActionFamily::Material, MutationClass::Mutate));
    registry.register(create_test_action("sky.author", ActionFamily::SkyEnvironment, MutationClass::Mutate));
    registry.register(create_test_action("runtime.play", ActionFamily::Runtime, MutationClass::Mutate));
    registry.register(create_test_action("diagnostics.inspect", ActionFamily::DiagnosticsProof, MutationClass::Read));
    registry.register(create_test_action("build.export", ActionFamily::BuildRelease, MutationClass::Mutate));
    
    // With full context (project + world)
    let context = create_context(true, true, false, false);
    let available = registry.query_available_actions(&context);
    
    // All actions should be available
    assert_eq!(available.len(), 9);
}
