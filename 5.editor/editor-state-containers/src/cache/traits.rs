//! Cache traits

pub trait RebuildableCache<K: std::hash::Hash + Eq> {
    fn is_dirty(&self) -> bool;
    fn entry_count(&self) -> usize;
    fn invalidate_key(&mut self, key: &K);
    fn rebuild_if_needed(&mut self);
}

pub trait CacheEntry: std::any::Any {
    fn as_any(&self) -> &dyn std::any::Any;

    fn is_valid(&self) -> bool {
        true
    }
    fn invalidate(&mut self) {}
    fn rebuild(&mut self, _state_system: &crate::StateContainerSystem) {}
    fn dependencies(&self) -> Vec<crate::StateId> {
        Vec::new()
    }
}

// Blanket implementation for Box<T> where T: CacheEntry
impl<T: CacheEntry + ?Sized> CacheEntry for Box<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        (**self).as_any()
    }

    fn is_valid(&self) -> bool {
        (**self).is_valid()
    }
    fn invalidate(&mut self) {
        (**self).invalidate()
    }
    fn rebuild(&mut self, state_system: &crate::StateContainerSystem) {
        (**self).rebuild(state_system)
    }
    fn dependencies(&self) -> Vec<crate::StateId> {
        (**self).dependencies()
    }
}
