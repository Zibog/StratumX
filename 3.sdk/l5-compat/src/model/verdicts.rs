//! Compatibility verdict types
//! Merged from l5.7-compat-verdicts
//!
//! Narrow re-export module for controlled transition after monolith split.

pub mod helpers;
pub mod types;

pub use helpers::negotiate_downgrade;
pub use types::{
    CompatVerdictId, CompatibilityLadder, CompatibilityReasonCode, CompatibilityVerdict,
    CompatibilityVerdictState, DomainRejectionReason, LegalityRejection, LegalityRejectionReason,
    LegalityVerdict,
};
