//! Persistence query views

use crate::owners::workspace_owner::WorkspaceOwner;
use crate::queries::ReadModel;
use std::path::PathBuf;

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

impl ReadModel<WorkspaceOwner> for ContentBrowserFilteredView {
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
