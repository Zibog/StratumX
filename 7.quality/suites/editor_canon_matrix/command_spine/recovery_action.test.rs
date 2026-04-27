//! Recovery Action Tests
//!
//! **Validates: Requirements 25.3, 25.7**
//!
//! This test module verifies that ActionResult properly includes next_legal_recovery_action
//! for failed operations, and that FocusTarget supports all required types.
//!
//! ## Requirements Coverage
//!
//! - **Requirement 25.3**: ActionResult SHALL include next_legal_recovery_action for failed operations
//! - **Requirement 25.7**: FocusTarget SHALL support: specific panel ID, owner lab, diagnostics, or no change
//!
//! ## Test Strategy
//!
//! 1. Verify ActionResult can be created with recovery actions
//! 2. Verify all FocusTarget types are supported
//! 3. Verify recovery actions are only provided for failed operations
//! 4. Verify recovery actions guide operators to next legal action

use crate::{ActionResult, ActionId, FocusTarget, ErrorCode, DisabledReason};

// ============================================================================
// Unit Tests - ActionResult with Recovery Actions
// ============================================================================

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_failure_with_recovery_includes_recovery_action() {
        // Requirement 25.3: ActionResult SHALL include next_legal_recovery_action for failed operations
        let recovery_action = ActionId::new("world.open");
        let result = ActionResult::failure_with_recovery(
            ErrorCode::DisabledReason(DisabledReason::NoLegalProject),
            recovery_action.clone(),
            FocusTarget::Diagnostics,
        );
        
        assert!(!result.success, "Result should be a failure");
        assert!(result.next_legal_recovery_action.is_some(), "Recovery action should be present");
        assert_eq!(
            result.next_legal_recovery_action.unwrap().as_str(),
            "world.open",
            "Recovery action should match the provided action"
        );
    }

    #[test]
    fn test_failure_without_recovery_has_none() {
        // Verify that regular failures don't have recovery actions
        let result = ActionResult::failure(
            ErrorCode::DisabledReason(DisabledReason::NoLegalProject),
            FocusTarget::Diagnostics,
        );
        
        assert!(!result.success, "Result should be a failure");
        assert!(result.next_legal_recovery_action.is_none(), "Recovery action should be None for regular failures");
    }

    #[test]
    fn test_success_has_no_recovery_action() {
        // Verify that successful operations don't have recovery actions
        let result = ActionResult::success(FocusTarget::SpecificPanel("viewport".to_string()));
        
        assert!(result.success, "Result should be a success");
        assert!(result.next_legal_recovery_action.is_none(), "Success should not have recovery action");
    }

    // ============================================================================
    // FocusTarget Type Support Tests
    // ============================================================================

    #[test]
    fn test_focus_target_specific_panel() {
        // Requirement 25.7: FocusTarget SHALL support specific panel ID
        let result = ActionResult::failure_with_recovery(
            ErrorCode::DisabledReason(DisabledReason::NoLegalProject),
            ActionId::new("project.open"),
            FocusTarget::SpecificPanel("project_browser".to_string()),
        );
        
        assert_eq!(
            result.focus_target,
            FocusTarget::SpecificPanel("project_browser".to_string()),
            "FocusTarget should support specific panel ID"
        );
    }

    #[test]
    fn test_recovery_action_for_owner_lab() {
        // Requirement 25.7: FocusTarget SHALL support owner lab
        let result = ActionResult::failure_with_recovery(
            ErrorCode::DisabledReason(DisabledReason::DiagnosticsSourceUnavailable),
            ActionId::new("diagnostics.initialize"),
            FocusTarget::OwnerLab,
        );
        
        assert_eq!(
            result.focus_target,
            FocusTarget::OwnerLab,
            "FocusTarget should support owner lab"
        );
    }

    #[test]
    fn test_focus_target_diagnostics() {
        // Requirement 25.7: FocusTarget SHALL support diagnostics
        let result = ActionResult::failure_with_recovery(
            ErrorCode::DisabledReason(DisabledReason::NoLegalProject),
            ActionId::new("project.open"),
            FocusTarget::Diagnostics,
        );
        
        assert_eq!(
            result.focus_target,
            FocusTarget::Diagnostics,
            "FocusTarget should support diagnostics"
        );
    }

    #[test]
    fn test_focus_target_no_change() {
        // Requirement 25.7: FocusTarget SHALL support no change
        let result = ActionResult::failure_with_recovery(
            ErrorCode::DisabledReason(DisabledReason::ActiveTransactionForbidsMutation),
            ActionId::new("transaction.commit"),
            FocusTarget::NoChange,
        );
        
        assert_eq!(
            result.focus_target,
            FocusTarget::NoChange,
            "FocusTarget should support no change"
        );
    }

    // ============================================================================
    // Recovery Action Scenarios
    // ============================================================================

    #[test]
    fn test_recovery_action_for_no_project() {
        // Scenario: User tries to open world without a project
        // Recovery: Suggest opening a project first
        let result = ActionResult::failure_with_recovery(
            ErrorCode::DisabledReason(DisabledReason::NoLegalProject),
            ActionId::new("project.open"),
            FocusTarget::SpecificPanel("project_browser".to_string()),
        );
        
        assert!(!result.success);
        assert_eq!(
            result.next_legal_recovery_action.unwrap().as_str(),
            "project.open",
            "Should suggest opening a project"
        );
        assert_eq!(
            result.focus_target,
            FocusTarget::SpecificPanel("project_browser".to_string()),
            "Should focus project browser"
        );
    }

    #[test]
    fn test_recovery_action_for_diagnostics_unavailable() {
        // Scenario: Diagnostics system not available
        // Recovery: Suggest initializing diagnostics
        let result = ActionResult::failure_with_recovery(
            ErrorCode::DisabledReason(DisabledReason::DiagnosticsSourceUnavailable),
            ActionId::new("diagnostics.initialize"),
            FocusTarget::Diagnostics,
        );
        
        assert!(!result.success);
        assert_eq!(
            result.next_legal_recovery_action.unwrap().as_str(),
            "diagnostics.initialize",
            "Should suggest initializing diagnostics"
        );
    }

    #[test]
    fn test_recovery_action_for_compare_baseline_missing() {
        // Scenario: Compare baseline missing
        // Recovery: Suggest capturing baseline
        let result = ActionResult::failure_with_recovery(
            ErrorCode::DisabledReason(DisabledReason::CompareBaselineMissing),
            ActionId::new("baseline.capture"),
            FocusTarget::Diagnostics,
        );
        
        assert!(!result.success);
        assert_eq!(
            result.next_legal_recovery_action.unwrap().as_str(),
            "baseline.capture",
            "Should suggest capturing baseline"
        );
    }

    #[test]
    fn test_recovery_action_for_no_target_selected() {
        // Scenario: User tries to operate without selection
        // Recovery: Suggest selecting a target
        let result = ActionResult::failure_with_recovery(
            ErrorCode::DisabledReason(DisabledReason::NoLegalTargetSelected),
            ActionId::new("selection.prompt"),
            FocusTarget::SpecificPanel("world_outliner".to_string()),
        );
        
        assert!(!result.success);
        assert_eq!(
            result.next_legal_recovery_action.unwrap().as_str(),
            "selection.prompt",
            "Should suggest selecting a target"
        );
    }

    #[test]
    fn test_recovery_action_for_active_transaction() {
        // Scenario: User tries to mutate during active transaction
        // Recovery: Suggest committing or rolling back transaction
        let result = ActionResult::failure_with_recovery(
            ErrorCode::DisabledReason(DisabledReason::ActiveTransactionForbidsMutation),
            ActionId::new("transaction.commit"),
            FocusTarget::NoChange,
        );
        
        assert!(!result.success);
        assert_eq!(
            result.next_legal_recovery_action.unwrap().as_str(),
            "transaction.commit",
            "Should suggest committing transaction"
        );
    }

    #[test]
    fn test_no_recovery_available() {
        // Scenario: Failure with no legal recovery action
        // Recovery: None available
        let result = ActionResult::failure(
            ErrorCode::DisabledReason(DisabledReason::NoLegalRecoveryAvailable),
            FocusTarget::Diagnostics,
        );
        
        assert!(!result.success);
        assert!(result.next_legal_recovery_action.is_none(), "Should have no recovery action");
    }

    // ============================================================================
    // Focus Target Routing Tests
    // ============================================================================

    #[test]
    fn test_recovery_routes_to_owner_lab_for_authoring_failures() {
        // Scenario: Authoring operation fails
        // Recovery: Route to owner lab (authoring workspace)
        let result = ActionResult::failure_with_recovery(
            ErrorCode::DisabledReason(DisabledReason::PlaceholderBlocksAction),
            ActionId::new("placeholder.resolve"),
            FocusTarget::OwnerLab,
        );
        
        assert_eq!(
            result.focus_target,
            FocusTarget::OwnerLab,
            "Authoring failures should route to owner lab"
        );
    }

    #[test]
    fn test_recovery_routes_to_diagnostics_for_system_failures() {
        // Scenario: System-level failure
        // Recovery: Route to diagnostics panel
        let result = ActionResult::failure_with_recovery(
            ErrorCode::DisabledReason(DisabledReason::NoLegalRecoveryAvailable),
            ActionId::new("system.restart"),
            FocusTarget::Diagnostics,
        );
        
        assert_eq!(
            result.focus_target,
            FocusTarget::Diagnostics,
            "System failures should route to diagnostics"
        );
    }

    #[test]
    fn test_recovery_routes_to_specific_panel_for_workflow_failures() {
        // Scenario: Workflow-specific failure
        // Recovery: Route to specific panel to continue workflow
        let result = ActionResult::failure_with_recovery(
            ErrorCode::DisabledReason(DisabledReason::NoLegalProject),
            ActionId::new("project.open"),
            FocusTarget::SpecificPanel("project_browser".to_string()),
        );
        
        assert_eq!(
            result.focus_target,
            FocusTarget::SpecificPanel("project_browser".to_string()),
            "Workflow failures should route to specific panel"
        );
    }

    #[test]
    fn test_recovery_no_change_for_transient_failures() {
        // Scenario: Transient failure that doesn't require focus change
        // Recovery: Keep current focus
        let result = ActionResult::failure_with_recovery(
            ErrorCode::DisabledReason(DisabledReason::ActiveTransactionForbidsMutation),
            ActionId::new("transaction.commit"),
            FocusTarget::NoChange,
        );
        
        assert_eq!(
            result.focus_target,
            FocusTarget::NoChange,
            "Transient failures should not change focus"
        );
    }
}
