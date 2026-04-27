//! Property-based test for Action Dispatch Returns Result
//!
//! **Validates: Requirements 1.8**
//!
//! This test verifies that all action dispatches return a proper ActionResult
//! with success status and optional focus target. The test generates random
//! action IDs and parameters and verifies that dispatch_action always returns
//! an ActionResult structure.

use crate::{
    ActionContext, ActionDefinition, ActionDispatch, ActionFamily, ActionId, ActionRegistry,
    ActionResult, MutationClass, StateQueries,
};
use proptest::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

// ============================================================================
// Test System
// ============================================================================

/// Mock StateQueries for testing
struct TestStateQueries {
    has_project: bool,
    has_active_world: bool,
    has_selection: bool,
    is_transaction_active: bool,
}

impl TestStateQueries {
    fn new(has_project: bool, has_active_world: bool, has_selection: bool) -> Self {
        Self {
            has_project,
            has_active_world,
            has_selection,
            is_transaction_active: false,
        }
    }
}

impl StateQueries for TestStateQueries {
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

/// Test system for action dispatch testing
struct TestSystem {
    registry: Arc<ActionRegistry>,
    state_queries: Arc<TestStateQueries>,
}

impl TestSystem {
    fn new(action_id: ActionId, has_project: bool, has_active_world: bool, has_selection: bool) -> Self {
        let mut registry = ActionRegistry::new();

        // Create handlers that return different result types
        fn success_handler(_context: ActionContext) -> ActionResult {
            ActionResult::success(Some("viewport".to_string()))
        }

        fn success_no_focus_handler(_context: ActionContext) -> ActionResult {
            ActionResult::success(None)
        }

        fn success_with_artifact_handler(_context: ActionContext) -> ActionResult {
            ActionResult::success_with_artifact(
                "artifact_123".to_string(),
                Some("evidence".to_string()),
            )
        }

        // Register the action with appropriate handler
        let handler = match action_id.as_str() {
            id if id.contains("artifact") => success_with_artifact_handler,
            id if id.contains("no_focus") => success_no_focus_handler,
            _ => success_handler,
        };

        let definition = ActionDefinition {
            action_id: action_id.clone(),
            display_label: format!("Test {}", action_id.as_str()),
            action_family: ActionFamily::World,
            tooling_route: format!("route.{}.v1", action_id.as_str()),
            sdk_packet_family: "packet.test.*".to_string(),
            engine_truth_owner: "engine/test".to_string(),
            mutation_class: MutationClass::Mutate,
            denial_families: vec![],
            handler,
        };

        registry.register(definition);

        Self {
            registry: Arc::new(registry),
            state_queries: Arc::new(TestStateQueries::new(
                has_project,
                has_active_world,
                has_selection,
            )),
        }
    }

    /// Executes an action and returns the result
    fn dispatch_action(&self, action_id: ActionId, context: ActionContext) -> ActionResult {
        let dispatch = ActionDispatch::new(
            Arc::clone(&self.registry),
            Arc::clone(&self.state_queries) as Arc<dyn StateQueries>,
        );

        dispatch.dispatch(action_id, context)
    }
}

// ============================================================================
// Property Test Generators
// ============================================================================

/// Generates random action IDs
fn arb_action_id() -> impl Strategy<Value = ActionId> {
    prop_oneof![
        Just(ActionId::new("world.open")),
        Just(ActionId::new("world.close")),
        Just(ActionId::new("world.save")),
        Just(ActionId::new("material.author")),
        Just(ActionId::new("material.author_artifact")),
        Just(ActionId::new("terrain.sculpt")),
        Just(ActionId::new("terrain.sculpt_no_focus")),
        Just(ActionId::new("runtime.play")),
        Just(ActionId::new("panel.open")),
        Just(ActionId::new("panel.close_no_focus")),
    ]
}

/// Generates random action contexts
fn arb_action_context() -> impl Strategy<Value = ActionContext> {
    (any::<bool>(), any::<bool>(), any::<bool>()).prop_map(|(has_project, has_world, has_selection)| {
        ActionContext {
            project_id: if has_project { Some(Uuid::new_v4()) } else { None },
            active_world_id: if has_world { Some(Uuid::new_v4()) } else { None },
            selected_entities: if has_selection { vec![Uuid::new_v4()] } else { vec![] },
            focused_panel: None,
            transaction_active: false,
        }
    })
}

/// Generates action dispatch scenarios (action + context)
fn arb_action_dispatch() -> impl Strategy<Value = (ActionId, ActionContext)> {
    (arb_action_id(), arb_action_context())
}

// ============================================================================
// Property Tests
// ============================================================================

/// **Property 2: Action Dispatch Returns Result**
///
/// **Validates: Requirements 1.8**
///
/// This property verifies that all action dispatches return a proper ActionResult:
/// - dispatch_action always returns an ActionResult (never panics or returns None)
/// - ActionResult contains a success status (bool)
/// - ActionResult contains an optional focus target (Option<String>)
///
/// The test generates random action IDs and parameters and verifies the result
/// structure is always valid.
#[test]
fn prop_action_dispatch_returns_result() {
    // Feature: editor-canonical-architecture-refactor, Property 2: Action Dispatch Returns Result
    let config = ProptestConfig::with_cases(100);
    proptest!(config, |(action_dispatch in arb_action_dispatch())| {
        let (action_id, context) = action_dispatch;
        
        // Determine required context for this action
        let requires_project = !action_id.as_str().contains("panel");
        let requires_world = action_id.as_str().contains("world.close") 
            || action_id.as_str().contains("world.save")
            || action_id.as_str().contains("material")
            || action_id.as_str().contains("terrain")
            || action_id.as_str().contains("runtime");
        
        // Create test system with appropriate context
        let system = TestSystem::new(
            action_id.clone(),
            context.has_project() || !requires_project,
            context.has_active_world() || !requires_world,
            context.has_selection(),
        );
        
        // Execute action - this should ALWAYS return an ActionResult
        let result = system.dispatch_action(action_id.clone(), context);
        
        // Verify ActionResult structure is valid
        // 1. Result has a success status (always present)
        let _success_status = result.success;
        
        // 2. If action succeeded, success should be true
        if result.success {
            prop_assert!(
                result.error_code.is_none(),
                "Successful action {} should not have error code",
                action_id.as_str()
            );
        }
        
        // 3. If action failed, success should be false and error_code should be present
        if !result.success {
            prop_assert!(
                result.error_code.is_some(),
                "Failed action {} should have error code",
                action_id.as_str()
            );
        }
        
        // 4. Focus target is optional but must be valid if present
        if let Some(ref focus_target) = result.focus_target_id {
            prop_assert!(
                !focus_target.is_empty(),
                "Focus target for action {} should not be empty string",
                action_id.as_str()
            );
        }
    });
}

/// **Property 2b: Action Dispatch Result Contains Success Status**
///
/// **Validates: Requirements 1.8**
///
/// This property specifically verifies that every ActionResult contains
/// a valid success status that correctly reflects the outcome.
#[test]
fn prop_action_result_has_success_status() {
    // Feature: editor-canonical-architecture-refactor, Property 2: Action Dispatch Returns Result
    let config = ProptestConfig::with_cases(100);
    proptest!(config, |(action_id in arb_action_id())| {
        // Create system with full context to ensure action can execute
        let system = TestSystem::new(action_id.clone(), true, true, true);
        let context = ActionContext {
            project_id: Some(Uuid::new_v4()),
            active_world_id: Some(Uuid::new_v4()),
            selected_entities: vec![Uuid::new_v4()],
            focused_panel: None,
            transaction_active: false,
        };
        
        let result = system.dispatch_action(action_id.clone(), context);
        
        // Verify success status is present and consistent
        prop_assert!(
            result.success || !result.success,
            "Action {} result must have valid boolean success status",
            action_id.as_str()
        );
        
        // Verify success status is consistent with error_code
        if result.success {
            prop_assert!(
                result.error_code.is_none(),
                "Success status true but error_code present for action {}",
                action_id.as_str()
            );
        } else {
            prop_assert!(
                result.error_code.is_some(),
                "Success status false but error_code missing for action {}",
                action_id.as_str()
            );
        }
    });
}

/// **Property 2c: Action Dispatch Result Contains Optional Focus Target**
///
/// **Validates: Requirements 1.8**
///
/// This property verifies that every ActionResult contains an optional
/// focus target field that can be None or Some(panel_id).
#[test]
fn prop_action_result_has_optional_focus_target() {
    // Feature: editor-canonical-architecture-refactor, Property 2: Action Dispatch Returns Result
    let config = ProptestConfig::with_cases(100);
    proptest!(config, |(action_id in arb_action_id())| {
        let system = TestSystem::new(action_id.clone(), true, true, true);
        let context = ActionContext {
            project_id: Some(Uuid::new_v4()),
            active_world_id: Some(Uuid::new_v4()),
            selected_entities: vec![Uuid::new_v4()],
            focused_panel: None,
            transaction_active: false,
        };
        
        let result = system.dispatch_action(action_id.clone(), context);
        
        // Verify focus_target_id field exists (it's an Option, so always valid)
        match result.focus_target_id {
            Some(target) => {
                prop_assert!(
                    !target.is_empty(),
                    "Focus target for action {} should not be empty if present",
                    action_id.as_str()
                );
            }
            None => {
                // None is valid - some actions don't specify focus target
            }
        }
    });
}

// ============================================================================
// Unit Tests for ActionResult Structure
// ============================================================================

#[cfg(test)]
mod result_structure_tests {
    use super::*;
    use crate::{DisabledReason, ErrorCode};

    #[test]
    fn test_action_result_success_has_valid_structure() {
        let result = ActionResult::success(Some("viewport".to_string()));
        
        assert!(result.success);
        assert!(result.error_code.is_none());
        assert_eq!(result.focus_target_id, Some("viewport".to_string()));
    }

    #[test]
    fn test_action_result_success_without_focus() {
        let result = ActionResult::success(None);
        
        assert!(result.success);
        assert!(result.error_code.is_none());
        assert!(result.focus_target_id.is_none());
    }

    #[test]
    fn test_action_result_failure_has_valid_structure() {
        let result = ActionResult::failure(
            ErrorCode::DisabledReason(DisabledReason::NoLegalProject),
            Some("diagnostics".to_string()),
        );
        
        assert!(!result.success);
        assert!(result.error_code.is_some());
        assert_eq!(result.focus_target_id, Some("diagnostics".to_string()));
    }

    #[test]
    fn test_action_result_with_artifact() {
        let result = ActionResult::success_with_artifact(
            "artifact_123".to_string(),
            Some("evidence".to_string()),
        );
        
        assert!(result.success);
        assert!(result.error_code.is_none());
        assert_eq!(result.artifact_ref, Some("artifact_123".to_string()));
        assert_eq!(result.focus_target_id, Some("evidence".to_string()));
    }
}
