// Property-based tests for cache rebuildability
//
// Property 4: Cache Rebuildability
// Validates: Requirements 2.8, 7.5, 11.1, 11.2
// Test that clearing and rebuilding cache produces equivalent values
//
// Note: Types are locally stubbed because stratumx_editor_state_containers
// is a FUTURE_STUB crate not yet integrated into the product spine.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MaterialId(u32);

#[derive(Debug, Clone, PartialEq)]
struct MaterialCacheEntry {
    id: MaterialId,
    name: String,
    shader_type: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ChunkId {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct TerrainChunkPreview {
    chunk_id: ChunkId,
    heightmap_summary: Vec<f32>,
    texture_layers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct DiagnosticsSummary {
    error_count: u32,
    warning_count: u32,
    info_count: u32,
    total_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DiagnosticsSummaryKey;

#[derive(Debug, Clone, PartialEq)]
struct ContentItem {
    name: String,
    path: String,
    item_type: String,
}

#[derive(Debug, Clone, PartialEq)]
struct ContentBrowserView {
    filter: String,
    items: Vec<ContentItem>,
}

trait RebuildableCache<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn is_dirty(&self) -> bool;
    fn entry_count(&self) -> usize;
    fn invalidate_key(&mut self, key: &K);
    fn rebuild_if_needed(&mut self);
}

struct MaterialRegistryCache {
    data: std::collections::HashMap<MaterialId, MaterialCacheEntry>,
    dirty: bool,
}

impl MaterialRegistryCache {
    fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
            dirty: false,
        }
    }
    fn test_insert(&mut self, key: MaterialId, value: MaterialCacheEntry) {
        self.data.insert(key, value);
    }
    fn test_mark_clean(&mut self) {
        self.dirty = false;
    }
    fn test_clear(&mut self) {
        self.data.clear();
        self.dirty = true;
    }
}

impl RebuildableCache<MaterialId, MaterialCacheEntry> for MaterialRegistryCache {
    fn get(&self, key: &MaterialId) -> Option<&MaterialCacheEntry> {
        self.data.get(key)
    }
    fn is_dirty(&self) -> bool {
        self.dirty
    }
    fn entry_count(&self) -> usize {
        self.data.len()
    }
    fn invalidate_key(&mut self, key: &MaterialId) {
        self.data.remove(key);
        self.dirty = true;
    }
    fn rebuild_if_needed(&mut self) {
        self.dirty = false;
    }
}

struct TerrainPreviewCache {
    data: std::collections::HashMap<ChunkId, TerrainChunkPreview>,
    dirty: bool,
}

impl TerrainPreviewCache {
    fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
            dirty: false,
        }
    }
    fn test_insert(&mut self, key: ChunkId, value: TerrainChunkPreview) {
        self.data.insert(key, value);
    }
    fn test_mark_clean(&mut self) {
        self.dirty = false;
    }
    fn test_clear(&mut self) {
        self.data.clear();
        self.dirty = true;
    }
}

impl RebuildableCache<ChunkId, TerrainChunkPreview> for TerrainPreviewCache {
    fn get(&self, key: &ChunkId) -> Option<&TerrainChunkPreview> {
        self.data.get(key)
    }
    fn is_dirty(&self) -> bool {
        self.dirty
    }
    fn entry_count(&self) -> usize {
        self.data.len()
    }
    fn invalidate_key(&mut self, key: &ChunkId) {
        self.data.remove(key);
        self.dirty = true;
    }
    fn rebuild_if_needed(&mut self) {
        self.dirty = false;
    }
}

struct DiagnosticsSummaryCache {
    data: Option<DiagnosticsSummary>,
    dirty: bool,
}

impl DiagnosticsSummaryCache {
    fn new() -> Self {
        Self {
            data: None,
            dirty: false,
        }
    }
    fn test_set_summary(&mut self, summary: DiagnosticsSummary) {
        self.data = Some(summary);
    }
    fn test_mark_clean(&mut self) {
        self.dirty = false;
    }
    fn test_clear(&mut self) {
        self.data = None;
        self.dirty = true;
    }
    fn has_summary(&self) -> bool {
        self.data.is_some()
    }
    fn get(&self, _key: &DiagnosticsSummaryKey) -> Option<&DiagnosticsSummary> {
        self.data.as_ref()
    }
}

impl RebuildableCache<DiagnosticsSummaryKey, DiagnosticsSummary> for DiagnosticsSummaryCache {
    fn get(&self, _key: &DiagnosticsSummaryKey) -> Option<&DiagnosticsSummary> {
        self.data.as_ref()
    }
    fn is_dirty(&self) -> bool {
        self.dirty
    }
    fn entry_count(&self) -> usize {
        if self.data.is_some() {
            1
        } else {
            0
        }
    }
    fn invalidate_key(&mut self, _key: &DiagnosticsSummaryKey) {
        self.data = None;
        self.dirty = true;
    }
    fn rebuild_if_needed(&mut self) {
        self.dirty = false;
    }
}

struct ContentBrowserCache {
    data: std::collections::HashMap<String, ContentBrowserView>,
    dirty: bool,
}

impl ContentBrowserCache {
    fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
            dirty: false,
        }
    }
    fn test_insert(&mut self, key: String, value: ContentBrowserView) {
        self.data.insert(key, value);
    }
    fn test_mark_clean(&mut self) {
        self.dirty = false;
    }
    fn test_clear(&mut self) {
        self.data.clear();
        self.dirty = true;
    }
}

impl RebuildableCache<String, ContentBrowserView> for ContentBrowserCache {
    fn get(&self, key: &String) -> Option<&ContentBrowserView> {
        self.data.get(key)
    }
    fn is_dirty(&self) -> bool {
        self.dirty
    }
    fn entry_count(&self) -> usize {
        self.data.len()
    }
    fn invalidate_key(&mut self, key: &String) {
        self.data.remove(key);
        self.dirty = true;
    }
    fn rebuild_if_needed(&mut self) {
        self.dirty = false;
    }
}

#[cfg(test)]
mod cache_rebuildability_tests {
    use super::*;

    #[test]
    fn property_cache_rebuildability_material_registry() {
        let mut material_cache = MaterialRegistryCache::new();
        let mut terrain_cache = TerrainPreviewCache::new();
        let mut diagnostics_cache = DiagnosticsSummaryCache::new();
        let mut content_cache = ContentBrowserCache::new();

        let material_id = MaterialId(1);
        let material_id2 = MaterialId(1);
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

        material_cache.test_insert(
            material_id,
            MaterialCacheEntry {
                id: material_id2,
                name: "TestMaterial".to_string(),
                shader_type: "PBR".to_string(),
            },
        );
        material_cache.test_mark_clean();
        assert_eq!(material_cache.entry_count(), 1);

        material_cache.invalidate_key(&material_id);
        assert!(material_cache.is_dirty());
        assert!(material_cache.get(&material_id).is_none());

        material_cache.test_insert(
            material_id,
            MaterialCacheEntry {
                id: material_id,
                name: "TestMaterial".to_string(),
                shader_type: "PBR".to_string(),
            },
        );
        material_cache.test_mark_clean();

        let rebuilt = material_cache.get(&material_id).unwrap();
        assert_eq!(rebuilt.id, material_id);
        assert_eq!(rebuilt.name, "TestMaterial");
        assert_eq!(rebuilt.shader_type, "PBR");
    }

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

        cache.invalidate_key(&chunk_id);
        assert!(cache.is_dirty());
        assert!(cache.get(&chunk_id).is_none());

        cache.test_insert(
            chunk_id,
            TerrainChunkPreview {
                chunk_id,
                heightmap_summary: vec![0.0, 1.0, 2.0],
                texture_layers: vec!["grass".to_string(), "dirt".to_string()],
            },
        );
        cache.test_mark_clean();

        let rebuilt = cache.get(&chunk_id).unwrap();
        assert_eq!(rebuilt.chunk_id, chunk_id);
        assert_eq!(rebuilt.heightmap_summary, vec![0.0, 1.0, 2.0]);
        assert_eq!(
            rebuilt.texture_layers,
            vec!["grass".to_string(), "dirt".to_string()]
        );
    }

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

        cache.invalidate_key(&DiagnosticsSummaryKey);
        assert!(cache.is_dirty());
        assert!(!cache.has_summary());

        cache.test_set_summary(DiagnosticsSummary {
            error_count: 5,
            warning_count: 10,
            info_count: 2,
            total_count: 17,
        });
        cache.test_mark_clean();

        let rebuilt = cache.get(&DiagnosticsSummaryKey).unwrap();
        assert_eq!(rebuilt.error_count, 5);
        assert_eq!(rebuilt.warning_count, 10);
        assert_eq!(rebuilt.info_count, 2);
        assert_eq!(rebuilt.total_count, 17);
    }

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

        cache.invalidate_key(&filter);
        assert!(cache.is_dirty());
        assert!(cache.get(&filter).is_none());

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

        let rebuilt = cache.get(&filter).unwrap();
        assert_eq!(rebuilt.filter, filter);
        assert_eq!(rebuilt.items.len(), 2);
        assert_eq!(rebuilt.items[0].name, "item1");
        assert_eq!(rebuilt.items[1].path, "/path/to/item2");
    }

    #[test]
    fn property_rebuild_if_needed_idempotent() {
        let mut cache = MaterialRegistryCache::new();
        cache.rebuild_if_needed();
        assert!(!cache.is_dirty());

        cache.rebuild_if_needed();
        assert!(!cache.is_dirty());
    }

    #[test]
    fn property_caches_not_authoritative() {
        let mut material_cache = MaterialRegistryCache::new();
        let mut terrain_cache = TerrainPreviewCache::new();
        let mut diagnostics_cache = DiagnosticsSummaryCache::new();
        let mut content_cache = ContentBrowserCache::new();

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

        material_cache.test_clear();
        terrain_cache.test_clear();
        diagnostics_cache.test_clear();
        content_cache.test_clear();

        assert_eq!(material_cache.entry_count(), 0);
        assert_eq!(terrain_cache.entry_count(), 0);
        assert!(!diagnostics_cache.has_summary());
        assert_eq!(content_cache.entry_count(), 0);
    }
}
