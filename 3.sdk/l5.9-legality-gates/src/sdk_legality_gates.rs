//! SDK Legality Gates — legality decisions and guard results.
//!
//! Narrow re-export module for controlled transition after monolith split.

pub mod command_gates;
pub mod types;
pub mod validation;

pub use command_gates::*;
pub use types::{GateDenyReason, LegalityGate, LegalityGateId, LegalityGateRegistry};
pub use validation::{compatibility_verdict, default_legality_gate_registry, transport_legality};

pub const CANONICAL_LEVEL: &str = "l5.9-legality-gates";
