//! l4-startup: Thin engine assembly and initialization
//!
//! Provides startup orchestration, world initialization, asset bootstrapping,
//! and assembly of core engine systems from lower foundation layers.

mod types;
mod runtime;
mod validation;
mod queries;
mod exports;

pub use exports::*;
