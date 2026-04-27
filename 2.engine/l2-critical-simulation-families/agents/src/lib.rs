//! l2-critical-simulation-families/agents: NPC and agent simulation family
//!
//! Provides creature ecology, NPC behavior trees, faction systems, personality
//! models, tactical squad coordination, and crime escalation mechanics.

mod types;
mod runtime;
mod validation;
mod queries;
mod exports;

pub use exports::*;
