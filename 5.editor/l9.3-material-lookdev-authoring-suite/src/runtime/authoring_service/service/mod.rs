//! Material Authoring Service
//!
//! Manages material profiles, entity bindings, and diagnostics.
//!
//! This module is organized by role:
//! - `session`: Session lifecycle and state management
//! - `commands`: Command handlers for material property binding
//! - `cache`: Coverage tracking and registry field binding
//! - `preview`: Material preview and texture assignment
//! - `validation`: Material validation logic
//! - `diagnostics`: Diagnostic information reporting

mod cache;
mod commands;
mod diagnostics;
mod preview;
mod session;
mod validation;

// Re-export the main service struct for backward compatibility
pub use session::MaterialAuthoringService;
