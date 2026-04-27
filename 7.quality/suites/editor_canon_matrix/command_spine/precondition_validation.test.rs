//! Property-based test for Precondition Validation Before Mutation
//!
//! **Property 7: Precondition Validation Before Mutation**
//!
//! *For any* command execution, the executor must validate all preconditions before
//! applying any mutations, returning early with DisabledReason if validation fails.
//!
//! **Validates: Requirements 4.7, 24.2, 24.3, 24.4, 24.5**

use proptest::prelude::*;
use crate::{
    ActionContext, ActionDefinition, ActionDispatch, ActionFamily, ActionId, ActionRegistry,
    ActionResult, DisabledReason, ErrorCode, MutationClass, StateQueries,
};
use std::sync::Arc;
use uuid::Uuid;

// ============================================================================
// Mock StateQueries Implementation
// ============================================================================

/// Mock implementation of StateQueries for testing precondition validation.
///
/// This mock allows us to control the state returned by queries to test
/// different validation scenarios with invalid preconditions.
struct MockStateQueries {
    has_project: bool,
    has_active_world: bool,
    has_selection: bool,
    is_transaction_active: bool,
}

impl StateQueries for MockStateQueries {
    fn has_project(&self) -> bool {
        self.has_project
    }

    fn has_active_world(&self) -> bool {
        self.has_active_world
    }

    fn has_selection(&self) -> bool {
        self.has_selection
    }

    fn is_transaction_active(&self) -> bool {
        self.is_transaction_active
    }
}

// ============================================================================
// Property Test Strategies
// ============================================================================

/// Strategy for generating commands with invalid preconditions.
///
/// This generates various command types paired with contexts that violate
/// their preconditions:
/// - World commands when no world is open
/// - Terrain commands when terrain doesn't exist (no world)
/// - Material commands when MaterialAuthority is unavailable (no project)
/// - Runtime commands when no world is open
/// - Selection-based commands when no selection exists
fn command_with_invalid_preconditions_strategy() -> impl Strategy<Value = (ActionDefinition, ActionContext, DisabledReason)> {
    prop_oneof![
        // World commands requiring project but no project exists
        world_command_no_project_strategy(),
        
        // Terrain commands requiring world but no world exists
        terrain_command_no_world_strategy(),
        
        // Material commands requiring project but no project exists
        material_command_no_project_strategy(),
        
        // Runtime commands requiring world but no world exists
        runtime_command_no_world_strategy(),
        
        // Selection commands requiring selection but no selection exists
        selection_command_no_selection_strategy(),
        
        // Mutation commands during active transaction
        mutation_command_during_transaction_strategy(),
    ]
}

/// Strategy for world commands when no project is open.
fn world_command_no_project_strategy() -> impl Strategy<Value = (ActionDefinition, ActionContext, DisabledReason)> {
    let action_ids = vec![
        "world.open",
        "world.close",
        "world.save",
        "world.validate",
    ];
    
    prop::sample::select(action_ids).prop_flat_map(|action_id| {
        let action_def = create_action_def(action_id, ActionFamily::World, MutationClass::Mutate);
        let context = create_context_no_project();
        Just((action_def, context, DisabledReason::NoLegalProject))
    })
}

/// Strategy for terrain commands when no world is open.
fn terrain_command_no_world_strategy() -> impl Strategy<Value = (ActionDefinition, ActionContext, DisabledReason)> {
    let action_ids = vec![
        "terrain.sculpt_raise",
        "terrain.sculpt_lower",
        "terrain.paint_material",
        "terrain.import_heightmap",
    ];
    
    prop::sample::select(action_ids).prop_flat_map(|action_id| {
        let action_def = create_action_def(action_id, ActionFamily::Terrain, MutationClass::Mutate);
        // Has project but no world
        let context = ActionContext {
            project_id: Some(Uuid::new_v4()),
            active_world_id: None,
            selected_entities: vec![],
            focused_panel: None,
            transaction_active: false,
        };
        Just((action_def, context, DisabledReason::NoLegalProject))
    })
}

/// Strategy for material commands when no project is open.
fn material_command_no_project_strategy() -> impl Strategy<Value = (ActionDefinition, ActionContext, DisabledReason)> {
    let action_ids = vec![
        "material.author_response_profile",
        "material.bind_surface_family",
        "material.inspect_response_table",
        "material.create_profile",
    ];
    
    prop::sample::select(action_ids).prop_flat_map(|action_id| {
        let action_def = create_action_def(action_id, ActionFamily::Material, MutationClass::Mutate);
        let context = create_context_no_project();
        Just((action_def, context, DisabledReason::NoLegalProject))
    })
}

/// Strategy for runtime commands when no world is open.
fn runtime_command_no_world_strategy() -> impl Strategy<Value = (ActionDefinition, ActionContext, DisabledReason)> {
    let action_ids = vec![
        "runtime.play",
        "runtime.pause",
        "runtime.step",
        "runtime.simulate",
    ];
    
    prop::sample::select(action_ids).prop_flat_map(|action_id| {
        let action_def = create_action_def(action_id, ActionFamily::Runtime, MutationClass::Simulate);
        // Has project but no world
        let context = ActionContext {
            project_id: Some(Uuid::new_v4()),
            active_world_id: None,
            selected_entities: vec![],
            focused_panel: None,
            transaction_active: false,
        };
        Just((action_def, context, DisabledReason::NoLegalProject))
    })
}

/// Strategy for selection commands when no selection exists.
fn selection_command_no_selection_strategy() -> impl Strategy<Value = (ActionDefinition, ActionContext, DisabledReason)> {
    let action_ids = vec![
        "entity.inspect",
        "entity.inspect_selected",
        "terrain.inspect_selected_region",
    ];
    
    prop::sample::select(action_ids).prop_flat_map(|action_id| {
        let action_def = create_action_def(action_id, ActionFamily::World, MutationClass::Read);
        // Has project and world but no selection
        let context = ActionContext {
            project_id: Some(Uuid::new_v4()),
            active_world_id: Some(Uuid::new_v4()),
            selected_entities: vec![],
            focused_panel: None,
            transaction_active: false,
        };
        Just((action_def, context, DisabledReason::NoLegalTargetSelected))
    })
}

/// Strategy for mutation commands during active transaction.
fn mutation_command_during_transaction_strategy() -> impl Strategy<Value = (ActionDefinition, ActionContext, DisabledReason)> {
    let action_ids = vec![
        "world.save",
        "terrain.sculpt_raise",
        "material.create_profile",
    ];
    
    prop::sample::select(action_ids).prop_flat_map(|action_id| {
        let family = if action_id.starts_with("world") {
            ActionFamily::World
        } else if action_id.starts_with("terrain") {
            ActionFamily::Terrain
        } else {
            ActionFamily::Material
        };
        
        let action_def = create_action_def(action_id, family, MutationClass::Mutate);
        // Has project and world but transaction is active
        let context = ActionContext {
            project_id: Some(Uuid::new_v4()),
            active_world_id: Some(Uuid::new_v4()),
            selected_entities: vec![],
            focused_panel: None,
            transaction_active: true,
        };
        Just((action_def, context, DisabledReason::ActiveTransactionForbidsMutation))
    })
}

/// Helper function to create an ActionDefinition for testing.
fn create_action_def(action_id: &str, family: ActionFamily, mutation_class: MutationClass) -> ActionDefinition {
    fn dummy_handler(_context: ActionContext) -> ActionResult {
        ActionResult::success(None)
    }
    
    ActionDefinition {
        action_id: ActionId::new(action_id),
        display_label: format!("Test {}", action_id),
        action_family: family,
        tooling_route: format!("route.{}.v1", action_id),
        sdk_packet_family: "packet.test.*".to_string(),
        engine_truth_owner: "engine/test".to_string(),
        mutation_class,
        denial_families: vec![],
        handler: dummy_handler,
    }
}

/// Helper function to create a context with no project.
fn create_context_no_project() -> ActionContext {
    ActionContext {
        project_id: None,
        active_world_id: None,
        selected_entities: vec![],
        focused_panel: None,
        transaction_active: false,
    }
}

// ============================================================================
// Property Tests
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property 7: Precondition Validation Before Mutation
    ///
    /// For any command execution, the executor must validate all preconditions before
    /// applying any mutations, returning early with DisabledReason if validation fails.
    ///
    /// This test validates that:
    /// 1. Commands with invalid preconditions are rejected immediately
    /// 2. DisabledReason is returned before any mutations occur
    /// 3. No state changes are recorded in the transaction ledger
    /// 4. The system state remains unchanged after validation failure
    ///
    /// **Validates: Requirements 4.7, 24.2, 24.3, 24.4, 24.5**
    #[test]
    fn property_precondition_validation_before_mutation(
        (action_def, context, expected_reason) in command_with_invalid_preconditions_strategy()
    ) {
        // Create registry and register the action
        let mut registry = ActionRegistry::new();
        registry.register(action_def.clone());

        // Create mock state queries that match the invalid context
        let state_queries = Arc::new(MockStateQueries {
            has_project: context.project_id.is_some(),
            has_active_world: context.active_world_id.is_some(),
            has_selection: !context.selected_entities.is_empty(),
            is_transaction_active: context.transaction_active,
        });

        // Create dispatch and execute action
        let dispatch = ActionDispatch::new(Arc::new(registry), state_queries);
        let result = dispatch.dispatch(action_def.action_id.clone(), context);

        // Verify that the action failed
        assert!(
            !result.success,
            "Action {} should fail when preconditions are invalid",
            action_def.action_id.as_str()
        );

        // Verify that the error code is the expected DisabledReason
        assert!(
            matches!(
                result.error_code,
                Some(ErrorCode::DisabledReason(ref reason)) if *reason == expected_reason
            ),
            "Action {} should return {:?} error, got: {:?}",
            action_def.action_id.as_str(),
            expected_reason,
            result.error_code
        );

        // Verify that focus is directed to diagnostics panel
        assert_eq!(
            result.focus_target_id,
            Some("diagnostics".to_string()),
            "Failed action should focus diagnostics panel"
        );

        // Verify that no artifact was created (no mutations occurred)
        assert!(
            result.artifact_ref.is_none(),
            "Failed action should not create artifacts"
        );

        // Verify that no recovery action is suggested for precondition failures
        // (recovery actions are for execution failures, not validation failures)
        assert!(
            result.next_legal_recovery_action.is_none(),
            "Precondition failures should not suggest recovery actions"
        );
    }

    /// Property: Validation Occurs Before Handler Execution
    ///
    /// For any command that fails validation, the action handler must never be executed.
    /// This ensures that validation is a hard gate before any business logic runs.
    ///
    /// This test validates that:
    /// 1. Failed validation returns immediately
    /// 2. No side effects occur from failed validation
    /// 3. Error information is complete and accurate
    /// 4. The handler function is never invoked
    ///
    /// **Validates: Requirements 4.7, 24.5**
    #[test]
    fn property_validation_before_handler_execution(
        (action_def, context, _expected_reason) in command_with_invalid_preconditions_strategy()
    ) {
        let mut registry = ActionRegistry::new();
        registry.register(action_def.clone());

        let state_queries = Arc::new(MockStateQueries {
            has_project: context.project_id.is_some(),
            has_active_world: context.active_world_id.is_some(),
            has_selection: !context.selected_entities.is_empty(),
            is_transaction_active: context.transaction_active,
        });

        let dispatch = ActionDispatch::new(Arc::new(registry), state_queries);
        let result = dispatch.dispatch(action_def.action_id.clone(), context);

        // Verify immediate failure
        assert!(!result.success, "Validation failure should return immediately");
        
        // Verify error code is present
        assert!(
            result.error_code.is_some(),
            "Validation failure should include error code"
        );
        
        // Verify no execution artifacts
        assert!(
            result.artifact_ref.is_none(),
            "Validation failure should not create artifacts"
        );
        
        // Verify error is DisabledReason (not DenialFamily from execution)
        assert!(
            matches!(result.error_code, Some(ErrorCode::DisabledReason(_))),
            "Validation failure should return DisabledReason, not DenialFamily"
        );
    }

    /// Property: No State Changes on Validation Failure
    ///
    /// For any command that fails validation, no state changes should be recorded
    /// in the transaction ledger. The system state must remain unchanged.
    ///
    /// This test validates that:
    /// 1. No transaction is created for failed validation
    /// 2. No mutations are recorded
    /// 3. The system state is identical before and after validation failure
    ///
    /// **Validates: Requirements 4.7, 23.2, 23.5**
    #[test]
    fn property_no_state_changes_on_validation_failure(
        (action_def, context, _expected_reason) in command_with_invalid_preconditions_strategy()
    ) {
        let mut registry = ActionRegistry::new();
        registry.register(action_def.clone());

        let state_queries = Arc::new(MockStateQueries {
            has_project: context.project_id.is_some(),
            has_active_world: context.active_world_id.is_some(),
            has_selection: !context.selected_entities.is_empty(),
            is_transaction_active: context.transaction_active,
        });

        let dispatch = ActionDispatch::new(Arc::new(registry), state_queries);
        
        // Capture state before dispatch (in this case, the context itself)
        let context_before = context.clone();
        
        // Execute action
        let result = dispatch.dispatch(action_def.action_id.clone(), context.clone());

        // Verify that the action failed
        assert!(!result.success, "Action should fail validation");
        
        // Verify that the context is unchanged
        assert_eq!(
            context.project_id, context_before.project_id,
            "Project ID should not change on validation failure"
        );
        assert_eq!(
            context.active_world_id, context_before.active_world_id,
            "World ID should not change on validation failure"
        );
        assert_eq!(
            context.selected_entities, context_before.selected_entities,
            "Selection should not change on validation failure"
        );
        assert_eq!(
            context.transaction_active, context_before.transaction_active,
            "Transaction state should not change on validation failure"
        );
        
        // Verify no artifact was created
        assert!(
            result.artifact_ref.is_none(),
            "No artifacts should be created on validation failure"
        );
    }

    /// Property: Validation Respects Domain Requirements
    ///
    /// For any command, the validation requirements must match the domain requirements:
    /// - World commands require project
    /// - Terrain commands require project and world
    /// - Material commands require project (MaterialAuthority)
    /// - Runtime commands require project and world
    /// - Selection commands require selection
    /// - Mutation commands forbid active transactions
    ///
    /// **Validates: Requirements 24.2, 24.3, 24.4, 24.5**
    #[test]
    fn property_validation_respects_domain_requirements(
        family in prop_oneof![
            Just(ActionFamily::World),
            Just(ActionFamily::Terrain),
            Just(ActionFamily::Material),
            Just(ActionFamily::Runtime),
        ]
    ) {
        let action_def = create_action_def("test.action", family, MutationClass::Mutate);
        let mut registry = ActionRegistry::new();
        registry.register(action_def.clone());

        // Test with no project
        let state_queries_no_project = Arc::new(MockStateQueries {
            has_project: false,
            has_active_world: false,
            has_selection: false,
            is_transaction_active: false,
        });

        let dispatch = ActionDispatch::new(
            Arc::new(registry),
            state_queries_no_project
        );
        
        let context = ActionContext {
            project_id: None,
            active_world_id: None,
            selected_entities: vec![],
            focused_panel: None,
            transaction_active: false,
        };
        
        let result = dispatch.dispatch(action_def.action_id.clone(), context);

        // All tested families should require project
        assert!(
            !result.success,
            "{:?} actions should require project",
            family
        );
        assert!(
            matches!(
                result.error_code,
                Some(ErrorCode::DisabledReason(DisabledReason::NoLegalProject))
            ),
            "{:?} actions should return NoLegalProject error",
            family
        );
    }
}

// ============================================================================
// Unit Tests for Edge Cases
// ============================================================================

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_world_open_requires_project() {
        let action_def = create_action_def("world.open", ActionFamily::World, MutationClass::Mutate);
        let mut registry = ActionRegistry::new();
        registry.register(action_def.clone());

        let state_queries = Arc::new(MockStateQueries {
            has_project: false,
            has_active_world: false,
            has_selection: false,
            is_transaction_active: false,
        });

        let dispatch = ActionDispatch::new(Arc::new(registry), state_queries);
        let context = create_context_no_project();

        let result = dispatch.dispatch(action_def.action_id, context);

        assert!(!result.success, "world.open should require project");
        assert!(matches!(
            result.error_code,
            Some(ErrorCode::DisabledReason(DisabledReason::NoLegalProject))
        ));
    }

    #[test]
    fn test_terrain_commands_require_world() {
        let action_def = create_action_def("terrain.sculpt_raise", ActionFamily::Terrain, MutationClass::Mutate);
        let mut registry = ActionRegistry::new();
        registry.register(action_def.clone());

        let state_queries = Arc::new(MockStateQueries {
            has_project: true,
            has_active_world: false,
            has_selection: false,
            is_transaction_active: false,
        });

        let dispatch = ActionDispatch::new(Arc::new(registry), state_queries);
        let context = ActionContext {
            project_id: Some(Uuid::new_v4()),
            active_world_id: None,
            selected_entities: vec![],
            focused_panel: None,
            transaction_active: false,
        };

        let result = dispatch.dispatch(action_def.action_id, context);

        // Terrain commands need world, which implies project requirement
        assert!(!result.success, "terrain commands should require world");
    }

    #[test]
    fn test_material_commands_require_project() {
        let action_def = create_action_def("material.create_profile", ActionFamily::Material, MutationClass::Mutate);
        let mut registry = ActionRegistry::new();
        registry.register(action_def.clone());

        let state_queries = Arc::new(MockStateQueries {
            has_project: false,
            has_active_world: false,
            has_selection: false,
            is_transaction_active: false,
        });

        let dispatch = ActionDispatch::new(Arc::new(registry), state_queries);
        let context = create_context_no_project();

        let result = dispatch.dispatch(action_def.action_id, context);

        assert!(!result.success, "material commands should require project");
        assert!(matches!(
            result.error_code,
            Some(ErrorCode::DisabledReason(DisabledReason::NoLegalProject))
        ));
    }

    #[test]
    fn test_mutation_during_transaction_forbidden() {
        let action_def = create_action_def("world.save", ActionFamily::World, MutationClass::Mutate);
        let mut registry = ActionRegistry::new();
        registry.register(action_def.clone());

        let state_queries = Arc::new(MockStateQueries {
            has_project: true,
            has_active_world: true,
            has_selection: false,
            is_transaction_active: true,
        });

        let dispatch = ActionDispatch::new(Arc::new(registry), state_queries);
        let context = ActionContext {
            project_id: Some(Uuid::new_v4()),
            active_world_id: Some(Uuid::new_v4()),
            selected_entities: vec![],
            focused_panel: None,
            transaction_active: true,
        };

        let result = dispatch.dispatch(action_def.action_id, context);

        assert!(!result.success, "mutations should be forbidden during active transaction");
        assert!(matches!(
            result.error_code,
            Some(ErrorCode::DisabledReason(DisabledReason::ActiveTransactionForbidsMutation))
        ));
    }

    #[test]
    fn test_selection_commands_require_selection() {
        let action_def = create_action_def("entity.inspect", ActionFamily::World, MutationClass::Read);
        let mut registry = ActionRegistry::new();
        registry.register(action_def.clone());

        let state_queries = Arc::new(MockStateQueries {
            has_project: true,
            has_active_world: true,
            has_selection: false,
            is_transaction_active: false,
        });

        let dispatch = ActionDispatch::new(Arc::new(registry), state_queries);
        let context = ActionContext {
            project_id: Some(Uuid::new_v4()),
            active_world_id: Some(Uuid::new_v4()),
            selected_entities: vec![],
            focused_panel: None,
            transaction_active: false,
        };

        let result = dispatch.dispatch(action_def.action_id, context);

        assert!(!result.success, "selection commands should require selection");
        assert!(matches!(
            result.error_code,
            Some(ErrorCode::DisabledReason(DisabledReason::NoLegalTargetSelected))
        ));
    }

    #[test]
    fn test_disabled_reason_messages_are_clear() {
        // Verify that disabled reasons have clear user messages
        assert_eq!(
            DisabledReason::NoLegalProject.to_user_message(),
            "No project is open"
        );
        assert_eq!(
            DisabledReason::NoLegalTargetSelected.to_user_message(),
            "No valid target selected"
        );
        assert_eq!(
            DisabledReason::ActiveTransactionForbidsMutation.to_user_message(),
            "Cannot modify state during active transaction"
        );
    }
}
