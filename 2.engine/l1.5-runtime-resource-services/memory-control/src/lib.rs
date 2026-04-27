//! Memory resource control and allocation governance
//! **Owner**: engine_memory_control — Memory allocation and budget enforcement

pub mod types;
pub mod runtime;
pub mod validation;
pub mod queries;
pub mod exports;

pub use exports::*;
