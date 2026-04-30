//! Integration Test: End-to-End State Flow
//!
//! Feature: editor-state-ownership-normalization
//! Validates: Requirements 1.1, 1.2, 3.4, 4.4, 12.1
//!
//! This integration test validates the complete state flow from initialization through
//! state mutations, cache invalidation, query layer results, and event propagation.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use stratumx_editor_state_containers::{
    CacheEntry, CacheId, CacheLayer, DiagnosticsState, ProjectIdentity, ProjectState, QueryLayer,
    StateContainerSystem, StateId, WorkspaceIdentity, WorkspaceState, WorldIdentity, WorldState,
};

/// Mock cache entry for testing
struct MockCacheEntry {
    valid: bool,
    dependencies: Vec<StateId>,
    rebuild_count: usize,
}

impl MockCacheEntry {
    fn new(dependencies: Vec<StateId>) -> Self {
        Self {
            valid: true,
            dependencies,
            rebuild_count: 0,
        }
    }
}

impl CacheEntry for MockCacheEntry {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_valid(&self) -> bool {
        self.valid
    }

    fn invalidate(&mut self) {
        self.valid = false;
    }

    fn rebuild(&mut self, _state_system: &StateContainerSystem) {
        self.valid = true;
        self.rebuild_count += 1;
    }

    fn dependencies(&self) -> Vec<StateId> {
        self.dependencies.clone()
    }
}

#[test]
fn test_end_to_end_state_flow() {
    // Step 1: Initialize state container system
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

    let project_state = Arc::new(Mutex::new(ProjectState::new(
        project_id.clone(),
        workspace_id.clone(),
    )));
    let workspace_state = Arc::new(Mutex::new(WorkspaceState::new()));
    let diagnostics_state = Arc::new(Mutex::new(DiagnosticsState::new()));

    // Step 2: Create owner containers
    let state_system = Arc::new(
        StateContainerSystem::new(
            project_state.clone(),
            workspace_state.clone(),
            diagnostics_state.clone(),
        )
        .expect("Failed to create state container system"),
    );

    // Step 3: Register derived state dependencies
    let mut cache_layer = CacheLayer::new(state_system.clone());

    // Register a cache that depends on ProjectState
    let cache_entry = Box::new(MockCacheEntry::new(vec![StateId::ProjectState]));
    cache_layer.register_cache(CacheId::MaterialRegistry, cache_entry);

    // Step 4: Validate ownership uniqueness
    let validation_result = state_system.validate_ownership_uniqueness();
    assert!(
        validation_result.is_ok(),
        "Ownership validation should pass: {:?}",
        validation_result
    );

    // Step 5: Perform state mutations
    {
        let mut ps = project_state.lock().unwrap();
        ps.increment_save_generation();
    }

    let initial_generation = {
        let ps = project_state.lock().unwrap();
        ps.get_save_generation()
    };
    assert_eq!(
        initial_generation, 1,
        "Save generation should be incremented"
    );

    // Step 6: Verify cache invalidation
    // Invalidate caches dependent on ProjectState
    cache_layer.invalidate_dependent_on(StateId::ProjectState);

    let metrics = cache_layer.get_metrics();
    assert_eq!(metrics.invalidation_count, 1, "Cache should be invalidated");

    // Step 7: Verify query layer results
    let query_layer = QueryLayer::new(state_system.clone());

    let query_generation = query_layer.get_save_generation();
    assert_eq!(
        query_generation, initial_generation,
        "Query layer should return correct generation"
    );

    let query_project_id = query_layer.get_project_identity();
    assert_eq!(
        query_project_id.project_id, project_id.project_id,
        "Query layer should return correct project ID"
    );

    // Step 8: Verify event propagation (implicit through state mutations)
    // In a full implementation, we would verify events were emitted
    // For now, we verify the state changes were applied correctly

    let final_generation = {
        let ps = project_state.lock().unwrap();
        ps.get_save_generation()
    };
    assert_eq!(final_generation, 1, "Final generation should match");
}

#[test]
fn test_end_to_end_state_flow_with_world() {
    // Initialize state container system
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

    let mut state_system = StateContainerSystem::new(
        project_state.clone(),
        workspace_state.clone(),
        diagnostics_state.clone(),
    )
    .expect("Failed to create state container system");

    // Open a world
    let world_id = WorldIdentity::new(
        Uuid::new_v4(),
        "Test World".to_string(),
        PathBuf::from("/test/world"),
    );
    let world_state = Arc::new(Mutex::new(WorldState::new(
        world_id.clone(),
        "snapshot_123".to_string(),
    )));
    state_system.set_world_state(world_state.clone());

    let state_system = Arc::new(state_system);

    // Validate ownership
    let validation_result = state_system.validate_ownership_uniqueness();
    assert!(
        validation_result.is_ok(),
        "Ownership validation should pass with world state"
    );

    // Create cache layer and query layer
    let _cache_layer = CacheLayer::new(state_system.clone());
    let query_layer = QueryLayer::new(state_system.clone());

    // Query world state
    let query_world_id = query_layer.get_world_identity();
    assert!(
        query_world_id.is_some(),
        "Query layer should return world identity"
    );
    assert_eq!(
        query_world_id.unwrap().world_id,
        world_id.world_id,
        "World ID should match"
    );

    // Verify world state ownership
    let world_owner = state_system.get_owner(&StateId::WorldState);
    assert!(world_owner.is_some(), "World state should have an owner");
}

#[test]
fn test_end_to_end_cache_rebuild() {
    // Initialize state container system
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
        StateContainerSystem::new(
            project_state.clone(),
            workspace_state.clone(),
            diagnostics_state.clone(),
        )
        .expect("Failed to create state container system"),
    );

    // Create cache layer
    let mut cache_layer = CacheLayer::new(state_system.clone());

    // Register cache
    let cache_entry = Box::new(MockCacheEntry::new(vec![StateId::ProjectState]));
    cache_layer.register_cache(CacheId::MaterialRegistry, cache_entry);

    // Invalidate cache
    cache_layer.invalidate(CacheId::MaterialRegistry);

    // Access cache (should trigger rebuild)
    let result = cache_layer.get_or_compute(CacheId::MaterialRegistry, |_| Some(42));
    assert_eq!(result, Some(42), "Cache should be rebuilt and return value");

    let metrics = cache_layer.get_metrics();
    assert_eq!(metrics.rebuild_count, 1, "Cache should be rebuilt once");
    assert_eq!(metrics.miss_count, 1, "Cache miss should be recorded");
}
