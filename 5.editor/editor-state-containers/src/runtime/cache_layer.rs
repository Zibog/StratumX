//! Cache layer

use crate::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CacheId {
    MaterialRegistry,
    TerrainPreview,
    DiagnosticsSummary,
    ContentBrowser,
    ViewportStatistics,
    Custom(String),
}

impl CacheId {
    pub fn new(id: String) -> Self {
        Self::Custom(id)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CacheMetrics {
    pub total_caches: usize,
    pub valid_caches: usize,
    pub invalid_caches: usize,
    pub rebuild_count: usize,
    pub hit_count: usize,
    pub miss_count: usize,
    pub invalidation_count: usize,
}

struct CacheStore {
    caches: HashMap<CacheId, Box<dyn cache::CacheEntry + Send>>,
    metrics: CacheMetrics,
}

pub struct CacheLayer {
    state_system: Arc<StateContainerSystem>,
    store: Mutex<CacheStore>,
}

impl Default for CacheLayer {
    fn default() -> Self {
        Self::new(Arc::new(StateContainerSystem::default()))
    }
}

impl CacheLayer {
    pub fn new(state_system: Arc<StateContainerSystem>) -> Self {
        Self {
            state_system,
            store: Mutex::new(CacheStore {
                caches: HashMap::new(),
                metrics: CacheMetrics {
                    total_caches: 0,
                    valid_caches: 0,
                    invalid_caches: 0,
                    rebuild_count: 0,
                    hit_count: 0,
                    miss_count: 0,
                    invalidation_count: 0,
                },
            }),
        }
    }

    pub fn register_cache<T: cache::CacheEntry + Send + 'static>(
        &mut self,
        cache_id: CacheId,
        cache: T,
    ) {
        let mut store = self.store.lock().unwrap();
        store.caches.insert(cache_id, Box::new(cache));
        Self::refresh_counts(&mut store);
    }

    pub fn invalidate_dependent_on(&mut self, state_id: StateId) {
        let mut store = self.store.lock().unwrap();
        let mut invalidated = 0usize;

        for cache in store.caches.values_mut() {
            if cache.dependencies().contains(&state_id) {
                cache.invalidate();
                invalidated += 1;
            }
        }

        store.metrics.invalidation_count += invalidated;
        Self::refresh_counts(&mut store);
    }

    pub fn is_cache_valid(&self, cache_id: CacheId) -> Option<bool> {
        let store = self.store.lock().unwrap();
        store.caches.get(&cache_id).map(|cache| cache.is_valid())
    }

    pub fn get_metrics(&self) -> CacheMetrics {
        self.store.lock().unwrap().metrics.clone()
    }

    pub fn get_or_compute<T, F>(&self, cache_id: CacheId, compute_fn: F) -> Option<T>
    where
        F: FnOnce(&dyn cache::CacheEntry) -> Option<T>,
    {
        let mut store = self.store.lock().unwrap();
        let (was_valid, is_valid_after_rebuild) = {
            let cache = store.caches.get_mut(&cache_id)?;
            let was_valid = cache.is_valid();

            if !was_valid {
                cache.rebuild(&self.state_system);
            }

            (was_valid, cache.is_valid())
        };

        if was_valid {
            store.metrics.hit_count += 1;
        } else {
            store.metrics.miss_count += 1;
            store.metrics.rebuild_count += 1;
        }

        Self::refresh_counts(&mut store);

        if !is_valid_after_rebuild {
            return None;
        }

        let cache = store.caches.get(&cache_id)?;
        compute_fn(cache.as_ref())
    }

    pub fn invalidate(&mut self, cache_id: CacheId) {
        let mut store = self.store.lock().unwrap();
        let mut invalidated = false;

        if let Some(cache) = store.caches.get_mut(&cache_id) {
            cache.invalidate();
            invalidated = true;
        }

        if invalidated {
            store.metrics.invalidation_count += 1;
            Self::refresh_counts(&mut store);
        }
    }

    pub fn clear_all(&mut self) {
        let mut store = self.store.lock().unwrap();
        let mut invalidated = 0usize;

        for cache in store.caches.values_mut() {
            cache.invalidate();
            invalidated += 1;
        }

        store.metrics.invalidation_count += invalidated;
        Self::refresh_counts(&mut store);
    }

    pub fn rebuild_all(&mut self) {
        let mut store = self.store.lock().unwrap();
        let mut rebuilt = 0usize;

        for cache in store.caches.values_mut() {
            cache.rebuild(&self.state_system);
            rebuilt += 1;
        }

        store.metrics.rebuild_count += rebuilt;
        Self::refresh_counts(&mut store);
    }

    pub fn reset_metrics(&mut self) {
        let mut store = self.store.lock().unwrap();
        store.metrics.rebuild_count = 0;
        store.metrics.hit_count = 0;
        store.metrics.miss_count = 0;
        store.metrics.invalidation_count = 0;
        Self::refresh_counts(&mut store);
    }

    fn refresh_counts(store: &mut CacheStore) {
        store.metrics.total_caches = store.caches.len();
        store.metrics.valid_caches = store.caches.values().filter(|cache| cache.is_valid()).count();
        store.metrics.invalid_caches = store.metrics.total_caches - store.metrics.valid_caches;
    }
}
