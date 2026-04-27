//! l1-runtime-kernel/runtime: Core simulation runtime kernel
//!
//! Provides the main simulation tick loop, frame synchronization, and
//! scheduling for all runtime behaviors and substrate updates.

mod types;
mod runtime;
mod validation;
mod queries;
mod exports;

pub use exports::*;
