//! Property Test: Derived State Rebuildability
//!
//! **Validates: Requirements 3.1, 3.2, 5.1, 5.2**
//!
//! **Property 3: Derived State Rebuildability**
//! For any derived state D, invalidating D and rebuilding it from owner
//! container state produces a value equivalent to the original D
//! (assuming owner state unchanged).

use proptest::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use stratumx_editor_state_containers::{
    CacheId, CacheLayer, ChunkId, DiagnosticsState, MaterialRegistryCache, ProjectIdentity,
    ProjectState, StateContainerSystem, TerrainPreviewCache, TerrainState, WorkspaceIdentity,
    WorkspaceState, WorldIdentity, WorldState,
};
use uuid::Uuid;

// Strategy for generating world identities
fn world_identity_strategy() -> impl Strategy<Value = WorldIdentity> {
    (any::<u128>(), "[a-zA-Z0-9 ]{5,20}", "[a-z/]{5,20}").prop_map(|(id_num, name, path)| {
        let uuid = Uuid::from_u128(id_num);
        WorldIdentity::new(uuid, name, PathBuf::from(format!("/{}", path)))
    })
}

// Strategy for generating terrain state
fn terrain_state_strategy() -> impl Strategy<Value = TerrainState> {
    (
        256u32..2048u32,
        256u32..2048u32,
        100.0f32..10000.0f32,
        "[a-z]{5,10}",
    )
        .prop_map(|(width, height, size_x, layer)| {
            TerrainState::new((width, height), (size_x, size_x), layer)
        })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Feature: editor-state-ownership-normalization, Property 3: Derived State Rebuildability
    ///
    /// Tests that invalidating and rebuilding derived state produces equivalent values.
    #[test]
    fn property_derived_state_rebuildability_terrain_preview(
        world_id in world_identity_strategy(),
        terrain in terrain_state_strategy(),
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
        let mut world_state = WorldState::new(world_id, "snapshot_123".to_string());
        world_state.set_terrain_state(Some(terrain));
        state_system.set_world_state(Arc::new(Mutex::new(world_state)));

        let state_system = Arc::new(state_system);
        let mut cache_layer = CacheLayer::new(state_system.clone());

        // Register terrain preview cache
        let terrain_cache = Box::new(TerrainPreviewCache::new());
        cache_layer.register_cache(CacheId::TerrainPreview, terrain_cache);

        // Build initial cache
        let initial_preview = cache_layer.get_or_compute(CacheId::TerrainPreview, |entry| {
            let terrain_entry = entry
                .as_any()
                .downcast_ref::<TerrainPreviewCache>()
                .expect("terrain preview cache entry");
            Some((
                terrain_entry.entry_count(),
                terrain_entry
                    .get_preview_data(&ChunkId { x: 0, y: 0 })
                    .cloned(),
            ))
        });

        // Invalidate cache
        cache_layer.invalidate(CacheId::TerrainPreview);

        // Rebuild cache
        let rebuilt_preview = cache_layer.get_or_compute(CacheId::TerrainPreview, |entry| {
            let terrain_entry = entry
                .as_any()
                .downcast_ref::<TerrainPreviewCache>()
                .expect("terrain preview cache entry");
            Some((
                terrain_entry.entry_count(),
                terrain_entry
                    .get_preview_data(&ChunkId { x: 0, y: 0 })
                    .cloned(),
            ))
        });

        // Verify equivalence
        prop_assert_eq!(initial_preview, rebuilt_preview);
    }
}

#[cfg(test)]
mod material_registry_tests {
    use super::*;

    /// Feature: editor-state-ownership-normalization, Property 3: Derived State Rebuildability
    ///
    /// Tests that invalidating and rebuilding material registry cache produces equivalent values.
    #[test]
    fn property_derived_state_rebuildability_material_registry() {
        // Create state container system
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
                .expect("Failed to create state system"),
        );

        let mut cache_layer = CacheLayer::new(state_system.clone());

        // Register material registry cache
        let material_cache = Box::new(MaterialRegistryCache::new());
        cache_layer.register_cache(CacheId::MaterialRegistry, material_cache);

        // Build initial cache
        let initial_count = cache_layer.get_or_compute(CacheId::MaterialRegistry, |entry| {
            let material_entry = entry
                .as_any()
                .downcast_ref::<MaterialRegistryCache>()
                .expect("material registry cache entry");
            Some(material_entry.get_all_profiles().len())
        });

        // Invalidate cache
        cache_layer.invalidate(CacheId::MaterialRegistry);

        // Rebuild cache
        let rebuilt_count = cache_layer.get_or_compute(CacheId::MaterialRegistry, |entry| {
            let material_entry = entry
                .as_any()
                .downcast_ref::<MaterialRegistryCache>()
                .expect("material registry cache entry");
            Some(material_entry.get_all_profiles().len())
        });

        // Verify equivalence
        assert_eq!(initial_count, rebuilt_count);
    }
}
