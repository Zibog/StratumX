//! Material registry cache — rebuildable from owner state.

use super::cache_keys::{MaterialCacheEntry, MaterialId, RebuildableCache};
use crate::{CacheEntry, StateContainerSystem, StateId};
use std::collections::HashMap;

pub struct MaterialOwnerData;

#[derive(Debug, Clone)]
pub struct MaterialRegistryCache {
    pub(crate) materials: HashMap<MaterialId, MaterialCacheEntry>,
    pub(crate) dirty: bool,
}

impl MaterialRegistryCache {
    pub fn new() -> Self {
        Self {
            materials: HashMap::new(),
            dirty: true,
        }
    }

    pub fn rebuild_from_owner(&mut self, _owner_data: &MaterialOwnerData) {
        self.dirty = false;
    }

    pub fn test_insert(&mut self, id: MaterialId, entry: MaterialCacheEntry) {
        self.materials.insert(id, entry);
    }

    pub fn test_mark_clean(&mut self) {
        self.dirty = false;
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn entry_count(&self) -> usize {
        self.materials.len()
    }
    pub fn test_clear(&mut self) {
        self.materials.clear();
    }
    pub fn get_all_profiles(&self) -> Vec<&MaterialCacheEntry> {
        self.materials.values().collect()
    }
}

impl Default for MaterialRegistryCache {
    fn default() -> Self {
        Self::new()
    }
}

impl RebuildableCache<MaterialId, MaterialCacheEntry> for MaterialRegistryCache {
    fn invalidate_key(&mut self, key: &MaterialId) {
        self.materials.remove(key);
        self.dirty = true;
    }
    fn rebuild_if_needed(&mut self) {
        if self.dirty {
            self.dirty = false;
        }
    }
    fn get(&self, key: &MaterialId) -> Option<&MaterialCacheEntry> {
        self.materials.get(key)
    }
}

impl CacheEntry for MaterialRegistryCache {
    fn is_valid(&self) -> bool {
        !self.dirty
    }
    fn invalidate(&mut self) {
        self.materials.clear();
        self.dirty = true;
    }
    fn rebuild(&mut self, _state_system: &StateContainerSystem) {
        self.dirty = false;
    }
    fn dependencies(&self) -> Vec<StateId> {
        vec![StateId::WorldState]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_cache_invalidate() {
        let mut cache = MaterialRegistryCache::new();
        let id = MaterialId(1);
        cache.materials.insert(
            id,
            MaterialCacheEntry {
                id,
                name: "Test".to_string(),
                shader_type: "PBR".to_string(),
            },
        );
        cache.dirty = false;
        cache.invalidate_key(&id);
        assert!(cache.dirty);
        assert!(cache.get(&id).is_none());
    }

    #[test]
    fn test_rebuild_if_needed() {
        let mut cache = MaterialRegistryCache::new();
        assert!(cache.dirty);
        cache.rebuild_if_needed();
        assert!(!cache.dirty);
    }
}
