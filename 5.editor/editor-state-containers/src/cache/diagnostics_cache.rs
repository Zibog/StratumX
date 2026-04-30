//! Diagnostics cache types

use super::traits::RebuildableCache;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiagnosticsSummaryKey;

#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosticsSummary {
    pub error_count: u32,
    pub warning_count: u32,
    pub info_count: u32,
    pub total_count: u32,
}

pub struct DiagnosticsSummaryCache {
    data: Option<DiagnosticsSummary>,
    dirty: bool,
}

impl Default for DiagnosticsSummaryCache {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagnosticsSummaryCache {
    pub fn new() -> Self {
        Self {
            data: None,
            dirty: false,
        }
    }
    pub fn test_set_summary(&mut self, s: DiagnosticsSummary) {
        self.data = Some(s);
    }
    pub fn test_mark_clean(&mut self) {
        self.dirty = false;
    }
    pub fn test_clear(&mut self) {
        self.data = None;
        self.dirty = true;
    }
    pub fn has_summary(&self) -> bool {
        self.data.is_some()
    }
    pub fn get(&self, _key: &DiagnosticsSummaryKey) -> Option<&DiagnosticsSummary> {
        self.data.as_ref()
    }
    pub fn invalidate_key(&mut self, _key: &DiagnosticsSummaryKey) {
        self.data = None;
        self.dirty = true;
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn entry_count(&self) -> usize {
        if self.data.is_some() {
            1
        } else {
            0
        }
    }
    pub fn rebuild_if_needed(&mut self) {
        self.dirty = false;
    }
}

impl RebuildableCache<DiagnosticsSummaryKey> for DiagnosticsSummaryCache {
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
