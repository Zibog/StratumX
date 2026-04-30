//! Layout query views

use crate::owners::workspace_owner::{PanelId, WorkspaceOwner};
use crate::queries::ReadModel;

/// Workspace layout view
///
/// Read-only view of workspace panel layout and configuration.
#[derive(Debug, Clone)]
pub struct WorkspaceLayoutView {
    pub open_panels: Vec<PanelId>,
    pub focused_panel: Option<PanelId>,
    pub panel_count: usize,
}

impl ReadModel<WorkspaceOwner> for WorkspaceLayoutView {
    fn build(owner: &WorkspaceOwner) -> Self {
        Self {
            open_panels: owner.open_panel_ids.clone(),
            focused_panel: owner.focused_panel.clone(),
            panel_count: owner.open_panel_ids.len(),
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
    fn test_query_is_read_only() {
        // This test verifies that queries take &Owner, not &mut Owner
        let owner = WorkspaceOwner::new();

        // These calls should compile with &owner (not &mut owner)
        let _view1 = WorkspaceLayoutView::build(&owner);

        // Owner should be unchanged
        assert_eq!(owner.open_panel_ids.len(), 0);
    }
}
