//! l0-world-truth: World state authority and ground truth
//!
//! Provides the canonical world state substrate as the single source of truth
//! for all simulation state and entity data.
//!
//! ## Crate Invariants
//! - World state is the single source of truth (no duplicated state)
//! - World state updates are atomic and consistent
//! - Entity existence is tracked accurately
//! - World boundaries are enforced

mod types;
mod runtime;
mod validation;
mod queries;
mod exports;

pub use exports::*;
