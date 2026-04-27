//! SDK Compatibility Layer
//!
//! Consolidated compatibility checking for versions, capabilities, profiles, and verdicts.
//! Per Phase 6 canon: merged from l5.4-compat-versions, l5.5-compat-capabilities,
//! l5.6-compat-profiles, l5.7-compat-verdicts.
//!
//! This crate defines the compatibility negotiation contract for all 18 SDK domain families:
//! world, terrain, material, destruction, weather, audio, animation, living, tactics, society,
//! ecology, wounds, photoreal, diagnostics, runtime, build, capture, proof.

pub mod api;
pub mod model;

// Re-export model types
pub use model::*;

// Re-export API helpers
pub use api::*;

pub const CANONICAL_LEVEL: &str = "l5-compat";
