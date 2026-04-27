//! Workspace Persistence View
//!
//! Separate persistence type for WorkspaceOwner.

use crate::owners::workspace_owner::{DockingConfig, PanelGeometry, PanelId, WorkspaceOwner};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Workspace persistence view - contains only persistable state
///
/// This is the same as WorkspaceOwner since it has no runtime-only fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePersistenceView {
    /// Schema version for migration support
    pub schema_version: u32,

    /// List of open panel IDs
    pub open_panel_ids: Vec<PanelId>,

    /// Panel positions and sizes
    pub panel_positions: HashMap<PanelId, PanelGeometry>,

    /// Currently focused panel
    pub focused_panel: Option<PanelId>,

    /// Docking configuration
    pub docking_configuration: DockingConfig,
}

impl WorkspacePersistenceView {
    /// Creates a new workspace persistence view
    pub fn new(
        schema_version: u32,
        open_panel_ids: Vec<PanelId>,
        panel_positions: HashMap<PanelId, PanelGeometry>,
        focused_panel: Option<PanelId>,
        docking_configuration: DockingConfig,
    ) -> Self {
        Self {
            schema_version,
            open_panel_ids,
            panel_positions,
            focused_panel,
            docking_configuration,
        }
    }
}

/// Convert from WorkspaceOwner reference to WorkspacePersistenceView
///
/// Note: WorkspaceOwner has no runtime-only fields, so this is a direct conversion.
impl From<&WorkspaceOwner> for WorkspacePersistenceView {
    fn from(owner: &WorkspaceOwner) -> Self {
        Self {
            schema_version: owner.schema_version,
            open_panel_ids: owner.open_panel_ids.clone(),
            panel_positions: owner.panel_positions.clone(),
            focused_panel: owner.focused_panel.clone(),
            docking_configuration: owner.docking_configuration.clone(),
        }
    }
}

/// Convert from WorkspacePersistenceView to WorkspaceOwner
impl TryFrom<WorkspacePersistenceView> for WorkspaceOwner {
    type Error = String;

    fn try_from(view: WorkspacePersistenceView) -> Result<Self, Self::Error> {
        Ok(Self {
            schema_version: view.schema_version,
            open_panel_ids: view.open_panel_ids,
            panel_positions: view.panel_positions,
            focused_panel: view.focused_panel,
            docking_configuration: view.docking_configuration,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persistence_view_creation() {
        let view = WorkspacePersistenceView::new(
            1,
            Vec::new(),
            HashMap::new(),
            None,
            DockingConfig::default(),
        );

        assert_eq!(view.schema_version, 1);
        assert!(view.open_panel_ids.is_empty());
    }

    #[test]
    fn test_persistence_view_serialization() {
        let mut panel_positions = HashMap::new();
        panel_positions.insert(
            PanelId::new("viewport"),
            PanelGeometry::floating(0.0, 0.0, 800.0, 600.0),
        );

        let view = WorkspacePersistenceView::new(
            1,
            vec![PanelId::new("viewport")],
            panel_positions,
            Some(PanelId::new("viewport")),
            DockingConfig::default(),
        );

        // Test serialization round-trip
        let json = serde_json::to_string(&view).unwrap();
        let deserialized: WorkspacePersistenceView = serde_json::from_str(&json).unwrap();

        assert_eq!(view, deserialized);
    }

    #[test]
    fn test_from_workspace_owner() {
        let mut owner = WorkspaceOwner::new();
        owner.add_panel(
            PanelId::new("viewport"),
            PanelGeometry::floating(0.0, 0.0, 800.0, 600.0),
        );

        let view = WorkspacePersistenceView::from(&owner);

        assert_eq!(view.schema_version, WorkspaceOwner::CURRENT_SCHEMA_VERSION);
        assert_eq!(view.open_panel_ids.len(), 1);
    }

    #[test]
    fn test_try_from_persistence_view() {
        let mut panel_positions = HashMap::new();
        panel_positions.insert(
            PanelId::new("viewport"),
            PanelGeometry::floating(0.0, 0.0, 800.0, 600.0),
        );

        let view = WorkspacePersistenceView::new(
            1,
            vec![PanelId::new("viewport")],
            panel_positions,
            Some(PanelId::new("viewport")),
            DockingConfig::default(),
        );

        let owner = WorkspaceOwner::try_from(view).unwrap();

        assert_eq!(owner.schema_version, 1);
        assert_eq!(owner.open_panel_ids.len(), 1);
        assert_eq!(owner.focused_panel, Some(PanelId::new("viewport")));
    }

    #[test]
    fn test_round_trip_conversion() {
        let mut original = WorkspaceOwner::new();
        original.add_panel(
            PanelId::new("viewport"),
            PanelGeometry::floating(0.0, 0.0, 800.0, 600.0),
        );
        original.set_focused_panel(Some(PanelId::new("viewport")));

        // Convert to persistence view and back
        let view = WorkspacePersistenceView::from(&original);
        let restored = WorkspaceOwner::try_from(view).unwrap();

        // All state should match (no runtime-only fields)
        assert_eq!(original.schema_version, restored.schema_version);
        assert_eq!(original.open_panel_ids, restored.open_panel_ids);
        assert_eq!(original.panel_positions, restored.panel_positions);
        assert_eq!(original.focused_panel, restored.focused_panel);
        assert_eq!(
            original.docking_configuration,
            restored.docking_configuration
        );
    }
}
