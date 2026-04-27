//! Residency resource control and location affinity
//! **Owner**: engine_residency_control — Resource residency and locality enforcement

pub mod types;
pub mod runtime;
pub mod validation;
pub mod queries;
pub mod exports;

pub use exports::*;
