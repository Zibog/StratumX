//! Material cache types

use super::traits::{CacheEntry, RebuildableCache};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialId(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct MaterialCacheEntry {
    pub id: MaterialId,
    pub name: String,
    pub shader_type: String,
}

impl CacheEntry for MaterialCacheEntry {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct MaterialRegistryCache {
    data: HashMap<MaterialId, MaterialCacheEntry>,
    dirty: bool,
}

impl Default for MaterialRegistryCache {
    fn default() -> Self {
        Self::new()
    }
}

impl MaterialRegistryCache {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            dirty: true,
        }
    }
    pub fn test_insert(&mut self, key: MaterialId, value: MaterialCacheEntry) {
        self.data.insert(key, value);
    }
    pub fn test_mark_clean(&mut self) {
        self.dirty = false;
    }
    pub fn test_clear(&mut self) {
        self.data.clear();
        self.dirty = true;
    }
    pub fn get(&self, key: &MaterialId) -> Option<&MaterialCacheEntry> {
        self.data.get(key)
    }
    pub fn invalidate_key(&mut self, key: &MaterialId) {
        self.data.remove(key);
        self.dirty = true;
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn entry_count(&self) -> usize {
        self.data.len()
    }
    pub fn get_all_profiles(&self) -> Vec<&MaterialCacheEntry> {
        self.data.values().collect()
    }
    pub fn rebuild_if_needed(&mut self) {
        self.dirty = false;
    }
}

impl RebuildableCache<MaterialId> for MaterialRegistryCache {
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

impl CacheEntry for MaterialRegistryCache {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_valid(&self) -> bool {
        !self.dirty
    }

    fn invalidate(&mut self) {
        self.dirty = true;
    }

    fn rebuild(&mut self, _state_system: &crate::StateContainerSystem) {
        self.dirty = false;
    }

    fn dependencies(&self) -> Vec<crate::StateId> {
        vec![crate::StateId::ProjectState]
    }
}
