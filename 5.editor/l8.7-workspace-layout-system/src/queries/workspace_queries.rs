//! Workspace query views
//!
//! Read-only views of WorkspaceOwner state implementing ReadModel trait.

use crate::owners::workspace_owner::{
    DockPosition, DockingConfig, PanelGeometry, PanelId, WorkspaceOwner,
};
use crate::queries::ReadModel;
use std::collections::HashMap;
use std::path::PathBuf;

/// Workspace layout view
///
/// Read-only view of workspace panel layout and configuration.
#[derive(Debug, Clone)]
pub struct WorkspaceLayoutView {
    pub open_panels: Vec<PanelId>,
    pub focused_panel: Option<PanelId>,
    pub panel_count: usize,
}

impl ReadModel<WorkspaceOwner, WorkspaceLayoutView> for WorkspaceLayoutView {
    fn build(owner: &WorkspaceOwner) -> Self {
        Self {
            open_panels: owner.open_panel_ids.clone(),
            focused_panel: owner.focused_panel.clone(),
            panel_count: owner.open_panel_ids.len(),
        }
    }
}

/// Workspace panel positions view
///
/// Read-only view of panel positions and geometries.
#[derive(Debug, Clone)]
pub struct WorkspacePanelPositionsView {
    pub panel_positions: HashMap<PanelId, PanelGeometry>,
    pub docking_configuration: DockingConfig,
}

impl ReadModel<WorkspaceOwner, WorkspacePanelPositionsView> for WorkspacePanelPositionsView {
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

impl ReadModel<WorkspaceOwner, WorkspaceDockingSummaryView> for WorkspaceDockingSummaryView {
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

/// Content browser filtered view
///
/// Read-only view of content browser items filtered by search query.
#[derive(Debug, Clone)]
pub struct ContentBrowserFilteredView {
    pub filter: String,
    pub items: Vec<ContentBrowserItem>,
    pub item_count: usize,
}

/// Content browser item information
#[derive(Debug, Clone)]
pub struct ContentBrowserItem {
    pub name: String,
    pub path: PathBuf,
    pub item_type: ContentItemType,
}

/// Content item type classification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentItemType {
    Asset,
    Folder,
    World,
    Material,
    Terrain,
    Audio,
    Other,
}

impl ReadModel<WorkspaceOwner, ContentBrowserFilteredView> for ContentBrowserFilteredView {
    fn build(_owner: &WorkspaceOwner) -> Self {
        // WorkspaceOwner does not currently carry content inventory state.
        // Expose that absence as an explicit empty read model.
        let filter = String::new();
        let items = Vec::new();

        Self {
            filter,
            item_count: items.len(),
            items,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_layout_view() {
        let mut owner = WorkspaceOwner::new();

        let panel1 = PanelId::new("viewport");
        let panel2 = PanelId::new("inspector");

        owner.add_open_panel(panel1.clone());
        owner.add_open_panel(panel2.clone());
        owner.set_focused_panel(Some(panel1.clone()));

        let view = WorkspaceLayoutView::build(&owner);

        assert_eq!(view.panel_count, 2);
        assert_eq!(view.open_panels.len(), 2);
        assert_eq!(view.focused_panel, Some(panel1));
    }

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
        let _view1 = WorkspaceLayoutView::build(&owner);
        let _view2 = WorkspacePanelPositionsView::build(&owner);
        let _view3 = WorkspaceDockingSummaryView::build(&owner);

        // Owner should be unchanged
        assert_eq!(owner.open_panel_ids.len(), 0);
    }

    #[test]
    fn test_content_browser_filtered_view() {
        let owner = WorkspaceOwner::new();

        // Build filtered view
        let view = ContentBrowserFilteredView::build(&owner);

        // Verify it's a read-only view
        assert_eq!(view.item_count, 0);
        assert_eq!(view.items.len(), 0);
    }

    #[test]
    fn test_content_browser_query_is_read_only() {
        // This test verifies that ContentBrowserFilteredView takes &Owner, not &mut Owner
        let owner = WorkspaceOwner::new();

        // This call should compile with &owner (not &mut owner)
        let _view = ContentBrowserFilteredView::build(&owner);

        // Owner should be unchanged
        assert_eq!(owner.open_panel_ids.len(), 0);
    }
}
