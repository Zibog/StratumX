// Feature: editor-canonical-architecture-refactor, Property 2: Action Context Validation
// Feature: editor-canonical-architecture-refactor, Property 3: Selection Requirement Validation
//
// These property tests validate that:
// - Property 2: For any action requiring a project, when the action is dispatched without a valid
//   project in the context, the Command Spine must return DisabledReason::NoLegalProject and never
//   execute the action.
// - Property 3: For any action requiring selection, when the action is dispatched with empty
//   selection state, the Command Spine must return DisabledReason::NoLegalTargetSelected and never
//   execute the action.
//
// **Validates: Requirements 2.4, 16.2, 16.3, 16.5**

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

/// Mock implementation of StateQueries for testing.
///
/// This mock allows us to control the state returned by queries to test
/// different validation scenarios.
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

/// Strategy for generating actions that require a project.
///
/// These are actions from families that need a project to be open:
/// - World actions (except "new")
/// - Terrain actions
/// - Material actions
/// - SkyEnvironment actions
/// - Runtime actions
/// - DiagnosticsProof actions
/// - BuildRelease actions
/// - ProjectFile actions (except "new")
fn project_requiring_action_strategy() -> impl Strategy<Value = ActionDefinition> {
    prop_oneof![
        // World actions (except "open" which is special-cased)
        Just(create_action_def("world.close", ActionFamily::World)),
        Just(create_action_def("world.save", ActionFamily::World)),
        Just(create_action_def("world.validate", ActionFamily::World)),
        
        // Terrain actions
        Just(create_action_def("terrain.simulate_blast_profile", ActionFamily::Terrain)),
        Just(create_action_def("terrain.author_destruction_profile", ActionFamily::Terrain)),
        Just(create_action_def("terrain.compare_results", ActionFamily::Terrain)),
        
        // Material actions
        Just(create_action_def("material.author_response_profile", ActionFamily::Material)),
        Just(create_action_def("material.bind_surface_family", ActionFamily::Material)),
        Just(create_action_def("material.inspect_response_table", ActionFamily::Material)),
        
        // SkyEnvironment actions
        Just(create_action_def("sky.author_sun_profile", ActionFamily::SkyEnvironment)),
        Just(create_action_def("sky.bind_weather_system", ActionFamily::SkyEnvironment)),
        
        // Runtime actions
        Just(create_action_def("runtime.play", ActionFamily::Runtime)),
        Just(create_action_def("runtime.simulate", ActionFamily::Runtime)),
        Just(create_action_def("runtime.debug", ActionFamily::Runtime)),
        
        // DiagnosticsProof actions
        Just(create_action_def("diagnostics.capture_trace", ActionFamily::DiagnosticsProof)),
        Just(create_action_def("diagnostics.inspect_evidence", ActionFamily::DiagnosticsProof)),
        
        // BuildRelease actions
        Just(create_action_def("build.export_product", ActionFamily::BuildRelease)),
        Just(create_action_def("build.freeze_release", ActionFamily::BuildRelease)),
        
        // ProjectFile actions (except "new")
        Just(create_action_def("project.open", ActionFamily::ProjectFile)),
        Just(create_action_def("project.save", ActionFamily::ProjectFile)),
        Just(create_action_def("project.close", ActionFamily::ProjectFile)),
    ]
}

/// Strategy for generating actions that require selection.
///
/// These are actions that operate on selected entities or objects.
/// We use a heuristic: actions with "inspect" or "selected" in their ID.
fn selection_requiring_action_strategy() -> impl Strategy<Value = ActionDefinition> {
    prop_oneof![
        Just(create_action_def("entity.inspect", ActionFamily::World)),
        Just(create_action_def("entity.inspect_selected", ActionFamily::World)),
        Just(create_action_def("material.inspect_response_table", ActionFamily::Material)),
        Just(create_action_def("terrain.inspect_selected_region", ActionFamily::Terrain)),
        Just(create_action_def("diagnostics.inspect_evidence", ActionFamily::DiagnosticsProof)),
    ]
}

/// Strategy for generating ActionContext without a project.
fn context_without_project_strategy() -> impl Strategy<Value = ActionContext> {
    (
        prop::option::of(any::<[u8; 16]>().prop_map(Uuid::from_bytes)),
        prop::collection::vec(any::<[u8; 16]>().prop_map(Uuid::from_bytes), 0..5),
        prop::option::of("[a-z_]{5,15}"),
        any::<bool>(),
    ).prop_map(|(active_world_id, selected_entities, focused_panel, transaction_active)| {
        ActionContext {
            project_id: None,  // No project
            active_world_id,
            selected_entities,
            focused_panel,
            transaction_active,
        }
    })
}

/// Strategy for generating ActionContext without selection.
fn context_without_selection_strategy() -> impl Strategy<Value = ActionContext> {
    (
        any::<[u8; 16]>().prop_map(Uuid::from_bytes),
        prop::option::of(any::<[u8; 16]>().prop_map(Uuid::from_bytes)),
        prop::option::of("[a-z_]{5,15}"),
        any::<bool>(),
    ).prop_map(|(project_id, active_world_id, focused_panel, transaction_active)| {
        ActionContext {
            project_id: Some(project_id),
            active_world_id,
            selected_entities: vec![],  // No selection
            focused_panel,
            transaction_active,
        }
    })
}

/// Helper function to create an ActionDefinition for testing.
fn create_action_def(action_id: &str, family: ActionFamily) -> ActionDefinition {
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
        mutation_class: MutationClass::Read,
        denial_families: vec![],
        handler: dummy_handler,
    }
}

// ============================================================================
// Property Tests
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property 2: Action Context Validation
    ///
    /// For any action requiring a project, when the action is dispatched without a valid
    /// project in the context, the Command Spine must return DisabledReason::NoLegalProject
    /// and never execute the action.
    ///
    /// This test validates that:
    /// 1. Actions requiring a project are properly identified
    /// 2. Context validation detects missing project
    /// 3. DisabledReason::NoLegalProject is returned
    /// 4. The action is never executed (success = false)
    ///
    /// **Validates: Requirements 2.4, 16.2, 16.5**
    #[test]
    fn property_action_requires_project_validation(
        action_def in project_requiring_action_strategy(),
        context in context_without_project_strategy()
    ) {
        // Create registry and register the action
        let mut registry = ActionRegistry::new();
        registry.register(action_def.clone());

        // Create mock state queries that report no project
        let state_queries = Arc::new(MockStateQueries {
            has_project: false,
            has_active_world: false,
            has_selection: false,
            is_transaction_active: false,
        });

        // Create dispatch and execute action
        let dispatch = ActionDispatch::new(Arc::new(registry), state_queries);
        let result = dispatch.dispatch(action_def.action_id.clone(), context);

        // Verify that the action failed
        assert!(
            !result.success,
            "Action {} should fail when no project is open",
            action_def.action_id.as_str()
        );

        // Verify that the error code is DisabledReason::NoLegalProject
        assert!(
            matches!(
                result.error_code,
                Some(ErrorCode::DisabledReason(DisabledReason::NoLegalProject))
            ),
            "Action {} should return NoLegalProject error, got: {:?}",
            action_def.action_id.as_str(),
            result.error_code
        );

        // Verify that focus is directed to diagnostics panel
        assert_eq!(
            result.focus_target_id,
            Some("diagnostics".to_string()),
            "Failed action should focus diagnostics panel"
        );

        // Verify that no artifact was created
        assert!(
            result.artifact_ref.is_none(),
            "Failed action should not create artifacts"
        );
    }

    /// Property 3: Selection Requirement Validation
    ///
    /// For any action requiring selection, when the action is dispatched with empty
    /// selection state, the Command Spine must return DisabledReason::NoLegalTargetSelected
    /// and never execute the action.
    ///
    /// This test validates that:
    /// 1. Actions requiring selection are properly identified
    /// 2. Context validation detects empty selection
    /// 3. DisabledReason::NoLegalTargetSelected is returned
    /// 4. The action is never executed (success = false)
    ///
    /// **Validates: Requirements 2.4, 16.3, 16.5**
    #[test]
    fn property_action_requires_selection_validation(
        action_def in selection_requiring_action_strategy(),
        context in context_without_selection_strategy()
    ) {
        // Create registry and register the action
        let mut registry = ActionRegistry::new();
        registry.register(action_def.clone());

        // Create mock state queries that report no selection
        let state_queries = Arc::new(MockStateQueries {
            has_project: true,
            has_active_world: true,
            has_selection: false,
            is_transaction_active: false,
        });

        // Create dispatch and execute action
        let dispatch = ActionDispatch::new(Arc::new(registry), state_queries);
        let result = dispatch.dispatch(action_def.action_id.clone(), context);

        // Verify that the action failed
        assert!(
            !result.success,
            "Action {} should fail when no selection exists",
            action_def.action_id.as_str()
        );

        // Verify that the error code is DisabledReason::NoLegalTargetSelected
        assert!(
            matches!(
                result.error_code,
                Some(ErrorCode::DisabledReason(DisabledReason::NoLegalTargetSelected))
            ),
            "Action {} should return NoLegalTargetSelected error, got: {:?}",
            action_def.action_id.as_str(),
            result.error_code
        );

        // Verify that focus is directed to diagnostics panel
        assert_eq!(
            result.focus_target_id,
            Some("diagnostics".to_string()),
            "Failed action should focus diagnostics panel"
        );

        // Verify that no artifact was created
        assert!(
            result.artifact_ref.is_none(),
            "Failed action should not create artifacts"
        );
    }

    /// Property: Context Validation Never Executes Action
    ///
    /// For any action that fails context validation, the action handler must never
    /// be executed. This ensures that validation is a hard gate before execution.
    ///
    /// This test validates that:
    /// 1. Failed validation returns immediately
    /// 2. No side effects occur from failed validation
    /// 3. Error information is complete and accurate
    #[test]
    fn property_context_validation_never_executes(
        action_def in project_requiring_action_strategy(),
        context in context_without_project_strategy()
    ) {
        let mut registry = ActionRegistry::new();
        registry.register(action_def.clone());

        let state_queries = Arc::new(MockStateQueries {
            has_project: false,
            has_active_world: false,
            has_selection: false,
            is_transaction_active: false,
        });

        let dispatch = ActionDispatch::new(Arc::new(registry), state_queries);
        let result = dispatch.dispatch(action_def.action_id.clone(), context);

        // Verify immediate failure
        assert!(!result.success);
        
        // Verify error code is present
        assert!(result.error_code.is_some());
        
        // Verify no execution artifacts
        assert!(result.artifact_ref.is_none());
        assert!(result.next_legal_recovery_action.is_none());
    }

    /// Property: Validation Respects Action Family Requirements
    ///
    /// For any action, the validation requirements must match the action family.
    /// Different action families have different precondition requirements.
    ///
    /// This test validates that:
    /// 1. ViewPanel actions don't require project
    /// 2. World actions require project
    /// 3. Terrain/Material/Sky actions require project and world
    /// 4. Runtime actions require project and world
    #[test]
    fn property_validation_respects_action_family(
        family in prop_oneof![
            Just(ActionFamily::ViewPanel),
            Just(ActionFamily::World),
            Just(ActionFamily::Terrain),
            Just(ActionFamily::Material),
            Just(ActionFamily::SkyEnvironment),
            Just(ActionFamily::Runtime),
        ]
    ) {
        // Use action IDs that match the expected prefixes for each family
        let action_id = match family {
            ActionFamily::ViewPanel => "panel.open",
            ActionFamily::World => "world.close",
            ActionFamily::Terrain => "terrain.sculpt",
            ActionFamily::Material => "material.author",
            ActionFamily::SkyEnvironment => "environment.configure_sky",
            ActionFamily::Runtime => "runtime.play",
            _ => "test.action",
        };
        
        let action_def = create_action_def(action_id, family);
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

        // ViewPanel actions should succeed without project
        // All other families should fail
        match family {
            ActionFamily::ViewPanel => {
                assert!(
                    result.success,
                    "ViewPanel actions should not require project"
                );
            }
            _ => {
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
    }
}

// ============================================================================
// Unit Tests for Edge Cases
// ============================================================================

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_project_new_does_not_require_project() {
        // "project.new" is a special case that doesn't require an existing project
        let action_def = create_action_def("project.new", ActionFamily::ProjectFile);
        let mut registry = ActionRegistry::new();
        registry.register(action_def.clone());

        let state_queries = Arc::new(MockStateQueries {
            has_project: false,
            has_active_world: false,
            has_selection: false,
            is_transaction_active: false,
        });

        let dispatch = ActionDispatch::new(Arc::new(registry), state_queries);
        let context = ActionContext {
            project_id: None,
            active_world_id: None,
            selected_entities: vec![],
            focused_panel: None,
            transaction_active: false,
        };

        let result = dispatch.dispatch(action_def.action_id, context);

        // "project.new" should succeed without a project
        assert!(result.success, "project.new should not require existing project");
    }

    #[test]
    fn test_world_open_requires_project_but_not_world() {
        // "world.open" requires a project but not an active world
        let action_def = create_action_def("world.open", ActionFamily::World);
        let mut registry = ActionRegistry::new();
        registry.register(action_def.clone());

        // Test with project but no world
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

        // "world.open" should succeed with project but no world
        assert!(result.success, "world.open should not require active world");
    }

    #[test]
    fn test_multiple_validation_failures_returns_first() {
        // When multiple validations fail, the first failure should be returned
        let action_def = create_action_def("entity.inspect", ActionFamily::World);
        let mut registry = ActionRegistry::new();
        registry.register(action_def.clone());

        // No project and no selection
        let state_queries = Arc::new(MockStateQueries {
            has_project: false,
            has_active_world: false,
            has_selection: false,
            is_transaction_active: false,
        });

        let dispatch = ActionDispatch::new(Arc::new(registry), state_queries);
        let context = ActionContext {
            project_id: None,
            active_world_id: None,
            selected_entities: vec![],
            focused_panel: None,
            transaction_active: false,
        };

        let result = dispatch.dispatch(action_def.action_id, context);

        // Should return NoLegalProject (checked first)
        assert!(!result.success);
        assert!(matches!(
            result.error_code,
            Some(ErrorCode::DisabledReason(DisabledReason::NoLegalProject))
        ));
    }

    #[test]
    fn test_disabled_reason_user_messages() {
        // Verify that disabled reasons have clear user messages
        assert_eq!(
            DisabledReason::NoLegalProject.to_user_message(),
            "No project is open"
        );
        assert_eq!(
            DisabledReason::NoLegalTargetSelected.to_user_message(),
            "No valid target selected"
        );
    }

    #[test]
    fn test_disabled_reason_tooltips() {
        // Verify that disabled reasons have helpful tooltips
        let project_tooltip = DisabledReason::NoLegalProject.to_tooltip();
        assert!(project_tooltip.contains("project"));
        assert!(project_tooltip.contains("Create or open"));

        let selection_tooltip = DisabledReason::NoLegalTargetSelected.to_tooltip();
        assert!(selection_tooltip.contains("selection"));
        assert!(selection_tooltip.contains("Select"));
    }
}
