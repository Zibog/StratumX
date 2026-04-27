//! Command Spine Module
//!
//! This module has been decomposed into three focused submodules as part of
//! Phase 6 architectural refactoring (Task 22):
//!
//! - **spine_core.rs** - Core spine logic with CommandSpine struct and execution methods
//! - **spine_routing.rs** - Command routing logic for executor dispatch
//! - **spine_lifecycle.rs** - Initialization and shutdown lifecycle management
//!
//! Each module is under 150 lines and has a single clear responsibility.

mod spine_core;
mod spine_lifecycle;
mod spine_routing;

// Re-export core types
pub use spine_core::CommandSpine;
pub use spine_lifecycle::{build_registry, create_executors, initialize, shutdown};
pub use spine_routing::{ExecutorId, SpineRouting};
