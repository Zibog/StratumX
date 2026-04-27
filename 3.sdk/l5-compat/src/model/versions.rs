//! Compatibility version types
//! Merged from l5.4-compat-versions
//!
//! Narrow re-export module for controlled transition after monolith split.

pub mod domains;
pub mod types;

pub use domains::{CompatDomain, DomainVersionMarker};
pub use types::{
    BridgeVersion, CompatVersionId, SupportState, VersionFact, VersionRejection,
    VersionRejectionReason,
};
