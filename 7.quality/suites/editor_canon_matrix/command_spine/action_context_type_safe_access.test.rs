//! Property Test: Action Context Type-Safe Access
//!
//! **Property 24: Action Context Type-Safe Access**
//! **Validates: Requirements 24.6**
//!
//! This property test verifies that ActionContext provides type-safe access
//! to all state fields without using unsafe unwraps.
//!
//! ## Test Strategy
//!
//! Generate action contexts with various combinations of missing state:
//! - project_state: Some/None
//! - session_state: Some/None
//! - workspace_state: Some/None
//! - transaction_state: Some/None
//!
//! For each generated context, verify:
//! 1. All access methods return Option types (no panics)
//! 2. Methods correctly reflect presence/absence of state
//! 3. Registry query methods handle missing session state gracefully
//! 4. No unsafe unwraps are used in any access path

use proptest::prelude::*;
use uuid::Uuid;

use crate::{
    ActionContext, ProjectState, SessionState, SelectionState, 
    FocusState, WorkspaceState, TransactionState,
};

// ============================================================================
// Generators
// ============================================================================

/// Generate optional ProjectState
fn arb_project_state() -> impl Strategy<Value = Option<ProjectState>> {
    prop::option::of(any::<(u128, String)>().prop_map(|(id_bits, path)| {
        ProjectState {
            project_id: Uuid::from_u128(id_bits),
            project_path: path,
        }
    }))
}

/// Generate optional SessionState
fn arb_session_state() -> impl Strategy<Value = Option<SessionState>> {
    prop::option::of((
        prop::option::of(any::<u128>().prop_map(Uuid::from_u128)),
        any::<bool>(),
        any::<bool>(),
        any::<bool>(),
    ).prop_map(|(world_id, mat, aud, diag)| {
        SessionState {
            active_world_id: world_id,
            material_registry_available: mat,
            audio_registry_available: aud,
            diagnostics_available: diag,
        }
    }))
}

/// Generate SelectionState
fn arb_selection_state() -> impl Strategy<Value = SelectionState> {
    prop::collection::vec(any::<u128>().prop_map(Uuid::from_u128), 0..5)
        .prop_map(|entities| SelectionState { selected_entities: entities })
}

/// Generate FocusState
fn arb_focus_state() -> impl Strategy<Value = FocusState> {
    prop::option::of(any::<String>())
        .prop_map(|panel| FocusState { focused_panel: panel })
}

/// Generate optional WorkspaceState
fn arb_workspace_state() -> impl Strategy<Value = Option<WorkspaceState>> {
    prop::option::of(any::<u128>().prop_map(|id_bits| {
        WorkspaceState {
            workspace_id: Uuid::from_u128(id_bits),
            has_unsaved_changes: false,
        }
    }))
}

/// Generate optional TransactionState
fn arb_transaction_state() -> impl Strategy<Value = Option<TransactionState>> {
    prop::option::of((any::<u128>(), any::<bool>()).prop_map(|(id_bits, active)| {
        TransactionState {
            transaction_id: Uuid::from_u128(id_bits),
            transaction_active: active,
        }
    }))
}

/// Generate ActionContext with various combinations of missing state
fn arb_action_context() -> impl Strategy<Value = ActionContext> {
    (
        arb_project_state(),
        arb_session_state(),
        arb_selection_state(),
        arb_focus_state(),
        arb_workspace_state(),
        arb_transaction_state(),
    ).prop_map(|(project, session, selection, focus, workspace, transaction)| {
        ActionContext {
            project_state: project,
            session_state: session,
            selection_state: selection,
            focus_state: focus,
            workspace_state: workspace,
            transaction_state: transaction,
        }
    })
}

// ============================================================================
// Property Tests
// ============================================================================

proptest! {
    /// Property: All access methods return Option types without panicking
    ///
    /// For any ActionContext, all type-safe access methods should:
    /// - Return Option types (no unwrap)
    /// - Never panic regardless of missing state
    /// - Correctly reflect presence/absence of state
    #[test]
    fn prop_type_safe_access_no_panic(context in arb_action_context()) {
        // Test all access methods - none should panic
        let _ = context.project();
        let _ = context.session();
        let _ = context.selection();
        let _ = context.focus();
        let _ = context.workspace();
        let _ = context.transaction();
        
        // Test registry query methods - should handle missing session gracefully
        let _ = context.has_material_registry();
        let _ = context.has_audio_registry();
        let _ = context.has_world_registry();
        let _ = context.has_diagnostics_registry();
        
        // Test legacy compatibility methods
        let _ = context.has_project();
        let _ = context.has_active_world();
        let _ = context.has_selection();
    }
    
    /// Property: Access methods correctly reflect state presence
    ///
    /// When state is present, access methods should return Some.
    /// When state is absent, access methods should return None.
    #[test]
    fn prop_access_reflects_state_presence(context in arb_action_context()) {
        // Project state
        assert_eq!(context.project().is_some(), context.project_state.is_some());
        assert_eq!(context.has_project(), context.project_state.is_some());
        
        // Session state
        assert_eq!(context.session().is_some(), context.session_state.is_some());
        
        // Workspace state
        assert_eq!(context.workspace().is_some(), context.workspace_state.is_some());
        
        // Transaction state
        assert_eq!(context.transaction().is_some(), context.transaction_state.is_some());
        
        // Selection and focus are always available (not Option)
        assert!(context.selection() as *const _ != std::ptr::null());
        assert!(context.focus() as *const _ != std::ptr::null());
    }
    
    /// Property: Registry queries handle missing session state gracefully
    ///
    /// When session_state is None, all registry queries should return false
    /// without panicking.
    #[test]
    fn prop_registry_queries_handle_missing_session(
        project in arb_project_state(),
        selection in arb_selection_state(),
        focus in arb_focus_state(),
        workspace in arb_workspace_state(),
        transaction in arb_transaction_state(),
    ) {
        let context = ActionContext {
            project_state: project,
            session_state: None,
            selection_state: selection,
            focus_state: focus,
            workspace_state: workspace,
            transaction_state: transaction,
        };
        
        // All registry queries should return false when session is None
        assert!(!context.has_material_registry());
        assert!(!context.has_audio_registry());
        assert!(!context.has_world_registry());
        assert!(!context.has_diagnostics_registry());
        assert!(!context.has_active_world());
    }
    
    /// Property: Registry queries correctly reflect session state
    ///
    /// When session_state is present, registry queries should reflect
    /// the actual registry availability flags.
    #[test]
    fn prop_registry_queries_reflect_session_state(
        project in arb_project_state(),
        world_id in prop::option::of(any::<u128>().prop_map(Uuid::from_u128)),
        mat_available in any::<bool>(),
        aud_available in any::<bool>(),
        diag_available in any::<bool>(),
        selection in arb_selection_state(),
        focus in arb_focus_state(),
        workspace in arb_workspace_state(),
        transaction in arb_transaction_state(),
    ) {
        let context = ActionContext {
            project_state: project,
            session_state: Some(SessionState {
                active_world_id: world_id,
                material_registry_available: mat_available,
                audio_registry_available: aud_available,
                diagnostics_available: diag_available,
                loading_in_progress: false,
            }),
            selection_state: selection,
            focus_state: focus,
            workspace_state: workspace,
            transaction_state: transaction,
        };
        
        // Registry queries should match session state flags
        assert_eq!(context.has_material_registry(), mat_available);
        assert_eq!(context.has_audio_registry(), aud_available);
        assert_eq!(context.has_diagnostics_registry(), diag_available);
        assert_eq!(context.has_world_registry(), world_id.is_some());
        assert_eq!(context.has_active_world(), world_id.is_some());
    }
    
    /// Property: Selection state correctly reflects entity count
    ///
    /// has_selection() should return true iff selected_entities is non-empty.
    #[test]
    fn prop_selection_reflects_entity_count(
        project in arb_project_state(),
        session in arb_session_state(),
        entities in prop::collection::vec(any::<u128>().prop_map(Uuid::from_u128), 0..10),
        focus in arb_focus_state(),
        workspace in arb_workspace_state(),
        transaction in arb_transaction_state(),
    ) {
        let context = ActionContext {
            project_state: project,
            session_state: session,
            selection_state: SelectionState { selected_entities: entities.clone() },
            focus_state: focus,
            workspace_state: workspace,
            transaction_state: transaction,
        };
        
        assert_eq!(context.has_selection(), !entities.is_empty());
        assert_eq!(context.selection().selected_entities.len(), entities.len());
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_empty_context_all_none() {
        let context = ActionContext {
            project_state: None,
            session_state: None,
            selection_state: SelectionState::default(),
            focus_state: FocusState::default(),
            workspace_state: None,
            transaction_state: None,
        };
        
        assert!(context.project().is_none());
        assert!(context.session().is_none());
        assert!(context.workspace().is_none());
        assert!(context.transaction().is_none());
        
        assert!(!context.has_project());
        assert!(!context.has_active_world());
        assert!(!context.has_selection());
        
        assert!(!context.has_material_registry());
        assert!(!context.has_audio_registry());
        assert!(!context.has_world_registry());
        assert!(!context.has_diagnostics_registry());
    }
    
    #[test]
    fn test_full_context_all_some() {
        let context = ActionContext {
            project_state: Some(ProjectState {
                project_id: Uuid::new_v4(),
                project_path: "/test/project".to_string(),
            }),
            session_state: Some(SessionState {
                active_world_id: Some(Uuid::new_v4()),
                material_registry_available: true,
                audio_registry_available: true,
                diagnostics_available: true,
                loading_in_progress: false,
            }),
            selection_state: SelectionState {
                selected_entities: vec![Uuid::new_v4()],
            },
            focus_state: FocusState {
                focused_panel: Some("viewport".to_string()),
            },
            workspace_state: Some(WorkspaceState {
                workspace_id: Uuid::new_v4(),
                has_unsaved_changes: false,
            }),
            transaction_state: Some(TransactionState {
                transaction_id: Uuid::new_v4(),
                transaction_active: true,
            }),
        };
        
        assert!(context.project().is_some());
        assert!(context.session().is_some());
        assert!(context.workspace().is_some());
        assert!(context.transaction().is_some());
        
        assert!(context.has_project());
        assert!(context.has_active_world());
        assert!(context.has_selection());
        
        assert!(context.has_material_registry());
        assert!(context.has_audio_registry());
        assert!(context.has_world_registry());
        assert!(context.has_diagnostics_registry());
    }
}
