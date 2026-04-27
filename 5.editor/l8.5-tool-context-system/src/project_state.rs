//! Project State - Re-exports from owners module
//!
//! This module provides backward compatibility by re-exporting types from the owners module.

pub use crate::owners::project_owner::{
    ContentSnapshot, ProjectIdentity, ProjectOwner as ProjectState, ProjectStateEvent,
    StateEventCallback, WorkspaceIdentity,
};
