//! Command Palette State and catalog.

pub mod filters;
pub mod history;
pub mod query;
pub mod selection;

// Re-export all public items to preserve API compatibility
pub use filters::{filter_command_palette, CommandPaletteEntry, COMMAND_PALETTE_ENTRIES};
pub use query::CommandPaletteState;
