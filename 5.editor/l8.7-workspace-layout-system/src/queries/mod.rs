//! Query views for workspace layout system

pub mod workspace_queries;

// Re-export ReadModel trait from stratumx_editor_state_containers
// This allows internal modules to use crate::queries::ReadModel
pub use stratumx_editor_state_containers::queries::ReadModel;
