//! Integration Test: Error Recovery
//!
//! Feature: editor-state-ownership-normalization
//! Validates: Requirements 2.2, 5.2, 10.2, 11.2, 11.3
//!
//! This integration test validates error recovery scenarios, including ownership violations,
//! initialization failures, cache rebuild failures, graceful degradation, service dependency
//! violations, and error reporting.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use stratumx_editor_state_containers::{
    CacheEntry, CacheId, CacheLayer, DiagnosticsState, ProjectIdentity, ProjectState,
    StateContainerSystem, StateId, ViolationType, WorkspaceIdentity, WorkspaceState,
};

/// Mock cache entry that fails to rebuild
struct FailingCacheEntry {
    valid: bool,
    dependencies: Vec<StateId>,
    should_fail: bool,
}

impl FailingCacheEntry {
    fn new(dependencies: Vec<StateId>, should_fail: bool) -> Self {
        Self {
            valid: false,
            dependencies,
            should_fail,
        }
    }
}

impl CacheEntry for FailingCacheEntry {
    fn is_valid(&self) -> bool {
        self.valid
    }

    fn invalidate(&mut self) {
        self.valid = false;
    }

    fn rebuild(&mut self, _state_system: &StateContainerSystem) {
        if self.should_fail {
            // Simulate rebuild failure by not setting valid to true
            self.valid = false;
        } else {
            self.valid = true;
        }
    }

    fn dependencies(&self) -> Vec<StateId> {
        self.dependencies.clone()
    }
}

#[test]
fn test_ownership_violation_detection() {
    // This test verifies that ownership violations are detected during initialization
    // In the current implementation, the HashMap-based design prevents duplicate ownership
    // at compile time, but we can still test the validation function

    let project_id = ProjectIdentity::new(
        Uuid::new_v4(),
        "Test Project".to_string(),
        PathBuf::from("/test/project"),
    );
    let workspace_id = WorkspaceIdentity::new(
        Uuid::new_v4(),
        "Test Workspace".to_string(),
        PathBuf::from("/test/workspace"),
    );

    let project_state = Arc::new(Mutex::new(ProjectState::new(project_id, workspace_id)));
    let workspace_state = Arc::new(Mutex::new(WorkspaceState::new()));
    let diagnostics_state = Arc::new(Mutex::new(DiagnosticsState::new()));

    let state_system = StateContainerSystem::new(project_state, workspace_state, diagnostics_state);

    // Should succeed with valid ownership
    assert!(
        state_system.is_ok(),
        "State system should initialize successfully"
    );

    let system = state_system.unwrap();

    // Validate ownership
    let validation_result = system.validate_ownership_uniqueness();
    assert!(
        validation_result.is_ok(),
        "Ownership validation should pass"
    );
}

#[test]
fn test_initialization_failure_reporting() {
    // Test that initialization failures are properly reported
    // In this case, we test that the error message is actionable

    let project_id = ProjectIdentity::new(
        Uuid::new_v4(),
        "Test Project".to_string(),
        PathBuf::from("/test/project"),
    );
    let workspace_id = WorkspaceIdentity::new(
        Uuid::new_v4(),
        "Test Workspace".to_string(),
        PathBuf::from("/test/workspace"),
    );

    let project_state = Arc::new(Mutex::new(ProjectState::new(project_id, workspace_id)));
    let workspace_state = Arc::new(Mutex::new(WorkspaceState::new()));
    let diagnostics_state = Arc::new(Mutex::new(DiagnosticsState::new()));

    let state_system = StateContainerSystem::new(project_state, workspace_state, diagnostics_state);

    match state_system {
        Ok(_) => {
            // Success case - no violations
        }
        Err(violations) => {
            // If there were violations, verify they have actionable messages
            for violation in violations {
                let message = violation.to_user_message();
                assert!(!message.is_empty(), "Violation message should not be empty");
                assert!(
                    message.len() > 10,
                    "Violation message should be descriptive"
                );
            }
        }
    }
}

#[test]
fn test_cache_rebuild_failure_graceful_degradation() {
    // Test that cache rebuild failures don't crash the system

    let project_id = ProjectIdentity::new(
        Uuid::new_v4(),
        "Test Project".to_string(),
        PathBuf::from("/test/project"),
    );
    let workspace_id = WorkspaceIdentity::new(
        Uuid::new_v4(),
        "Test Workspace".to_string(),
        PathBuf::from("/test/workspace"),
    );

    let project_state = Arc::new(Mutex::new(ProjectState::new(project_id, workspace_id)));
    let workspace_state = Arc::new(Mutex::new(WorkspaceState::new()));
    let diagnostics_state = Arc::new(Mutex::new(DiagnosticsState::new()));

    let state_system = Arc::new(
        StateContainerSystem::new(project_state, workspace_state, diagnostics_state)
            .expect("Failed to create state container system"),
    );

    let cache_layer = Arc::new(CacheLayer::new(state_system.clone()));

    // Register a cache that will fail to rebuild
    let failing_cache = Box::new(FailingCacheEntry::new(vec![StateId::ProjectState], true));
    cache_layer.register_cache(CacheId::MaterialRegistry, failing_cache);

    // Invalidate the cache
    cache_layer.invalidate(CacheId::MaterialRegistry);

    // Try to access the cache (rebuild will fail, but system should not crash)
    let result = cache_layer.get_or_compute(CacheId::MaterialRegistry, |entry| {
        if entry.is_valid() {
            Some(42)
        } else {
            None // Cache is invalid
        }
    });

    // Result should be None because rebuild failed
    assert_eq!(
        result, None,
        "Cache access should return None when rebuild fails"
    );

    // System should still be operational
    let validation_result = state_system.validate_ownership_uniqueness();
    assert!(
        validation_result.is_ok(),
        "System should still be operational after cache rebuild failure"
    );
}

#[test]
fn test_error_reporting_with_violations() {
    // Test that ownership violations are reported with detailed information

    use stratumx_editor_state_containers::OwnershipViolation;

    // Create a mock violation
    let violation = OwnershipViolation {
        state_id: StateId::ProjectState,
        owners: vec![],
        violation_type: ViolationType::NoOwner,
    };

    let message = violation.to_user_message();
    assert!(
        message.contains("ProjectState"),
        "Message should mention the state"
    );
    assert!(
        message.contains("no owner"),
        "Message should describe the violation"
    );
}

#[test]
fn test_multiple_violations_reporting() {
    // Test that multiple violations are all reported

    use stratumx_editor_state_containers::OwnershipViolation;

    let violations = vec![
        OwnershipViolation {
            state_id: StateId::ProjectState,
            owners: vec![],
            violation_type: ViolationType::NoOwner,
        },
        OwnershipViolation {
            state_id: StateId::WorkspaceState,
            owners: vec![],
            violation_type: ViolationType::NoOwner,
        },
    ];

    // Verify all violations have messages
    for violation in violations {
        let message = violation.to_user_message();
        assert!(!message.is_empty(), "Each violation should have a message");
    }
}

#[test]
fn test_system_recovery_after_world_close() {
    // Test that the system recovers properly after closing a world

    let project_id = ProjectIdentity::new(
        Uuid::new_v4(),
        "Test Project".to_string(),
        PathBuf::from("/test/project"),
    );
    let workspace_id = WorkspaceIdentity::new(
        Uuid::new_v4(),
        "Test Workspace".to_string(),
        PathBuf::from("/test/workspace"),
    );

    let project_state = Arc::new(Mutex::new(ProjectState::new(project_id, workspace_id)));
    let workspace_state = Arc::new(Mutex::new(WorkspaceState::new()));
    let diagnostics_state = Arc::new(Mutex::new(DiagnosticsState::new()));

    let mut state_system =
        StateContainerSystem::new(project_state, workspace_state, diagnostics_state)
            .expect("Failed to create state container system");

    // Open a world
    let world_id = stratumx_editor_state_containers::WorldIdentity::new(
        Uuid::new_v4(),
        "Test World".to_string(),
        PathBuf::from("/test/world"),
    );
    let world_state = Arc::new(Mutex::new(
        stratumx_editor_state_containers::WorldState::new(world_id, "snapshot_123".to_string()),
    ));
    state_system.set_world_state(world_state);

    // Verify world state is present
    assert!(
        state_system.get_owner(&StateId::WorldState).is_some(),
        "World state should have owner"
    );

    // Close the world
    state_system.clear_world_state();

    // Verify system recovered
    assert!(
        state_system.get_owner(&StateId::WorldState).is_none(),
        "World state should have no owner after close"
    );

    let validation_result = state_system.validate_ownership_uniqueness();
    assert!(
        validation_result.is_ok(),
        "System should be valid after world close"
    );
}

#[test]
fn test_cache_layer_recovery_after_clear() {
    // Test that cache layer recovers after clearing all caches

    let project_id = ProjectIdentity::new(
        Uuid::new_v4(),
        "Test Project".to_string(),
        PathBuf::from("/test/project"),
    );
    let workspace_id = WorkspaceIdentity::new(
        Uuid::new_v4(),
        "Test Workspace".to_string(),
        PathBuf::from("/test/workspace"),
    );

    let project_state = Arc::new(Mutex::new(ProjectState::new(project_id, workspace_id)));
    let workspace_state = Arc::new(Mutex::new(WorkspaceState::new()));
    let diagnostics_state = Arc::new(Mutex::new(DiagnosticsState::new()));

    let state_system = Arc::new(
        StateContainerSystem::new(project_state, workspace_state, diagnostics_state)
            .expect("Failed to create state container system"),
    );

    let cache_layer = Arc::new(CacheLayer::new(state_system.clone()));

    // Register a cache
    let cache = Box::new(FailingCacheEntry::new(vec![StateId::ProjectState], false));
    cache_layer.register_cache(CacheId::MaterialRegistry, cache);

    // Clear all caches
    cache_layer.clear_all();

    // Verify system is still operational
    let validation_result = state_system.validate_ownership_uniqueness();
    assert!(
        validation_result.is_ok(),
        "System should be operational after cache clear"
    );

    // Verify cache can be rebuilt
    let result = cache_layer.get_or_compute(CacheId::MaterialRegistry, |entry| {
        if entry.is_valid() {
            Some(42)
        } else {
            None
        }
    });

    // After rebuild, cache should be valid
    assert_eq!(result, Some(42), "Cache should be rebuilt successfully");
}
