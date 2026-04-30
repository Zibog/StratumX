//! World Session Service
//!
//! Domain service for world lifecycle management including world open/close/save operations.
//! Abstraction Level: L3 (World Operations)
//!
//! This module is organized by role:
//! - `session`: Session lifecycle and state management
//! - `commands`: Command handlers for world open/close/save operations
//! - `queries`: Query handlers for world summary information
//! - `diagnostics`: Diagnostics-related functionality (minimal)
//! - `persistence`: Persistence-related functionality (minimal)

mod commands;
mod diagnostics;
mod persistence;
mod queries;
mod session;

// Re-export the main service struct and DTO for backward compatibility
pub use queries::WorldSummaryDto;
pub use session::WorldSessionService;
