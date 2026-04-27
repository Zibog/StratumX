//! Diagnostics summary cache — rebuildable from owner state.

use super::cache_keys::{DiagnosticsSummary, DiagnosticsSummaryKey, RebuildableCache};
use crate::{CacheEntry, StateContainerSystem, StateId};

pub struct DiagnosticsOwnerData;

#[derive(Debug, Clone)]
pub struct DiagnosticsSummaryCache {
    pub(crate) summary: Option<DiagnosticsSummary>,
    pub(crate) dirty: bool,
}

impl DiagnosticsSummaryCache {
    pub fn new() -> Self {
        Self {
            summary: None,
            dirty: true,
        }
    }

    pub fn rebuild_from_owner(&mut self, _owner_data: &DiagnosticsOwnerData) {
        self.summary = Some(DiagnosticsSummary {
            error_count: 0,
            warning_count: 0,
            info_count: 0,
            total_count: 0,
        });
        self.dirty = false;
    }

    pub fn test_set_summary(&mut self, summary: DiagnosticsSummary) {
        self.summary = Some(summary);
    }
    pub fn test_mark_clean(&mut self) {
        self.dirty = false;
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn has_summary(&self) -> bool {
        self.summary.is_some()
    }
    pub fn test_clear(&mut self) {
        self.summary = None;
    }
}

impl Default for DiagnosticsSummaryCache {
    fn default() -> Self {
        Self::new()
    }
}

impl RebuildableCache<DiagnosticsSummaryKey, DiagnosticsSummary> for DiagnosticsSummaryCache {
    fn invalidate_key(&mut self, _key: &DiagnosticsSummaryKey) {
        self.summary = None;
        self.dirty = true;
    }
    fn rebuild_if_needed(&mut self) {
        if self.dirty {
            self.dirty = false;
        }
    }
    fn get(&self, _key: &DiagnosticsSummaryKey) -> Option<&DiagnosticsSummary> {
        self.summary.as_ref()
    }
}

impl CacheEntry for DiagnosticsSummaryCache {
    fn is_valid(&self) -> bool {
        !self.dirty
    }
    fn invalidate(&mut self) {
        self.summary = None;
        self.dirty = true;
    }
    fn rebuild(&mut self, _state_system: &StateContainerSystem) {
        self.summary = Some(DiagnosticsSummary {
            error_count: 0,
            warning_count: 0,
            info_count: 0,
            total_count: 0,
        });
        self.dirty = false;
    }
    fn dependencies(&self) -> Vec<StateId> {
        vec![StateId::DiagnosticsState]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostics_cache_invalidate() {
        let mut cache = DiagnosticsSummaryCache::new();
        cache.summary = Some(DiagnosticsSummary {
            error_count: 5,
            warning_count: 10,
            info_count: 2,
            total_count: 17,
        });
        cache.dirty = false;
        cache.invalidate_key(&DiagnosticsSummaryKey);
        assert!(cache.dirty);
        assert!(cache.get(&DiagnosticsSummaryKey).is_none());
    }
}
