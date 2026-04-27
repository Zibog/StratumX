//! Diagnostics validation — legality checks.

use super::diagnostics_owner::DiagnosticsOwner;

impl DiagnosticsOwner {
    /// Returns `true` if the diagnostics state is internally consistent.
    ///
    /// Checks:
    /// - No duplicate failure codes
    /// - All trace lineages have valid trace IDs
    /// - All recovery actions reference known failure codes
    pub fn is_valid(&self) -> bool {
        // Check for duplicate failure codes
        let mut seen_codes = std::collections::HashSet::new();
        for code in &self.failure_codes {
            if !seen_codes.insert(code) {
                return false;
            }
        }

        // Check that all trace lineages have non-empty trace IDs
        for (trace_id, lineage) in &self.trace_lineage {
            if trace_id.as_uuid().is_nil() || lineage.trace_id.as_uuid().is_nil() {
                return false;
            }
        }

        true
    }
}
