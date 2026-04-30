//! Content browser cache types

use super::traits::RebuildableCache;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ContentItem {
    pub name: String,
    pub path: String,
    pub item_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContentBrowserView {
    pub filter: String,
    pub items: Vec<ContentItem>,
}

pub struct ContentBrowserCache {
    data: HashMap<String, ContentBrowserView>,
    dirty: bool,
}

impl Default for ContentBrowserCache {
    fn default() -> Self {
        Self::new()
    }
}

impl ContentBrowserCache {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            dirty: false,
        }
    }
    pub fn test_insert(&mut self, key: String, value: ContentBrowserView) {
        self.data.insert(key, value);
    }
    pub fn test_mark_clean(&mut self) {
        self.dirty = false;
    }
    pub fn test_clear(&mut self) {
        self.data.clear();
        self.dirty = true;
    }
    pub fn get(&self, key: &String) -> Option<&ContentBrowserView> {
        self.data.get(key)
    }
    pub fn invalidate_key(&mut self, key: &String) {
        self.data.remove(key);
        self.dirty = true;
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn entry_count(&self) -> usize {
        self.data.len()
    }
    pub fn rebuild_if_needed(&mut self) {
        self.dirty = false;
    }
}

impl RebuildableCache<String> for ContentBrowserCache {
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
