//! Workspace Validation
//!
//! Validation checks for WorkspaceOwner — schema validation,
//! panel consistency checks.

use super::workspace_owner::WorkspaceOwner;

impl WorkspaceOwner {
    /// Validates the workspace state
    ///
    /// Returns true if all validation checks pass.
    pub fn is_valid(&self) -> bool {
        self.validate_schema_version() && self.validate_panel_combos()
    }

    /// Validates that the schema version is supported
    fn validate_schema_version(&self) -> bool {
        self.schema_version <= Self::CURRENT_SCHEMA_VERSION && self.schema_version > 0
    }

    /// Validates panel combinations
    ///
    /// Checks that:
    /// - All open panel IDs have corresponding geometry entries
    /// - No geometry entries exist without corresponding open panel IDs
    /// - Focused panel (if set) is in the open panels list
    pub fn validate_panel_combos(&self) -> bool {
        // Check all open panel IDs have geometry
        for panel_id in &self.open_panel_ids {
            if !self.panel_positions.contains_key(panel_id) {
                return false;
            }
        }

        // Check all geometry entries have open panel IDs
        for panel_id in self.panel_positions.keys() {
            if !self.open_panel_ids.contains(panel_id) {
                return false;
            }
        }

        // Check focused panel is in open panels list
        if let Some(ref focused) = self.focused_panel {
            if !self.open_panel_ids.contains(focused) {
                return false;
            }
        }

        true
    }
}
