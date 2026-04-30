//! Workspace query views
//!
//! Read-only views of WorkspaceOwner state implementing ReadModel trait.

mod focus_queries;
mod layout_queries;
mod panel_queries;
mod persistence_queries;

pub use focus_queries::*;
pub use layout_queries::*;
pub use panel_queries::*;
pub use persistence_queries::*;
