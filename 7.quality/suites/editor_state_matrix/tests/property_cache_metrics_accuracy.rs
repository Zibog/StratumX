//! Property Test: Cache Metrics Accuracy
//!
//! **Validates: Requirements 12.4**
//!
//! **Property 17: Cache Metrics Accuracy**
//! For any cache operation (hit, miss, invalidation, rebuild), the cache
//! metrics are updated to reflect the operation, and querying metrics
//! returns accurate counts.

use proptest::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use stratumx_editor_state_containers::{
    CacheId, CacheLayer, DiagnosticsState, ProjectIdentity, ProjectState, StateContainerSystem,
    TerrainPreviewCache, TerrainState, WorkspaceIdentity, WorkspaceState, WorldIdentity,
    WorldState,
};
use uuid::Uuid;

// Strategy for generating operation sequences
#[derive(Debug, Clone, Copy)]
enum CacheOperation {
    Access,
    Invalidate,
    RebuildAll,
    ClearAll,
}

fn operation_sequence_strategy() -> impl Strategy<Value = Vec<CacheOperation>> {
    prop::collection::vec(
        prop_oneof![
            Just(CacheOperation::Access),
            Just(CacheOperation::Invalidate),
            Just(CacheOperation::RebuildAll),
            Just(CacheOperation::ClearAll),
        ],
        1..20,
    )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Feature: editor-state-ownership-normalization, Property 17: Cache Metrics Accuracy
    ///
    /// Tests that cache metrics accurately reflect all cache operations.
    #[test]
    fn property_cache_metrics_accuracy(
        operations in operation_sequence_strategy(),
    ) {
        // Create state container system with world state
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
            project_state,
            workspace_state,
            diagnostics_state,
        ).expect("Failed to create state system");

        // Add world state with terrain
        let world_id = WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );
        let mut world_state = WorldState::new(world_id, "snapshot_123".to_string());
        let terrain = TerrainState::new(
            (1024, 1024),
            (1000.0, 1000.0),
            "default".to_string(),
        );
        world_state.set_terrain_state(Some(terrain));
        state_system.set_world_state(Arc::new(Mutex::new(world_state)));

        let state_system = Arc::new(state_system);
        let cache_layer = CacheLayer::new(state_system.clone());

        // Register terrain preview cache
        let terrain_cache = Box::new(TerrainPreviewCache::new());
        cache_layer.register_cache(CacheId::TerrainPreview, terrain_cache);

        // Track expected metrics manually
        let mut expected_hits = 0u64;
        let mut expected_misses = 0u64;
        let mut expected_invalidations = 0u64;
        let mut expected_rebuilds = 0u64;

        // Cache starts invalid, so first access will be a miss + rebuild
        let mut cache_is_valid = false;

        // Execute operations and track expected metrics
        for op in operations {
            match op {
                CacheOperation::Access => {
                    let _ = cache_layer.get_or_compute(CacheId::TerrainPreview, |entry| {
                        let terrain_entry = entry as *const dyn stratumx_editor_state_containers::CacheEntry;
                        let terrain_entry = unsafe { &*(terrain_entry as *const TerrainPreviewCache) };
                        Some(terrain_entry.get_preview_data().to_vec())
                    });

                    if cache_is_valid {
                        expected_hits += 1;
                    } else {
                        expected_misses += 1;
                        expected_rebuilds += 1;
                        cache_is_valid = true;
                    }
                }
                CacheOperation::Invalidate => {
                    cache_layer.invalidate(CacheId::TerrainPreview);
                    expected_invalidations += 1;
                    cache_is_valid = false;
                }
                CacheOperation::RebuildAll => {
                    cache_layer.rebuild_all();
                    expected_rebuilds += 1;
                    cache_is_valid = true;
                }
                CacheOperation::ClearAll => {
                    cache_layer.clear_all();
                    expected_invalidations += 1;
                    cache_is_valid = false;
                }
            }
        }

        // Get actual metrics
        let metrics = cache_layer.get_metrics();

        // Verify metrics match expected values
        prop_assert_eq!(metrics.hit_count, expected_hits,
            "Hit count mismatch: expected {}, got {}", expected_hits, metrics.hit_count);
        prop_assert_eq!(metrics.miss_count, expected_misses,
            "Miss count mismatch: expected {}, got {}", expected_misses, metrics.miss_count);
        prop_assert_eq!(metrics.invalidation_count, expected_invalidations,
            "Invalidation count mismatch: expected {}, got {}", expected_invalidations, metrics.invalidation_count);
        prop_assert_eq!(metrics.rebuild_count, expected_rebuilds,
            "Rebuild count mismatch: expected {}, got {}", expected_rebuilds, metrics.rebuild_count);
    }
}
