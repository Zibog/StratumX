//! Compatibility version type definitions
//!
//! Defines version compatibility markers, support states, and version facts
//! for SDK compatibility negotiation.

use serde::{Deserialize, Serialize};

/// Opaque identifier for a compatibility version entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CompatVersionId(pub u64);

/// Semantic wire version used at the SDK bridge boundary.
///
/// Cadence: evaluated once per session negotiation.
/// Delivery guarantee: synchronous — both sides must agree before proceeding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BridgeVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl BridgeVersion {
    /// Construct a new bridge version.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns true if this version is strictly greater than another.
    pub fn is_newer_than(&self, other: &BridgeVersion) -> bool {
        (self.major, self.minor, self.patch) > (other.major, other.minor, other.patch)
    }

    /// Returns true if this version is compatible with another (same major, equal or newer minor).
    pub fn is_compatible_with(&self, other: &BridgeVersion) -> bool {
        self.major == other.major && (self.minor, self.patch) >= (other.minor, other.patch)
    }
}

/// Support state for a version entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportState {
    /// Fully supported.
    Supported,
    /// Functional but deprecated; warns but allows.
    Deprecated,
    /// Explicitly denied; connection refused.
    Denied,
}

/// A fact about a specific wire version.
///
/// Cadence: loaded once during startup negotiation.
/// Delivery guarantee: static — part of the compatibility registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionFact {
    pub compat_version_id: CompatVersionId,
    pub wire_version: BridgeVersion,
    pub schema_generation: u64,
    pub supersedes_version_id: Option<CompatVersionId>,
    pub support_state: SupportState,
}

impl VersionFact {
    pub fn new(
        compat_version_id: CompatVersionId,
        wire_version: BridgeVersion,
        schema_generation: u64,
    ) -> Self {
        Self {
            compat_version_id,
            wire_version,
            schema_generation,
            supersedes_version_id: None,
            support_state: SupportState::Supported,
        }
    }

    pub fn with_supersedes(mut self, supersedes: CompatVersionId) -> Self {
        self.supersedes_version_id = Some(supersedes);
        self
    }

    pub fn with_support_state(mut self, state: SupportState) -> Self {
        self.support_state = state;
        self
    }
}

/// Typed version rejection payload.
///
/// Replaces generic string errors when a version is rejected.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionRejection {
    pub presented_version: BridgeVersion,
    pub required_minimum: BridgeVersion,
    pub reason: VersionRejectionReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionRejectionReason {
    MajorMismatch {
        expected: u16,
        got: u16,
    },
    VersionTooOld {
        presented: BridgeVersion,
        minimum: BridgeVersion,
    },
    VersionDenied {
        presented: BridgeVersion,
    },
    SchemaGenerationMismatch {
        presented: u64,
        required: u64,
    },
}
