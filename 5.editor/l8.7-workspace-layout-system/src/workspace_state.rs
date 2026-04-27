//! Workspace State - Re-exports from owners module
//!
//! This module provides backward compatibility by re-exporting types from the owners module.

pub use crate::owners::workspace_owner::{
    DockPosition, DockingConfig, PanelGeometry, PanelId, WorkspaceOwner as WorkspaceState,
};
