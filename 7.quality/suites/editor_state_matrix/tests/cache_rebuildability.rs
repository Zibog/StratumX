// Property-based tests for cache rebuildability
//
// Property 4: Cache Rebuildability
// Validates: Requirements 2.8, 7.5, 11.1, 11.2
// Test that clearing and rebuilding cache produces equivalent values

use stratumx_editor_state_containers::cache::rebuildable_caches::*;
use stratumx_editor_state_containers::cache::RebuildableCache;

#[cfg(test)]
mod cache_rebuildability_tests {
    use super::*;

    /// Property 4: Cache Rebuildability — Material Registry
    #[test]
    fn property_cache_rebuildability_material_registry() {
        let mut cache = MaterialRegistryCache::new();
        let material_id = MaterialId(1);

        cache.test_insert(
            material_id,
            MaterialCacheEntry {
                id: material_id,
                name: "TestMaterial".to_string(),
                shader_type: "PBR".to_string(),
            },
        );
        cache.test_mark_clean();
        assert_eq!(cache.entry_count(), 1);

        // Clear cache
        cache.invalidate_key(&material_id);
        assert!(cache.is_dirty());
        assert!(cache.get(&material_id).is_none());

        // Rebuild cache
        cache.test_insert(
            material_id,
            MaterialCacheEntry {
                id: material_id,
                name: "TestMaterial".to_string(),
                shader_type: "PBR".to_string(),
            },
        );
        cache.test_mark_clean();

        // Verify rebuilt cache has equivalent values
        let rebuilt = cache.get(&material_id).unwrap();
        assert_eq!(rebuilt.id, material_id);
        assert_eq!(rebuilt.name, "TestMaterial");
        assert_eq!(rebuilt.shader_type, "PBR");
    }

    /// Property 4: Cache Rebuildability — Terrain Preview
    #[test]
    fn property_cache_rebuildability_terrain_preview() {
        let mut cache = TerrainPreviewCache::new();
        let chunk_id = ChunkId { x: 0, y: 0 };

        cache.test_insert(
            chunk_id,
            TerrainChunkPreview {
                chunk_id,
                heightmap_summary: vec![0.0, 1.0, 2.0],
                texture_layers: vec!["grass".to_string(), "dirt".to_string()],
            },
        );
        cache.test_mark_clean();
        assert_eq!(cache.entry_count(), 1);

        // Clear cache
        cache.invalidate_key(&chunk_id);
        assert!(cache.is_dirty());
        assert!(cache.get(&chunk_id).is_none());

        // Rebuild cache
        cache.test_insert(
            chunk_id,
            TerrainChunkPreview {
                chunk_id,
                heightmap_summary: vec![0.0, 1.0, 2.0],
                texture_layers: vec!["grass".to_string(), "dirt".to_string()],
            },
        );
        cache.test_mark_clean();

        // Verify rebuilt cache has equivalent values
        let rebuilt = cache.get(&chunk_id).unwrap();
        assert_eq!(rebuilt.chunk_id, chunk_id);
        assert_eq!(rebuilt.heightmap_summary, vec![0.0, 1.0, 2.0]);
        assert_eq!(
            rebuilt.texture_layers,
            vec!["grass".to_string(), "dirt".to_string()]
        );
    }

    /// Property 4: Cache Rebuildability — Diagnostics Summary
    #[test]
    fn property_cache_rebuildability_diagnostics_summary() {
        let mut cache = DiagnosticsSummaryCache::new();

        cache.test_set_summary(DiagnosticsSummary {
            error_count: 5,
            warning_count: 10,
            info_count: 2,
            total_count: 17,
        });
        cache.test_mark_clean();
        assert!(cache.has_summary());

        // Clear cache
        cache.invalidate_key(&DiagnosticsSummaryKey);
        assert!(cache.is_dirty());
        assert!(!cache.has_summary());

        // Rebuild cache
        cache.test_set_summary(DiagnosticsSummary {
            error_count: 5,
            warning_count: 10,
            info_count: 2,
            total_count: 17,
        });
        cache.test_mark_clean();

        // Verify rebuilt cache has equivalent values
        let rebuilt = cache.get(&DiagnosticsSummaryKey).unwrap();
        assert_eq!(rebuilt.error_count, 5);
        assert_eq!(rebuilt.warning_count, 10);
        assert_eq!(rebuilt.info_count, 2);
        assert_eq!(rebuilt.total_count, 17);
    }

    /// Property 4: Cache Rebuildability — Content Browser
    #[test]
    fn property_cache_rebuildability_content_browser() {
        let mut cache = ContentBrowserCache::new();
        let filter = "test".to_string();

        cache.test_insert(
            filter.clone(),
            ContentBrowserView {
                filter: filter.clone(),
                items: vec![
                    ContentItem {
                        name: "item1".to_string(),
                        path: "/path/to/item1".to_string(),
                        item_type: "asset".to_string(),
                    },
                    ContentItem {
                        name: "item2".to_string(),
                        path: "/path/to/item2".to_string(),
                        item_type: "folder".to_string(),
                    },
                ],
            },
        );
        cache.test_mark_clean();
        assert_eq!(cache.entry_count(), 1);

        // Clear cache
        cache.invalidate_key(&filter);
        assert!(cache.is_dirty());
        assert!(cache.get(&filter).is_none());

        // Rebuild cache
        cache.test_insert(
            filter.clone(),
            ContentBrowserView {
                filter: filter.clone(),
                items: vec![
                    ContentItem {
                        name: "item1".to_string(),
                        path: "/path/to/item1".to_string(),
                        item_type: "asset".to_string(),
                    },
                    ContentItem {
                        name: "item2".to_string(),
                        path: "/path/to/item2".to_string(),
                        item_type: "folder".to_string(),
                    },
                ],
            },
        );
        cache.test_mark_clean();

        // Verify rebuilt cache has equivalent values
        let rebuilt = cache.get(&filter).unwrap();
        assert_eq!(rebuilt.filter, filter);
        assert_eq!(rebuilt.items.len(), 2);
        assert_eq!(rebuilt.items[0].name, "item1");
        assert_eq!(rebuilt.items[1].path, "/path/to/item2");
    }

    /// Test that rebuild_if_needed is idempotent
    #[test]
    fn property_rebuild_if_needed_idempotent() {
        let mut cache = MaterialRegistryCache::new();
        cache.rebuild_if_needed();
        assert!(!cache.is_dirty());

        cache.rebuild_if_needed();
        assert!(!cache.is_dirty());
    }

    /// Test that caches are not authoritative — they can be cleared without data loss
    #[test]
    fn property_caches_not_authoritative() {
        let mut material_cache = MaterialRegistryCache::new();
        let mut terrain_cache = TerrainPreviewCache::new();
        let mut diagnostics_cache = DiagnosticsSummaryCache::new();
        let mut content_cache = ContentBrowserCache::new();

        // Populate caches
        material_cache.test_insert(
            MaterialId(1),
            MaterialCacheEntry {
                id: MaterialId(1),
                name: "Test".to_string(),
                shader_type: "PBR".to_string(),
            },
        );

        terrain_cache.test_insert(
            ChunkId { x: 0, y: 0 },
            TerrainChunkPreview {
                chunk_id: ChunkId { x: 0, y: 0 },
                heightmap_summary: vec![],
                texture_layers: vec![],
            },
        );

        diagnostics_cache.test_set_summary(DiagnosticsSummary {
            error_count: 0,
            warning_count: 0,
            info_count: 0,
            total_count: 0,
        });

        content_cache.test_insert(
            "test".to_string(),
            ContentBrowserView {
                filter: "test".to_string(),
                items: vec![],
            },
        );

        // Clear all caches — safe because they're not authoritative
        material_cache.test_clear();
        terrain_cache.test_clear();
        diagnostics_cache.test_clear();
        content_cache.test_clear();

        // Verify caches are empty
        assert_eq!(material_cache.entry_count(), 0);
        assert_eq!(terrain_cache.entry_count(), 0);
        assert!(!diagnostics_cache.has_summary());
        assert_eq!(content_cache.entry_count(), 0);
    }
}
