//! Compatibility verdict type definitions
//!
//! Defines verdict states, reason codes, and typed rejection payloads
//! for SDK compatibility decisions.

use serde::{Deserialize, Serialize};

use super::super::profiles::CompatProfileId;
use super::super::versions::{BridgeVersion, CompatDomain, CompatVersionId};
use super::super::Capability;

/// Opaque identifier for a compatibility verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CompatVerdictId(pub u64);

/// The state of a compatibility verdict.
///
/// Cadence: produced once per negotiation cycle.
/// Delivery guarantee: synchronous — verdict blocks further communication if Deny.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompatibilityVerdictState {
    Allow,
    Warn,
    Deny,
}

/// Reason codes explaining why a verdict was reached.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompatibilityReasonCode {
    VersionTooOld,
    MissingCapability(Capability),
    Supported,
    DomainIncompatible(CompatDomain, DomainRejectionReason),
    ProfileFallbackApplied(CompatProfileId),
}

/// Domain-specific rejection reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DomainRejectionReason {
    VersionBelowMinimum { minimum: BridgeVersion },
    MissingRequiredCapability(Capability),
    SchemaGenerationMismatch { expected: u64, got: u64 },
    DeniedByPolicy,
}

/// A full compatibility verdict with reasons.
///
/// Cadence: produced once per negotiation cycle.
/// Delivery guarantee: synchronous — verdict must be accepted before proceeding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityVerdict {
    pub compat_verdict_id: CompatVerdictId,
    pub evaluated_profile_id: CompatProfileId,
    pub evaluated_version_id: CompatVersionId,
    pub verdict_state: CompatibilityVerdictState,
    pub reason_code_set: Vec<CompatibilityReasonCode>,
}

impl CompatibilityVerdict {
    pub fn allow(
        verdict_id: CompatVerdictId,
        profile_id: CompatProfileId,
        version_id: CompatVersionId,
    ) -> Self {
        Self {
            compat_verdict_id: verdict_id,
            evaluated_profile_id: profile_id,
            evaluated_version_id: version_id,
            verdict_state: CompatibilityVerdictState::Allow,
            reason_code_set: vec![CompatibilityReasonCode::Supported],
        }
    }

    pub fn deny(
        verdict_id: CompatVerdictId,
        profile_id: CompatProfileId,
        version_id: CompatVersionId,
        reasons: Vec<CompatibilityReasonCode>,
    ) -> Self {
        Self {
            compat_verdict_id: verdict_id,
            evaluated_profile_id: profile_id,
            evaluated_version_id: version_id,
            verdict_state: CompatibilityVerdictState::Deny,
            reason_code_set: reasons,
        }
    }

    pub fn warn(
        verdict_id: CompatVerdictId,
        profile_id: CompatProfileId,
        version_id: CompatVersionId,
        warnings: Vec<CompatibilityReasonCode>,
    ) -> Self {
        Self {
            compat_verdict_id: verdict_id,
            evaluated_profile_id: profile_id,
            evaluated_version_id: version_id,
            verdict_state: CompatibilityVerdictState::Warn,
            reason_code_set: warnings,
        }
    }

    pub fn is_allowed(&self) -> bool {
        matches!(
            self.verdict_state,
            CompatibilityVerdictState::Allow | CompatibilityVerdictState::Warn
        )
    }
}

/// Legality verdict for transport-level checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalityVerdict {
    Legal,
    Illegal,
}

/// Typed legality rejection payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegalityRejection {
    pub verdict: LegalityVerdict,
    pub reason: LegalityRejectionReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalityRejectionReason {
    PayloadTooLarge { size: usize, max: usize },
    MetricNotAllowed { policy_name: String },
    ArtifactRefNotAllowed { policy_name: String },
    FramingViolation { expected: String, got: String },
    InvalidInput { field: String, message: String },
}

/// Compatibility ladder: a sorted list of version facts that can be queried
/// for upgrade/downgrade paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityLadder {
    pub rungs: Vec<crate::model::versions::VersionFact>,
}

impl CompatibilityLadder {
    pub fn new() -> Self {
        Self { rungs: Vec::new() }
    }

    pub fn with_rung(mut self, fact: crate::model::versions::VersionFact) -> Self {
        self.rungs.push(fact);
        self
    }

    /// Sort rungs by version (ascending).
    pub fn sort(&mut self) {
        self.rungs
            .sort_by(|a, b| a.wire_version.cmp(&b.wire_version));
    }

    /// Find the highest supported version <= target.
    pub fn highest_supported_at_or_below(&self, target: &BridgeVersion) -> Option<&BridgeVersion> {
        self.rungs
            .iter()
            .filter(|r| {
                matches!(
                    r.support_state,
                    crate::model::versions::SupportState::Supported
                ) && !r.wire_version.is_newer_than(target)
            })
            .max_by(|a, b| a.wire_version.cmp(&b.wire_version))
            .map(|r| &r.wire_version)
    }

    /// Check if a version is supported on this ladder.
    pub fn is_supported(&self, version: &BridgeVersion) -> bool {
        self.rungs.iter().any(|r| {
            r.wire_version == *version
                && !matches!(
                    r.support_state,
                    crate::model::versions::SupportState::Denied
                )
        })
    }
}

impl Default for CompatibilityLadder {
    fn default() -> Self {
        Self::new()
    }
}
