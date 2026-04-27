//! Property Test: Cache Invalidation Propagation
//!
//! Feature: editor-state-ownership-normalization
//! Property 5: Cache Invalidation Propagation
//! Validates: Requirements 3.4, 12.1, 12.2
//!
//! For any owner container state change, only caches that depend on the changed state
//! (according to the state graph) are invalidated, and all such dependent caches are invalidated.

use proptest::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use stratumx_editor_state_containers::{
    CacheEntry, CacheId, CacheLayer, DiagnosticsState, ProjectIdentity, ProjectState,
    StateContainerSystem, StateId, WorkspaceIdentity, WorkspaceState,
};

fn create_test_system() -> Arc<StateContainerSystem> {
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

    Arc::new(
        StateContainerSystem::new(project_state, workspace_state, diagnostics_state)
            .expect("Failed to create test system"),
    )
}

/// Mock cache entry for testing
struct MockCacheEntry {
    valid: bool,
    dependencies: Vec<StateId>,
    rebuild_count: usize,
}

impl MockCacheEntry {
    fn new(dependencies: Vec<StateId>) -> Self {
        Self {
            valid: true, // Start valid
            dependencies,
            rebuild_count: 0,
        }
    }
}

impl CacheEntry for MockCacheEntry {
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

/// Strategy for generating owner container state IDs
fn owner_state_id() -> impl Strategy<Value = StateId> {
    prop_oneof![
        Just(StateId::ProjectState),
        Just(StateId::WorkspaceState),
        Just(StateId::DiagnosticsState),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// **Validates: Requirements 3.4, 12.1, 12.2**
    ///
    /// Property 5: Cache Invalidation Propagation (selective invalidation)
    ///
    /// When a specific owner state changes, only caches that depend on that state
    /// are invalidated, not all caches.
    #[test]
    fn property_cache_invalidation_propagation_selective(changed_state in owner_state_id()) {
        let state_system = create_test_system();
        let cache_layer = CacheLayer::new(state_system.clone());

        // Register caches with different dependencies
        let cache1 = Box::new(MockCacheEntry::new(vec![StateId::ProjectState]));
        let cache2 = Box::new(MockCacheEntry::new(vec![StateId::WorkspaceState]));
        let cache3 = Box::new(MockCacheEntry::new(vec![StateId::DiagnosticsState]));

        cache_layer.register_cache(CacheId::MaterialRegistry, cache1);
        cache_layer.register_cache(CacheId::TerrainPreview, cache2);
        cache_layer.register_cache(CacheId::ViewportStatistics, cache3);

        // Reset metrics
        cache_layer.reset_metrics();

        // Invalidate caches dependent on the changed state
        cache_layer.invalidate_dependent_on(changed_state.clone());

        let metrics = cache_layer.get_metrics();

        // Exactly one cache should be invalidated (the one depending on changed_state)
        prop_assert_eq!(
            metrics.invalidation_count, 1,
            "Expected exactly 1 cache to be invalidated for state {:?}, but got {}",
            changed_state, metrics.invalidation_count
        );

        // Verify the correct cache was invalidated by checking which one is invalid
        let material_valid = cache_layer.is_cache_valid(CacheId::MaterialRegistry);
        let terrain_valid = cache_layer.is_cache_valid(CacheId::TerrainPreview);
        let viewport_valid = cache_layer.is_cache_valid(CacheId::ViewportStatistics);

        match changed_state {
            StateId::ProjectState => {
                prop_assert_eq!(material_valid, Some(false), "MaterialRegistry should be invalid");
                prop_assert_eq!(terrain_valid, Some(true), "TerrainPreview should be valid");
                prop_assert_eq!(viewport_valid, Some(true), "ViewportStatistics should be valid");
            }
            StateId::WorkspaceState => {
                prop_assert_eq!(material_valid, Some(true), "MaterialRegistry should be valid");
                prop_assert_eq!(terrain_valid, Some(false), "TerrainPreview should be invalid");
                prop_assert_eq!(viewport_valid, Some(true), "ViewportStatistics should be valid");
            }
            StateId::DiagnosticsState => {
                prop_assert_eq!(material_valid, Some(true), "MaterialRegistry should be valid");
                prop_assert_eq!(terrain_valid, Some(true), "TerrainPreview should be valid");
                prop_assert_eq!(viewport_valid, Some(false), "ViewportStatistics should be invalid");
            }
            _ => {}
        }
    }

    /// **Validates: Requirements 3.4, 12.1, 12.2**
    ///
    /// Property 5: Cache Invalidation Propagation (completeness)
    ///
    /// When a state changes, ALL caches that depend on it are invalidated.
    #[test]
    fn property_cache_invalidation_propagation_completeness(changed_state in owner_state_id()) {
        let state_system = create_test_system();
        let cache_layer = CacheLayer::new(state_system.clone());

        // Register multiple caches that depend on the same state
        let cache1 = Box::new(MockCacheEntry::new(vec![changed_state.clone()]));
        let cache2 = Box::new(MockCacheEntry::new(vec![changed_state.clone()]));
        let cache3 = Box::new(MockCacheEntry::new(vec![changed_state.clone()]));

        cache_layer.register_cache(CacheId::MaterialRegistry, cache1);
        cache_layer.register_cache(CacheId::TerrainPreview, cache2);
        cache_layer.register_cache(CacheId::ViewportStatistics, cache3);

        // Reset metrics
        cache_layer.reset_metrics();

        // Invalidate caches dependent on the changed state
        cache_layer.invalidate_dependent_on(changed_state.clone());

        let metrics = cache_layer.get_metrics();

        // All 3 caches should be invalidated
        prop_assert_eq!(
            metrics.invalidation_count, 3,
            "Expected all 3 caches to be invalidated for state {:?}, but got {}",
            changed_state, metrics.invalidation_count
        );

        // Verify all caches are invalid
        let material_valid = cache_layer.is_cache_valid(CacheId::MaterialRegistry);
        let terrain_valid = cache_layer.is_cache_valid(CacheId::TerrainPreview);
        let viewport_valid = cache_layer.is_cache_valid(CacheId::ViewportStatistics);

        prop_assert_eq!(material_valid, Some(false), "MaterialRegistry should be invalid");
        prop_assert_eq!(terrain_valid, Some(false), "TerrainPreview should be invalid");
        prop_assert_eq!(viewport_valid, Some(false), "ViewportStatistics should be invalid");
    }

    /// **Validates: Requirements 3.4, 12.1, 12.2**
    ///
    /// Property 5: Cache Invalidation Propagation (no false positives)
    ///
    /// When a state changes, caches that do NOT depend on it are NOT invalidated.
    #[test]
    fn property_cache_invalidation_propagation_no_false_positives(changed_state in owner_state_id()) {
        let state_system = create_test_system();
        let cache_layer = CacheLayer::new(state_system.clone());

        // Determine an independent state (one that's different from changed_state)
        let independent_state = match changed_state {
            StateId::ProjectState => StateId::WorkspaceState,
            StateId::WorkspaceState => StateId::DiagnosticsState,
            StateId::DiagnosticsState => StateId::ProjectState,
            _ => StateId::ProjectState,
        };

        // Register a cache that depends on the independent state
        let cache = Box::new(MockCacheEntry::new(vec![independent_state.clone()]));
        cache_layer.register_cache(CacheId::MaterialRegistry, cache);

        // Reset metrics
        cache_layer.reset_metrics();

        // Invalidate caches dependent on the changed state
        cache_layer.invalidate_dependent_on(changed_state.clone());

        let metrics = cache_layer.get_metrics();

        // No caches should be invalidated (the cache depends on a different state)
        prop_assert_eq!(
            metrics.invalidation_count, 0,
            "Expected 0 caches to be invalidated for state {:?}, but got {}",
            changed_state, metrics.invalidation_count
        );

        // Verify the cache is still valid
        let cache_valid = cache_layer.get_or_compute(CacheId::MaterialRegistry, |e| Some(e.is_valid()));
        prop_assert_eq!(cache_valid, Some(true), "Cache should still be valid");
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_selective_invalidation() {
        let state_system = create_test_system();
        let cache_layer = CacheLayer::new(state_system.clone());

        // Register caches with different dependencies
        let cache1 = Box::new(MockCacheEntry::new(vec![StateId::ProjectState]));
        let cache2 = Box::new(MockCacheEntry::new(vec![StateId::WorkspaceState]));

        cache_layer.register_cache(CacheId::MaterialRegistry, cache1);
        cache_layer.register_cache(CacheId::TerrainPreview, cache2);

        // Invalidate caches dependent on ProjectState
        cache_layer.invalidate_dependent_on(StateId::ProjectState);

        // Only MaterialRegistry should be invalidated
        let material_valid = cache_layer.is_cache_valid(CacheId::MaterialRegistry);
        let terrain_valid = cache_layer.is_cache_valid(CacheId::TerrainPreview);

        assert_eq!(material_valid, Some(false));
        assert_eq!(terrain_valid, Some(true));
    }

    #[test]
    fn test_complete_invalidation() {
        let state_system = create_test_system();
        let cache_layer = CacheLayer::new(state_system.clone());

        // Register multiple caches that depend on the same state
        let cache1 = Box::new(MockCacheEntry::new(vec![StateId::ProjectState]));
        let cache2 = Box::new(MockCacheEntry::new(vec![StateId::ProjectState]));

        cache_layer.register_cache(CacheId::MaterialRegistry, cache1);
        cache_layer.register_cache(CacheId::TerrainPreview, cache2);

        // Invalidate caches dependent on ProjectState
        cache_layer.invalidate_dependent_on(StateId::ProjectState);

        let metrics = cache_layer.get_metrics();
        assert_eq!(metrics.invalidation_count, 2);
    }

    #[test]
    fn test_no_false_positives() {
        let state_system = create_test_system();
        let cache_layer = CacheLayer::new(state_system.clone());

        // Register a cache that depends on WorkspaceState
        let cache = Box::new(MockCacheEntry::new(vec![StateId::WorkspaceState]));
        cache_layer.register_cache(CacheId::MaterialRegistry, cache);

        // Invalidate caches dependent on ProjectState
        cache_layer.invalidate_dependent_on(StateId::ProjectState);

        // Cache should still be valid
        let cache_valid = cache_layer.is_cache_valid(CacheId::MaterialRegistry);
        assert_eq!(cache_valid, Some(true));

        let metrics = cache_layer.get_metrics();
        assert_eq!(metrics.invalidation_count, 0);
    }
}
