//! Public API for compatibility checking
//!
//! Provides negotiation helpers, compatibility verification, and downgrade path utilities
//! used by tooling and editor layers to establish SDK bridge contracts.

use crate::model::capabilities::{CapabilityFact, CapabilityRejection, DomainCapabilityFact};
use crate::model::profiles::CompatProfileId;
use crate::model::profiles::ProfileFact;
use crate::model::verdicts::{
    negotiate_downgrade, CompatVerdictId, CompatibilityLadder, CompatibilityVerdict,
};
use crate::model::versions::{
    BridgeVersion, CompatDomain, CompatVersionId, VersionRejection, VersionRejectionReason,
};
use crate::model::{Capability, CapabilityDefaultState, CompatCapabilityId, CompatibilityProfile};

/// Result of a compatibility negotiation attempt.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NegotiationResult {
    pub verdict: CompatibilityVerdict,
    pub agreed_version: Option<BridgeVersion>,
    pub version_rejection: Option<VersionRejection>,
    pub capability_rejection: Option<crate::model::capabilities::CapabilityRejection>,
}

/// Negotiate compatibility between two SDK peers.
///
/// Evaluates the presented version and capabilities against a baseline profile
/// and returns a typed negotiation result with rejection details if applicable.
pub fn negotiate_compatibility(
    presented_version: BridgeVersion,
    presented_capabilities: &[Capability],
    profile: CompatibilityProfile,
    baseline_version: BridgeVersion,
    ladder: &CompatibilityLadder,
) -> NegotiationResult {
    use crate::model::verdicts::CompatibilityReasonCode;

    // Check version compatibility
    let version_rejection = if presented_version.major < baseline_version.major {
        Some(VersionRejection {
            presented_version,
            required_minimum: baseline_version,
            reason: VersionRejectionReason::VersionTooOld {
                presented: presented_version,
                minimum: baseline_version,
            },
        })
    } else if !ladder.is_supported(&presented_version) {
        // Try downgrade
        let downgrade = negotiate_downgrade(
            presented_version,
            &ladder
                .rungs
                .iter()
                .map(|r| r.wire_version)
                .collect::<Vec<_>>(),
        );
        downgrade.map(|downgraded| VersionRejection {
            presented_version,
            required_minimum: baseline_version,
            reason: VersionRejectionReason::VersionTooOld {
                presented: presented_version,
                minimum: downgraded,
            },
        })
    } else {
        None
    };

    // Check capability compatibility
    let required = profile.required_capabilities();
    let missing: Vec<Capability> = required
        .iter()
        .filter(|c| !presented_capabilities.contains(c))
        .copied()
        .collect();

    let capability_rejection = if missing.is_empty() {
        None
    } else {
        Some(CapabilityRejection {
            missing_capabilities: missing,
            domain: None,
            message: "missing required capabilities for profile".to_string(),
        })
    };

    // Build verdict
    let verdict = if version_rejection.is_some() || capability_rejection.is_some() {
        let mut reasons = Vec::new();
        if version_rejection.is_some() {
            reasons.push(CompatibilityReasonCode::VersionTooOld);
        }
        if let Some(ref cap_rej) = capability_rejection {
            for cap in &cap_rej.missing_capabilities {
                reasons.push(CompatibilityReasonCode::MissingCapability(*cap));
            }
        }
        CompatibilityVerdict::deny(
            CompatVerdictId(0),
            CompatProfileId(profile as u64),
            CompatVersionId(0),
            reasons,
        )
    } else {
        CompatibilityVerdict::allow(
            CompatVerdictId(0),
            CompatProfileId(profile as u64),
            CompatVersionId(0),
        )
    };

    let agreed_version = if verdict.is_allowed() {
        Some(presented_version)
    } else {
        negotiate_downgrade(
            presented_version,
            &ladder
                .rungs
                .iter()
                .map(|r| r.wire_version)
                .collect::<Vec<_>>(),
        )
    };

    NegotiationResult {
        verdict,
        agreed_version,
        version_rejection,
        capability_rejection,
    }
}

/// Check if a set of capabilities satisfies the requirements for a specific domain.
pub fn check_domain_capabilities(
    domain: CompatDomain,
    presented: &[Capability],
    registry: &crate::model::capabilities::CapabilityRegistry,
) -> Result<(), crate::model::capabilities::CapabilityRejection> {
    registry.check_domain_capabilities(domain, presented)
}

/// Build a default capability registry with all 5 base capabilities.
pub fn default_capability_registry() -> crate::model::capabilities::CapabilityRegistry {
    let mut registry = crate::model::capabilities::CapabilityRegistry::new();
    for (i, cap) in Capability::ALL.iter().enumerate() {
        registry = registry.with_capability(CapabilityFact::new(
            CompatCapabilityId(i as u64),
            cap.name(),
            CapabilityDefaultState::Allowed,
        ));
    }
    // Add domain-specific requirements
    for domain in CompatDomain::ALL {
        let required = match domain {
            CompatDomain::World => vec![Capability::Snapshots, Capability::Controls],
            CompatDomain::Terrain => vec![Capability::Snapshots, Capability::Observations],
            CompatDomain::Material => vec![Capability::Snapshots, Capability::Controls],
            CompatDomain::Destruction => vec![Capability::Snapshots, Capability::Observations],
            CompatDomain::Weather => vec![Capability::Observations],
            CompatDomain::Audio => vec![Capability::Observations],
            CompatDomain::Animation => vec![Capability::Snapshots, Capability::Controls],
            CompatDomain::Living => vec![Capability::Observations, Capability::Controls],
            CompatDomain::Tactics => vec![Capability::Snapshots, Capability::Metrics],
            CompatDomain::Society => vec![Capability::Observations],
            CompatDomain::Ecology => vec![Capability::Observations],
            CompatDomain::Wounds => vec![Capability::Snapshots, Capability::Observations],
            CompatDomain::Photoreal => vec![Capability::Snapshots, Capability::ArtifactRefs],
            CompatDomain::Diagnostics => vec![Capability::Metrics, Capability::Observations],
            CompatDomain::Runtime => vec![Capability::Snapshots, Capability::Controls],
            CompatDomain::Build => vec![Capability::ArtifactRefs, Capability::Snapshots],
            CompatDomain::Capture => vec![Capability::Snapshots, Capability::Observations],
            CompatDomain::Proof => vec![Capability::Snapshots, Capability::ArtifactRefs],
        };
        registry = registry.with_domain_capabilities(DomainCapabilityFact {
            domain: *domain,
            required_capabilities: required,
            optional_capabilities: Vec::new(),
        });
    }
    registry
}

/// Build a default profile registry with all 4 profiles.
pub fn default_profile_registry() -> crate::model::profiles::ProfileRegistry {
    use crate::model::profiles::ProfileRegistry;
    let mut registry = ProfileRegistry::new();
    for (i, profile) in CompatibilityProfile::ALL.iter().enumerate() {
        registry = registry.with_profile(ProfileFact::new(
            CompatProfileId(i as u64),
            profile.name(),
            Vec::new(),
            profile.required_capabilities().to_vec(),
        ));
    }
    registry
}
