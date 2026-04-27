//! Compatibility model types

pub mod capabilities;
pub mod profiles;
pub mod verdicts;
pub mod versions;

// Re-export all model types
pub use capabilities::*;
pub use profiles::*;
pub use verdicts::{negotiate_downgrade, CompatVerdictId, CompatibilityLadder, CompatibilityReasonCode, CompatibilityVerdict, CompatibilityVerdictState, DomainRejectionReason, LegalityRejection, LegalityRejectionReason, LegalityVerdict};
pub use versions::{CompatDomain, CompatVersionId, BridgeVersion, DomainVersionMarker, SupportState, VersionFact, VersionRejection, VersionRejectionReason};
