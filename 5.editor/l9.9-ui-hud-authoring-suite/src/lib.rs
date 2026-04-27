//! StratumX Editor - L9.9 UI HUD Authoring Suite
//!
//! Provides UI and HUD authoring capabilities including widget creation,
//! layout management, styling, and event handling.

pub mod api;
pub mod widget;
pub mod layout;
pub mod style;
pub mod canvas;
mod model;
mod runtime;
mod validation;

// Re-export public API
pub use api::*;
pub use widget::*;
pub use layout::*;
pub use style::*;
pub use canvas::*;
