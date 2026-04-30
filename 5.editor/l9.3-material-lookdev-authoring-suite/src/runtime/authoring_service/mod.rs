//! Material Authoring Service
//!
//! Manages material profiles, entity bindings, and diagnostics.

pub mod service;
mod types;

// Re-export all public types and functions for backward compatibility
pub use service::MaterialAuthoringService;
pub use types::{CheapnessReport, MaterialDiagnostics};
