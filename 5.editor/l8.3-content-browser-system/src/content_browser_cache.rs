//! Content browser cache — rebuildable from workspace/project owner state.

use super::cache_keys::{ContentBrowserView, RebuildableCache};
use crate::{CacheEntry, StateContainerSystem, StateId};
use std::collections::HashMap;

pub struct ContentOwnerData;

#[derive(Debug, Clone)]
pub struct ContentBrowserCache {
    pub(crate) filtered_views: HashMap<String, ContentBrowserView>,
    pub(crate) dirty: bool,
}

impl ContentBrowserCache {
    pub fn new() -> Self {
        Self {
            filtered_views: HashMap::new(),
            dirty: true,
        }
    }

    pub fn rebuild_from_owner(&mut self, _owner_data: &ContentOwnerData) {
        self.dirty = false;
    }

    pub fn test_insert(&mut self, filter: String, view: ContentBrowserView) {
        self.filtered_views.insert(filter, view);
    }

    pub fn test_mark_clean(&mut self) {
        self.dirty = false;
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn entry_count(&self) -> usize {
        self.filtered_views.len()
    }
    pub fn test_clear(&mut self) {
        self.filtered_views.clear();
    }
}

impl Default for ContentBrowserCache {
    fn default() -> Self {
        Self::new()
    }
}

impl RebuildableCache<String, ContentBrowserView> for ContentBrowserCache {
    fn invalidate_key(&mut self, key: &String) {
        self.filtered_views.remove(key);
        self.dirty = true;
    }
    fn rebuild_if_needed(&mut self) {
        if self.dirty {
            self.dirty = false;
        }
    }
    fn get(&self, key: &String) -> Option<&ContentBrowserView> {
        self.filtered_views.get(key)
    }
}

impl CacheEntry for ContentBrowserCache {
    fn is_valid(&self) -> bool {
        !self.dirty
    }
    fn invalidate(&mut self) {
        self.filtered_views.clear();
        self.dirty = true;
    }
    fn rebuild(&mut self, _state_system: &StateContainerSystem) {
        self.dirty = false;
    }
    fn dependencies(&self) -> Vec<StateId> {
        vec![StateId::ProjectState, StateId::WorkspaceState]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_browser_cache_invalidate() {
        let mut cache = ContentBrowserCache::new();
        let filter = "test".to_string();
        cache.filtered_views.insert(
            filter.clone(),
            ContentBrowserView {
                filter: filter.clone(),
                items: vec![],
            },
        );
        cache.dirty = false;
        cache.invalidate_key(&filter);
        assert!(cache.dirty);
        assert!(cache.get(&filter).is_none());
    }
}
