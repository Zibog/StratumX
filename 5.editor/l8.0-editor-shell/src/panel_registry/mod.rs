//! Panel Registry
//!
//! The Panel Registry manages available panels, their lifecycle, and instantiation.
//! Owned by l8.0-editor-shell as the shell-owned panel catalog/registry.

mod registry;
mod types;
mod workspace;

// Re-export all public types and functions for backward compatibility
pub use registry::PanelRegistry;
pub use types::{
    DockPosition, DockingConfig, PanelDefinition, PanelDependency, PanelGeometry, PanelLifecycle,
    SplitConfig, SplitDirection, WorkspaceState,
};
