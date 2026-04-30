//! Panel query views

use crate::owners::workspace_owner::{
    DockPosition, DockingConfig, PanelGeometry, PanelId, WorkspaceOwner,
};
use crate::queries::ReadModel;
use std::collections::HashMap;

/// Workspace panel positions view
///
/// Read-only view of panel positions and geometries.
#[derive(Debug, Clone)]
pub struct WorkspacePanelPositionsView {
    pub panel_positions: HashMap<PanelId, PanelGeometry>,
    pub docking_configuration: DockingConfig,
}

impl ReadModel<WorkspaceOwner> for WorkspacePanelPositionsView {
    fn build(owner: &WorkspaceOwner) -> Self {
        Self {
            panel_positions: owner.panel_positions.clone(),
            docking_configuration: owner.docking_configuration.clone(),
        }
    }
}

/// Workspace docking summary view
///
/// Read-only summary of docked vs floating panels.
#[derive(Debug, Clone)]
pub struct WorkspaceDockingSummaryView {
    pub total_panels: usize,
    pub docked_panels: usize,
    pub floating_panels: usize,
    pub docking_enabled: bool,
}

impl ReadModel<WorkspaceOwner> for WorkspaceDockingSummaryView {
    fn build(owner: &WorkspaceOwner) -> Self {
        let total_panels = owner.panel_positions.len();
        let docked_panels = owner
            .panel_positions
            .values()
            .filter(|g| g.dock_position != DockPosition::Floating)
            .count();
        let floating_panels = total_panels - docked_panels;

        Self {
            total_panels,
            docked_panels,
            floating_panels,
            docking_enabled: owner.docking_configuration.docking_enabled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_panel_positions_view() {
        let mut owner = WorkspaceOwner::new();

        let panel_id = PanelId::new("viewport");
        let geometry = PanelGeometry::floating(100.0, 200.0, 800.0, 600.0);

        owner.add_panel(panel_id.clone(), geometry.clone());

        let view = WorkspacePanelPositionsView::build(&owner);

        assert_eq!(view.panel_positions.len(), 1);
        assert!(view.panel_positions.contains_key(&panel_id));
        assert!(view.docking_configuration.docking_enabled);
    }

    #[test]
    fn test_workspace_docking_summary_view() {
        let mut owner = WorkspaceOwner::new();

        // Add floating panel
        owner.add_panel(
            PanelId::new("viewport"),
            PanelGeometry::floating(0.0, 0.0, 800.0, 600.0),
        );

        // Add docked panel
        owner.add_panel(
            PanelId::new("inspector"),
            PanelGeometry::docked(0.0, 0.0, 300.0, 600.0, DockPosition::Right),
        );

        let view = WorkspaceDockingSummaryView::build(&owner);

        assert_eq!(view.total_panels, 2);
        assert_eq!(view.docked_panels, 1);
        assert_eq!(view.floating_panels, 1);
        assert!(view.docking_enabled);
    }

    #[test]
    fn test_query_is_read_only() {
        // This test verifies that queries take &Owner, not &mut Owner
        let owner = WorkspaceOwner::new();

        // These calls should compile with &owner (not &mut owner)
        let _view2 = WorkspacePanelPositionsView::build(&owner);
        let _view3 = WorkspaceDockingSummaryView::build(&owner);

        // Owner should be unchanged
        assert_eq!(owner.open_panel_ids.len(), 0);
    }
}
