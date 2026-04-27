//! StratumX Editor - L9.1 Scene Entity Authoring Suite
//!
//! Provides scene entity authoring capabilities including entity creation,
//! hierarchy management, component editing, and transform manipulation.

pub mod api;
pub mod entity_state;
pub mod hierarchy;
pub mod transform;
pub mod component_editor;
pub mod selection;
mod model;
mod runtime;
mod validation;

// Re-export public API
pub use api::*;
pub use entity_state::*;
pub use hierarchy::*;
pub use transform::*;
pub use component_editor::*;
pub use selection::*;
