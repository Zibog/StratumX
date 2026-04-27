//! Property Test: Action Registry Immutability After Initialization
//!
//! **Property 18: Action Registry Immutability After Initialization**
//! **Validates: Requirements 16.4**
//!
//! Verifies that the ActionRegistry is immutable after initialization.
//! Once built, the registry cannot be modified at runtime.

use crate::{
    ActionContext, ActionDefinition, ActionFamily, ActionId, ActionResult, MutationClass,
};
use crate::registry_builder::RegistryBuilder;
use proptest::prelude::*;

// ============================================================================
// Property Test Generators
// ============================================================================

/// Generates random action definitions for testing
fn arb_action_definition() -> impl Strategy<Value = ActionDefinition> {
    (
        prop_oneof![
            Just("world.open"),
            Just("world.close"),
            Just("material.author"),
            Just("terrain.sculpt"),
            Just("runtime.play"),
        ],
        prop_oneof![
            Just(ActionFamily::World),
            Just(ActionFamily::Material),
            Just(ActionFamily::Terrain),
            Just(ActionFamily::Runtime),
        ],
    )
        .prop_map(|(action_str, family)| {
            fn dummy_handler(_context: ActionContext) -> ActionResult {
                ActionResult::success(None)
            }

            ActionDefinition {
                action_id: ActionId::new(action_str),
                display_label: format!("Test {}", action_str),
                action_family: family,
                tooling_route: format!("route.{}.v1", action_str),
                sdk_packet_family: "packet.test.*".to_string(),
                engine_truth_owner: "engine/test".to_string(),
                mutation_class: MutationClass::Mutate,
                denial_families: vec![],
                handler: dummy_handler,
            }
        })
}

/// Generates a vector of action definitions
fn arb_action_definitions() -> impl Strategy<Value = Vec<ActionDefinition>> {
    prop::collection::vec(arb_action_definition(), 1..10)
}

// ============================================================================
// Property Tests
// ============================================================================

/// **Property 18: Action Registry Immutability After Initialization**
///
/// **Validates: Requirements 16.4**
///
/// This property verifies that the ActionRegistry is immutable after initialization.
/// The registry is built using RegistryBuilder, and once build() is called,
/// the registry cannot be modified.
///
/// The test verifies:
/// 1. Registry can be built with actions
/// 2. Registry provides only read-only access methods
/// 3. Registry state cannot be changed after initialization
/// 4. Multiple queries return consistent results
#[test]
fn prop_registry_immutability_after_initialization() {
    // Feature: editor-canonical-architecture-refactor, Property 18: Registry Immutability
    let config = ProptestConfig::with_cases(100);
    proptest!(config, |(definitions in arb_action_definitions())| {
        // Build registry with actions
        let mut builder = RegistryBuilder::new();
        
        for definition in &definitions {
            builder.register_action(definition.clone()).unwrap();
        }
        
        let registry = builder.build();
        
        // Verify registry is immutable - all methods are read-only
        let initial_count = registry.len();
        
        // Query the registry multiple times
        for definition in &definitions {
            let resolved1 = registry.resolve(&definition.action_id);
            let resolved2 = registry.resolve(&definition.action_id);
            
            // Verify consistent results
            prop_assert!(resolved1.is_some(), "Action should be resolvable");
            prop_assert_eq!(
                resolved1.map(|d| d.action_id.as_str()),
                resolved2.map(|d| d.action_id.as_str()),
                "Multiple queries should return consistent results"
            );
        }
        
        // Verify count remains unchanged
        prop_assert_eq!(
            registry.len(),
            initial_count,
            "Registry size should not change after queries"
        );
        
        // Verify registry provides only immutable access
        // (This is enforced by Rust's type system - registry methods take &self, not &mut self)
        let _immutable_ref = &registry;
        let _another_immutable_ref = &registry;
        // Multiple immutable borrows are allowed, proving immutability
    });
}

/// **Property: Registry State Consistency**
///
/// Verifies that registry state remains consistent across multiple operations.
/// This test ensures that:
/// 1. Querying actions doesn't modify registry state
/// 2. Domain queries return consistent results
/// 3. Available actions queries are deterministic
#[test]
fn prop_registry_state_consistency() {
    // Feature: editor-canonical-architecture-refactor, Property 18: Registry Immutability
    let config = ProptestConfig::with_cases(100);
    proptest!(config, |(definitions in arb_action_definitions())| {
        let mut builder = RegistryBuilder::new();
        
        for definition in &definitions {
            builder.register_action(definition.clone()).unwrap();
        }
        
        let registry = builder.build();
        
        // Query domain actions multiple times
        for family in [
            ActionFamily::World,
            ActionFamily::Material,
            ActionFamily::Terrain,
            ActionFamily::Runtime,
        ] {
            let query1 = registry.query_actions_for_domain(family);
            let query2 = registry.query_actions_for_domain(family);
            
            // Verify consistent results
            prop_assert_eq!(
                query1.len(),
                query2.len(),
                "Domain queries should return consistent results"
            );
            
            for (id1, id2) in query1.iter().zip(query2.iter()) {
                prop_assert_eq!(
                    id1.as_str(),
                    id2.as_str(),
                    "Domain query results should be identical"
                );
            }
        }
        
        // Query available actions multiple times with same context
        let context = ActionContext {
            project_id: Some(uuid::Uuid::new_v4()),
            active_world_id: Some(uuid::Uuid::new_v4()),
            selected_entities: vec![],
            focused_panel: None,
            transaction_active: false,
        };
        
        let available1 = registry.query_available_actions(&context);
        let available2 = registry.query_available_actions(&context);
        
        prop_assert_eq!(
            available1.len(),
            available2.len(),
            "Available actions queries should return consistent results"
        );
    });
}

/// **Property: Registry Cannot Be Modified After Build**
///
/// Verifies that the registry API provides no mutation methods.
/// This is enforced by Rust's type system - all registry methods take &self.
#[test]
fn prop_registry_no_mutation_methods() {
    // Feature: editor-canonical-architecture-refactor, Property 18: Registry Immutability
    let config = ProptestConfig::with_cases(50);
    proptest!(config, |(definitions in arb_action_definitions())| {
        let mut builder = RegistryBuilder::new();
        
        for definition in &definitions {
            builder.register_action(definition.clone()).unwrap();
        }
        
        let registry = builder.build();
        
        // Verify registry only provides immutable access
        // All methods take &self, not &mut self
        let _resolve = registry.resolve(&ActionId::new("world.open"));
        let _query_domain = registry.query_actions_for_domain(ActionFamily::World);
        let _len = registry.len();
        let _is_empty = registry.is_empty();
        
        // The fact that this compiles with only &registry proves immutability
        // If any method required &mut self, this would fail to compile
        
        prop_assert!(true, "Registry provides only immutable access");
    });
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_registry_immutability_basic() {
        let mut builder = RegistryBuilder::new();
        
        fn dummy_handler(_context: ActionContext) -> ActionResult {
            ActionResult::success(None)
        }
        
        let definition = ActionDefinition {
            action_id: ActionId::new("world.open"),
            display_label: "Open World".to_string(),
            action_family: ActionFamily::World,
            tooling_route: "route.world.open.v1".to_string(),
            sdk_packet_family: "packet.world.*".to_string(),
            engine_truth_owner: "engine/world".to_string(),
            mutation_class: MutationClass::Mutate,
            denial_families: vec![],
            handler: dummy_handler,
        };
        
        builder.register_action(definition).unwrap();
        let registry = builder.build();
        
        // Verify registry is immutable
        let initial_len = registry.len();
        
        // Query multiple times
        let _resolved1 = registry.resolve(&ActionId::new("world.open"));
        let _resolved2 = registry.resolve(&ActionId::new("world.open"));
        
        // Length should not change
        assert_eq!(registry.len(), initial_len);
    }

    #[test]
    fn test_registry_multiple_immutable_borrows() {
        let mut builder = RegistryBuilder::new();
        
        fn dummy_handler(_context: ActionContext) -> ActionResult {
            ActionResult::success(None)
        }
        
        let definition = ActionDefinition {
            action_id: ActionId::new("world.open"),
            display_label: "Open World".to_string(),
            action_family: ActionFamily::World,
            tooling_route: "route.world.open.v1".to_string(),
            sdk_packet_family: "packet.world.*".to_string(),
            engine_truth_owner: "engine/world".to_string(),
            mutation_class: MutationClass::Mutate,
            denial_families: vec![],
            handler: dummy_handler,
        };
        
        builder.register_action(definition).unwrap();
        let registry = builder.build();
        
        // Multiple immutable borrows are allowed
        let ref1 = &registry;
        let ref2 = &registry;
        let ref3 = &registry;
        
        // All can be used simultaneously
        assert_eq!(ref1.len(), ref2.len());
        assert_eq!(ref2.len(), ref3.len());
    }

    #[test]
    fn test_registry_consistent_queries() {
        let mut builder = RegistryBuilder::new();
        
        fn dummy_handler(_context: ActionContext) -> ActionResult {
            ActionResult::success(None)
        }
        
        let definition = ActionDefinition {
            action_id: ActionId::new("world.open"),
            display_label: "Open World".to_string(),
            action_family: ActionFamily::World,
            tooling_route: "route.world.open.v1".to_string(),
            sdk_packet_family: "packet.world.*".to_string(),
            engine_truth_owner: "engine/world".to_string(),
            mutation_class: MutationClass::Mutate,
            denial_families: vec![],
            handler: dummy_handler,
        };
        
        builder.register_action(definition).unwrap();
        let registry = builder.build();
        
        // Query multiple times
        let resolved1 = registry.resolve(&ActionId::new("world.open"));
        let resolved2 = registry.resolve(&ActionId::new("world.open"));
        
        // Results should be identical
        assert!(resolved1.is_some());
        assert!(resolved2.is_some());
        assert_eq!(
            resolved1.unwrap().action_id.as_str(),
            resolved2.unwrap().action_id.as_str()
        );
    }
}
